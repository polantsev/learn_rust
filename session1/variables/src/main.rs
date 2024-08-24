fn double(n: i32) -> i32 {
    n * 2
}

fn double_or_nothing(n: i32) -> i32 {
    if n > 0 {
        n * 2
    } else {
        0
    }
}

fn main() {
    let i: i32 = 2;
    let n: i32 = if i > 0 { 3 } else { 10 };

    println!("{}", double_or_nothing(n));
}

// Result is an enumeration, often called an enum, which is a type that can be in one of multiple possible states. We call each possible state a variant.
