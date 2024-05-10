use std::{
    fs::{self, File},
    io,
    path::Path,
};

use dialoguer::{Input, Password};

use crate::SERVER_HOST;

struct Auth {
    username: String,
    password: String,
}

fn collect_auth() -> Auth {
    let username: String = Input::new()
        .with_prompt("username")
        .interact_text()
        .expect("user should have entered username");
    let password = Password::new()
        .with_prompt("password")
        .interact()
        .expect("user should have entered password");

    Auth { username, password }
}

pub fn upload(site_name: &str) {
    let auth = &collect_auth();
    let Auth { username, password } = auth;

    let client = reqwest::blocking::Client::new();

    // if needed, create a new site for this user
    match client
        .post(&format!("{}/new/{}", SERVER_HOST, site_name))
        .basic_auth(&username, Some(&password))
        .send()
        .and_then(|body| body.error_for_status())
        .and_then(|body| body.text())
    {
        Ok(text) => {
            println!("{}", text);
        }
        Err(e) => return println!("Error: {}", e),
    };

    // delete the existing site files
    match client
        .post(&format!("{}/delete/{}", SERVER_HOST, site_name))
        .basic_auth(&username, Some(&password))
        .send()
        .and_then(|body| body.error_for_status())
        .and_then(|body| body.text())
    {
        Ok(_) => (),
        Err(e) => return println!("Error: {}", e),
    };

    // https://doc.rust-lang.org/std/fs/fn.read_dir.html#examples
    fn visit_dirs(dir: &Path, site_name: &str, auth: &Auth) -> io::Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path
                    .file_stem()
                    .and_then(|stem| stem.to_str())
                    .is_some_and(|stem| stem.starts_with("."))
                {
                    // skip dot files and directories
                    continue;
                }
                if path.is_dir() {
                    visit_dirs(&path, site_name, &auth)?;
                } else {
                    let file = match File::open(&path) {
                        Ok(file) => file,
                        Err(e) => {
                            println!("Error: {}", e);
                            return Err(io::Error::new(
                                io::ErrorKind::Other,
                                "failed to open file",
                            ));
                        }
                    };
                    let client = reqwest::blocking::Client::new();
                    match client
                        .post(&format!(
                            "{}/upload/{}/{}",
                            SERVER_HOST,
                            site_name,
                            path.display()
                        ))
                        .basic_auth(&auth.username, Some(&auth.password))
                        .body(file)
                        .send()
                        .and_then(|body| body.error_for_status())
                        .and_then(|body| body.text())
                    {
                        Ok(_) => {
                            println!("Uploaded {}", path.display());
                        }
                        Err(e) => println!("Error: {}", e),
                    };
                }
            }
        }
        Ok(())
    }

    match visit_dirs(Path::new("."), site_name, auth) {
        Ok(_) => println!("upload successful"),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn delete(site_name: &str) {
    let Auth { username, password } = collect_auth();

    let client = reqwest::blocking::Client::new();
    let response_text = client
        .post(&format!("{}/delete/{}", SERVER_HOST, site_name))
        .basic_auth(username, Some(password))
        .send()
        .and_then(|body| body.error_for_status())
        .and_then(|body| body.text());

    match response_text {
        Ok(text) => {
            println!("{}", text);
        }
        Err(e) => println!("Error: {}", e),
    }
}

pub fn change_password() {
    let username: String = Input::new()
        .with_prompt("username")
        .interact_text()
        .expect("user should have entered username");
    let old_password = Password::new()
        .with_prompt("old password")
        .interact()
        .expect("user should have entered password");

    let mut new_password;
    let mut new_password2;

    loop {
        new_password = Password::new()
            .with_prompt("new password")
            .interact()
            .expect("user should have entered password");

        new_password2 = Password::new()
            .with_prompt("confirm new password")
            .interact()
            .expect("user should have entered password");

        if new_password != new_password2 {
            println!("new passwords don't match!")
        } else {
            break;
        }
    }

    let client = reqwest::blocking::Client::new();
    let response_text = client
        .post(&format!("{}/reset-password", SERVER_HOST))
        .basic_auth(username, Some(old_password))
        .body(new_password)
        .send()
        .and_then(|body| body.error_for_status())
        .and_then(|body| body.text());

    match response_text {
        Ok(text) => {
            println!("{}", text);
        }
        Err(e) => println!("Error: {}", e),
    }
}
