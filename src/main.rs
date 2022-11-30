use crate::chapter1::{iter_newton_cube_root::cube_root, iter_newton_sqrt::sqrt};

mod chapter1;

fn main() {
    exercise1_1();
    // No exercise 1.2 since it is a pen and paper exercise
    exercise1_3();
    exercise1_4();
    // No exercise 1.5 since it is a pen and paper exercise
    // No exercise 1.6 since it is an exercise getting at how if-cond works in LISP
    exercise1_7();
    exercise1_8();
}



fn exercise1_1() {
    // Exercise 1.1 page 20
    // This exercise is asking what would the output be of these expressions

    println!("Starting Exercise 1.1");
    println!("10");

    println!("{}", 5 + 3 + 4);
    println!("{}", 9 - 1);
    println!("{}", 6 / 2);
    println!("{}", 2 * 4 + (4 - 6));

    let a = 3;
    println!("{}", a);

    let b = a + 1;
    println!("{}", b);

    println!("{}", a + b + (a * b));

    println!("{}", a == b);

    if b > a && b < a * b {
        println!("{}", b);
    } else {
        println!("{}", a);
    }

    if a == 4 {
        println!("{}", 6);
    } else if b == 4 {
        println!("{}", 6 + 7 + a);
    } else {
        println!("{}", 25);
    }

    if b > a {
        println!("{}", 2 + b);
    } else {
        println!("{}", 2 + a);
    }

    if a > b {
        println!("{}", a * (a + 1));
    } else if a < b {
        println!("{}", b * (a + 1));
    } else {
        println!("{}", -1 * (a + 1));
    }
    println!("Ending Exercise 1.1")
}

fn sum_largest(a: f64, b: f64, c: f64) -> f64 {
    let ab = a > b;
    let bc = b > c;

    return if ab && bc {
        // a and b are the largest
        a + b
    } else if ab && !bc {
        // a and c are the largest
        a + c
    } else {
        // b and c are largest
        b + c
    };
}

fn exercise1_3() {
    println!("Starting Exercise 1.3");
    println!("{}", sum_largest(3.0, 2.0, 1.0));
    println!("Ending Exercise 1.3");
}

fn exercise1_4() {
    fn a_plus_abs_b(a: f64, b: f64) -> f64 {
        return if b > 0.0 { a + b } else { a - b };
    }
    println!("Starting Exercise 1.4");
    println!("{}", a_plus_abs_b(1.0, -1.0));
    println!("Ending Exercise 1.4");
}

fn exercise1_7() {
    println!("Starting Exercise 1.7");
    println!("sqrt(9.0) = {}", sqrt(9.0));
    println!("Ending Exercise 1.7");
}

fn exercise1_8() {
    println!("Starting Exercise 1.8");
    println!("cube_root(27.0) = {}", cube_root(27.0));
    println!("Ending Exercise 1.8");
}

#[test]
fn test_sum_largest() {
    // a and b are largest
    assert_eq!(sum_largest(3.0, 2.0, 0.0), 5.0);

    // a and c are largest
    assert_eq!(sum_largest(5.0, 1.0, 5.0), 10.0);

    // b and c are largest
    assert_eq!(sum_largest(1.0, 5.0, 5.0), 10.0);
}
