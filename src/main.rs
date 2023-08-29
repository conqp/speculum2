use speculum2::Mirrors;

#[tokio::main]
async fn main() {
    match Mirrors::retrieve().await {
        Ok(mirrors) => {
            for mirror in mirrors {
                eprintln!("{mirror:?}");
            }
        }
        Err(error) => eprintln!("{error}"),
    }
}
