mod auth;
mod error;
mod collections;
mod ideas;
mod health;
mod role_assignments;
mod utils;

#[cfg(test)] pub mod test;

use actix_web::web;

pub use self::auth::auth_validator;

pub use error::APIError;
pub use auth::{AuthToken, Auth};
pub use utils::ensure_user_collection;

pub fn configure(cfg: &mut web::ServiceConfig) {
    health::configure(cfg);
    collections::configure(cfg);
    role_assignments::configure(cfg);
    ideas::configure(cfg);
}