// use std::io::Write;
// use std::net::TcpStream;
//
// pub fn client(host: &String, port: &u16) -> Result<(), String> {
//     let addr = format!("{}:{}", host, port);
//     let mut client = TcpStream::connect(addr.as_str()).map_err(|_| format!("failed to connect  {}", addr))?;
//
//     client.write("Hello TCP".as_bytes()).map_err(|_| format!("failed to send"))?;
//
//     Ok(())
// }