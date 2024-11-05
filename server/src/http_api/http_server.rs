// use axum::Router;
// use tokio::{runtime::Handle, task::JoinHandle};
// use tower_http::cors::{Any, CorsLayer};

// use crate::http_api::api::get_api_router;

// #[derive(Clone, Debug)]
// pub struct AppState {}

// impl AppState {}

// #[derive(Debug)]
// pub struct HttpServer {
//     handle: Handle,
//     http_server_thread: JoinHandle<()>,
//     pub state: AppState,
// }

// impl HttpServer {
//     pub fn new(address: &str) -> Result<Self, String> {
//         tracing::info!("Starting up HTTP-Server");
//         let state = AppState {};

//         let cors = CorsLayer::new()
//             // allow `GET` and `POST` when accessing the resource
//             // .allow_methods([Method::GET, Method::POST])
//             // allow requests from any origin
//             .allow_origin(Any);

//         let api_router = get_api_router(state.clone());

//         let frontend_router = Router::new()
//             // .route(
//             //     "/frontend",
//             //     get(|| async { Redirect::permanent("/frontend/") }),
//             // )
//             .nest("/frontend/", axum_static::static_router("../frontend/dist"));

//         let docs_router = Router::new()
//             .route(
//                 "/docs",
//                 get(|| async { Redirect::permanent("/docs/central/index.html") }),
//             )
//             .nest("/docs/", axum_static::static_router("target/doc"));

//         let router = Router::new().merge(frontend_router).nest("/api", api_router).merge(docs_router).route(
//             "/",
//             get(|| async {
//                 Html(format!(
//                     "<h1>Hello, world!</h1>
//                     <p>Request processed at: {:?}</p>
//                     <ul><li><a href='/frontend'>Frontend</a></li><li><a href='/docs'>Local docs</a></li><li><a href='/api'>API</a></ul>",
//                     Utc::now().to_string()
//                 ))
//             }),
//         );

//         let handle = Handle::new();

//         let handle_clone = handle.clone();
//         let server = axum_server::bind(SocketAddr::from_str(address).unwrap())
//             .handle(handle_clone)
//             .serve(router.layer(cors).into_make_service());

//         let http_server_thread = tokio::spawn(async {
//             server.await.unwrap();
//         });

//         tracing::info!("Started Server on http://{}", address);

//         Ok(HttpServer {
//             handle,
//             http_server_thread,
//             state,
//         })
//     }

//     pub async fn shutdown(&self) {
//         tracing::info!("Sending shutdown-signal to HTTP-server...");
//         // _ = self.close_tx.send(());
//         self.handle.shutdown();
//         _ = self.http_server_thread;
//         tracing::info!("HTTP-Server shut down.");
//     }
// }
