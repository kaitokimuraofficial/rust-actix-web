use actix_web::{get, post, web, HttpResponse, Responder, Result};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Info {
    info: String,
    info_id: i32
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

// This struct represents state
pub struct AppState {
    pub app_name: String,
}

#[get("/data1")]
async fn data1(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name
    format!("Hello {app_name}!") // <- response with app_name
}

#[get("/data2")]
async fn data2(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name
    format!("GoodBye {app_name}!") // <- response with app_name
}

#[get("/users/{user_id}/{friend}")]
async fn users(path: web::Path<(u32, String)>) -> Result<String> {
    let (user_id, friend) = path.into_inner();
    Ok(format!("Welcome {}, user_id {}!", friend, user_id))
}


#[get("/infos/{info_id}/{info}")] // <- define path parameters
async fn info(info: web::Path<Info>) -> Result<String> {
    Ok(format!(
        "Welcome {}, user_id {}!",
        info.info, info.info_id
    ))
}