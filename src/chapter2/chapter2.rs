use std::ops::Rem;

pub fn chapter2() {
    // This chapter is often including the LISP Pair, which doesn't really have a Rust analogy
    // So, the exercises might seem thin or unusual in some places
    exercise2_1();
    exercise2_2();
    exercise2_3();
    // No exercise 2.4, since it is about Lisp Pairs (we don't have!)
    // No exercise 2.4, since it is about Lisp Pairs (we don't have!)
    exercise2_6();
    exercise2_7();
    exercise2_19();
    exercise2_20();
    exercise2_34();
    // TODO: find some kind of graphics package to do the painter exercises
    exercise2_59();
    exercise2_61();
}

fn exercise2_1() {
    fn make_rat(a: f64, b: f64) -> (f64, f64) {
        if b < 0.0 {
            return (-1.0 * a, -1.0 * b);
        } else {
            return (a, b);
        }
    }

    println!("Starting exercise 2.1");
    println!("-2 / -0.3 is {:?}", make_rat(-2.0, -0.3));
    println!("Ending exercise 2.1");
}

#[derive(Debug, Clone)]
struct Coordinate {
    pub x: f64,
    pub y: f64,
}

impl Coordinate {
    pub fn new(x: f64, y: f64) -> Self {
        return Self { x, y };
    }
}

struct Segment {
    pub start: Coordinate,
    pub end: Coordinate,
}

impl Segment {
    pub fn new(start: Coordinate, end: Coordinate) -> Segment {
        return Segment { start, end };
    }

    pub fn midpoint(&self) -> Coordinate {
        let mid_x = (self.start.x + self.end.x) / 2.0;
        let mid_y = (self.start.y + self.end.y) / 2.0;

        return Coordinate::new(mid_x, mid_y);
    }

    pub fn length(&self) -> f64 {
        let x = self.end.x - self.start.x;
        let y = self.end.y - self.start.y;

        return (x.powf(2_f64) + y.powf(2_f64)).sqrt();
    }
}

fn exercise2_2() {
    println!("Starting exercise 2.2");
    let a = Coordinate::new(1.0, 3.0);
    let b = Coordinate::new(6.0, -2.0);
    let seg = Segment::new(a, b);
    let mid = seg.midpoint();
    println!("The midpoint of (1, 3)-(6, -2) is {:?}", mid);
    println!("Ending exercise 2.2");
}

fn exercise2_3() {
    // This function discusses using a second representation
    // I'm not going to do that (for now!) but another way is to store 4 line segments instead
    // It makes the area and perimeter functions much simpler, at the expense of storing more data
    struct Rectangle {
        pub a: Coordinate,
        pub b: Coordinate,
        pub c: Coordinate,
        pub d: Coordinate,
    }

    impl Rectangle {
        pub fn new(a: Coordinate, b: Coordinate, c: Coordinate, d: Coordinate) -> Self {
            // We want to represent rectangles only, so these make sure they have right angle intersections
            // Alternative checks include looking at angles directly
            // We could also expand this to a data structure that represents any 4 sided shape!
            assert_eq!(a.y, b.y, "a and b must have equal y value");
            assert_eq!(b.x, c.x, "b and c must have equal x value");
            assert_eq!(c.y, d.y, "d and c must have equal y value");
            assert_eq!(d.x, a.x, "d and a must have equal x value");
            return Self { a, b, c, d };
        }

        pub fn area(&self) -> f64 {
            // This function is why it's important to make sure the angles are square
            let ab = Segment::new(self.a.clone(), self.b.clone()).length();
            let bc = Segment::new(self.b.clone(), self.c.clone()).length();

            return ab * bc;
        }

        pub fn perimeter(&self) -> f64 {
            let ab = Segment::new(self.a.clone(), self.b.clone()).length();
            let bc = Segment::new(self.b.clone(), self.c.clone()).length();

            return 2.0 * ab + 2.0 * bc;
        }
    }

    println!("Starting exercise 2.3");
    let a = Coordinate::new(0.0, 0.0);
    let b = Coordinate::new(5.0, 0.0);
    let c = Coordinate::new(5.0, -5.0);
    let d = Coordinate::new(0.0, -5.0);

    let rec = Rectangle::new(a, b, c, d);

    println!(
        "The rectangle has area {} and perimeter {} (I expect area 25 and perimeter 20)",
        rec.area(),
        rec.perimeter()
    );
    println!("Ending exercise 2.3");
}

fn exercise2_6() {
    // TODO: I don't think I really understood this one...
    fn church_zero(x: f64) -> f64 {
        x
    }
    fn church_one(x: f64) -> f64 {
        x + 1.0
    }
    fn church_two(x: f64) -> f64 {
        return church_one(church_one(church_zero(x)));
    }

    println!("Starting exercise 2.6");
    println!("Church two from 0 is {}", church_two(0.0));
    println!("Ending exercise 2.6");
}

