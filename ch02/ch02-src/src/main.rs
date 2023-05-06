fn fib(count: i32) -> i32 {
    let mut a = 1;
    let mut b = 1;
    let mut tmp: i32;

    for _ in 0..count {
        tmp = a + b;
        a = b;
        b = tmp;
    }
    b
}

fn fib_localimmutable(count: i32) -> i32 {
    let mut a = 1;
    let mut b = 1;

    for _ in 0..count {
        let tmp = a + b;
        a = b;
        b = tmp;
    }
    b
}

fn main() {
    println!("Hello, world!");
}
