use actix_web::{get, App, HttpServer, Responder};
mod controller;
use controller::collect_infected_data::collect_infected_num;
use controller::infected_prefectures::{
    get_num_of_infected_over_jp, get_num_of_infected_prefecture,
};

#[get("/")]
async fn index() -> impl Responder {
    format!("Hello!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(get_num_of_infected_over_jp)
            .service(get_num_of_infected_prefecture)
            .service(collect_infected_num)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
