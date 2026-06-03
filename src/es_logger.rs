use gloo_net::http::Request;
use serde::Serialize;
use leptos::task::spawn_local;
use chrono;

#[derive(Serialize)]
struct EsLog {
    timestamp: String,
    tab: String,
    user_input: String,
    metadata: Option<String>,
}

// Функция для асинхронного логирования действий в ElasticSearch
pub fn log_to_es(tab_name: &str, input: &str, extra_info: Option<String>) {
    let log_entry = EsLog {
        timestamp: chrono::Utc::now().to_rfc3339(), // Требуется доп. крейт chrono с feature = "wasmbind"
        tab: tab_name.to_string(),
        user_input: input.to_string(),
        metadata: extra_info,
    };

    spawn_local(async move {
        // Замените URL на адрес вашего Elasticsearch или вашего бэкенд-прокси
        let _ = Request::post("http://localhost:9200/user-actions/_doc")
            .json(&log_entry)
            .unwrap()
            .send()
            .await;
    });
}
