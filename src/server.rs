use rocket::{
    fs::{NamedFile, TempFile},
    http::Status,
    request::{FromRequest, Outcome},
    Build, Request, Rocket,
};
use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
};
use std::{fs::remove_dir_all, str};

use crate::{
    auth_guard::AuthGuard,
    users::{get_site_owner, get_user_sites, insert_site, reset_password, Site},
    SITES_FOLDER,
};

const BASE_HOST_SUFFIX: &str = ".bleebo.dev";
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
    let path = Path::new(SITES_FOLDER).join(subdomain.0).join(file);
    if path.is_file() {
        NamedFile::open(&path).await.ok()
    } else {
        NamedFile::open(path.join("index.html")).await.ok()
    }
}

#[post("/reset-password", data = "<password>")]
async fn set_password(username: AuthGuard, password: String) -> Result<&'static str, &'static str> {
    if password.len() < 8 {
        return Err("Password must be at least 8 characters long");
    }
    println!("{}", password);
    match reset_password(&username.0, &password) {
        Ok(_) => Ok("Successfully changed password"),
        Err(_) => Err("Reset password failed"),
    }
}

#[post("/upload/<path..>", data = "<file>")]
async fn upload(username: AuthGuard, path: PathBuf, mut file: TempFile<'_>) -> std::io::Result<()> {
    let site_name = path
        .ancestors()
        .collect::<Vec<_>>()
        .iter()
        .rev()
        .skip(1)
        .next() // get the last parent
        .and_then(|p| p.to_str())
        .ok_or(std::io::Error::new(
            std::io::ErrorKind::Other,
            "no site name found in path, or site name was not valid unicode",
        ))?;
    let all_sites = get_user_sites(&username.0).map_err(|_| {
        std::io::Error::new(std::io::ErrorKind::Other, "failed to retrieve user sites")
    })?;
    if site_name == "" {
        // the site name probably was not the last parent
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "no filename was provided",
        ));
    }
    if !all_sites.contains(&Site {
        site_name: site_name.to_string(),
        owner_name: username.0.to_string(),
    }) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("{} does not belong to {}", site_name, username.0),
        ));
    }
    create_dir_all(
        Path::new(SITES_FOLDER).join(path.parent().ok_or(std::io::Error::new(
            std::io::ErrorKind::Other,
            "could not get site parent",
        ))?),
    )?;
    file.persist_to(Path::new(SITES_FOLDER).join(path)).await
}

#[post("/delete/<site_name>")]
async fn delete_site(username: AuthGuard, site_name: &str) -> Result<&'static str, &'static str> {
    let all_sites = get_user_sites(&username.0).or(Err("failed to retrieve user sites"))?;
    if site_name == "" {
        return Err("no site name was provided");
    }
    if !all_sites.contains(&Site {
        site_name: site_name.to_string(),
        owner_name: username.0.to_string(),
    }) {
        return Err("site does not belong to user");
    }
    if !Path::new(SITES_FOLDER).join(site_name).is_dir() {
        return Ok("site already doesn't exist");
    }
    remove_dir_all(Path::new(SITES_FOLDER).join(site_name)).or(Err("failed to delete site"))?;
    Ok("successfully deleted site")
}

#[post("/new/<site_name>")]
async fn new_site(username: AuthGuard, site_name: &str) -> Result<(), &'static str> {
    if site_name == "" {
        return Err("no site name was provided");
    }
    match get_site_owner(&site_name) {
        Ok(owner) => match owner {
            Some(owner) => {
                if owner != username.0 {
                    return Err("site belongs to a different user");
                }
                return Ok(());
            }
            None => (),
        },
        Err(_) => return Err("failed to verify current owner of site"),
    }
    insert_site(&username.0, &site_name).or(Err("failed to create new site"))?;
    Ok(())
}

#[get("/", rank = 2)]
async fn home() -> &'static str {
    "🦈"
}

pub fn build() -> Rocket<Build> {
    rocket::build().mount(
        "/",
        routes![files, home, set_password, upload, delete_site, new_site],
    )
}
