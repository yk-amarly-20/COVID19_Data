use crate::db::models::num_infected::NumInfected;
use diesel::PgConnection;

pub trait INumInfectedRepository {
  fn insert<'a>(
    conn: &PgConnection,
    date: &'a str,
    prefecture_id: &'a i32,
    num_of_infected: &'a i32,
  ) -> NumInfected;
}
