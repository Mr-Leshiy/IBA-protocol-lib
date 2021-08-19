use std::io::{self};

fn main() {
    let mut service = network::NetworkService::new("chat".into(), |data| {
        println!("Received: '{:?}'", String::from_utf8_lossy(&data));
    })
    .unwrap();

        network::NetworkService::run(&mut service);

    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        service.broadcast_msg(line);
    }
}
