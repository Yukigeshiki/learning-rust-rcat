// use std::io::{Read, stdout, Write};
// use std::net::TcpListener;
//
// pub fn server(bind_host: &String, port: &u16) -> Result<(), String> {
//     let addr = format!("{}:{}", bind_host, port);
//     let listener = TcpListener::bind(addr.as_str()).map_err(|_| format!("failed to bind to {}", addr))?;
//
//     for stream in listener.incoming() {
//         match stream {
//             Ok(mut s) => {
//                 println!("connection accepted");
//
//                 let mut buf = [0; 128];
//                 let mut read_bytes = 0;
//                 while read_bytes == 0 {
//                     read_bytes = s.read(&mut buf).map_err(|_| "failed to read from socket")?;
//                     println!("received bytes {}", read_bytes);
//                 }
//
//                 stdout().write(&buf[0..read_bytes]).map_err(|_| "failed to write to stdout")?;
//                 stdout().flush().unwrap();
//             }
//             Err(e) => {
//                 println!("error while accepting incoming connection - {}", e);
//             }
//         }
//     }
//
//     Ok(())
// }