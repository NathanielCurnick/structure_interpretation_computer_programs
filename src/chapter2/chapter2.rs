use std::cmp::min;

pub fn chapter2() {
    exercise2_1();
    exercise2_2();
    exercise2_3();
    // No exercise 2.4, since it is about Lisp Pairs (we don't have!)
    // No exercise 2.4, since it is about Lisp Pairs (we don't have!)
    exercise2_6();
    exercise2_7();
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
