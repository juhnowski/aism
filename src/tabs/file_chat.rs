use leptos::prelude::*;
use crate::tabs::chat::ChatTab;
use crate::es_logger::log_to_es;

#[component]
pub fn FileChatTab() -> impl IntoView {
    // РАСПАКОВЫВАЕМ КОРТЕЖ НА СИГНАЛЫ ЧТЕНИЯ И ЗАПИСИ
    let (selected_file, set_selected_file) = signal(String::new());

    let on_file_change = move |ev| {
        let value = event_target_value(&ev);
        set_selected_file.set(value.clone()); // Используем пишущий сигнал
        log_to_es("File Chat", "Загружен новый файл контекста", Some(value));
    };

    view! {
        <div style="display: flex; flex-direction: column; height: 100%;">
            // Верхняя панель загрузки файла
            <div style="padding: 15px 20px; background: white; border-bottom: 1px solid #e2e8f0; margin-bottom: 15px; border-radius: 8px; display: flex; align-items: center; gap: 15px;">
                <strong>"Контекст файла:"</strong>
                <input type="file" on:change=on_file_change style="font-size: 14px;" />
                
                // Читающий сигнал используется внутри динамического блока
                {move || if !selected_file.get().is_empty() {
                    view! { <span style="color: green; font-size: 12px;">"Файл успешно прикреплен"</span> }.into_any()
                } else {
                    view! { <span style="color: #718096; font-size: 12px;">"Файл не выбран"</span> }.into_any()
                }}
            </div>

            // Основной интерфейс чата подгружается ниже
            <div style="flex-grow: 1;">
                <ChatTab />
            </div>
        </div>
    }
}
