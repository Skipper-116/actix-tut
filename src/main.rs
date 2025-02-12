use actix_web::{get, post, put, web, App, HttpResponse, HttpServer, Responder};

#[get("/api/v1/hello")]
async fn hello_get() -> impl Responder {
    HttpResponse::Ok().body("Hello Rust!")
}

#[post("/api/v1/hello")]
async fn hello_post(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[put("/api/v1/hello/{id}")]
async fn hello_put(req_body: String) -> impl Responder {
    let response = format!("Hello {}!", req_body);
    HttpResponse::Ok().body(response)
}

async fn manual_hello(name: web::Path<String>) -> impl Responder {
    let response = format!("Hey {}!", name);
    HttpResponse::Ok().body(response)
}

async fn index() -> impl Responder {
    // lists all available routes
    let routes = vec![
        "GET /api/v1/hello",
        "POST /api/v1/hello",
        "PUT /api/v1/hello/{id}",
        "GET /api/v1/hey/{name}",
    ];

    HttpResponse::Ok().json(routes)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    HttpServer::new(|| {
        App::new()
            .service(hello_get)
            .service(hello_post)
            .service(hello_put)
            .route("/api/v1/hey/{name}", web::get().to(manual_hello))
            .route("/", web::get().to(index))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
