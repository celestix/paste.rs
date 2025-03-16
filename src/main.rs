use std::sync::Arc;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

mod storage;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[get("/")]
async fn index() -> impl Responder {
    format!("paste.rs v{}", VERSION)
}

#[get("/{name}")]
async fn get_paste(
    name: web::Path<String>,
    store: web::Data<Arc<dyn storage::Storage>>,
) -> impl Responder {
    let s = store.get(name.into_inner());
    match s {
        Some(s) => s,
        None => format!("paste not found"),
    }
}

#[post("/{name}")]
async fn save_paste(
    name: web::Path<String>,
    pval: String,
    store: web::Data<Arc<dyn storage::Storage>>,
) -> impl Responder {
    store.set(name.into_inner(), pval);
    HttpResponse::Accepted()
}

fn setup_app_config(cfg: &mut web::ServiceConfig) {
    cfg.service(index).service(save_paste).service(get_paste);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let paste_map = storage::get_storage();
    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .app_data(web::Data::new(paste_map.clone()))
            .configure(setup_app_config)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
