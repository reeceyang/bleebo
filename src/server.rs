use rocket::{
    fs::NamedFile,
    http::Status,
    request::{FromRequest, Outcome},
    Build, Request, Rocket,
};
use std::path::{Path, PathBuf};
use std::str;

use crate::auth_guard::AuthGuard;

const BASE_HOST_SUFFIX: &str = ".bleebo.reeceyang.xyz";
struct Subdomain<'r>(&'r str);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Subdomain<'r> {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let host = match req.host() {
            Some(host) => host,
            None => return Outcome::Error((Status::BadRequest, ())),
        };
        let domain = host.domain();
        let subdomain = match domain.as_str().strip_suffix(BASE_HOST_SUFFIX) {
            Some(subdomain) => subdomain,
            None => return Outcome::Forward(Status::NotFound),
        };
        Outcome::Success(Subdomain(subdomain))
    }
}

#[get("/<file..>")]
async fn files(file: PathBuf, subdomain: Subdomain<'_>) -> Option<NamedFile> {
    let path = Path::new("site/").join(subdomain.0).join(file);
    if path.is_file() {
        NamedFile::open(&path).await.ok()
    } else {
        NamedFile::open(path.join("index.html")).await.ok()
    }
}

#[post("/set-password")]
async fn set_password(username: AuthGuard) -> String {
    username.0.clone()
}

#[get("/")]
async fn home() -> &'static str {
    "🦈"
}

pub fn build() -> Rocket<Build> {
    rocket::build().mount("/", routes![files, home, set_password])
}
