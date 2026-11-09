
pub mod parser;

use parser::parameters::Parameters;
use std::{env, println};

use std::io::Write;
use std::net::{TcpStream, SocketAddr};
use std::time::Duration;

use std::os::raw::{c_char, c_int, c_uint, c_uchar};
use std::ffi::CString;

unsafe extern "C" {
    unsafe fn send_custom_tcp(ip_str: *const c_char, dport: c_int, sport: c_int, seq: c_uint, flags: c_uchar) -> c_int;
    unsafe fn wait_for_response(target_port: c_int, timeout_sec: c_int) -> c_int;
}

pub fn scan_port(ip: &str, port: u16, timeout: Duration) -> bool {
    let addr: SocketAddr = match format!("{}:{}", ip, port).parse() {
        Ok(a) => a,
        Err(_) => return false,
    };
    let c_ip = CString::new(ip).unwrap();
    unsafe {
        send_custom_tcp(c_ip.as_ptr(), port as c_int, 54321, 42, 0x02);
        wait_for_response(port as c_int, 2);
    }

    true
}

fn main() -> Result<(), String> {

    let args = env::args().skip(1);
    // let param = Parameters::new_argv(args)?;

    scan_port("127.0.0.1", 8080, Duration::from_secs(2));
    Ok(())
}


