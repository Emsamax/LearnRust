pub fn test_shared_ref() {
    let a = 'A';
    let b = 'B';

    let mut r: &char = &a;
    dbg!(*r);

    r = &b;
    dbg!(*r);
}

pub fn test_tuple_ref() {
    let mut point = (1, 2);
    let x_coord = &mut point.0;
    *x_coord = 20;
    println!("point: {point:?}");
}

pub fn test_slice() {
    let mut a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a: {a:?}");

    let s: &mut [i32] = &mut a[2..4];
    s[0] = 300;
    println!("s: {s:?}");
    println!("a after mut in s : {a:?}");
}

pub fn test_string() {
    let s1: &str = "World";
    println!("s1: {s1}");

    let mut s2: String = String::from("Hello ");
    println!("s2: {s2}");

    s2.push_str(s1);
    println!("s2: {s2}");

    let s3: &str = &s2[2..9];
    println!("s3: {s3}");

    println!("{:?}", "abc");
    println!("{:?}", &[97, 98, 99]);

    println!(r#"<a href="link.html">link</a>"#);
    println!("<a href=\"link.html\">link</a>");
}

/*pub fn test_outlive() {
    let x_ref = {
        let x = 10;
        &x
    };
    dbg!(x_ref);
}*/

// Calculate the magnitude of a vector by summing the squares of its coordinates
// and taking the square root. Use the `sqrt()` method to calculate the square
// root, like `v.sqrt()`.
pub fn magnitude(vect: &[f64; 3]) -> f64 {
    let mut result = 0.0;
    for coord in vect {
        result += coord * coord;
    }
    result
}

// Normalize a vector by calculating its magnitude and dividing all of its
// coordinates by that magnitude.
pub fn normalize(vect: &mut[f64; 3]) {
    let mag: f64 = magnitude(vect);
    for coord in vect {
        *coord /= mag;
    }
}
