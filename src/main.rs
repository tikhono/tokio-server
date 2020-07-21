use futures::stream::StreamExt;
use rand::Rng;
use std::{thread, time};
use tokio::net::TcpListener;
use tokio::prelude::*;

#[tokio::main]

async fn main() {
    let addr = "127.0.0.1:6142";

    let mut listener = TcpListener::bind(addr).await.unwrap();
    //    uncomment to see how server works asyncronosly with 2 or more connected clients
    //    thread::sleep(time::Duration::from_millis(
    //        rand::thread_rng().gen_range(3000, 6000),
    //    ));
    let server = {
        async move {
            let mut incoming = listener.incoming();
            while let Some(conn) = incoming.next().await {
                match conn {
                    Err(e) => eprintln!("accept failed = {:?}", e),
                    Ok(mut sock) => {
                        tokio::spawn(async move {
                            let (reader, mut writer) = sock.split();
                            let cap: i64 = rand::thread_rng().gen_range(1, 21);
                            let numbers: Vec<i64> = (0..cap)
                                .map(|_| rand::thread_rng().gen_range(1, 21))
                                .collect();
                            println!("cap is: {}", cap);
                            println!("numbers is: {:?}", numbers);
                            match writer.write_i64(cap).await {
                                Ok(amt) => {
                                    println!("wrote {}", cap);
                                }
                                Err(err) => {
                                    eprintln!("IO error {:?}", err);
                                }
                            }
                            for i in numbers {
                                match writer.write_i64(i).await {
                                    Ok(amt) => {
                                        println!("wrote {}", i);
                                    }
                                    Err(err) => {
                                        eprintln!("IO error {:?}", err);
                                    }
                                }
                            }
                        });
                    }
                }
            }
        }
    };
    println!("Server running on localhost:6142");
    server.await;
}
