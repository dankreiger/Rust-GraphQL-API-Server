use std::net::SocketAddr;

mod routes;

// Start the API Server
pub async fn start(addr: impl Into<SocketAddr>) {
    warp::serve(routes::makes_routes()).run(addr).await;
}