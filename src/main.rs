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

#[component]
fn App() -> impl IntoView {
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
        <div style="font-family: sans-serif; max-width: 700px; margin: 2rem auto; padding: 1.5rem; border: 1px solid #444; border-radius: 12px; background: #fdfdfd; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
            <h2 style="color: #2c3e50; border-bottom: 2px solid #3498db; padding-bottom: 10px;">"AI Storage Controller"</h2>
            
            <div style="display: flex; flex-direction: column; gap: 15px; margin-top: 20px;">
                <label>
                    <strong>"Имя пула хранения:"</strong>
                    <input type="text" 
                        on:input=move |ev| set_pool_name.set(event_target_value(&ev))
                        prop:value=move || pool_name.get()
                        style="width: 100%; padding: 8px; margin-top: 5px; border: 1px solid #ccc; border-radius: 4px;"
                    />
                </label>

                <label>
                    <strong>"Уровень RAID:"</strong>
                    <select on:change=move |ev| set_raid_level.set(event_target_value(&ev)) style="width: 100%; padding: 8px; margin-top: 5px;">
                        <option value="RAID-5">"RAID-5 (Оптимально)"</option>
                        <option value="RAID-6" selected>"RAID-6 (Надежно)"</option>
                        <option value="RAID-10">"RAID-10 (Быстро)"</option>
                    </select>
                </label>

                <label>
                    <strong>"Приоритет:"</strong>
                    <select on:change=move |ev| set_goal.set(event_target_value(&ev)) style="width: 100%; padding: 8px; margin-top: 5px;">
                        <option value="Throughput">"Макс. скорость (МБ/с)"</option>
                        <option value="Latency">"Мин. задержка (IOPS)"</option>
                        <option value="Capacity">"Макс. объем"</option>
                    </select>
                </label>

                <button 
                    on:click=get_ai_advice
                    disabled=move || loading.get()
                    style="background: #3498db; color: white; border: none; padding: 12px; border-radius: 6px; font-weight: bold; cursor: pointer;"
                >
                    {move || if loading.get() { "Анализ DeepSeek..." } else { "Сгенерировать конфигурацию ИИ" }}
                </button>
            </div>

            <div style="margin-top: 25px; padding: 15px; background: #f8f9fa; border-radius: 8px; border: 1px solid #e9ecef; min-height: 100px;">
                <h4 style="margin-top: 0; color: #2c3e50;">"Рекомендация модели:"</h4>
                <p style="white-space: pre-wrap; font-size: 0.95rem; line-height: 1.5; color: #34495e;">
                    {move || ai_response.get()}
                </p>
            </div>
        </div>
    }
}

fn main() {
    leptos::mount::mount_to_body(App);
}
