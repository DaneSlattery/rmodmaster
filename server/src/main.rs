use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use axum::{routing::get, Json, Router};
use log::info;
use serialport::SerialPortType;

#[tokio::main]
async fn main() {
    env_logger::init();

    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route("/api/ports", get(get_ports));

    let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 3000);

    let listener = tokio::net::TcpListener::bind(socket_addr).await.unwrap();
    log::info!("Listening on [http://{socket_addr}]");

    axum::serve(listener, app).await.unwrap();
}

#[axum::debug_handler]
async fn get_ports() -> Json<Vec<String>> {
    let mut ports = serialport::available_ports().expect("No ports found");

    // filter out those with an unknown type
    ports.retain(|x| !matches!(x.port_type, SerialPortType::Unknown));
    let port_names = ports
        .into_iter()
        .filter_map(|w| Some(w.port_name))
        .collect();
    info!("Found ports :{:?}", &port_names);
    Json(port_names)
}
