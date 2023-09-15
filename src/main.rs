use anniv_edit::{AnnivClient, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let anniv = AnnivClient::new("http://127.0.0.1:8080".parse().unwrap());
    let info = anniv.login("snylonue@proton.me", "local").await?;

    println!("{info:#?}");

    Ok(())
}
