mod server;
mod schema;

extern crate log;
extern crate pretty_env_logger;

#[tokio::main]
async fn main() {
    // Initialize logger,
    pretty_env_logger::init();

    // Start the API Server to handle requests.
    server::start(([127,0,0,1], 3030)).await;
}
