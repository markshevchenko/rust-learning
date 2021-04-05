use std::fs::File;
use std::io::Result;
use std::io::Write;

fn main() {
    let mut local_file = File::create("hello.txt").expect("error creating file");
    say_hello(&mut local_file).expect("error writing file");

    let mut bytes = vec![];
    say_hello(&mut bytes).expect("error writing buffer");
    assert_eq!(bytes, b"hello world\n");
}

fn say_hello(out: &mut dyn Write) -> Result<()> {
    out.write_all(b"hello world\n")?;
    out.flush()
}