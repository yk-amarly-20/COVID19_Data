use actix_web::{get, web, HttpResponse, Responder};

#[get("/api/data/infected/all")]
pub async fn get_num_of_infected_over_jp() -> impl Responder {
  HttpResponse::Ok().body(format!("Num of Infected"))
}

#[get("/api/data/infected/{prefecture_id}")]
pub async fn get_num_of_infected_prefecture(
  web::Path(prefecture_id): web::Path<String>,
) -> impl Responder {
  HttpResponse::Ok().body(format!("Num of Infected of {}", prefecture_id))
}
