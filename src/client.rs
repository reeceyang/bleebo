use dialoguer::{Input, Password};

const SERVER_HOST: &'static str = "http://localhost:8080";

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
