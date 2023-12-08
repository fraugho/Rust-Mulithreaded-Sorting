use actix_files::Files;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::path::Path;
use std::fs::read_to_string;
mod svg;

#[get("/")]
async fn index() -> HttpResponse {
    // Construct a relative path to "index.html" from the project directory
    let html_path = Path::new("templates/index.html");

    // Read the HTML content from the file
    let html_content = match read_to_string(html_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading HTML file: {:?}", err);
            return HttpResponse::InternalServerError().finish();
        }
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(html_content)
}

#[get("/generate_svg")]
async fn generate_svg() -> impl Responder {
    match svg::svgmake() {
        Ok(_) => HttpResponse::Ok().body("SVG generated successfully."),
        Err(err) => {
            eprintln!("Error generating SVG: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/multi_threaded_sort_demo")]
async fn multi_threaded_sort_demo() -> HttpResponse {    
    // Construct a relative path to "index.html" from the project directory
    let html_path = Path::new("templates/multi_threaded_sort_demo.html");

    // Read the HTML content from the file
    let html_content = match read_to_string(html_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading HTML file: {:?}", err);
            return HttpResponse::InternalServerError().finish();
        }
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(html_content)
}

#[get("/background_info")]
async fn background_info() -> HttpResponse {
    // Construct a relative path to "index.html" from the project directory
    let html_path = Path::new("templates/background_info.html");

    // Read the HTML content from the file
    let html_content = match read_to_string(html_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading HTML file: {:?}", err);
            return HttpResponse::InternalServerError().finish();
        }
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(html_content)
}

#[get("/math")]
async fn mathematical() -> impl Responder {
    let result = 1 + 1;
    HttpResponse::Ok().body(format!("1 + 1 = {}", result))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(mathematical)
            .service(echo)
            .service(background_info)
            .service(multi_threaded_sort_demo)
            .service(generate_svg) // Add this line
            .service(Files::new("/static", "static").show_files_listing())
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
