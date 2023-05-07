fn fizzbuzz_1(max: i32) {
    for i in 1..=max {
        let rem_three = i % 3;
        let rem_five: i32 = i % 5;

        if rem_three == 0 && rem_five == 0 {
            println!("{} - FizzBuzz", i);
        } else if rem_three == 0 {
            println!("{} - Fizz", i);
        } else if rem_five == 0 {
            println!("{} - Buzz", i);
        }
    }
}

fn fizzbuzz_2(max: i32) {
    for i in 1..=max {
        match (i % 3, i % 5) {
            (0, 0) => println!("{} - FizzBuzz", i),
            (0, _) => println!("{} - Fizz", i),
            (_, 0) => println!("{} - Buzz", i),
            (_, _) => (),
        }
    }
}

fn fizzbuzz_3(max: i32) {
    let ret = (1..=max)
        .into_iter()
        .map(|i| match (i % 3, i % 5) {
            (0, 0) => format!("{} - FizzBuzz\n", i),
            (0, _) => format!("{} - Fizz\n", i),
            (_, 0) => format!("{} - Buzz\n", i),
            (_, _) => "".to_string(),
        })
        .collect::<Vec<String>>()
        .join("");
    println!("{}", ret);
}

fn ownership(nums: Vec<i32>) {
    // What happens if switch .iter and .into_iter?
    // What are the types of first i and the second i?
    let ret = nums
        .iter()
        .map(|i| format!("{}", i))
        .collect::<Vec<String>>()
        .join("");
    println!("{}", ret);

    let ret = nums
        .into_iter()
        .map(|i| format!("{}", i))
        .collect::<Vec<String>>()
        .join("");
    println!("{}", ret);
}

fn ownership_ref(_nums: &Vec<i32>) {
    () // do nothing
}

fn sum(start: i32, max: i32) -> i32 {
    if start < max {
        start + sum(start + 1, max)
    } else {
        max
    }
}

fn fizzbuzz_ternary() {
    let num = 10;
    let var = if num % 3 == 0 { 3 } else {
        if num % 5 == 0 { 5 } else { 0}
    };
    println!("{}", var);
}

fn fizzcheck(n: i32) -> bool {
    n % 3 == 0
}

fn buzzcheck(n: i32) -> bool {
    n % 5 == 0
}
fn fizzbuzz_fn(fizzfn: fn(i32) -> bool, buzzfn: fn(i32) -> bool) {
    for i in 1..=100 {
        if fizzfn(i) && buzzfn(i) {
            println!("FizzBizz");
        } else if fizzfn(i) {
            println!("Fizz");
        } else if buzzfn(i) {
            println!("Buzz");
        }
    }
}

fn fib(mut index: i32) -> i32 {
    let mut a = 1;
    let mut b = 1;
    let mut t;

    loop {
        t = a + b;
        a = b;
        b = t;

        index -= 1;
        if index <= 0 {
            break;
        }
    }
    b
}

fn main() {
    println!("{}", fib(3));
    //println!("{}", sum(1, 10));
    //fizzbuzz_ternary();
    //fizzbuzz_fn(fizzcheck, buzzcheck);
    //fizzbuzz_fn(|i| i % 3 == 0, |k| k % 5 == 0);

    /*
    fizzbuzz_1(100);
    fizzbuzz_2(100);
    fizzbuzz_3(100);

    let nums = (1..=100).collect::<Vec<i32>>();
    ownership(nums); // Is this a copy-value?

    let nums = (1..=100).collect::<Vec<i32>>();
    ownership_ref(&nums);
    ownership(nums);
    */
}
