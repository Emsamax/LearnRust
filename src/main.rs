mod control_flow_basics;
mod tuples_arrays;
mod references;
mod user_defined_types;
mod elevator;

static BANNER: &str = "Welcome to RustOS 3.14";

fn main() {
    println!("Hello, 🌍!");
   /* let mut x: i32 = 10;
    println!("x: {x}");
    x = 20;
    println!("x: {x}");
    println!("result: {}", interprodct(120, 100, 248));

    let x = 10;
    let y = 20;

    takes_u32(x);
    takes_i8(y);
    //takes_u32(y);
    */
    let n = 20;
    println!("fib({n}) = {}", fib(n));
    control_flow_basics::print_dbg();
    control_flow_basics::test_if();
    control_flow_basics::check();
    control_flow_basics::test_loop();
    control_flow_basics::test_loop_2();
    dbg!(control_flow_basics::gcd(40, 12));
    dbg!(control_flow_basics::factorial(5));
    dbg!(control_flow_basics::collatz_length(11));
    tuples_arrays::test_arrays();
    tuples_arrays::test_tuples();
    let tuple = (1, 5, 3);
    println!(
        "{tuple:?}: {}",
        if tuples_arrays::check_order(tuple) { "ordered" } else { "unordered" }
    );
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];
    dbg!(matrix);
    let transposed = tuples_arrays::transpose(matrix);
    dbg!(transposed);
    references::test_shared_ref();
    references::test_tuple_ref();
    references::test_slice();
    references::test_string();
    //references::test_outlive()

    println!("Magnitude of a unit vector: {}", references::magnitude(&[0.0, 1.0, 0.0]));

    let mut v = [1.0, 2.0, 9.0];
    println!("Magnitude of {v:?}: {}", references::magnitude(&v));
    references::normalize(&mut v);
    println!("Magnitude of {v:?} after normalization: {}", references::magnitude(&v));
    user_defined_types::peter();
    let digest = user_defined_types::compute_digest("hello");
    println!("digest: {digest:?}");
    println!("{BANNER}");
    /// ELEVATOR EX
    println!(
        "A ground floor passenger has pressed the up button: {:?}",
        elevator::lobby_call_button_pressed(0, elevator::Direction::Up)
    );
    println!("The car has arrived on the ground floor: {:?}", elevator::car_arrived(0));
    println!("The car door opened: {:?}", elevator::car_door_opened());
    println!(
        "A passenger has pressed the 3rd floor button: {:?}",
        elevator::car_floor_button_pressed(3)
    );
    println!("The car door closed: {:?}", elevator::car_door_closed());
    println!("The car has arrived on the 3rd floor: {:?}", elevator::car_arrived(3));
}

/*
fn interprodct(a: i32, b: i32, c: i32) -> i32 {
    return a * b + b * c + c * a;
}

fn takes_u32(x: u32) {
    println!("u32: {x}");
}

fn takes_i8(y: i8) {
    println!("i8: {y}");
}*/

fn fib(n: u32) -> u32 {
    if n < 2 {
        // The base case.
        return 1;
    } else {
        // The recursive case.
        return fib(n - 1) + fib(n - 2);
    }
}


