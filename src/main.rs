use speculum2::{measure, Mirrors};

#[tokio::main]
async fn main() {
    env_logger::init();

    match Mirrors::retrieve().await {
        Ok(mirrors) => {
            let mut mirrors: Vec<_> = mirrors.urls().iter().cloned().collect();
            measure(&mut mirrors).await;

            for mirror in mirrors {
                println!("{mirror:?}");
            }
        }
        Err(error) => eprintln!("{error}"),
    }
}
