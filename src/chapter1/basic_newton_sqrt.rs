fn sqrt(x: f64) -> f64 {
    return square_iter(1.0, x);
}

fn square_iter(guess: f64, x: f64) -> f64 {
    if good_enough(guess, x) {
        return guess;
    } else {
        return square_iter(improve(guess, x), x);
    }
}

fn good_enough(guess: f64, x: f64) -> bool {
    return (guess.powf(2.0) - x).abs() < 0.001;
    // We could use the methods for square and abs defined in the book but I am
    // happy to use the Rust built in versions
}

fn improve(guess: f64, x: f64) -> f64 {
    return average(guess, x / guess);
}

fn average(x: f64, y: f64) -> f64 {
    return (x + y) / 2.0;
}

#[test]
fn test_sqrt() {
    let sq = sqrt(9.0);
    println!("sqrt(9.0) = {}", sq);
    assert!((3.0 - sq).abs() < 0.001);
}
