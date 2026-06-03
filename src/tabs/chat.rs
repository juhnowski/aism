use leptos::prelude::*;
use crate::es_logger::log_to_es;

#[derive(Clone)]
pub struct Message {
    pub is_user: bool,
    pub text: String,
}

#[component]
pub fn ChatTab() -> impl IntoView {
    let (messages, set_messages) = signal(Vec::<Message>::new());
    let (input_text, set_input_text) = signal(String::new());

    let send_message = move || {
        let text = input_text.get().trim().to_string();
        if text.is_empty() { return; }

        log_to_es("Chat", &text, None);

        set_messages.update(|msgs| msgs.push(Message { is_user: true, text: text.clone() }));
        set_input_text.set(String::new());
    };

    view! {
        <div style="display: flex; flex-direction: column; height: calc(100vh - 140px); max-width: 800px; margin: 0 auto; background: white; border-radius: 8px; box-shadow: 0 4px 12px rgba(0,0,0,0.05);">
            <div style="flex-grow: 1; overflow-y: auto; padding: 20px; display: flex; flex-direction: column; gap: 15px;">
                {move || messages.get().into_iter().map(|msg| {
                    let bg = if msg.is_user { "#3498db" } else { "#f1f0f0" };
                    let color = if msg.is_user { "white" } else { "black" };
                    let align = if msg.is_user { "flex-end" } else { "flex-start" };
                    view! {
                        <div style=format!("align-self: {}; max-width: 70%; padding: 12px 16px; border-radius: 12px; background: {}; color: {}; white-space: pre-wrap;", align, bg, color)>
                            {msg.text}
                        </div>
                    }
                }).collect_view()}
            </div>

            <div style="padding: 20px; border-top: 1px solid #eee; display: flex; gap: 10px;">
                <input type="text" 
                    placeholder="Введите ваш вопрос к модели..."
                    prop:value=move || input_text.get()
                    on:input=move |ev| set_input_text.set(event_target_value(&ev))
                    on:keydown=move |ev| if ev.key() == "Enter" { send_message(); }
                    style="flex-grow: 1; padding: 12px; border: 1px solid #ddd; border-radius: 6px; font-size: 14px;"
                />
                <button on:click=move |_| send_message() style="background: #3498db; color: white; border: none; padding: 0 20px; border-radius: 6px; cursor: pointer; font-weight: bold;">
                    "Отправить"
                </button>
            </div>
        </div>
    }
}
