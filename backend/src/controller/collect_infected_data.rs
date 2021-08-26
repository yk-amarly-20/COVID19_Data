use actix_web::{get, HttpResponse, Responder};

#[get("/api/collect/data/infected/all")]
pub async fn collect_infected_num() -> impl Responder {
    HttpResponse::Ok().body(format!("collect data of infected"))
}
