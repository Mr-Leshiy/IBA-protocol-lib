use std::io::{stdin, BufRead};

#[tokio::main]
async fn main() {
    let (mut service, mut worker) = network::build_network("chat".into(), |data| {
        println!("Received: '{:?}'", String::from_utf8_lossy(&data));
    })
    .unwrap();

    tokio::spawn(async move {
        worker.run().await;
    });

    loop {
        let mut line = String::new();
        stdin().lock().read_line(&mut line).unwrap();
        service.broadcast_msg(line.into()).await.unwrap();
    }
}
