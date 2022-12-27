use rand::prelude::*;

pub fn chapter3() {
    exercise3_1();
    exercise3_3();
    exercise3_4();
    exercise3_5();
}

struct Accumulator(f64);

impl Accumulator {
    pub fn new(x: f64) -> Self {
        return Self(x);
    }

    pub fn accumulate(&mut self, x: f64) -> f64 {
        self.0 = self.0 + x;
        return self.0;
    }
}

fn exercise3_1() {
    println!("Starting exercise 3.1");
    let mut a = Accumulator::new(5.0);
    println!("{}", a.accumulate(10.0));
    println!("{}", a.accumulate(10.0));
    println!("Ending exercise 3.1");
}

fn exercise3_3() {
    struct BankAccount {
        amount: f64,
        password: String,
    }

    #[derive(Debug)]
    enum BankAccountError {
        InsufficientFunds,
        WrongPassword,
    }

    impl BankAccount {
        pub fn new(amount: f64, password: String) -> Self {
            return Self { amount, password };
        }

        pub fn withdraw(&mut self, amount: f64, pass: String) -> Result<f64, BankAccountError> {
            if !pass.eq(&self.password) {
                return Err(BankAccountError::WrongPassword);
            } else if amount > self.amount {
                return Err(BankAccountError::InsufficientFunds);
            } else {
                self.amount = self.amount - amount;
                return Ok(self.amount);
            }
        }
    }

    println!("Starting exercise 3.3");
    let mut ba = BankAccount::new(100.0, "pass".to_string());
    match ba.withdraw(40.0, "pass".to_string()) {
        Ok(x) => println!("{}", x),
        Err(y) => println!("{:?}", y),
    };
    match ba.withdraw(40.0, "not-pass".to_string()) {
        Ok(x) => println!("{}", x),
        Err(y) => println!("{:?}", y),
    };
    println!("Ending exercise 3.3");
}

fn exercise3_4() {
    const WRONG_CALLS: u8 = 2;
    struct BankAccount {
        amount: f64,
        password: String,
        wrong_calls: u8,
    }

    #[derive(Debug)]
    enum BankAccountError {
        InsufficientFunds,
        WrongPassword,
    }

    impl BankAccount {
        pub fn new(amount: f64, password: String) -> Self {
            return Self {
                amount,
                password,
                wrong_calls: 0,
            };
        }

        pub fn withdraw(&mut self, amount: f64, pass: String) -> Result<f64, BankAccountError> {
            if !pass.eq(&self.password) {
                self.wrong_calls = self.wrong_calls + 1;
                if self.wrong_calls >= WRONG_CALLS {
                    self.call_cops();
                }
                return Err(BankAccountError::WrongPassword);
            } else if amount > self.amount {
                return Err(BankAccountError::InsufficientFunds);
            } else {
                self.amount = self.amount - amount;
                return Ok(self.amount);
            }
        }

        fn call_cops(&self) {
            println!("999!!!!!!");
        }
    }

    println!("Starting exercise 3.4");
    let mut ba = BankAccount::new(100.0, "pass".to_string());
    match ba.withdraw(40.0, "not-pass".to_string()) {
        Ok(x) => println!("{}", x),
        Err(y) => println!("{:?}", y),
    };
    match ba.withdraw(40.0, "not-pass".to_string()) {
        Ok(x) => println!("{}", x),
        Err(y) => println!("{:?}", y),
    };
    println!("Ending exercise 3.4");
}

fn exercise3_5() {
    fn estimate_pi(r: f64, trails: i32) -> f64 {
        let mut rng = rand::thread_rng();

        let mut inside_circle = 0;
        let mut outside_circle = 0;
        for _ in 0..trails {
            let x = rng.gen::<f64>() * 2.0 * r;
            let y = rng.gen::<f64>() * 2.0 * r;

            match in_circle(x, y, r) {
                true => inside_circle += 1,
                false => outside_circle += 1,
            };
        }

        return 4.0 * (inside_circle as f64 / (outside_circle + inside_circle) as f64);
    }

    fn in_circle(x: f64, y: f64, r: f64) -> bool {
        return (x - r).powf(2_f64) + (y - r).powf(2_f64) <= r.powf(2_f64);
    }

    println!("Starting exercise 3.5");
    println!("num trails\tpi");
    println!("10\t{}", estimate_pi(10.0, 10));
    println!("1,000\t{}", estimate_pi(10.0, 1_000));
    println!("100,000\t{}", estimate_pi(10.0, 100_000));
    // Uncomment the below line for better estimate (at the cost of runtime)
    // println!("1,000,000\t{}", estimate_pi(10.0, 1_000_000));
    println!("Ending exercise 3.5");
}
