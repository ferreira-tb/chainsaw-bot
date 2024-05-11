pub mod prelude {
  // pub use super::Chainsaw;
}

use sea_orm::DatabaseConnection;

pub struct Chainsaw {
  #[allow(dead_code)]
  pub db: DatabaseConnection,
}
