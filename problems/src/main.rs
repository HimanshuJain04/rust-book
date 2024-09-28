fn main() {
    // let ans = is_even(4);
    // println!("is even: {}", ans);

    let fib = fibonacci(5);
    println!("fib: {}", fib);
}

// check number is even or not
fn is_even(num: i32) -> bool {
    return num % 2 == 0;
}

// fibonacci number
fn fibonacci(num: u8) -> u32 {
    let mut first = 0;
    let mut sec = 1;

    if (num == 0) {
        return first;
    }

    if (num == 1) {
        return sec;
    }

    for i in 1..num {
        let temp = first + sec;
        first = sec;
        sec = temp;
    }

    return sec;
}
