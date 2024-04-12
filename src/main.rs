use std::sync::Mutex;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

struct AppState {
    version: String,
    ping: Mutex<u64>,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> impl Responder {
    let version = &data.version;
    let html = format!("<h1>Trove v{}</h1>", version);

    HttpResponse::Ok().body(html)
}

#[get("/inc")]
async fn inc(data: web::Data<AppState>) -> impl Responder {
    let mut ping = data.ping.lock().unwrap();
    *ping += 1;

    HttpResponse::Ok().body(format!("ping: {}", *ping))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(AppState {
                version: String::from("0.1.0"),
                ping: Mutex::new(0),
            })
            .service(index)
            .service(echo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
