use actix_web::{web, App, HttpServer};

mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let scope_data = web::scope("/data")
    //     .app_data(web::Data::new(routes::AppState {
    //         app_name: String::from("Actix Web"),
    //     }))
    //     .service(routes::data1)
    //     .service(routes::data2);

    HttpServer::new(|| {
        App::new()
            .service(routes::hello)
            .app_data(web::Data::new(routes::AppState {
                app_name: String::from("actix Web"),
            }))
            .service(routes::data1)
            .service(routes::users)
            .service(routes::info)
        // .route("/", web::to(HttpResponse::Ok))
        //     routes::hello)
        // .service(routes::echo)
        // .service(scope_data)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
