pub fn sqrt(x: f64) -> f64 {
    return square_iter(1.0, x);
}

fn square_iter(guess: f64, x: f64) -> f64 {
    let improve = improve(guess, x);

    if (improve - guess).abs() < 0.001 {
        return improve;
    } else {
        return square_iter(improve, x);
    }
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
