use futures::stream::StreamExt;
use rand::Rng;
use std::env;
//use std::{thread, time};
use tokio::net::TcpListener;
use tokio::prelude::*;

#[tokio::main]

async fn main() {
    let args: Vec<String> = env::args().collect();
    let arg_len = args.len();
    match arg_len {
        2 => println!("Get adress: \"{}\"", args[1]),
        _ => {
            println!("Specify only one adress in format <host:port>");
            return;
        }
    };

    let mut listener = TcpListener::bind(&args[1]).await.unwrap();
    //    uncomment to explisitly see how server works asyncronosly with 2 or more clients
    //    thread::sleep(time::Duration::from_millis(
    //        rand::thread_rng().gen_range(3000, 6000),
    //    ));
    let server = {
        async move {
            let mut incoming = listener.incoming();
            while let Some(conn) = incoming.next().await {
                match conn {
                    Ok(mut sock) => {
                        tokio::spawn(async move {
                            let (_reader, mut writer) = sock.split();
                            let numbers_count: u8 = rand::thread_rng().gen_range(1, 21);
                            let numbers: Vec<i64> = (0..numbers_count)
                                .map(|_| {
                                    rand::thread_rng().gen_range(i64::MIN, i64::MAX)
                                        + rand::thread_rng().gen_range(0, 2)
                                    // to cover full range [i64::MIN; i64::MAX]
                                })
                                .collect();
                            println!("numbers_count is: {}", numbers_count);
                            println!("numbers are: {:?}", numbers);
                            match writer.write_u8(numbers_count).await {
                                Ok(_amt) => {
                                    println!("wrote {}", numbers_count);
                                }
                                Err(err) => {
                                    eprintln!("IO error {:?}", err);
                                }
                            }
                            for i in numbers {
                                match writer.write_i64(i).await {
                                    Ok(_amt) => {
                                        println!("wrote {}", i);
                                    }
                                    Err(err) => {
                                        eprintln!("IO error {:?}", err);
                                    }
                                }
                            }
                        });
                    }
                    Err(e) => eprintln!("accept failed = {:?}", e),
                }
            }
        }
    };
    println!("Server running on localhost:6142");
    server.await;
}
