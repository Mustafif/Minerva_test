mod minerva;
use minerva::fetch::fetch_book;

#[tokio::main]
async fn main(){
    fetch_book().await.unwrap();
}