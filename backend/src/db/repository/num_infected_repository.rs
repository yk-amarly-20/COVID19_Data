use crate::db::models::new_num_infected::NewNumInfected;
use crate::db::models::num_infected::NumInfected;
use crate::db::usecase::repository::Inum_infected_repository::INumInfectedRepository;
// これよくわかってない
use crate::diesel::RunQueryDsl;
use diesel::PgConnection;

pub struct NumInfectedRepository {}

impl INumInfectedRepository for NumInfectedRepository {
  fn insert<'a>(
    conn: &PgConnection,
    date: &'a str,
    prefecture_id: &'a i32,
    num_of_infected: &'a i32,
  ) -> NumInfected {
    use crate::schema::num_infected;

    let new_num_infected = NewNumInfected {
      date,
      prefecture_id,
      num_of_infected,
    };

    diesel::insert_into(num_infected::table)
      .values(new_num_infected)
      .get_result(conn)
      .expect("Error saving new num_infected")
  }
}
