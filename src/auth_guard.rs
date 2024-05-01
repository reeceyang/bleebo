use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};
use rocket_authorization::{basic::Basic, Credential};
use rusqlite::{params, Connection};

#[derive(Debug)]
pub struct AuthGuard(pub String);

#[derive(Debug)]
struct User {
    username: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthGuard {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let provided_auth = match Credential::<Basic>::from_request(request).await {
            Outcome::Success(auth) => auth,
            Outcome::Error(error) => return Outcome::Error((error.0, ())),
            Outcome::Forward(status) => return Outcome::Forward(status),
        };

        let conn = match Connection::open("db") {
            Ok(conn) => conn,
            Err(_) => return Outcome::Error((Status::InternalServerError, ())),
        };

        let mut stmt = match conn.prepare("SELECT username FROM users WHERE username = ?1") {
            Ok(stmt) => stmt,
            Err(_) => return Outcome::Error((Status::InternalServerError, ())),
        };
        let mut users_iter = match stmt.query_map(params![&provided_auth.username], |row| {
            Ok(User {
                username: row.get(0)?,
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
        Outcome::Success(AuthGuard(user.username))
    }
}
