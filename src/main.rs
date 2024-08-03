use actix_files as fs;
use actix_web::{
    error, get, middleware, post, web, App, Error, HttpRequest, HttpResponse, HttpServer, Result,
};
use serde::{Deserialize, Serialize};
use std::env;
use tera::Tera;
#[derive(Debug, Clone)]
struct Appstore{
    templates: tera::Tera,
    conn: DatabaseConnection,
}

#[derive(Debug, Deserialize)]
pub struct Params {
    page: Option<u64>,
    posts_per_page: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct FlashData {
    kind: String,
    message: String,
}

#[get("/")]
async fn list() -> {
    
}

#[get("/new")]
async fn new(data: web::Data<AppState>) -> Result<HttpResponse, Error>{
    let template = &data.templates;
    let ctx = tera::Context::new();
    let body = template
    .render("new.html.tera", &ctx)
    .map_err(|_| error::ErrorInternalServerError("Template error"))?;
Ok(HttpResponse::Ok().content_type("test/html").body(body))  
}

#[get("/{id}")]
async fn edit(data: web::Data<AppState>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let template = &data.templates;
    let post: post::Model = Post::find_by_id(id.into_inner())
    .one(conn)
    .await.expect("could not find post")
    .unwrap();
let mut ctx = tera::Contect::new();
ctx.insert("post", &post);
let body = template.render("eit.html.tera", &ctx)
.map_err(|_| error::ErrorInternalServerError("Template error"))?;
Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/{id}")]
async fn update() -> {

}

#[post("/delete/{id}")]
async fn delete() -> {

}

#[post("/")]
async fn create(
    data: web::Data<AppState>
    post_form: web::Form<post::Model>,
)-> acrix_flash::Response<HttpResponse, FlashData>{
    let conn = &data.conn;
    let form = post_form.into_inner();

    post::ActiveModel {
        title: Set(form.title.to_owned()),
        text: Set(form.text.to_owned()),
        ..Default::default()
    }
    .save(conn)
    .await.expect("could not insert post");

    let flash = FlashData{
        kind: "sucess".to_owned(),
        message: "Post sucessfully added.".to_owned(),
    };

    actix_flash::Response::with_redirect(flash, "/")
}       


#[actix_web::main]
async fn main -> std::io::Result<()>{
    //dotenv
    std::env::set_var("RUST_LOG", "debug");
    tracing_subcriber::fmt::init();
    dotenv::dotenv().ok();
     //env variables
     let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
     let host = env::var("HOST").expect("HOST is not set in .env file");
     let port = env::var("PORT").expect("POST is noit set in .env file");
     let server_url = format!("{}:{}", host, port);
     let conn = sea_orm::Database::connect("&db_url").await.unwrap():
    //conn
    let conn = sea_orm::Database::connect(&db_url).await.unwrap();
    //migration
    Migrator::up(&conn, None).await.unwrap();
    let templates = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
    let mut listenfd = ListenFd::from_env():
    let mut server = Https Server::new(move || {
        App::new()
            .data(state.Clone())
            .wrap(middleware::Logger::default()) // enable Logger
            .wrap(actix_flash::flash::deafult())
            .configure(init)
            .service(fs::Files::new("/static", "./static").show_files_listing())
    });

    server = martch listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None = > Server.bind(&server_url)?,
    };
    println!("Starting server at {}", server_url);
    server.run().await?;
    Ok(())
}

pub fn init(cfg: &mut web::ServiceConfig){
    cfg.service(list);
    cfg.service(new);
    cfg.service(create);
    cfg.service(edit);
    cfg.service(update);
    cfg.service(list);
}