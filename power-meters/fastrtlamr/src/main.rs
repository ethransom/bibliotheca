use std::io::Read;
use std::net::TcpStream;
use std::str::from_utf8;

// /// Read the stream data and return stream data & its length
// fn read_stream(stream: &mut TcpStream) -> (Vec<u8>, usize) {
//     let buffer_size = 512;
//     let mut request_buffer = vec![];
//     // let us loop & try to read the whole request data
//     let mut request_len = 0usize;
//     loop {
//         let mut buffer = vec![0; buffer_size];
//         match stream.read(&mut buffer) {
//             Ok(n) => {
//                 if n == 0 {
//                     break;
//                 } else {
//                     request_len += n;
//                 }
//             Err(e) {
//                 println!("Error in reading stream data: {:?}", e);
//                 break;
//             }
//         }
//     }

//     (request_buffer, request_len)
// }
const RTLTCP_MAGIC: &[u8; 4] = b"RTL0";

fn main() {
    println!("Hello, world!");

    match TcpStream::connect("localhost:1234") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 1234");

            let mut data = [0 as u8; 4];
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    if &data == RTLTCP_MAGIC {
                        println!("Magic number ok");
                    } else {
                        let text = from_utf8(&data).unwrap();
                        println!("Unexpected magic number: {}", text);
                    }
                }
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}
