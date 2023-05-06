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

fn main() {
    fizzbuzz_1(100);
    fizzbuzz_2(100);
    fizzbuzz_3(100);

    let nums = (1..=100).collect::<Vec<i32>>();
    ownership(nums); // Is this a copy-value?

    let nums = (1..=100).collect::<Vec<i32>>();
    ownership_ref(&nums);
    ownership(nums);
}
