// use axum::{
//     extract::{Path, State},
//     response::Html,
//     routing::get,
//     Json, Router,
// };
// use axum_macros::debug_handler;
// use serde_json::json;
// use slapy::device::Device;

// // use slapy::metrics::QueueMetrics;

// use super::AppState;

// pub fn get_api_router(state: AppState) -> Router {
//     Router::new()
//         .route("/", get(root_handler))
//         .route("/stats", get(counter_handler))
//         .route("/stats/devices", get(device_handler))
//         .route("/stats/queue/:queue_name", get(get_queue_stats))
//         .with_state(state.clone())
// }

// #[debug_handler]
// pub async fn root_handler(State(state): State<AppState>) -> Html<String> {
//     Html(format!("{:?}", state))
// }

// pub async fn device_handler(State(state): State<AppState>) -> Json<String> {
//     let devices = state.devices.lock().await;
//     let mut devs = Vec::new();

//     for (key, value) in devices.pin().iter() {
//         devs.push((
//             key.to_string(),
//             value.get_preferred_network_list(),
//             value.get_fingerprint_list(),
//         ));
//     }

//     let json = json!({
//         "devices": devs,
//     });
//     Json(serde_json::to_string(&json).unwrap())
// }

// pub async fn counter_handler(State(state): State<AppState>) -> Json<String> {
//     let read_in = state.read_in.lock().await;
//     let remaining_in_queue = state.remaining_in_queue.lock().await;
//     let remaining_write_queue = state.remaining_write_queue.lock().await;
//     let queues = state.queue_stats;

//     let json = json!({
//         "read_in": *read_in,
//         "remaining_in_queue": *remaining_in_queue,
//         "remaining_write_queue": *remaining_write_queue,
//         "queues": *queues,
//         "sniffer_config": state.sniffer_config,
//     });

//     Json(serde_json::to_string(&json).unwrap())
// }

// pub async fn get_queue_stats(
//     State(state): State<AppState>,
//     Path(queue_name): Path<String>,
// ) -> Json<String> {
//     for mut queue in state.queue_stats {
//         if queue.get_name() == queue_name {
//             return Json(serde_json::to_string(&queue).unwrap());
//         }
//     }

//     Json(serde_json::to_string("{}").unwrap())
// }
