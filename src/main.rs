#![no_std]
#![no_main]

/// This is a shellcode loader Rust while also minimizing the size of the binary. In my tests the C
/// version binary turned out to be 4k whilst on Mac with rust I got to 4.2k after running strip
/// This can be optimized further. Conditional compiling for the static varibles would get you some
/// more room.
/// The main benefit is compliation with the toolchains Rust offers. It's a lot easier to compile a
/// windows binary without using mingw etc.


mod metasploit;
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! { loop {} }

//This is required to link against Libc seems to fail without it
#[link(name = "c")]
extern "C" {
}

static MAC_PAYLOAD: [u8; 51] = [
0xeb,0x1e,0x5e,0xb8,0x04,0x00,0x00,0x02,0xbf,0x01,
0x00,0x00,0x00,0xba,0x0e,0x00,0x00,0x00,0x0f,0x05,
0xb8,0x01,0x00,0x00,0x02,0xbf,0x00,0x00,0x00,0x00,
0x0f,0x05,0xe8,0xdd,0xff,0xff,0xff,0x48,0x65,0x6c,
0x6c,0x6f,0x20,0x57,0x6f,0x72,0x6c,0x64,0x21,0x0d,
0x0a];

static WINDOWS_PAYLOAD: [u8; 199] = [
0x33,0xc9,0x64,0x8b,0x49,0x30,0x8b,0x49,0x0c,0x8b,
0x49,0x1c,0x8b,0x59,0x08,0x8b,0x41,0x20,0x8b,0x09,
0x80,0x78,0x0c,0x33,0x75,0xf2,0x8b,0xeb,0x03,0x6d,
0x3c,0x8b,0x6d,0x78,0x03,0xeb,0x8b,0x45,0x20,0x03,
0xc3,0x33,0xd2,0x8b,0x34,0x90,0x03,0xf3,0x42,0x81,
0x3e,0x47,0x65,0x74,0x50,0x75,0xf2,0x81,0x7e,0x04,
0x72,0x6f,0x63,0x41,0x75,0xe9,0x8b,0x75,0x24,0x03,
0xf3,0x66,0x8b,0x14,0x56,0x8b,0x75,0x1c,0x03,0xf3,
0x8b,0x74,0x96,0xfc,0x03,0xf3,0x33,0xff,0x57,0x68,
0x61,0x72,0x79,0x41,0x68,0x4c,0x69,0x62,0x72,0x68,
0x4c,0x6f,0x61,0x64,0x54,0x53,0xff,0xd6,0x33,0xc9,
0x57,0x66,0xb9,0x33,0x32,0x51,0x68,0x75,0x73,0x65,
0x72,0x54,0xff,0xd0,0x57,0x68,0x6f,0x78,0x41,0x01,
0xfe,0x4c,0x24,0x03,0x68,0x61,0x67,0x65,0x42,0x68,
0x4d,0x65,0x73,0x73,0x54,0x50,0xff,0xd6,0x57,0x68,
0x72,0x6c,0x64,0x21,0x68,0x6f,0x20,0x57,0x6f,0x68,
0x48,0x65,0x6c,0x6c,0x8b,0xcc,0x57,0x57,0x51,0x57,
0xff,0xd0,0x57,0x68,0x65,0x73,0x73,0x01,0xfe,0x4c,
0x24,0x03,0x68,0x50,0x72,0x6f,0x63,0x68,0x45,0x78,
0x69,0x74,0x54,0x53,0xff,0xd6,0x57,0xff,0xd0];


static LINUX_PAYLOAD: [u8; 49] = [
0xeb,0x1e,0xb8,0x01,0x00,0x00,0x00,0xbf,0x01,0x00,
0x00,0x00,0x5e,0xba,0x0c,0x00,0x00,0x00,0x0f,0x05,
0xb8,0x3c,0x00,0x00,0x00,0xbf,0x00,0x00,0x00,0x00,
0x0f,0x05,0xe8,0xdd,0xff,0xff,0xff,0x48,0x65,0x6c,
0x6c,0x6f,0x20,0x57,0x6f,0x72,0x6c,0x64,0x0a];

#[no_mangle]
pub extern "C" fn main() {
    unsafe {
        if cfg!(target_os = "macos") {
            let exec_data: extern "C" fn () -> ! =  core::mem::transmute(&MAC_PAYLOAD as *const _ as *const ());
            exec_data();
        }
        else if cfg!(target_os = "windows") {
            let exec_data: extern "C" fn () -> ! =  core::mem::transmute(&WINDOWS_PAYLOAD as *const _ as *const ());
            exec_data();
        } else{
            let exec_data: extern "C" fn () -> ! =  core::mem::transmute(&LINUX_PAYLOAD as *const _ as *const ());
            exec_data();
        }
    };
}
