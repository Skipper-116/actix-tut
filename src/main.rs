use actix_web::{get, post, put, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello_get() -> impl Responder {
    HttpResponse::Ok().body("Hello Rust!")
}

#[post("/")]
async fn hello_post(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[put("/")]
async fn hello_put(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey Thoko!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    HttpServer::new(|| {
        App::new()
            .service(hello_get)
            .service(hello_post)
            .service(hello_put)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
