pub fn cube_root(x: f64) -> f64 {
    return cube_iter(1.0, x);
}

fn cube_iter(guess: f64, x: f64) -> f64 {
    let improve = improve(guess, x);

    if (improve - guess).abs() < 0.001 {
        return improve;
    } else {
        return cube_iter(improve, x);
    }
}

fn improve(guess: f64, x: f64) -> f64 {
    cube_average(x, guess)
}

fn cube_average(x: f64, y: f64) -> f64 {
    ((x / y.powf(2.0)) + 2.0 * y) / 3.0
}

#[test]
fn test_sqrt() {
    let sq = cube_root(27.0);
    println!("sqrt(27.0) = {}", sq);
    assert!((3.0 - sq).abs() < 0.001);
}
