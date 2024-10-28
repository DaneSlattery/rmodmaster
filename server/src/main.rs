use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};

use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use log::info;
use rmodbus::{client::ModbusRequest, guess_response_frame_len};
use serialport::{SerialPort, SerialPortType};
use tokio::sync::Mutex;

struct AppState {
    serial: Mutex<Option<Box<dyn SerialPort>>>,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let shared_state = Arc::new(AppState {
        serial: Mutex::new(None),
    });

    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route("/api/ports", get(get_ports))
        .route("/api/select_port", post(set_port))
        .with_state(shared_state);

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

async fn set_port(State(state): State<Arc<AppState>>, axum::Json(name): axum::Json<String>) {
    let mut ser = serialport::new(&name, 9600).open().unwrap();

    let mut mreq = ModbusRequest::new(1, rmodbus::ModbusProto::Rtu);

    let mut request = Vec::new();
    mreq.generate_get_holdings(350, 2, &mut request).unwrap();

    ser.write(&request).unwrap();

    let mut buff = [0u8; 6];
    ser.read_exact(&mut buff).unwrap();

    let mut response = Vec::new();
    response.extend_from_slice(&buff);

    let len = guess_response_frame_len(&buff, rmodbus::ModbusProto::Rtu).unwrap();

    if len > 6 {
        let mut rest = vec![0u8; (len - 6) as usize];
        ser.read_exact(&mut rest).unwrap();
        response.extend(rest);
    }

    // let mut data = Vec::new();

    let mut data = mreq.parse_slice(&response).unwrap();
    info!("Read: {:?}", data);
    // state.replace(ser);

    let mut port = state.serial.lock().await;
    (*port).replace(ser);
    info!("Opened Port:{:?}", &name);
}
