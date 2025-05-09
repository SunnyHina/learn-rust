fn main() {
    // let str_hello = dbg!("Hello, world!");
    // let greeting = dbg!("Hello".to_string() + " world!");
    // let _ = dbg!("Hello world!".to_string());

    unsafe {
        let msg = "Hello world!\n";
        libc::write(1, msg.as_ptr() as *const libc::c_void, msg.len());
    }
}
