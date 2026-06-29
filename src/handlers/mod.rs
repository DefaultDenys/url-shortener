mod health;
mod index;
mod redirect;
mod shorten;

pub use health::health_check_handler;
pub use index::index_handler;
pub use redirect::redirect_handler;
pub use shorten::shorten_handler;
