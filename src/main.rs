use chapter1::chapter1::chapter1;
use chapter2::chapter2::chapter2;

mod chapter1;
mod chapter2;

const CHAPTER1: bool = false;
const CHAPTER2: bool = true;

fn main() {
    if CHAPTER1 {
        chapter1();
    }

    if CHAPTER2 {
        chapter2();
    }
}
