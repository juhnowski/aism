use leptos::prelude::*;
use leptos::task::spawn_local;
use gloo_net::http::Request;
use serde::Deserialize;

#[derive(Deserialize, Clone, Default)]
struct SystemStats {
    cpu_cores: Vec<f32>,
    ram_total_gb: f32,
    ram_used_gb: f32,
    disk_total_gb: f32,
    disk_used_gb: f32,
    network_rx_mbps: f32,
    network_tx_mbps: f32,
    uptime_hours: f32,
}

#[component]
pub fn DashboardTab() -> impl IntoView {
    let (stats, set_stats) = signal(SystemStats::default());
    let (cpu_history, set_cpu_history) = signal(vec![0.0; 10]);

    let fetch_stats = move || {
        spawn_local(async move {
            if let Ok(response) = Request::get("/api/stats").send().await {
                if let Ok(new_stats) = response.json::<SystemStats>().await {
                    let avg_cpu: f32 = if !new_stats.cpu_cores.is_empty() {
                        new_stats.cpu_cores.iter().sum::<f32>() / new_stats.cpu_cores.len() as f32
                    } else {
                        0.0
                    };

                    set_cpu_history.update(|history| {
                        if history.len() >= 10 {
                            history.remove(0);
                        }
                        history.push(avg_cpu);
                    });

                    set_stats.set(new_stats);
                }
            }
        });
    };

    Effect::new(move |_| {
        fetch_stats();
        let worker = move || fetch_stats();
        
        #[cfg(target_arch = "wasm32")]
        {
            use gloo_timers::callback::Interval;
            let interval = Interval::new(15000, worker);
            interval.forget();
        }
    });

    view! {
        <div style="display: flex; flex-direction: column; gap: 25px; max-width: 1000px; margin: 0 auto;">
            
            // --- ВЕРХНЯЯ ПАНЕЛЬ С КРАТКИМ СТАТУСОМ ---
            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(220px, 1fr)); gap: 20px;">
                <div style="background: white; padding: 20px; border-radius: 10px; box-shadow: 0 4px 6px rgba(0,0,0,0.02); border-left: 4px solid #3498db;">
                    <div style="color: #718096; font-size: 14px; font-weight: bold;">"СЕТЬ (Вход / Выход)"</div>
                    <div style="font-size: 22px; font-weight: bold; margin-top: 10px; color: #2d3748;">
                        {move || format!("{:.1} / {:.1} Mbps", stats.get().network_rx_mbps, stats.get().network_tx_mbps)}
                    </div>
                </div>

                <div style="background: white; padding: 20px; border-radius: 10px; box-shadow: 0 4px 6px rgba(0,0,0,0.02); border-left: 4px solid #9b59b6;">
                    <div style="color: #718096; font-size: 14px; font-weight: bold;">"ВРЕМЯ РАБОТЫ СЕРВЕРА"</div>
                    <div style="font-size: 22px; font-weight: bold; margin-top: 10px; color: #2d3748;">
                        {move || format!("{:.1} ч", stats.get().uptime_hours)}
                    </div>
                </div>
            </div>

            // --- ОСНОВНЫЕ ГРАФИКИ И СЧЕТЧИКИ ---
            <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 25px;">
                
                // Блок 1: Нагрузка по ядрам CPU
                <div style="background: white; padding: 25px; border-radius: 12px; box-shadow: 0 4px 12px rgba(0,0,0,0.05);">
                    <h3 style="margin-top: 0; color: #2d3748;">"Нагрузка по ядрам CPU"</h3>
                    <div style="display: flex; flex-direction: column; gap: 12px; margin-top: 15px;">
                        {move || stats.get().cpu_cores.into_iter().enumerate().map(|(idx, core_load)| {
                            view! {
                                <div style="display: flex; align-items: center; gap: 15px;">
                                    <span style="width: 60px; font-size: 13px; color: #4a5568; font-weight: bold;">{format!("Ядро {}", idx)}</span>
                                    <div style="flex-grow: 1; background: #edf2f7; height: 12px; border-radius: 6px; overflow: hidden;">
                                        <div style=format!("background: #3498db; width: {}%; height: 100%; transition: width 0.5s ease-in-out;", core_load) />
                                    </div>
                                    <span style="width: 45px; text-align: right; font-size: 13px; font-weight: bold; color: #2d3748;">{format!("{:.0}%", core_load)}</span>
                                </div>
                            }
                        }).collect_view()}
                    </div>

                    <h4 style="margin-top: 25px; margin-bottom: 10px; color: #718096;">"История общей нагрузки (%)"</h4>
                    <div style="background: #f7fafc; padding: 10px; border-radius: 8px;">
                        <svg viewBox="0 0 100 30" style="width: 100%; height: 80px; overflow: visible;">
                            <polyline
                                fill="none"
                                stroke="#3498db"
                                stroke-width="1.5"
                                points={move || {
                                    cpu_history.get().iter().enumerate().map(|(i, &val)| {
                                        format!("{},{}", (i * 11), (30.0 - (val * 0.3)))
                                    }).collect::<Vec<String>>().join(" ")
                                }}
                            />
                        </svg>
                    </div>
                </div>

                // Блок 2: Память и Диски
                <div style="display: flex; flex-direction: column; gap: 25px;">
                    
                    // Подблок: Оперативная память (RAM)
                    <div style="background: white; padding: 25px; border-radius: 12px; box-shadow: 0 4px 12px rgba(0,0,0,0.05);">
                        <h3 style="margin-top: 0; color: #2d3748;">"Оперативная память (RAM)"</h3>
                        {move || {
                            let s = stats.get();
                            let percent = if s.ram_total_gb > 0.0 { (s.ram_used_gb / s.ram_total_gb) * 100.0 } else { 0.0 };
                            view! {
                                <div style="margin-top: 15px;">
                                    <div style="display: flex; justify-content: space-between; margin-bottom: 8px; font-size: 14px; font-weight: bold; color: #4a5568;">
                                        <span>{format!("Занято: {:.1} ГБ / {:.1} ГБ", s.ram_used_gb, s.ram_total_gb)}</span>
                                        <span style="color: #e67e22;">{format!("{:.1}%", percent)}</span>
                                    </div>
                                    <div style="background: #edf2f7; height: 16px; border-radius: 8px; overflow: hidden;">
                                        <div style=format!("background: #e67e22; width: {}%; height: 100%; transition: width 0.5s;", percent) />
                                    </div>
                                </div>
                            }
                        }}
                    </div>

                    // Подблок: Дисковое пространство (Пул хранения)
                    <div style="background: white; padding: 25px; border-radius: 12px; box-shadow: 0 4px 12px rgba(0,0,0,0.05);">
                        <h3 style="margin-top: 0; color: #2d3748;">"Объем дисков (Пул СХД)"</h3>
                        {move || {
                            let s = stats.get();
                            let free_gb = s.disk_total_gb - s.disk_used_gb;
                            let percent = if s.disk_total_gb > 0.0 { (s.disk_used_gb / s.disk_total_gb) * 100.0 } else { 0.0 };
                            view! {
                                <div style="margin-top: 15px;">
                                    <div style="display: flex; justify-content: space-between; margin-bottom: 8px; font-size: 14px; font-weight: bold; color: #4a5568;">
                                        <span>{format!("Свободно: {:.1} ГБ из {:.1} ГБ", free_gb, s.disk_total_gb)}</span>
                                        <span style="color: #2ecc71;">{format!("{:.1}% занято", percent)}</span>
                                    </div>
                                    <div style="background: #edf2f7; height: 16px; border-radius: 8px; overflow: hidden;">
                                        <div style=format!("background: #2ecc71; width: {}%; height: 100%; transition: width 0.5s;", percent) />
                                    </div>
                                </div>
                            }
                        }}
                    </div>

                </div>
            </div>
        </div>
    }
}
