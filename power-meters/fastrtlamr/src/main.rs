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

fn open_stream(addr: &str) -> Result<TcpStream, String> {
    match TcpStream::connect(addr) {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 1234");

            let mut data = [0 as u8; 4];
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    if &data == RTLTCP_MAGIC {
                        println!("Magic number ok");
                        Ok(stream)
                    } else {
                        let text = from_utf8(&data).unwrap();
                        Err(format!("Unexpected magic number: {}", text))
                    }
                }
                Err(e) => Err(format!("Failed to receive data: {}", e)),
            }
        }
        Err(e) => Err(format!("Failed to connect: {}", e)),
    }
}

fn main() {
    open_stream("localhost:1235").unwrap_or_else(|e| panic!("connection error: {}", e));
}
