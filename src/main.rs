use bleebo::server;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    server::build().launch().await?;

    Ok(())
}
