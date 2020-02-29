use afl::*;
use lazy_static::lazy_static;

fn delay() {
    #[cfg(feature = "delay")]
    std::thread::sleep(std::time::Duration::from_millis(500));
}

lazy_static! {
    static ref FOO: String = {
        delay();
        String::from("foo")
    };
    static ref BAR: String = {
        delay();
        String::from("bar")
    };
    static ref BAZ: String = {
        delay();
        String::from("baz")
    };
}

fn main() {
    fuzz!(|data: &[u8]| {
        for x in data {
            match x % 3 {
                0 => println!("{}", *FOO),
                1 => println!("{}", *BAR),
                _ => println!("{}", *BAZ),
            }
        }
        if data.len() > 3 {
            #[cfg(feature = "panic")]
            panic!("Oh no!");
        }
    });
}
