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
const RTLTCP_MAGIC_NUM: &[u8; 4] = b"RTL0";

fn open_stream(addr: &str) -> Result<TcpStream, String> {
    let mut stream = match TcpStream::connect(addr) {
        Ok(stream) => stream,
        Err(e) => return Err(format!("Failed to connect: {}", e)),
    };
    println!("Successfully connected to server {}", addr);

    let mut data = [0 as u8; 4];
    if let Err(e) = stream.read_exact(&mut data) {
        return Err(format!("Failed to receive data: {}", e));
    }
    if &data != RTLTCP_MAGIC_NUM {
        let text = from_utf8(&data).unwrap();
        return Err(format!("Unexpected magic number: {}", text));
    }

    Ok(stream)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let default_addr = String::from("localhost:1234");
    let addr = args.get(1).unwrap_or(&default_addr);

    open_stream(addr).unwrap_or_else(|e| panic!("connection error: {}", e));
}
