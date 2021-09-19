use std::net::{TcpStream};
use std::time::Duration;
use std::str;
use std::io::{BufRead, BufReader, Write};

#[cfg(any(target_os = "unix", target_os = "wasi"))]
fn get_and_go(stream: TcpStream) {
  let fd = stream.as_raw_fd();

  disp_unix_or_wasi(fd);
}

#[cfg(target_os = "unix")]
use std::os::linux::io::{RawFd, AsRawFd, FromRawFd};
#[cfg(target_os = "wasi")]
use std::os::wasi::io::{RawFd, AsRawFd, FromRawFd};

#[cfg(any(target_os = "unix", target_os = "wasi"))]
#[no_mangle]
fn disp_unix_or_wasi(fd: RawFd) {
  let stream: TcpStream;
  unsafe {
    stream = TcpStream::from_raw_fd(fd);
  }

  disp(stream);
}

#[cfg(target_os = "windows")]
fn get_and_go(stream: TcpStream) {
  let fd = stream.as_raw_socket();

  disp_win(fd);  
}

#[cfg(target_os = "windows")]
use std::os::windows::io::{RawSocket, AsRawSocket, FromRawSocket};
#[cfg(target_os = "windows")]
#[no_mangle]
fn disp_win(fd: RawSocket) {
  let stream: TcpStream;
  unsafe {
    stream = TcpStream::from_raw_socket(fd);
  }

  disp(stream);
}

fn disp(mut stream: TcpStream) {
  stream.set_read_timeout(Some(Duration::from_secs(2))).unwrap();

  stream.write(b"GET / HTTP/1.0\r\n\r\n").unwrap();
  stream.flush().unwrap();

  let mut reader = BufReader::new(&stream);
  let mut buffer = Vec::new();
  reader.read_until(b'\n', &mut buffer).expect("failed to read from socket");
  print!("{}", str::from_utf8(&buffer).expect("failed to convert to String"));
}

fn main() {
  let stream = TcpStream::connect("www.google.com:80").unwrap();
  get_and_go(stream);
}
