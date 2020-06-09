/*
use winapi::um::winsock2::{WSAStartup, WSADATA, WSACleanup};
use winapi::shared::minwindef::MAKEWORD;

fn winsock_init() {
    unsafe{
    let wsaData = WSADATA::default();
	let wVersionRequested = MAKEWORD(2, 2);
        if WSAStartup(wVersionRequested, &wsaData) < 0 {
            WSACleanup();
            libc::exit(1);
        }
    }
}
*/
use winapi::um::winuser;
