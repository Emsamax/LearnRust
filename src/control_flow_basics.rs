pub fn print_dbg() {
    let _z = 13;
    let y = 10;
    dbg!(y);
}

pub fn test_if() {
    let x = 10;
    if x == 0 {
        println!("zero!");
    } else if x < 100 {
        println!("biggish");
    } else {
        println!("huge");
    }

    let z = 10;
    let size = if z < 20 { "small" } else { "large" };
    println!("number size: {}", size);
}

pub fn check() {
    let val = 50;
    match val {
        1 => println!("one"),
        10 => println!("ten"),
        100 => println!("one hundred"),
        _ => {
            println!("something else");
        }
    }

    let flag = true;
    let val = match flag {
        true => 1,
        false => 0,
    };
    println!("The value of {flag} is {val}");
}

pub fn test_loop() {
    let mut x = 200;
    while x >= 10 {
        x = x / 2;
    }

    dbg!(x);
    for x in 1..5 {
        dbg!(x);
    }

    for elem in [2, 4, 8, 16, 32] {
        dbg!(elem);
    }
    let mut i = 0;
    loop {
        i += 1;
        dbg!(i);
        if i > 100 {
            break;
        }
    }

    let mut j = 0;
    loop {
        j += 1;
        if j > 5 {
            break;
        }
        if j % 2 == 0 {
            continue;
        }
        dbg!(j);
    }
}

pub fn test_loop_2() {
    let s = [[5, 6, 7], [8, 9, 10], [21, 15, 32]];
    let mut elements_searched = 0;
    let target_value = 10;
    'outer: for i in 0..=2 {
        for j in 0..=2 {
            elements_searched += 1;
            if s[i][j] == target_value {
                break 'outer;
            }
        }
    }
    dbg!(elements_searched);
}

pub fn gcd(a: u32, b: u32) -> u32 {
    if b > 0 { gcd(b, a % b) } else { a }
}

pub fn factorial(n: u32) -> u32 {
    let mut product = 1;
    for i in 1..=n {
        product *= dbg!(i);
    }
    product
}

pub fn fizzbuzz(n: u32) -> u32 {
    todo!()
}

/// Determine the length of the collatz sequence beginning at `n`.
pub fn collatz_length(mut n: i32) -> u32 {
    let mut len = 1;
    while n > 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        len += 1;
    }
    len
}
