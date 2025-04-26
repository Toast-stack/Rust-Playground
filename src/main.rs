use actix_web::{web, App, HttpServer, HttpResponse};
use std::collections::HashMap;
use std::sync::Mutex;

// Shared state to store URLs
struct AppState {
    url_map: Mutex<HashMap<String, String>>,
}

// POST: Shorten URL
async fn shorten_url(data: web::Data<AppState>, input: web::Json<HashMap<String, String>>) -> HttpResponse {
    let url_value = input.get("url").unwrap_or(&String::new()).clone(); // Fix for temporary value issue
    let short_key = format!("{:x}", url_value.len()); // Simple hash for shortening
    data.url_map.lock().unwrap().insert(short_key.clone(), url_value);
    HttpResponse::Ok().json(HashMap::from([("shortened_url", format!("http://localhost:8080/{}", short_key))]))
}

// GET: Redirect to Original URL
async fn redirect_url(data: web::Data<AppState>, key: web::Path<String>) -> HttpResponse {
    let url_map = data.url_map.lock().unwrap();
    if let Some(original_url) = url_map.get(&key.into_inner()) {
        HttpResponse::Found().append_header(("Location", original_url.to_string())).finish() // Updated to use append_header and fixed trait issue
    } else {
        HttpResponse::NotFound().body("URL not found")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState {
        url_map: Mutex::new(HashMap::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/shorten", web::post().to(shorten_url))
            .route("/{key}", web::get().to(redirect_url))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}