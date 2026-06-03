use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::{Deserialize, Serialize};
use gloo_net::http::Request;
use web_sys::console;

mod es_logger;
mod tabs; 

use tabs::chat::ChatTab;
use tabs::file_chat::FileChatTab;
use tabs::fine_tune::FineTuneTab;
use tabs::dashboard::DashboardTab;

#[derive(Clone, Copy, PartialEq)]
enum MainTab {
    Chat,
    FileChat,
    FineTune,
    Dashboard,
}

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
pub fn App() -> impl IntoView {
    let (active_tab, set_active_tab) = signal(MainTab::Chat);

    view! {
        <div style="display: flex; height: 100vh; font-family: sans-serif; background: #f7fafc;">
            
            // --- ЛЕВОЕ МЕНЮ (НАВИГАЦИЯ) ---
            <div style="width: 260px; background: #1a202c; color: white; display: flex; flex-direction: column; padding: 20px 0;">
                <div style="padding: 0 20px 20px 20px; font-size: 18px; font-weight: bold; border-bottom: 1px solid #2d3748;">
                    "AI Контроллер СХД"
                </div>
                
                <div style="display: flex; flex-direction: column; gap: 5px; padding: 20px 10px;">
// Кнопка 1 (Чат) — здесь у вас уже всё правильно:
<button 
    on:click=move |_| set_active_tab.set(MainTab::Chat)
    style=move || format!("text-align: left; padding: 12px; border: none; border-radius: 6px; cursor: pointer; font-size: 14px; background: {}; color: white;", 
        if active_tab.get() == MainTab::Chat { "#3182ce" } else { "transparent" })
> "💬 Вопрос-Ответ" </button>

// Кнопка 2 (Файл Чат) — здесь тоже всё правильно:
<button 
    on:click=move |_| set_active_tab.set(MainTab::FileChat)
    style=move || format!("text-align: left; padding: 12px; border: none; border-radius: 6px; cursor: pointer; font-size: 14px; background: {}; color: white;", 
        if active_tab.get() == MainTab::FileChat { "#3182ce" } else { "transparent" })
> "📂 Файл Вопрос-Ответ" </button>

// Кнопка 3 (Дообучение) — ОШИБКА ЗДЕСЬ. Исправьте active_tab.set на set_active_tab.set:
<button 
    on:click=move |_| set_active_tab.set(MainTab::FineTune) // Было: active_tab.set
    style=move || format!("text-align: left; padding: 12px; border: none; border-radius: 6px; cursor: pointer; font-size: 14px; background: {}; color: white;", 
        if active_tab.get() == MainTab::FineTune { "#3182ce" } else { "transparent" })
> "⚙️ Дообучение модели" </button>

<button 
    on:click=move |_| set_active_tab.set(MainTab::Dashboard)
    style=move || format!("text-align: left; padding: 12px; border: none; border-radius: 6px; cursor: pointer; font-size: 14px; background: {}; color: white;", 
        if active_tab.get() == MainTab::Dashboard { "#3182ce" } else { "transparent" })
> "📊 Панель состояния" </button>
                </div>
            </div>

            // --- ОСНОВНАЯ ОБЛАСТЬ КОНТЕНТА ---
            <div style="flex-grow: 1; padding: 30px; overflow-y: auto;">
                {move || match active_tab.get() {
                    MainTab::Chat => view! { <ChatTab /> }.into_any(),
                    MainTab::FileChat => view! { <FileChatTab /> }.into_any(),
                    MainTab::FineTune => view! { <FineTuneTab /> }.into_any(),
                    MainTab::Dashboard => view! { <DashboardTab /> }.into_any(),
                }}
            </div>
        </div>
    }
}

fn main() {
    leptos::mount::mount_to_body(App);
}
