use arrayref::array_refs;
use std::io::Read;
use std::net::TcpStream;
use std::str::from_utf8;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let default_addr = String::from("localhost:1234");
    let addr = args.get(1).unwrap_or(&default_addr);

    open_stream(addr).unwrap_or_else(|e| panic!("connection error: {}", e));
}

const RTLTCP_MAGIC_NUM: &[u8; 4] = b"RTL0";

fn open_stream(addr: &str) -> Result<TcpStream, String> {
    let mut stream = match TcpStream::connect(addr) {
        Ok(stream) => stream,
        Err(e) => return Err(format!("Failed to connect: {}", e)),
    };
    println!("Successfully connected to server {}", addr);

    let mut data = [0 as u8; 12];
    if let Err(e) = stream.read_exact(&mut data) {
        return Err(format!("Failed to receive data: {}", e));
    }
    let (magic_bytes, tuner_bytes, gain_bytes) = array_refs![&data, 4, 4, 4];
    if magic_bytes != RTLTCP_MAGIC_NUM {
        let text = from_utf8(magic_bytes).unwrap();
        return Err(format!("Unexpected magic number: {}", text));
    }

    let tuner = u32::from_be_bytes(*tuner_bytes);
    println!("Tuner is {}", tuner_display_name(tuner));

    let gain_count = u32::from_be_bytes(*gain_bytes);
    println!("Gain count is {}", gain_count);

    Ok(stream)
}

fn tuner_display_name(tuner: u32) -> &'static str {
    match tuner {
        1 => "E4000",
        2 => "FC0012",
        3 => "FC0013",
        4 => "FC2580",
        5 => "R820T",
        6 => "R828D",
        _ => "UNKNOWN",
    }
}
