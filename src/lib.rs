#[macro_use]
extern crate rocket;

pub mod auth_guard;
pub mod client;
pub mod password;
pub mod server;
pub mod users;

const DB: &'static str = "db";

/// directory to serve sites from
// be careful when setting this!
const SITES_FOLDER: &'static str = "site";

/// host suffix to strip from the host header in the server
const BASE_HOST_SUFFIX: &'static str = ".bleebo.dev";

/// server host that the client will send requests to
const SERVER_HOST: &'static str = "https://bleebo.dev";
