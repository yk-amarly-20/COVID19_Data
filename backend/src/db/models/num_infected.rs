use diesel::Queryable;

#[derive(Queryable)]
pub struct NumInfected {
  pub id: i32,
  pub date: String,
  pub prefecture_id: i32,
  pub num_of_infected: i32,
}
