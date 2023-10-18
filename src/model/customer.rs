use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Customer {
  pub id: String,
  pub first_name: String,
  pub last_name: String,
  pub birth_date: NaiveDate,
  pub address: Vec<Address>,
}

#[derive(Debug,Deserialize, Serialize)]
pub enum AddressType {
  Primary,
  Shipping
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Address {
  pub address_type: AddressType,
  pub street: String,
  pub city: String,
  pub zip: String,
  pub country: Country,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Country {
  DE,
  IT,
  RU,

}

#[cfg(test)]
mod test {
  use tracing::info;
  use tracing_subscriber::EnvFilter;
  use crate::model::customer::{Address, AddressType, Country, Customer};

  #[test]
  fn custom_deserialize() {
    tracing_subscriber::fmt()
      .with_env_filter(EnvFilter::from_default_env())
      .init();


    let c = Customer{
      id: "4711".to_string(),
      first_name: "fn_test".to_string(),
      last_name: "ln_test".to_string(),
      birth_date: Default::default(),
      address: vec![
        Address {
          address_type: AddressType::Primary,
          street: "Bahnhofstr. 23".to_string(),
          city: "Nürnberg".to_string(),
          zip: "90402".to_string(),
          country: Country::DE,
        },
        Address {
          address_type: AddressType::Shipping,
          street: "Bahnhofstr. 4".to_string(),
          city: "Nürnberg".to_string(),
          zip: "90402".to_string(),
          country: Country::DE,
        },
      ],
    };
    let serialized  = serde_json::to_string(&c).unwrap();
    assert!(serialized.contains("id"));
    assert!(serialized.contains("4711"));

    info!("customer: {}", serialized);
  }
}