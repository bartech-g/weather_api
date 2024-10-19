use actix_web::{get, web, App, HttpServer, Responder};
use reqwest::Client;
use serde_json::{json, Value};  // Add `json` macro import
use std::env;

#[get("/weather/{city}")]
async fn get_weather(city: web::Path<String>) -> impl Responder {
    let client = Client::new();
    let api_key = env::var("API_KEY").unwrap_or_else(|_| "YOUR_API_KEY".to_string()); // Load from env variable
    let url = format!("http://api.openweathermap.org/data/2.5/weather?q={}&appid={}", city, api_key);

    match client.get(&url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                let weather_data: Value = response.json().await.unwrap();
                web::Json(weather_data)
            } else {
                web::Json(json!({"error": "City not found"}))
            }
        },
        Err(_) => web::Json(json!({"error": "Failed to fetch weather data"})),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(get_weather)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
