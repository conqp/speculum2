use speculum2::Mirrors;

#[tokio::main]
async fn main() {
    match Mirrors::retrieve().await {
        Ok(mut mirrors) => {
            mirrors.measure().await;

            for mirror in mirrors {
                println!("{mirror:?}");
                println!("{}", mirror.url());
                println!("{:?}", mirror.last_sync());
                println!("{:?}", mirror.download_time());
            }
        }
        Err(error) => eprintln!("{error}"),
    }
}
