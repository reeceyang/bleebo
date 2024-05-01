use rocket::{
    fs::NamedFile,
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};
use std::path::{Path, PathBuf};

#[macro_use]
extern crate rocket;

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
    NamedFile::open(Path::new("site/").join(subdomain.0).join(file))
        .await
        .ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![files])
}
