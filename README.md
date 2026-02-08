## Getting Started

the `core_server` crate requires a mongo database. In the production mode, it will pull the database
details from the `environment variables`. In the development mode however, it
will create a default mongo db playground with the name `dev_db`.
The mode is selected at the initialization stage. `let server = CoreServer::new(true).await?`
passing `true` will enable the development mod. If you set it to `false` you will have to set the
`env` variables to you production values.

## Include the auth_server as a basic server:

```rs
use core_server::CoreServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new core server
    // If true is passed it will start a dev server with default config values
    // In production you should pass false and set all env variables as listed below in this documentation
    let core_server = CoreServer::new(true).await?;

    // Inject core_server service_locator into your won service locator
    let sl = ServiceLocator::new(core_server.service_locator());

    // create your warp route filters (mock example)
    let your_routes = product_routes.or(cart_routes).or(other_routes);

    // Combine your routes with the core_server's routes
    // include the CoreEvents implementation if needed, if not you can
    let all_routes = core_server.build_routes(sl.core_events_impl(), app_routes);

    // if you don't provide a core_events implementation use NopeEventHandler instead
    // Example:
    // let all_routes = server.build_routes(Arc::new(NoopEventHandler {}), app_routes);

    // Run on localhost:8080
    println!("Server starting on http://localhost:8080");
    warp::serve(all_routes).run(([127, 0, 0, 1], 3000)).await;

    Ok(())
}
```

## Production `env`

All config are pulled from the env vars. Below are the expected env vars:

```bash
MONGODB_URI
DATABASE_NAME
JWT_SECRET
JWT_EXPIRATION
EMAIL_FROM
APP_NAME
UPLOADS_BAS
RESEND_TOKEN
```
