use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt").unwrap_or_else(|err| {
        if err.kind() == ErrorKind::NotFound {
            File::create("hello.txt")
                .unwrap_or_else(|err| panic!("Could not create file {:?}", err))
        } else {
            panic!("Problem opening the file {:?}", err)
        }
    });
}
