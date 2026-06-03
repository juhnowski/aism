use leptos::prelude::*;
use gloo_net::http::Request;
use crate::es_logger::log_to_es;

#[derive(Clone)]
struct Message {
    is_user: bool,
    text: String,
}

#[component]
pub fn FileChatTab() -> impl IntoView {
    let (messages, set_messages) = signal(Vec::<Message>::new());
    let (input_text, set_input_text) = signal(String::new());
    let (file_name, set_file_name) = signal(String::new());
    let (file_content, set_file_content) = signal(String::new());
    let (loading, set_loading) = signal(false);

    // Безопасный сбор имени файла через стандартный макрос Leptos
    let on_file_change = move |ev: leptos::ev::Event| {
        let full_path = event_target_value(&ev);
        if !full_path.is_empty() {
            // Извлекаем только имя файла из пути (убираем C:\fakepath\)
            let name = full_path.split('\\').last().unwrap_or(&full_path).to_string();
            set_file_name.set(name.clone());

            log_to_es("File Chat", &format!("Прикреплен контекст файла: {}", name), None);
            
            // Записываем имя файла как контекст для бэкенда
            set_file_content.set(format!("[Файл контекста: {}]", name));
        }
    };

    let send_message = move || {
        let user_query = input_text.get().trim().to_string();
        if user_query.is_empty() { return; }

        set_loading.set(true);
        set_messages.update(|msgs| msgs.push(Message { is_user: true, text: user_query.clone() }));
        set_input_text.set(String::new());

        let attached_file = file_name.get();
        
        // Передаем промпт. Настоящие логи СХД бэкенд заберет по имени файла
        let prompt = if !attached_file.is_empty() {
            format!(
                "Контекст: Пользователь прикрепил лог СХД '{}'. Проанализируй его и ответь на вопрос: {}", 
                attached_file, user_query
            )
        } else {
            user_query.clone()
        };

        log_to_es("File Chat", &user_query, Some(format!("Файл: {}", attached_file)));

        leptos::task::spawn_local(async move {
            let body = serde_json::json!({
                "model": "deepseek-coder-v2",
                "messages": [{ "role": "user", "content": user_query }],
                "stream": false,
                "file_name": file_name.get() // <-- Передаем имя файла бэкенду!
            });

            if let Ok(req) = Request::post("/api/chat").json(&body) {
                if let Ok(resp) = req.send().await {
                    if let Ok(json) = resp.json::<serde_json::Value>().await {
                        if let Some(content) = json.pointer("/message/content").and_then(|v| v.as_str()) {
                            set_messages.update(|msgs| msgs.push(Message { is_user: false, text: content.to_string() }));
                        }
                    }
                }
            }
            set_loading.set(false);
        });
    };

    view! {
        <div style="display: flex; flex-direction: column; height: calc(100vh - 140px); max-width: 800px; margin: 0 auto; background: white; border-radius: 8px; box-shadow: 0 4px 12px rgba(0,0,0,0.05);">
            <div style="padding: 15px 20px; background: #f8fafc; border-bottom: 1px solid #e2e8f0; border-radius: 8px 8px 0 0; display: flex; align-items: center; gap: 15px;">
                <strong style="color: #2d3748;">"Контекст файла:"</strong>
                <input type="file" on:change=on_file_change style="font-size: 14px;" />
                {move || if !file_name.get().is_empty() {
                    view! { <span style="color: #2ecc71; font-size: 13px; font-weight: bold;">{format!("✓ {}", file_name.get())}</span> }.into_any()
                } else {
                    view! { <span style="color: #718096; font-size: 13px;">"Файл не выбран"</span> }.into_any()
                }}
            </div>

            <div style="flex-grow: 1; overflow-y: auto; padding: 20px; display: flex; flex-direction: column; gap: 15px;">
                {move || messages.get().into_iter().map(|msg| {
                    let bg = if msg.is_user { "#3498db" } else { "#f1f0f0" };
                    let color = if msg.is_user { "white" } else { "black" };
                    let align = if msg.is_user { "flex-end" } else { "flex-start" };
                    view! {
                        <div style=format!("align-self: {}; max-width: 75%; padding: 12px 16px; border-radius: 12px; background: {}; color: {}; white-space: pre-wrap; line-height: 1.5;", align, bg, color)>
                            {msg.text}
                        </div>
                    }
                }).collect_view()}
                
                {move || loading.get().then(|| view! {
                    <div style="align-self: flex-start; background: #f1f0f0; padding: 12px 16px; border-radius: 12px; color: #718096; font-style: italic;">
                        "DeepSeek анализирует лог..."
                    </div>
                })}
            </div>

            <div style="padding: 20px; border-top: 1px solid #eee; display: flex; gap: 10px; background: white; border-radius: 0 0 8px 8px;">
                <input type="text" 
                    placeholder=move || if file_name.get().is_empty() { "Задайте вопрос или прикрепите файл..." } else { "Задайте вопрос по контексту файла..." }
                    prop:value=move || input_text.get()
                    disabled=move || loading.get()
                    on:input=move |ev| set_input_text.set(event_target_value(&ev))
                    on:keydown=move |ev| if ev.key() == "Enter" { send_message(); }
                    style="flex-grow: 1; padding: 12px; border: 1px solid #ddd; border-radius: 6px; font-size: 14px;"
                />
                <button 
                    on:click=move |_| send_message() 
                    disabled=move || loading.get()
                    style="background: #3498db; color: white; border: none; padding: 0 25px; border-radius: 6px; cursor: pointer; font-weight: bold;"
                >
                    "Отправить"
                </button>
            </div>
        </div>
    }
}
