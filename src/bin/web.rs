use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use music_practice_app::models::{PracticeSession, SheetMusic};
use music_practice_app::storage;
use uuid::Uuid;

async fn add_practice(practice: web::Json<PracticeSession>) -> impl Responder {
    match storage::save_practice_session(&practice.into_inner()) {
        Ok(_) => HttpResponse::Ok().json("Practice session added successfully"),
        Err(_) => HttpResponse::InternalServerError().json("Failed to add practice session"),
    }
}

async fn list_practices() -> impl Responder {
    match storage::list_practice_sessions() {
        Ok(sessions) => HttpResponse::Ok().json(sessions),
        Err(_) => HttpResponse::InternalServerError().json("Failed to list practice sessions"),
    }
}

async fn view_practice(id: web::Path<Uuid>) -> impl Responder {
    match storage::read_practice_session(&id) {
        Ok(session) => HttpResponse::Ok().json(session),
        Err(_) => HttpResponse::NotFound().json("Practice session not found"),
    }
}

async fn add_sheet(sheet: web::Json<SheetMusic>) -> impl Responder {
    match storage::save_sheet_music(&sheet.into_inner()) {
        Ok(_) => HttpResponse::Ok().json("Sheet music added successfully"),
        Err(_) => HttpResponse::InternalServerError().json("Failed to add sheet music"),
    }
}

async fn list_sheets() -> impl Responder {
    match storage::list_sheet_music() {
        Ok(sheets) => HttpResponse::Ok().json(sheets),
        Err(_) => HttpResponse::InternalServerError().json("Failed to list sheet music"),
    }
}

async fn view_sheet(name: web::Path<String>) -> impl Responder {
    match storage::read_sheet_music(&name) {
        Ok(sheet) => HttpResponse::Ok().json(sheet),
        Err(_) => HttpResponse::NotFound().json("Sheet music not found"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/practice", web::post().to(add_practice))
            .route("/practice", web::get().to(list_practices))
            .route("/practice/{id}", web::get().to(view_practice))
            .route("/sheet", web::post().to(add_sheet))
            .route("/sheet", web::get().to(list_sheets))
            .route("/sheet/{name}", web::get().to(view_sheet))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