fn exercise2_7() {
    // * Note: This exercise is extended, so it covers exercises 2.7 through to 2.16

    #[derive(Clone)]
    struct Interval {
        pub upper_bound: f64,
        pub lower_bound: f64,
    }

    impl Interval {
        pub fn new(upper_bound: f64, lower_bound: f64) -> Self {
            return Self {
                upper_bound,
                lower_bound,
            };
        }

        pub fn new_centre_width(c: f64, w: f64) -> Self {
            return Self {
                upper_bound: c + w,
                lower_bound: c - w,
            };
        }

        pub fn new_centre_tol(c: f64, tol: f64) -> Self {
            let w = c * tol;

            return Self::new_centre_width(c, w);
        }

        pub fn width(&self) -> f64 {
            return (self.upper_bound - self.lower_bound) / 2.0;
        }
    }

    // I think this is the first use of & in the software
    // LISP doesn't really have this concept, but I want to use & here to not consume the Intervals going in
    fn add_interval(x: &Interval, y: &Interval) -> Interval {
        return Interval::new(x.upper_bound + y.upper_bound, x.lower_bound + y.lower_bound);
    }

    fn mul_intervals(x: &Interval, y: &Interval) -> Interval {
        let p1 = x.lower_bound * y.lower_bound;
        let p2 = x.lower_bound * y.upper_bound;
        let p3 = x.upper_bound * y.lower_bound;
        let p4 = x.upper_bound * y.upper_bound;

        // I know this is UGLY but this is how one safely finds the max and min values in a list
        let min = vec![p1, p2, p3, p4].iter().cloned().fold(0. / 0., f64::min);
        let max = vec![p1, p2, p3, p4].iter().cloned().fold(0. / 0., f64::max);

        return Interval::new(max, min);
    }

    fn div_intervals(x: &Interval, y: &Interval) -> Interval {
        if y.width() == 0.0 {
            return x.clone();
        }
        let p1 = x.lower_bound / y.lower_bound;
        let p2 = x.lower_bound / y.upper_bound;
        let p3 = x.upper_bound / y.lower_bound;
        let p4 = x.upper_bound / y.upper_bound;

        // I know this is UGLY but this is how one safely finds the max and min values in a list
        let min = vec![p1, p2, p3, p4].iter().cloned().fold(0. / 0., f64::min);
        let max = vec![p1, p2, p3, p4].iter().cloned().fold(0. / 0., f64::max);

        return Interval::new(max, min);
    }

    fn sub_intervals(x: &Interval, y: &Interval) -> Interval {
        let p1 = x.lower_bound - y.lower_bound;
        let p2 = x.upper_bound - y.upper_bound;

        let min = vec![p1, p2].iter().cloned().fold(0. / 0., f64::min);
        let max = vec![p1, p2].iter().cloned().fold(0. / 0., f64::max);

        return Interval::new(max, min);
    }

    println!("Starting exercise 2.7");
    let a = Interval::new(3.0, 1.0);
    let b = Interval::new(5.0, 2.0);

    let wa = a.width();
    let wb = b.width();

    let c = add_interval(&a, &b);
    let wc = c.width();

    let d = sub_intervals(&a, &b);
    let wd = d.width();

    let e = mul_intervals(&a, &b);
    let we = e.width();

    let f = div_intervals(&a, &b);
    let wf = f.width();

    println!("wa + wb = {} | wc = {}", wa + wb, wc);
    println!("wa - wb = {} | wd = {}", wa - wb, wd); // TODO: My subtraction function does not behave as the book implies
                                                     // BUT I'm pretty confident that is how one would subtract intervals...
    println!("wa * wb = {} | we = {}", wa * wb, we);
    println!("wa / wb = {} | wf = {}", wa / wb, wf);
}

fn exercise2_17() {
    fn last(l: &Vec<f64>) -> f64 {
        return l[l.len()];
    }
}

fn exercise2_18() {
    fn reverse(l: &Vec<f64>) -> Vec<f64> {
        let mut a = l.clone();
        a.reverse();
        return a;
    }
}

