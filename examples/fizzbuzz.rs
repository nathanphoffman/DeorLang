// fizzbuzz.rs
// Coding test: print the numbers 1 to 20, but print "Fizz" for multiples of 3,
// "Buzz" for multiples of 5, and "FizzBuzz" for multiples of both.

fn fizzbuzz(num: i64) -> String {
    if num % 15 == 0 {
        return "FizzBuzz".to_string();
    }
    if num % 3 == 0 {
        return "Fizz".to_string();
    }
    if num % 5 == 0 {
        return "Buzz".to_string();
    }

    num.to_string()
}

fn main() {
    for num in 1..21 {
        println!("{}", fizzbuzz(num));
    }
}
