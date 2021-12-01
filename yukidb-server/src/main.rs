use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    net::{TcpListener, TcpStream},
};

use clap::Parser;
use threadpool::ThreadPool;

#[derive(Parser)]
#[clap(about = "The high performance concurrent key-value in-memory database")]
struct Opts {
    #[clap(short, long)]
    port: u16,
    #[clap(long, default_value = "1")]
    max_conn: usize,
}

fn handle_connection(read_stream: BufReader<TcpStream>, mut write_stream: BufWriter<TcpStream>) {
    let mut reads = read_stream.lines();

    while let Ok(line) = reads.next().unwrap() {
        println!("from client: {}", line);
        write_stream.write_all(b"received!\n").unwrap();
        write_stream.flush().unwrap();
    }
}

fn main() {
    let opts = Opts::parse();

    let pool = ThreadPool::new(opts.max_conn);
    let serveraddr = format!("0.0.0.0:{}", opts.port);
    let listener = TcpListener::bind(serveraddr.clone())
        .expect(&format!("Failed to open server. {}", serveraddr));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let read_stream = BufReader::new(stream.try_clone().unwrap());
                let mut write_stream = BufWriter::new(stream);

                if pool.active_count() == opts.max_conn {
                    write_stream
                        .write_all(b"The connections are full.\n")
                        .unwrap();
                    write_stream.flush().unwrap();
                } else {
                    write_stream.write_all(b"YukiDB v0.1.0\n").unwrap();
                    write_stream.flush().unwrap();
                    pool.execute(|| handle_connection(read_stream, write_stream));
                }
            }
            Err(_) => todo!("?"),
        }
    }
}
