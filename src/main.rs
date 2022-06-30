use actix_web::{get,post, web, App, HttpServer, Responder,HttpResponse};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body:String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(greet)
            .service(echo)
            .service(hello)
            .route("/hey",web::get().to(manual_hello))
            .route("/hello", web::get().to(|| async { "Hi Mayank , welcome to Rust" }))

    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
