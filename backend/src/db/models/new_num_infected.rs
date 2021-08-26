use crate::schema::num_infected;
use diesel::Insertable;

#[derive(Insertable)]
#[table_name = "num_infected"]
pub struct NewNumInfected<'a> {
  // pub date: &'a chrono::NaiveDateTime,
  pub date: &'a str,
  pub prefecture_id: &'a i32,
  pub num_of_infected: &'a i32,
}
