use speculum2::Mirrors;

#[tokio::main]
async fn main() {
    match Mirrors::retrieve().await {
        Ok(mirrors) => {
            for mirror in mirrors {
                println!("{mirror:?}");
                println!("{}", mirror.url());
                println!("{:?}", mirror.last_sync());
            }
        }
        Err(error) => eprintln!("{error}"),
    }
}
