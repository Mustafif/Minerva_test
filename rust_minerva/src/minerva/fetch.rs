use std::io::{self, Cursor};
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
const LINK: &str = "https://raw.githubusercontent.com/MKProj/MKProj/master/books.json";

pub async fn fetch_book()-> Result<()>{
    let response = reqwest::get(LINK).await?;
    let p = "../books.json";
    let mut file = std::fs::File::create(&p)?;
    let mut content = Cursor::new(response.bytes().await?);
    io::copy(&mut content, &mut file)?;
    Ok(())
}