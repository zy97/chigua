use axum::{
    Json, Router, extract,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
};
use serde::Serialize;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/chigua/{name}", get(get_chigua_url));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn get_chigua_url(extract::Path(name): extract::Path<String>) -> impl IntoResponse {
    // Load the JSON file and parse it
    let json_file =
        std::fs::read_to_string("chigua.json").expect("Failed to read chigua_urls.json");
    let chigua_urls: serde_json::Value =
        serde_json::from_str(&json_file).expect("Failed to parse JSON");
    let chigua_map: std::collections::HashMap<String, String> =
        serde_json::from_value(chigua_urls).unwrap();
    let response = Response {
        url: chigua_map.get(&name).cloned().unwrap_or_default(),
    };
    (StatusCode::OK, Json(response))
}

#[derive(Serialize)]
struct Response {
    url: String,
}
