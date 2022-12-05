use crate::chapter1::{iter_newton_cube_root::cube_root, iter_newton_sqrt::sqrt};
use num::{traits::Pow, Num};
use rand::prelude::*;
use std::{ops::Rem, time::Instant};
pub fn chapter1() {
    exercise1_1();
    // No exercise 1.2 since it is a pen and paper exercise
    exercise1_3();
    exercise1_4();
    // No exercise 1.5 since it is a pen and paper exercise
    // No exercise 1.6 since it is an exercise getting at how if-cond works in LISP
    exercise1_7();
    exercise1_8();
    // No exercise 1.9 as it is a pen and paper exercise
    exercise1_10();
    exercise1_11();
    exercise1_12();
    // No exercise 1.13 as it is a pen and paper exercise
    // No exercise 1.14 as it is a pen and paper exercise
    exercise1_15();
    exercise1_16();
    exercise1_17();
    exercise1_18();
    exercise1_19();
    // No exercise 1.20, since it is about interpreter orders
    exercise1_21();
    exercise1_22();
    exercise1_23();
    exercise1_24();
    // No exercise 1.25 since it is an explain exercise
    // NO exercise 1.26 since it is an explain exercise
    exercise1_27();
    exercise1_28();
    exercise1_29();
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

fn exercise1_10() {
    fn a(x: f64, y: f64) -> f64 {
        if y == 0.0 {
            0.0
        } else if x == 0.0 {
            2.0 * y
        } else if y == 1.0 {
            2.0
        } else {
            a(x - 1.0, a(x, y - 1.0))
        }
    }

    fn print_ackerman(x: f64, y: f64) {
        println!("When x={}, y={}, A={}", x, y, a(x, y));
    }

    println!("Ackermann's function");
    print_ackerman(1.0, 10.0);
    print_ackerman(2.0, 4.0);
    print_ackerman(3.0, 3.0);

    fn f(n: f64) -> f64 {
        a(0.0, n)
    }

    fn g(n: f64) -> f64 {
        a(1.0, n)
    }

    fn h(n: f64) -> f64 {
        a(2.0, n)
    }

    fn k(n: f64) -> f64 {
        5.0 * n * n
    }

    // TODO: significane of f, g, h, k
}

fn exercise1_11() {
    // f(n) = n if n < 3
    // f(n) = f(n-1) + 2f(n -2) + 3f(n-3) if n >= 3
    fn recursive(n: i32) -> i32 {
        if n < 3 {
            n
        } else {
            recursive(n - 1) + 2 * recursive(n - 2) + 3 * recursive(n - 3)
        }
    }

    fn iterative(n: i32) -> i32 {
        fn f_iter(a: i32, b: i32, c: i32, n: i32) -> i32 {
            if n == 0 {
                a
            } else {
                f_iter(b, c, c + 2 * b + 3 * a, n - 1)
            }
        }

        f_iter(0, 1, 2, n)
    }

    let n = 10;

    let recursive_solution = recursive(n);
    let iterative_solution = iterative(n);

    assert_eq!(recursive_solution, iterative_solution);

    println!("n = {}, iterative = recursive = {}", n, recursive_solution);
}

fn exercise1_12() {
    fn pascal(r: i32, c: i32) -> i32 {
        if c == 1 || r == c {
            1
        } else {
            pascal(r - 1, c - 1) + pascal(r - 1, c)
        }
    }

    let r = 4;
    let c = 2;
    println!("Pascal value of row {}, col {} is {}", r, c, pascal(r, c));
}

fn exercise1_15() {
    fn cube(x: f64) -> f64 {
        x * x * x
    }

    fn p(x: f64, count: &mut i32) -> f64 {
        *count += 1;
        3.0 * x - 4.0 * cube(x)
    }

    fn sine(angle: f64, count: &mut i32) -> f64 {
        if !(angle.abs() > 0.1) {
            angle
        } else {
            p(sine(angle / 3.0, count), count)
        }
    }

    let mut count = 0;

    let result = sine(12.15, &mut count);
    let rust = 12.15_f64.sin();
    let err = ((result - rust) / rust).abs() * 100.0;
    println!("Small angle approx of sine(12.15)={}", result);
    println!("p ran {} times", count);
    println!(
        "For comparison, Rust's built in sin(12.15)={}, the error is {}%",
        rust, err
    );

    let mut values: Vec<f64> = vec![];
    let mut counts: Vec<i32> = vec![];

    for i in 0..100 {
        let k = i as f64;

        let mut count = 0;

        let _ = sine(k, &mut count);

        values.push(k);
        counts.push(count);
    }

    println!(
        "Here are the number of times p was called for each value of the angle to approximate"
    );
    println!("angle\t|\tcount");
    for (v, c) in values.into_iter().zip(counts.into_iter()) {
        println!("{}\t|\t{}", v, c);
    }
}

fn even<T>(a: T) -> bool
where
    T: Rem + Num,
{
    a.rem(T::one() + T::one()) == T::zero()
}

fn double(a: f64) -> f64 {
    a + a
}

fn half(a: f64) -> f64 {
    a / 2.0
}

fn exercise1_16() {
    fn fast_expt(b: f64, n: i32) -> f64 {
        fn fast_iter(b: f64, counter: i32, product: f64) -> f64 {
            if counter == 0 {
                product
            } else if even(counter) {
                fast_iter(b.powf(2.0), counter / 2, product)
            } else {
                fast_iter(b, counter - 1, product * b)
            }
        }

        fast_iter(b, n, 1.0)
    }

    println!(
        "3^10={}, for reference, using Rust's built in power function 3^10={}",
        fast_expt(3.0, 10),
        3.0_f64.powf(10.0)
    );
}

fn exercise1_17() {
    fn mult(a: f64, b: f64) -> f64 {
        // NOTE: technically, the a==0 is unnecessary, but radically reduces computation when a==0
        if b == 0.0 || a == 0.0 {
            0.0
        } else if even(b) {
            mult(double(a), half(b))
        } else {
            a + mult(a, b - 1.0)
        }
    }

    println!("9*10={}", mult(9.0, 10.0));
}

fn exercise1_18() {
    fn fast_mult(a: f64, b: f64) -> f64 {
        fn mult_iter(a: f64, counter: f64, sum: f64) -> f64 {
            if counter == 0.0 {
                sum
            } else if even(counter) {
                mult_iter(double(a), half(counter), sum)
            } else {
                mult_iter(a, counter - 1.0, sum + a)
            }
        }

        mult_iter(a, b, 0.0)
    }

    println!("9*10={}", fast_mult(9.0, 10.0));
}

fn exercise1_19() {
    fn fib(n: f64) -> f64 {
        fib_iter(1.0, 0.0, 0.0, 1.0, n)
    }

    fn fib_iter(a: f64, b: f64, p: f64, q: f64, count: f64) -> f64 {
        if count == 0.0 {
            b
        } else if even(count) {
            fib_iter(
                a,
                b,
                p.powf(2.0) + q.powf(2.0),
                2.0 * p * q + q.powf(2.0),
                count / 2.0,
            )
        } else {
            fib_iter(b * q + a * q + a * p, b * p + a * q, p, q, count - 1.0)
        }
    }

    println!("The Fibonnaci sequence");
    for i in 0..20 {
        let k = i as f64;
        print!("{},", fib(k));
    }
    print!("\n");
}

fn find_divisor_slow(n: i32, test_divisor: i32) -> i32 {
    if test_divisor.pow(2) > n {
        n
    } else if divides(test_divisor, n) {
        test_divisor
    } else {
        find_divisor_slow(n, test_divisor + 1)
    }
}

fn find_divisor_fast(n: i32, test_divisor: i32) -> i32 {
    if test_divisor.pow(2) > n {
        n
    } else if divides(test_divisor, n) {
        test_divisor
    } else {
        find_divisor_fast(n, next(test_divisor))
    }
}

fn smallest_divisor_fast(n: i32) -> i32 {
    find_divisor_fast(n, 2)
}

fn smallest_divisor_slow(n: i32) -> i32 {
    find_divisor_slow(n, 2)
}

fn next(x: i32) -> i32 {
    if x == 2 {
        3
    } else {
        x + 2
    }
}

fn divides(a: i32, b: i32) -> bool {
    b.rem(a) == 0
}

fn prime_fast(n: i32) -> bool {
    smallest_divisor_fast(n) == n
}

fn prime_slow(n: i32) -> bool {
    smallest_divisor_slow(n) == n
}

fn exercise1_21() {
    println!("7 is prime? {}", prime_slow(7));
    println!("6 is prime? {}", prime_slow(6));

    println!("Smallest divisor of 199 is {}", smallest_divisor_slow(199));
    println!(
        "Smallest divisor of 1,999 is {}",
        smallest_divisor_slow(1_999)
    );
    println!(
        "Smallest divisor of 19,999 is {}",
        smallest_divisor_slow(19_999)
    );
}

fn exercise1_22() {
    fn find_three_larger_primes(n: i32) {
        let mut found_so_far = 0_i32;
        let mut m = n;
        while found_so_far < 3 {
            if prime_slow(m) {
                found_so_far += 1
            }

            m += 1;
        }
    }

    fn timed_prime(n: i32) -> u128 {
        let now = Instant::now();
        find_three_larger_primes(n);
        return now.elapsed().as_nanos();
    }

    println!("Starting Exercise 1.22");
    // Note: if you have a reallllllllly slow computer you might lose some accuracy in these timings
    let thousands = timed_prime(1000) as f64;
    let ten_thousands = timed_prime(10_000) as f64;
    let hundred_thousands = timed_prime(100_000) as f64;
    let millions = timed_prime(1_000_000) as f64;

    println!("Times taken to calculate three times larger than n in nanoseconds");
    println!("1,000\t|{}", thousands);
    println!("10,000\t|{}", ten_thousands);
    println!("100,000\t|{}", hundred_thousands);
    println!("1,000,000\t|{}", millions);

    println!(
        "We expect each stage to grow by around sqrt(10) which is {}",
        10_f64.powf(0.5)
    );

    println!("Here's how much each stage grew by");
    println!("Thousands to ten thousand {}", ten_thousands / thousands);
    println!(
        "Thousands to hundred thousands {}",
        hundred_thousands / ten_thousands
    );
    println!(
        "Hundred thousands to millions {}",
        millions / hundred_thousands
    );

    println!("Ending exercise 1.22")
}

fn exercise1_23() {
    fn find_three_larger_primes(n: i32) {
        let mut found_so_far = 0_i32;
        let mut m = n;
        while found_so_far < 3 {
            if prime_fast(m) {
                found_so_far += 1
            }

            m += 1;
        }
    }

    fn timed_prime(n: i32) -> u128 {
        let now = Instant::now();
        find_three_larger_primes(n);
        return now.elapsed().as_nanos();
    }

    println!("Starting exercise 1.23");

    let thousands = timed_prime(1000) as f64;
    let ten_thousands = timed_prime(10_000) as f64;
    let hundred_thousands = timed_prime(100_000) as f64;
    let millions = timed_prime(1_000_000) as f64;

    println!("Times taken to calculate three times larger than n in nanoseconds");
    println!("1,000\t|{}", thousands);
    println!("10,000\t|{}", ten_thousands);
    println!("100,000\t|{}", hundred_thousands);
    println!("1,000,000\t|{}", millions);

    println!(
        "We expect each stage to grow by around sqrt(10) which is {}",
        10_f64.powf(0.5)
    );

    println!("Here's how much each stage grew by");
    println!("Thousands to ten thousand {}", ten_thousands / thousands);
    println!(
        "Thousands to hundred thousands {}",
        hundred_thousands / ten_thousands
    );
    println!(
        "Hundred thousands to millions {}",
        millions / hundred_thousands
    );

    println!("Ending exercise 1.23");
}

fn square(x: i64) -> i64 {
    return x * x;
}

fn expmod(base: i64, exp: i64, m: i64) -> i64 {
    if exp == 0 {
        return 1;
    }

    if even(exp) {
        square(expmod(base, exp / 2, m)).rem(m)
    } else {
        (base * expmod(base, exp - 1, m)).rem(m)
    }
}

fn fermat_test(n: i64) -> bool {
    fn try_it(a: i64, n: i64) -> bool {
        expmod(a, n, n) == a
    }

    try_it(rand::thread_rng().gen_range(1..=n - 1), n)
}

fn fermat_prime(n: i64, num_times: i64) -> bool {
    if num_times == 0 {
        true
    } else if fermat_test(n) {
        fermat_prime(n, num_times - 1)
    } else {
        false
    }
}

fn exercise1_24() {
    fn find_three_larger_primes_fermat(n: i64) {
        let mut found_so_far = 0_i64;
        let mut m = n;
        while found_so_far < 3 {
            if fermat_prime(m, 3) {
                found_so_far += 1
            }

            m += 1;
        }
    }

    fn timed_prime(n: i64) -> u128 {
        let now = Instant::now();
        find_three_larger_primes_fermat(n);
        return now.elapsed().as_nanos();
    }

    println!("Starting exercise 1.24");

    let thousands = timed_prime(1000) as f64;
    let ten_thousands = timed_prime(10_000) as f64;
    let hundred_thousands = timed_prime(100_000) as f64;
    let millions = timed_prime(1_000_000) as f64;

    println!("Times taken to calculate three times larger than n in nanoseconds");
    println!("1,000\t|{}", thousands);
    println!("10,000\t|{}", ten_thousands);
    println!("100,000\t|{}", hundred_thousands);
    println!("1,000,000\t|{}", millions);

    println!(
        "We expect each stage to grow by around log(10) which is {}",
        10_f64.log10()
    );

    println!("Here's how much each stage grew by");
    println!("Thousands to ten thousand {}", ten_thousands / thousands);
    println!(
        "Thousands to hundred thousands {}",
        hundred_thousands / ten_thousands
    );
    println!(
        "Hundred thousands to millions {}",
        millions / hundred_thousands
    );

    println!("Ending exercise 1.24");
}

fn exercise1_27() {
    fn fermat_non_random_test(n: i64, a: i64) -> bool {
        expmod(a, n, n) == a
    }

    fn fermat_slow_prime(n: i64) -> bool {
        for i in 1..n {
            if !fermat_non_random_test(n, i) {
                return false;
            };
        }
        return true;
    }
    println!("Starting exercise 1.27");
    let carmichael_nums: Vec<i64> = vec![561, 1105, 1729, 2465, 2821, 6601];

    for carmichael in carmichael_nums {
        let prime = fermat_slow_prime(carmichael);
        if prime {
            println!("{carmichael} is prime (I have been fooled)");
        } else {
            println!("{carmichael} is not prime (true, but something has gone wrong...)");
        }
    }

    println!("Ending exercise 1.27");
}

fn exercise1_28() {
    fn expmod_nontrivial(base: i64, exp: i64, m: i64) -> i64 {
        fn nontrivial_test(x: i64, n: i64) -> i64 {
            if !(x == 1 || x == n - 1) && square(x).rem(n) == 1 {
                return 0;
            } else {
                return x;
            }
        }

        if exp == 0 {
            return 1;
        } else if even(exp) {
            return square(nontrivial_test(expmod_nontrivial(base, exp / 2, m), m)).rem(m);
        } else {
            return (base * expmod_nontrivial(base, exp - 1, m)).rem(m);
        }
    }

    fn miller_rabin_test(n: i64) -> bool {
        // n is the prime to test
        // a is the iteration step
        fn try_it(a: i64, n: i64) -> bool {
            return expmod_nontrivial(a, n - 1, n) == 1;
        }

        fn iter(a: i64, n: i64) -> bool {
            if a == 0 {
                return true;
            } else if try_it(rand::thread_rng().gen_range(1..=n - 1), n) {
                return iter(a - 1, n);
            } else {
                return false;
            }
        }

        return iter(10, n);
    }

    println!("Starting exercise 1.28");
    let carmichael_nums: Vec<i64> = vec![561, 1105, 1729, 2465, 2821, 6601];

    for carmichael in carmichael_nums {
        let prime = miller_rabin_test(carmichael);
        if prime {
            println!("{carmichael} is prime (I have been fooled)");
        } else {
            println!("{carmichael} is not prime (I have not been fooled)");
        }
    }

    println!("Ending exercise 1.28");
}

fn exercise1_29() {
    // This exercise is a little harder in Rust than LISP
    // Rust has no concept of a global state
    // And Rust inner functions like next() can not inherit variables from the super-scope
    // Therefore, we need to pass around many more variables than you can in LISP
    // While Rust can produce much more efficient code, it often requires more engineering

    fn simpson(f: &dyn Fn(f64) -> f64, a: f64, b: f64, n: i32) -> f64 {
        // There is a huge trade-off in this function between storing values and continually computing them
        // I went for storing and passing around values.
        // While this makes the code messier, for my application CPU cycles are more valuable than RAM
        // In other applications, you may prefer to calculate many variables such as c, h, n at every iteration
        fn sum(
            term: &dyn Fn(&dyn Fn(f64) -> f64, f64, i32, f64, i32) -> f64, // How to calculate the current Simpson term
            a: f64,                         // The starting lower bound
            k: i32,                         // The current integer step
            c: f64,                         // The current step value
            next: &dyn Fn(f64, f64) -> f64, // How to get to the next value of c
            b: f64,                         // The upper bound of the integral
            h: f64,                         // The step size
            f: &dyn Fn(f64) -> f64,         // The integral function to be calculated
            n: i32,                         // The total number of steps
        ) -> f64 {
            if k > n {
                return 0.0;
            } else {
                return term(f, a, k, h, n) + sum(term, a, k + 1, next(c, h), next, b, h, f, n);
            }
        }

        fn y(f: &dyn Fn(f64) -> f64, a: f64, k: i32, h: f64, n: i32) -> f64 {
            // Could reorganise this let-if for optimisation
            let mult = if k == 0 {
                1.0
            } else if k == n {
                1.0
            } else if even(k) {
                2.0
            } else {
                4.0
            };
            return mult * f(a + k as f64 * h);
        }

        fn next(n: f64, h: f64) -> f64 {
            return n + h;
        }

        let h = (b - a) / n as f64;
        let h_reduced = h / 3.0;

        let res = sum(&y, a, 0, a, &next, b, h, f, n);

        return h_reduced * res;
    }

    fn square(x: f64) -> f64 {
        return x * x;
    }

    println!("Starting exercise 1.29");
    println!("For this exercise, we will integrate x^2 between 0 and 1 numerically, the analytic answer is 1/3 = 0.33333.....");
    let now = Instant::now();
    let res = simpson(&square, 0.0, 1.0, 10);
    let time = now.elapsed().as_nanos();
    println!("The numerical calculation gives {}, in {}ns", res, time);
    println!("Ending exercise 1.29")
}

fn exercise1_30() {}

#[test]
fn test_sum_largest() {
    // a and b are largest
    assert_eq!(sum_largest(3.0, 2.0, 0.0), 5.0);

    // a and c are largest
    assert_eq!(sum_largest(5.0, 1.0, 5.0), 10.0);

    // b and c are largest
    assert_eq!(sum_largest(1.0, 5.0, 5.0), 10.0);
}
