// Module imports organized and grouped for clarity
use actix_files::Files;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::fs::read_to_string;
use std::path::Path;
use serde::Deserialize;

// Local module import
mod svg;

/// Serves the main index page.
#[get("/")]
async fn index() -> HttpResponse {
    let html_path = Path::new("templates/index.html");
    match read_to_string(html_path) {
        Ok(content) => HttpResponse::Ok().content_type("text/html").body(content),
        Err(err) => {
            eprintln!("Failed to read index HTML: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[derive(Deserialize)]
pub struct SvgQuery {
    user_input: u32, // Use the appropriate type for your parameter
}

/// Generates an SVG and responds with a success message.
#[get("/generate_svg")]
async fn generate_svg (query: web::Query<SvgQuery>) -> impl Responder {
    let user_input = query.user_input;
    match svg::svgmake(user_input) {
        Ok(_) => HttpResponse::Ok().body("SVG generated successfully."),
        Err(err) => {
            eprintln!("SVG generation failed: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Serves the multi-threaded sort demo page.
#[get("/multi_threaded_sort_demo")]
async fn multi_threaded_sort_demo() -> HttpResponse {
    let html_path = Path::new("templates/multi_threaded_sort_demo.html");
    match read_to_string(html_path) {
        Ok(content) => HttpResponse::Ok().content_type("text/html").body(content),
        Err(err) => {
            eprintln!("Failed to read multi-threaded sort demo HTML: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Serves the background information page.
#[get("/background_info")]
async fn background_info() -> HttpResponse {
    let html_path = Path::new("templates/background_info.html");
    match read_to_string(html_path) {
        Ok(content) => HttpResponse::Ok().content_type("text/html").body(content),
        Err(err) => {
            eprintln!("Failed to read background info HTML: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// A simple math calculation to demonstrate functionality.
#[get("/math")]
async fn mathematical() -> impl Responder {
    HttpResponse::Ok().body(format!("1 + 1 = {}", 1 + 1))
}

/// Echoes the request body.
#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

/// Responds with a simple greeting.
async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

/// Main function to start the Actix web server.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(mathematical)
            .service(echo)
            .service(background_info)
            .service(multi_threaded_sort_demo)
            .service(generate_svg)
            .service(Files::new("/static", "static").show_files_listing())
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
