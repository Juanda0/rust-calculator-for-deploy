mod calculadora;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use calculadora::Calculadora;

async fn hola(info: web::Path<(String,)>) -> impl Responder {
    let name = &info.0;
    HttpResponse::Ok().body(format!("¡Hola, {}!", name))
}

async fn cuadrado(info: web::Path<(f64,)>) -> impl Responder {
    // Obtener el parámetro de consulta "numero"
    let numero = info.0;
    let cuadrado = Calculadora::cuadrado(numero);
    HttpResponse::Ok().body(format!("El cuadrado de {} es {}", numero, cuadrado))
}

async fn cubo(info: web::Path<(f64,)>) -> impl Responder {
    let numero = info.0;
    let cubo = Calculadora::cubo(numero);
    HttpResponse::Ok().body(format!("El cubo de {} es {}", numero, cubo))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/hola/{name}", web::get().to(hola))
            .route("/cuadrado/{numero}", web::get().to(cuadrado))
            .route("/cubo/{numero}", web::get().to(cubo))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}