fn main() {
    // While
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }
    println!("LIFTOFF");

    // For
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("The value is {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF");

    // Ex1
    println!("{}", fahrenheit_to_celsius(1.0));
    println!("{}", fahrenheit_to_celsius(250.0));
    println!("{}", fahrenheit_to_celsius(500.0));

    // Ex2
    for i in 1..30 {
        println!("f({}) = {}", i, fib(i));
    }
}

// Exercise
// 1. 華氏を摂氏に変換する
fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) / 1.8
}

// 2. フィボナッチ数列のn番目を生成
// fn = f(n-1) + f(n-2) | n > 1
fn fib(n: i32) -> i32 {
    if n > 1 {
        fib(n - 1) + fib(n - 2)
    } else {
        n
    }
}
