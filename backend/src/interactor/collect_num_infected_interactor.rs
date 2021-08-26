use crate::usecase::Icollect_num_infected_usecase::ICollectNumInfectedUsecase;

pub struct CollectNumInfectedInteractor {}

impl ICollectNumInfectedUsecase for CollectNumInfectedInteractor {
  fn send_and_save_infected(prefecture_name: String) {
    let url = format!(
      "https://opendata.corona.go.jp/api/Covid19JapanAll?dataName={}",
      prefecture_name
    );

    let body = reqwest::get(&url).await?.text().await?;
  }
}
