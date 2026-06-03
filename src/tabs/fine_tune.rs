use leptos::prelude::*;
use crate::es_logger::log_to_es;

#[derive(Clone, Copy, PartialEq)]
enum SubTab {
    IsError,
    IsNotError,
}

#[component]
pub fn FineTuneTab() -> impl IntoView {
    // РАСПАКОВЫВАЕМ КОРТЕЖИ НА СИГНАЛЫ ЧТЕНИЯ И ЗАПИСИ
    let (active_sub_tab, set_active_sub_tab) = signal(SubTab::IsError);
    let (log_field, set_log_field) = signal(String::new());
    let (desc_field, set_desc_field) = signal(String::new());

    // Клонируем или перемещаем данные для функции отправки
    let submit_training = move |is_error_type: bool| {
        let log_data = log_field.get();
        let desc_data = desc_field.get();
        let category = if is_error_type { "Считать ошибкой" } else { "Не считать ошибкой" };

        let metadata = format!("Action: {}, Desc: {}", category, desc_data);
        log_to_es("Fine-Tuning", &log_data, Some(metadata));

        // Очищаем поля формы через пишущие сигналы
        set_log_field.set(String::new());
        set_desc_field.set(String::new());
    };

    view! {
        <div style="max-width: 800px; margin: 0 auto; background: white; padding: 30px; border-radius: 12px; box-shadow: 0 4px 12px rgba(0,0,0,0.05);">
            <h2>"Дообучение модели"</h2>
            
            // Переключатель подвкладок 3.1 и 3.2
            <div style="display: flex; gap: 10px; border-bottom: 2px solid #edf2f7; margin-bottom: 25px; padding-bottom: 10px;">
                <button 
                    on:click=move |_| set_active_sub_tab.set(SubTab::IsError)
                    style=move || format!("padding: 10px 20px; border: none; cursor: pointer; font-weight: bold; background: none; border-bottom: 3px solid {}; color: {};",
                        if active_sub_tab.get() == SubTab::IsError { "#e74c3c" } else { "transparent" },
                        if active_sub_tab.get() == SubTab::IsError { "#e74c3c" } else { "#718096" })
                > "3.1 Считать ошибкой" </button>
                
                <button 
                    on:click=move |_| set_active_sub_tab.set(SubTab::IsNotError)
                    style=move || format!("padding: 10px 20px; border: none; cursor: pointer; font-weight: bold; background: none; border-bottom: 3px solid {}; color: {};",
                        if active_sub_tab.get() == SubTab::IsNotError { "#2ecc71" } else { "transparent" },
                        if active_sub_tab.get() == SubTab::IsNotError { "#2ecc71" } else { "#718096" })
                > "3.2 Не считать ошибкой" </button>
            </div>

            // Форма ввода
            <div style="display: flex; flex-direction: column; gap: 20px;">
                <label>
                    <strong>"Поле лога"</strong>
                    <textarea 
                        rows="6"
                        placeholder="Вставьте сюда лог системы..."
                        prop:value=move || log_field.get()
                        on:input=move |ev| set_log_field.set(event_target_value(&ev))
                        style="width: 100%; padding: 12px; margin-top: 8px; border: 1px solid #cbd5e0; border-radius: 6px; font-family: monospace;"
                    />
                </label>

                <label>
                    <strong>"Описание действия"</strong>
                    <input type="text" 
                        placeholder="Что модель должна была сделать?"
                        prop:value=move || desc_field.get()
                        on:input=move |ev| set_desc_field.set(event_target_value(&ev))
                        style="width: 100%; padding: 12px; margin-top: 8px; border: 1px solid #cbd5e0; border-radius: 6px;"
                    />
                </label>

                // Динамическая кнопка отправки в зависимости от активной подвкладки
                {move || {
                    let is_error = active_sub_tab.get() == SubTab::IsError;
                    let btn_color = if is_error { "#e74c3c" } else { "#2ecc71" };
                    view! {
                        <button 
                            on:click=move |_| submit_training(is_error)
                            style=format!("background: {}; color: white; border: none; padding: 14px; border-radius: 6px; font-weight: bold; cursor: pointer;", btn_color)
                        >
                            "Отправить данные для дообучения"
                        </button>
                    }
                }}
            </div>
        </div>
    }
}
