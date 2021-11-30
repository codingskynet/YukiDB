use std::{net::TcpListener, io::Read};

use clap::Parser;

#[derive(Parser)]
#[clap(
    about = "The high performance concurrent key-value in-memory database"
)]
struct Opts {
    #[clap(short, long)]
    port: u16,
}

fn main() {
    let opts = Opts::parse();

    let serveraddr = format!("0.0.0.0:{}", opts.port);
    let listener = TcpListener::bind(serveraddr.clone()).expect(&format!("Failed to open server. {}", serveraddr));

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut data: [u8; 50] = [0; 50];
                match stream.read(&mut data) {
                    Ok(_) => {
                        let text = String::from_utf8(data.to_vec()).expect("error on utf8 converting");
                        println!("from client: {}", text);
                    },
                    Err(_) => todo!("err"),
                }
            },
            Err(_) => todo!("?"),
        }
    }
}
