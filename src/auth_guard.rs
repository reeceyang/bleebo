use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};
use rocket_authorization::{basic::Basic, Credential};
use rusqlite::{params, Connection};

use crate::{password::verify_password, users::User, DB};

#[derive(Debug)]
pub struct AuthGuard(pub String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthGuard {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let provided_auth = match Credential::<Basic>::from_request(request).await {
            Outcome::Success(auth) => auth,
            Outcome::Error(error) => return Outcome::Error((error.0, ())),
            Outcome::Forward(status) => return Outcome::Forward(status),
        };
        let conn = match Connection::open(DB) {
            Ok(conn) => conn,
            Err(_) => return Outcome::Error((Status::InternalServerError, ())),
        };

        let mut stmt = match conn.prepare("SELECT * FROM users WHERE username = ?1") {
            Ok(stmt) => stmt,
            Err(_) => return Outcome::Error((Status::InternalServerError, ())),
        };
        let mut users_iter = match stmt.query_map(params![&provided_auth.username], |row| {
            Ok(User {
                username: row.get(0)?,
                password_hash: row.get(1)?,
                reset_password: row.get(2)?,
            })
        }) {
            Ok(users_iter) => users_iter,
            Err(_) => return Outcome::Error((Status::InternalServerError, ())),
        };

        let user_result = match users_iter.next() {
            Some(user_result) => user_result,
            None => return Outcome::Error((Status::Unauthorized, ())),
        };

        let user = match user_result {
            Ok(user) => user,
            Err(_) => return Outcome::Error((Status::Unauthorized, ())),
        };

        // TODO: check the password
        match verify_password(&provided_auth.password, &user.password_hash) {
            Ok(_) => Outcome::Success(AuthGuard(user.username)),
            Err(_) => Outcome::Error((Status::Unauthorized, ())),
        }
    }
}