fn exercise2_19() {
    let us_coins = vec![1, 5, 10, 25, 50];
    let uk_coins = vec![1, 2, 5, 10, 20, 50, 100];
    fn count_change(amount: i32, coinage: &Vec<i32>) -> i32 {
        return cc(amount, coinage.len() as i32, coinage);
    }

    fn cc(amount: i32, kinds_of_coins: i32, coinage: &Vec<i32>) -> i32 {
        if amount == 0 {
            return 1;
        } else if amount < 0 || kinds_of_coins == 0 {
            return 0;
        } else {
            return cc(amount, kinds_of_coins - 1, coinage)
                + cc(
                    amount - get_coinage(coinage, kinds_of_coins),
                    kinds_of_coins,
                    coinage,
                );
        }
    }

    fn get_coinage(coinage: &Vec<i32>, kinds_of_coins: i32) -> i32 {
        return coinage[(kinds_of_coins - 1) as usize];
    }

    fn first_denomination(kinds_of_coins: i32) -> i32 {
        return match kinds_of_coins {
            1 => 1,
            2 => 5,
            3 => 10,
            4 => 25,
            5 => 50,
            _ => panic!("The USD only has these kinds of coins!"),
        };
    }

    println!("Starting Exercise 2.19");
    println!(
        "There are {} ways to split $1",
        count_change(100, &us_coins)
    );
    println!(
        "There are {} ways to split £1",
        count_change(100, &uk_coins)
    );
    println!("Ending Exercise 2.19");
}

fn exercise2_20() {
    fn same_parity(w: &Vec<i32>) -> Vec<i32> {
        let mut v = vec![];

        let even = if w[0].rem(2) == 0 { true } else { false };

        for i in w {
            if (i.rem(2) == 0) == even {
                v.push(i.clone());
            }
        }

        return v;
    }

    println!("Starting Exercise 2.20");
    println!(
        "The same parity of (2,3,4,5,6,7) is {:?}",
        same_parity(&vec![2, 3, 4, 5, 6, 7])
    );
    println!(
        "The same parity of (1,2,3,4,5,6,7) is {:?}",
        same_parity(&vec![1, 2, 3, 4, 5, 6, 7])
    );
    println!("Ending Exercise 2.20");
}

fn exercise2_34() {
    fn horner_eval(x: f64, coeffs: &Vec<f64>) -> f64 {
        let mut a = coeffs.clone();
        a.reverse();
        let mut sum = 0.0;
        for (i, coeff) in a.iter().enumerate() {
            if i == a.len() - 1 {
                sum = sum + coeff;
                continue;
            }
            sum = (sum + coeff) * x;
        }

        return sum;
    }

    println!("Starting exercise 2.34");
    println!(
        "Evaluating 1 + 3x + 5x^3 + x^5 at x=2 to be {} (theoretically 79)",
        horner_eval(2.0, &vec![1.0, 3.0, 0.0, 5.0, 0.0, 1.0])
    );
    println!("Ending exercise 2.34");
}

fn exercise2_59() {
    fn union(a: &Vec<f64>, b: &Vec<f64>) -> Vec<f64> {
        let mut c = vec![];

        for i in a {
            if c.contains(i) {
                continue;
            }
            c.push(i.clone());
        }
        for i in b {
            if c.contains(i) {
                continue;
            }
            c.push(i.clone());
        }

        return c;
    }

    println!("Starting exercise 2.59");
    println!(
        "The union of (1,2,3) and (3,4,5) is {:?}",
        union(&vec![1.0, 2.0, 3.0], &vec![3.0, 4.0, 5.0])
    );
    println!("Ending exercise 2.59");
}

fn exercise2_60() {
    fn element_of_set(element: f64, set: &Vec<f64>) -> bool {
        return set.contains(&element);
    }

    fn adjoin_set(a: &Vec<f64>, x: f64) -> Vec<f64> {
        let mut b = a.clone();
        b.push(x);
        return b;
    }

    fn union_set(a: &Vec<f64>, b: &Vec<f64>) -> Vec<f64> {
        let mut c = vec![];

        for i in a {
            c.push(i.clone());
        }
        for i in b {
            c.push(i.clone());
        }

        return c;
    }

    fn intersection_set(a: &Vec<f64>, b: &Vec<f64>) -> Vec<f64> {
        let mut c = vec![];

        for i in a {
            if b.contains(i) {
                c.push(i.clone());
            }
        }

        return c;
    }
}

fn exercise2_61() {
    // This function MUST take ordered, single instance, vectors
    fn adjoin_set(a: &Vec<f64>, x: f64) -> Vec<f64> {
        let mut b = vec![];
        let mut added = false;
        if a.contains(&x) {
            return a.clone();
        }

        for (i, element) in a.iter().enumerate() {
            b.push(element.clone());
            if !added {
                if element < &x && a[i + 1] > x {
                    b.push(x);
                    added = true;
                }
            }
        }

        return b;
    }

    println!("Starting exercise 2.61");
    let a = vec![1.0, 2.0, 3.0, 5.0];

    println!("{:?}", adjoin_set(&a, 4.0));
    println!("Ending exercise 2.61");
}

fn exercise2_62() {
    fn union_sets(a: &Vec<f64>, b: &Vec<f64>) -> Vec<f64> {
        todo!();
    }
}
