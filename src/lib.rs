// src/lib.rs
#![recursion_limit = "512"]
pub mod api;
pub mod core;
pub mod di;
pub mod websocket;

use std::sync::Arc;

use warp::Filter;

use crate::api::auth::domain::entities::Claims;
use crate::api::auth::UserFeature;
use crate::api::auth_token::AuthTokenFeature;
use crate::core::CoreEventHandler;
use crate::core::{
    check_server_status::check_server_status, errors::handle_app_rejection,
    middleware::auth_middleware, Config,
};
use crate::di::ServiceLocator;
use crate::websocket::ws_handler;

pub struct CoreServer {
    service_locator: Arc<ServiceLocator>,
}

impl CoreServer {
    /// Create a new QkonsServer instance
    pub async fn new(is_dev: bool) -> Result<Self, Box<dyn std::error::Error>> {
        let config = Config::new(is_dev)?;
        let service_locator = Arc::new(ServiceLocator::new(config).await?);

        Ok(Self { service_locator })
    }

    /// Create a new QkonsServer with custom configuration
    pub async fn with_config(config: Config) -> Result<Self, Box<dyn std::error::Error>> {
        let service_locator = Arc::new(ServiceLocator::new(config).await?);
        Ok(Self { service_locator })
    }

    /// Get access to the service locator for advanced use cases
    pub fn service_locator(&self) -> Arc<ServiceLocator> {
        Arc::clone(&self.service_locator)
    }

    /// Get only the routes for integration with existing Warp applications
    // pub fn routes<E: CoreEventHandler + 'static>(
    //     &self,
    //     event_handler: Arc<E>,
    // ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    //     self.build_routes(event_handler)
    // }

    /// Build all routes for the application
    pub fn build_routes<E: CoreEventHandler + 'static>(
        &self,
        event_handler: Arc<E>,
        app_routes: impl Filter<Extract = impl warp::Reply, Error = warp::Rejection>
            + Clone
            + Send
            + Sync
            + 'static,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        let base_routes = self.base_routes(event_handler);

        // Combine routes and box them to erase the complex type
        let combined_routes = base_routes.or(app_routes);

        let api_routes = combined_routes
            .with(self.cors_routes())
            .with(self.log_routes())
            .recover(handle_app_rejection);

        let landing_page = warp::fs::dir("src/static");

        warp::path("api").and(api_routes).or(landing_page)
    }

    fn base_routes<E: CoreEventHandler + 'static>(
        &self,
        event_handler: Arc<E>,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        // ---[ Websocket Setup ]-------------------------------------------------------------------
        let clients_filter = {
            let sl_clone = Arc::clone(&self.service_locator);
            warp::any().map(move || sl_clone.ws_clients())
        };

        let ws_route = warp::path("ws")
            .and(warp::ws())
            .and(clients_filter.clone())
            .and(auth_middleware(self.service_locator.jwt_service()))
            .map(|ws: warp::ws::Ws, clients, claims: Claims| {
                ws.on_upgrade(move |socket| ws_handler::handle_ws_client(socket, clients, claims))
            });

        /* ······································································· [ API ROUTES ] */
        let auth_routes =
            Arc::new(UserFeature::new(Arc::clone(&self.service_locator))).routes(event_handler);
        let auth_token_routes =
            Arc::new(AuthTokenFeature::new(Arc::clone(&self.service_locator))).routes();

        check_server_status(self.service_locator.jwt_service())
            .or(auth_routes)
            .or(auth_token_routes)
            .or(ws_route)
    }

    /* ········································································· [ CORS BUILDER ] */
    pub fn cors_routes(&self) -> warp::filters::cors::Builder {
        warp::cors()
            .allow_any_origin()
            .allow_headers(vec![
                "Content-Type",
                "Authorization",
                "Access-Control-Allow-Origin",
                "Access-Control-Allow-Headers",
                "X-App-Version",
                "X-API-TOKEN",
            ])
            .allow_methods(&[
                warp::http::Method::GET,
                warp::http::Method::POST,
                warp::http::Method::PUT,
                warp::http::Method::DELETE,
                warp::http::Method::OPTIONS,
            ])
            .allow_credentials(true)
            .expose_headers(["x-auth-token"])
    }

    /* ··········································································· [ LOG ROUTES ] */
    pub fn log_routes(&self) -> warp::log::Log<impl Fn(warp::log::Info) + Clone> {
        warp::log::custom(|info| {
            eprintln!(
                "[{}] {} -> {}",
                info.method(),
                info.path(),
                info.status().as_u16()
            );
        })
    }

    // Run the server on the specified address and port
    // pub async fn run(&self, addr: [u8; 4], port: u16) -> Result<(), Box<dyn std::error::Error>> {
    //     let noop_events = Arc::new(NoopEventHandler);
    //     let routes = self.build_routes(noop_events);

    //     println!(
    //         "Server running on {}:{}",
    //         addr.iter()
    //             .map(|b| b.to_string())
    //             .collect::<Vec<_>>()
    //             .join("."),
    //         port
    //     );
    //     warp::serve(routes).run((addr, port)).await;

    //     Ok(())
    // }

    // Run the server on default address (0.0.0.0) and port 3000
    // pub async fn run_default(&self) -> Result<(), Box<dyn std::error::Error>> {
    //     self.run([127, 0, 0, 1], 3000).await
    // }
}
