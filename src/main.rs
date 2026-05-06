use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::{Deserialize, Serialize};
use gloo_net::http::Request;
use web_sys::console;

// Структуры для запроса к Ollama API
#[derive(Serialize)]
struct OllamaMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    messages: Vec<OllamaMessage>,
    stream: bool,
}

// Структура для ответа от Ollama
#[derive(Deserialize, Clone)]
struct OllamaChatResponse {
    message: OllamaMessageContent,
}

#[derive(Deserialize, Clone)]
struct OllamaMessageContent {
    content: String,
}

#[derive(Clone, Copy, PartialEq)]
enum Page {
    Dashboard,
    Storage,
    Network,
    Settings,
}

#[component]
fn App() -> impl IntoView {
        // Сигнал текущей страницы
    let (active_page, set_active_page) = signal(Page::Dashboard);
    
    let (pool_name, set_pool_name) = signal(String::new());
    let (raid_level, set_raid_level) = signal("RAID-6".to_string());
    let (goal, set_goal) = signal("Throughput".to_string());
    
    let (ai_response, set_ai_response) = signal(String::new());
    let (loading, set_loading) = signal(false);

    let get_ai_advice = move |_| {
        spawn_local(async move {
            set_loading.set(true);
            set_ai_response.set("ИИ думает...".to_string());

            let prompt = format!(
                "Ты эксперт по СХД. Посоветуй оптимальные параметры для пула '{}', уровень RAID: {}, цель: {}. Ответь кратко.",
                pool_name.get(),
                raid_level.get(),
                goal.get()
            );

let body = OllamaRequest {
                model: "deepseek-coder-v2".to_string(),
                messages: vec![OllamaMessage {
                    role: "user".to_string(),
                    content: prompt,
                }],
                stream: false,
            };

            let request_result = Request::post("http://localhost:11434/api/chat")
                .json(&body);

            match request_result {
                Ok(request) => {
                    let result = request.send().await;
                    match result {
                        Ok(resp) => {
                            if let Ok(json) = resp.json::<OllamaChatResponse>().await {
                                set_ai_response.set(json.message.content);
                            } else {
                                set_ai_response.set("Ошибка: не удалось прочитать ответ модели.".to_string());
                            }
                        }
                        Err(e) => {
                            console::log_1(&format!("Ошибка сети: {:?}", e).into());
                            set_ai_response.set("Ошибка: Ollama недоступна. Проверьте CORS.".to_string());
                        }
                    }
                }
                Err(e) => {
                    set_ai_response.set(format!("Ошибка сериализации: {:?}", e));
                }
            }
            
            set_loading.set(false);
        });
    };

    view! {
        // Контейнер на весь экран
        <div style="display: flex; height: 100vh; font-family: sans-serif; background: #f0f2f5;">
            
            // --- ЛЕВАЯ ПАНЕЛЬ (TOOLBAR) ---
            <div style="width: 70px; background: #1a1c1e; display: flex; flex-direction: column; align-items: center; padding: 20px 0; gap: 20px;">
                
                // Кнопка Dashboard
                <button 
                    on:click=move |_| set_active_page.set(Page::Dashboard)
                    style=move || format!("background: none; border: none; cursor: pointer; font-size: 24px; opacity: {};", 
                        if active_page.get() == Page::Dashboard { "1.0" } else { "0.4" })
                > "📊" </button>

                // Кнопка Storage
                <button 
                    on:click=move |_| set_active_page.set(Page::Storage)
                    style=move || format!("background: none; border: none; cursor: pointer; font-size: 24px; opacity: {};", 
                        if active_page.get() == Page::Storage { "1.0" } else { "0.4" })
                > "💾" </button>

                // Кнопка Settings (внизу)
                <div style="margin-top: auto;">
                    <button 
                        on:click=move |_| set_active_page.set(Page::Settings)
                        style=move || format!("background: none; border: none; cursor: pointer; font-size: 24px; opacity: {};", 
                            if active_page.get() == Page::Settings { "1.0" } else { "0.4" })
                    > "⚙️" </button>
                </div>
            </div>

            // --- ОСНОВНОЙ КОНТЕНТ (ДИНАМИЧЕСКИЙ) ---
            <div style="flex-grow: 1; padding: 40px; background: #f5f7f9; overflow-y: auto;">
                {move || match active_page.get() {
                    Page::Dashboard => view! {
                        <div>
                            <h1>"Панель мониторинга"</h1>
                            <p>"Здесь будет общая статистика системы..."</p>
                        </div>
                    }.into_any(),
                    
                    Page::Storage => view! {
                        <div style="max-width: 700px; margin: 0 auto; padding: 30px; border-radius: 12px; background: white; box-shadow: 0 10px 25px rgba(0,0,0,0.05);">
                    <h2 style="color: #1a1a1a; margin-top: 0;">"AI Storage Controller"</h2>
                    
                    <div style="display: flex; flex-direction: column; gap: 20px;">
                        <label>
                            <strong>"Имя пула"</strong>
                            <input type="text" 
                                on:input=move |ev| set_pool_name.set(event_target_value(&ev))
                                prop:value=move || pool_name.get()
                                style="width: 100%; padding: 10px; margin-top: 8px; border: 1px solid #ddd; border-radius: 6px;"
                            />
                        </label>

                        <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 20px;">
                            <label>
                                <strong>"RAID"</strong>
                                <select on:change=move |ev| set_raid_level.set(event_target_value(&ev)) style="width: 100%; padding: 10px; margin-top: 8px;">
                                    <option value="RAID-5">"RAID-5"</option>
                                    <option value="RAID-6" selected>"RAID-6"</option>
                                    <option value="RAID-10">"RAID-10"</option>
                                </select>
                            </label>
                            <label>
                                <strong>"Приоритет"</strong>
                                <select on:change=move |ev| set_goal.set(event_target_value(&ev)) style="width: 100%; padding: 10px; margin-top: 8px;">
                                    <option value="Throughput">"Скорость"</option>
                                    <option value="Latency">"Задержка"</option>
                                </select>
                            </label>
                        </div>

                        <button 
                            on:click=get_ai_advice
                            disabled=move || loading.get()
                            style="background: #3498db; color: white; border: none; padding: 14px; border-radius: 8px; font-weight: bold; cursor: pointer; transition: background 0.2s;"
                        >
                            {move || if loading.get() { "Анализ DeepSeek..." } else { "Сгенерировать конфигурацию" }}
                        </button>
                    </div>

                    <div style="margin-top: 30px; padding: 20px; background: #fdfdfd; border-radius: 8px; border: 1px dashed #cbd5e0;">
                        <h4 style="margin-top: 0; color: #4a5568;">"Рекомендация ИИ:"</h4>
                        <p style="white-space: pre-wrap; line-height: 1.6; color: #2d3748;">
                            {move || ai_response.get()}
                        </p>
                    </div>
                </div>
                    }.into_any(),

                    Page::Settings => view! {
                        <div>
                            <h1>"Настройки"</h1>
                            <p>"Конфигурация Ollama API и системные параметры."</p>
                        </div>
                    }.into_any(),
                    
                    _ => view! { <div>"Страница в разработке"</div> }.into_any(),
                }}
            </div>
        </div>
    }
}

fn main() {
    leptos::mount::mount_to_body(App);
}
