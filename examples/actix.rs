use actix_files::{Files, NamedFile};
use actix_web::{
    error::ResponseError,
    web::{self, Json},
    App, HttpResponse, HttpServer, Responder, Result,
};
use editorjs2html::value_to_html;
use log::info;
use sailfish::TemplateOnce;
use serde_json::{json, Value};
use std::{
    fmt,
    fs::File,
    io::{BufReader, BufWriter},
    path::PathBuf,
};
use tracing_actix_web::TracingLogger;

// Config structure for better maintainability
#[derive(Clone)]
struct AppConfig {
    data_path: PathBuf,
    bind_address: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            data_path: PathBuf::from("./examples/data.json"),
            bind_address: String::from("0.0.0.0:8080"),
        }
    }
}

// Custom error type for unified error handling
#[derive(Debug)]
enum AppError {
    Io(std::io::Error),
    Json(serde_json::Error),
    Editor(anyhow::Error),
    Template(sailfish::RenderError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "I/O error: {e}"),
            AppError::Json(e) => write!(f, "JSON error: {e}"),
            AppError::Editor(e) => write!(f, "Editor error: {e}"),
            AppError::Template(e) => write!(f, "Template error: {e}"),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError().json(json!({ "error": self.to_string() }))
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::Io(e)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        AppError::Json(e)
    }
}

impl From<anyhow::Error> for AppError {
    fn from(e: anyhow::Error) -> Self {
        AppError::Editor(e)
    }
}

impl From<sailfish::RenderError> for AppError {
    fn from(e: sailfish::RenderError) -> Self {
        AppError::Template(e)
    }
}

// Data service layer for persistence
struct DataService {
    config: AppConfig,
}

impl DataService {
    fn new(config: &AppConfig) -> Self {
        Self {
            config: config.clone(),
        }
    }

    fn read_data(&self) -> Result<Value, AppError> {
        let file = File::open(&self.config.data_path)?;
        let reader = BufReader::new(file);
        Ok(serde_json::from_reader(reader)?)
    }

    fn write_data(&self, data: &Value) -> Result<(), AppError> {
        let file = File::create(&self.config.data_path)?;
        let writer = BufWriter::new(file);
        Ok(serde_json::to_writer(writer, data)?)
    }
}

#[derive(TemplateOnce)]
#[template(path = "home.stpl")]
struct HomeTemplate {
    data: String,
}

#[derive(TemplateOnce)]
#[template(path = "editorjs.stpl")]
struct EditorTemplate;

// Handlers
async fn index(data_service: web::Data<DataService>) -> Result<impl Responder> {
    let data = data_service.read_data()?;
    let html_data = value_to_html(&data).unwrap();

    info!("Rendered data with length: {}", html_data.len());
    Ok(HttpResponse::Ok().body(HomeTemplate { data: html_data }.render_once().unwrap()))
}

async fn editorjs() -> Result<impl Responder> {
    Ok(HttpResponse::Ok().body(EditorTemplate.render_once().unwrap()))
}

async fn editorjs_data_read(data_service: web::Data<DataService>) -> Result<Json<Value>> {
    data_service.read_data().map(Json).map_err(Into::into)
}

async fn editorjs_data_write(
    data_service: web::Data<DataService>,
    data: Json<Value>,
) -> Result<Json<Value>> {
    data_service.write_data(&data.into_inner())?;
    Ok(Json(json!({"success": true})))
}

async fn fallback() -> impl Responder {
    NamedFile::open("./examples/templates/assets/404.html").unwrap()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt().init();

    let config = AppConfig::default();
    let moved_config = config.clone();
    info!("Starting server at {}", config.bind_address);

    HttpServer::new(move || {
        let data_service = web::Data::new(DataService::new(&moved_config));

        App::new()
            .service(Files::new("/assets", "./examples/templates/assets").index_file("404.html"))
            .default_service(web::to(fallback))
            .wrap(TracingLogger::default())
            .app_data(data_service.clone())
            .route("/", web::get().to(index))
            .route("/editorjs", web::get().to(editorjs))
            .service(
                web::resource("/api/content")
                    .route(web::get().to(editorjs_data_read))
                    .route(web::post().to(editorjs_data_write)),
            )
    })
    .bind(&config.bind_address)?
    .run()
    .await
}
