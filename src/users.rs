use std::error;

use anyhow::bail;
use passwords::PasswordGenerator;
use rusqlite::{params, Connection};

use crate::{password::hash_password, DB};

const PG: PasswordGenerator = PasswordGenerator {
    length: 16,
    numbers: true,
    lowercase_letters: true,
    uppercase_letters: true,
    symbols: false,
    spaces: false,
    exclude_similar_characters: false,
    strict: true,
};

#[derive(Debug)]
pub struct User {
    pub username: String,
    pub password_hash: String,
    pub reset_password: bool,
}

// `Box`ing errors:
// https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/boxing_errors.html
pub fn insert_new_user(username: &str) -> Result<String, Box<dyn error::Error>> {
    let conn = Connection::open(DB)?;

    let password = PG.generate_one()?;
    let password_hash = hash_password(&password);

    let user = User {
        username: username.to_string(),
        password_hash: password_hash,
        reset_password: false,
    };

    conn.execute(
        "INSERT INTO users VALUES (?1, ?2, ?3)",
        params![user.username, user.password_hash, user.reset_password],
    )?;

    Ok(password)
}

pub fn reset_password(username: &str, password: &str) -> Result<(), Box<dyn error::Error>> {
    let conn = Connection::open(DB)?;

    let password_hash = hash_password(&password);

    conn.execute(
        "UPDATE users SET reset_password = true, password_hash = ?1 WHERE username = ?2",
        params![password_hash, username],
    )?;

    Ok(())
}

pub fn insert_site(username: &str, sitename: &str) -> Result<(), Box<dyn error::Error>> {
    let conn = Connection::open(DB)?;

    conn.execute(
        "INSERT INTO sites VALUES (?1, ?2)",
        params![sitename, username],
    )?;

    Ok(())
}

#[derive(PartialEq, Eq)]
pub struct Site {
    pub site_name: String,
    pub owner_name: String,
}
pub fn get_user_sites(username: &str) -> Result<Vec<Site>, anyhow::Error> {
    let conn = Connection::open(DB)?;

    let mut stmt = conn.prepare("SELECT * FROM sites WHERE owner_name = ?1")?;
    let sites_iter = stmt.query_map(params![username], |row| {
        Ok(Site {
            site_name: row.get(0)?,
            owner_name: row.get(1)?,
        })
    })?;

    let sites = sites_iter.filter_map(|x| x.ok()).collect();

    Ok(sites)
}

pub fn get_site_owner(site_name: &str) -> Result<Option<String>, anyhow::Error> {
    let conn = Connection::open(DB)?;

    let mut stmt = conn.prepare("SELECT * FROM sites WHERE site_name = ?1")?;
    let sites_iter = stmt.query_map(params![site_name], |row| {
        Ok(Site {
            site_name: row.get(0)?,
            owner_name: row.get(1)?,
        })
    })?;

    let sites: Vec<_> = sites_iter.filter_map(|x| x.ok()).collect();
    if sites.len() > 1 {
        bail!("there should only be one site with the name {}", site_name)
    }
    return Ok(sites.get(0).and_then(|site| Some(site.owner_name.clone())));
}
