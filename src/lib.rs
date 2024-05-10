#[macro_use]
extern crate rocket;

pub mod auth_guard;
pub mod client;
pub mod password;
pub mod server;
pub mod users;

const DB: &'static str = "db";

// be careful when setting this!
const SITES_FOLDER: &'static str = "sites";
