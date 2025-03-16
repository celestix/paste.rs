use std::{collections::HashMap, sync::RwLock};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

const VERSION: &str = env!("CARGO_PKG_VERSION");

trait Store {
    fn get(&self, key: String) -> Option<String>;
    fn set(&self, key: String, value: String);
}

struct InMemoryStore {
    map: RwLock<HashMap<String, String>>,
}

impl InMemoryStore {
    fn new() -> Self {
        InMemoryStore {
            map: RwLock::new(HashMap::new()),
        }
    }
}

impl Store for InMemoryStore {
    fn get(&self, key: String) -> Option<String> {
        self.map.read().unwrap().get(&key).cloned()
    }

    fn set(&self, key: String, value: String) {
        self.map.write().unwrap().insert(key, value);
    }
}

#[get("/")]
async fn index() -> impl Responder {
    format!("paste.rs v{}", VERSION)
}

#[get("/{name}")]
async fn get_paste(name: web::Path<String>, store: web::Data<Box<dyn Store>>) -> impl Responder {
    let s =  store.get(name.into_inner());
    match s {
        Some(s) => s,
        None => format!("paste not found"),
    }
}

#[post("/{name}")]
async fn save_paste(name: web::Path<String>, pval: String, store: web::Data<Box<dyn Store>>) -> impl Responder {
    store.set(name.into_inner(), pval);
    HttpResponse::Accepted()
}

fn setup_app_config(cfg: &mut web::ServiceConfig) {
    let paste_map = Box::new(InMemoryStore::new()) as Box<dyn Store>;
    cfg
    .service(index)
    .service(save_paste)
    .service(get_paste)
    .app_data(web::Data::new(paste_map));
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // std::env::set_var("RUST_LOG", "debug");
    // std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    HttpServer::new(
        || App::new()
            .wrap(actix_web::middleware::Logger::default())
            .configure(setup_app_config)
    ).bind(("127.0.0.1", 8080))?.run().await
}