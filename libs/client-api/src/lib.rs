mod http;
pub use http::*;

#[cfg(feature = "collab-sync")]
pub mod collab_sync;

pub mod notify;

#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(not(target_arch = "wasm32"))]
pub use native::*;

#[cfg(target_arch = "wasm32")]
mod wasm;
#[cfg(target_arch = "wasm32")]
pub use wasm::*;

pub mod ws;

pub mod error {
  pub use shared_entity::response::AppResponseError;
  pub use shared_entity::response::ErrorCode;
}

// Export all dto entities that will be used in the frontend application
pub mod entity {
  pub use collab_rt_entity::user::*;
  pub use database_entity::dto::*;
  pub use gotrue_entity::dto::*;
  pub use shared_entity::dto::*;
}

#[cfg(feature = "template")]
pub mod template {
  pub use workspace_template;
}
