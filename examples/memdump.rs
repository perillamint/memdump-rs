const FOO: &str = "Hello, world!\n What is your name?";

fn main() {
    unsafe {
        memdump::memdump(FOO.as_ptr(), FOO.len(), |s| println!("{}", s));
    }
}
