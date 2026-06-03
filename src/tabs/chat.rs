use leptos::prelude::*;
use gloo_net::http::Request;
use crate::es_logger::log_to_es;

#[derive(Clone)]
pub struct Message {
    pub is_user: bool,
    pub text: String,
}

#[component]
pub fn ChatTab() -> impl IntoView {
    // Сигналы для хранения истории сообщений и текущего ввода
    let (messages, set_messages) = signal(Vec::<Message>::new());
    let (input_text, set_input_text) = signal(String::new());
    // Сигнал анимации ожидания ответа ИИ
    let (loading, set_loading) = signal(false);

    let send_message = move || {
        let text = input_text.get().trim().to_string();
        if text.is_empty() { return; }

        set_loading.set(true);

        // 1. Логируем действие пользователя в общую базу OpenSearch/ES
        log_to_es("Chat", &text, None);

        // 2. Добавляем реплику пользователя в ленту чата
        set_messages.update(|msgs| msgs.push(Message { is_user: true, text: text.clone() }));
        set_input_text.set(String::new());

        // 3. Отправляем асинхронный запрос к нашему старому бэкенду (прокси Ollama)
        leptos::task::spawn_local(async move {
            let body = serde_json::json!({
                "model": "deepseek-coder-v2",
                "messages": [{ "role": "user", "content": text }],
                "stream": false
            });

            // Запрос идет на /api/chat. Nginx перенаправит его на старый бэк (порт 3100 или 3101)
            match Request::post("/api/chat").json(&body) {
                Ok(request) => {
                    match request.send().await {
                        Ok(resp) => {
                            if let Ok(json) = resp.json::<serde_json::Value>().await {
                                // Извлекаем текст ответа ИИ по JSON-пути /message/content
                                if let Some(content) = json.pointer("/message/content").and_then(|v| v.as_str()) {
                                    set_messages.update(|msgs| msgs.push(Message { is_user: false, text: content.to_string() }));
                                } else {
                                    set_messages.update(|msgs| msgs.push(Message { is_user: false, text: "Ошибка: не удалось распознать структуру ответа Ollama.".to_string() }));
                                }
                            } else {
                                set_messages.update(|msgs| msgs.push(Message { is_user: false, text: "Ошибка: сервер вернул некорректный JSON.".to_string() }));
                            }
                        }
                        Err(e) => {
                            set_messages.update(|msgs| msgs.push(Message { is_user: false, text: format!("Ошибка сети при обращении к бэкенду: {:?}", e) }));
                        }
                    }
                }
                Err(e) => {
                    set_messages.update(|msgs| msgs.push(Message { is_user: false, text: format!("Ошибка сериализации запроса: {:?}", e) }));
                }
            }
            
            set_loading.set(false);
        });
    };

    view! {
        <div style="display: flex; flex-direction: column; height: calc(100vh - 140px); max-width: 800px; margin: 0 auto; background: white; border-radius: 8px; box-shadow: 0 4px 12px rgba(0,0,0,0.05);">
            
            // Лента сообщений чата (новые уходят вниз, старые скроллятся вверх)
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

                // Индикатор того, что ИИ прямо сейчас генерирует ответ
                {move || loading.get().then(|| view! {
                    <div style="align-self: flex-start; background: #f1f0f0; padding: 12px 16px; border-radius: 12px; color: #718096; font-style: italic;">
                        "DeepSeek думает..."
                    </div>
                })}
            </div>

            // Фиксированная нижняя панель ввода
            <div style="padding: 20px; border-top: 1px solid #eee; display: flex; gap: 10px; background: white; border-radius: 0 0 8px 8px;">
                <input type="text" 
                    placeholder="Введите ваш вопрос к модели..."
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
