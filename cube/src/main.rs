#![allow(unused)]

use std::{ops::Mul, thread, time::Duration};

// only 9 bits used
#[derive(Clone, Copy)]
struct BitMatrix(u16);

fn las() -> ! {
    loop {
        thread::sleep(Duration::from_hours(1))
    }
}

impl BitMatrix {
    fn mul(self, rhs: Self) -> Self {
        self
    }
}

impl Mul for BitMatrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let a1 = self.0 & 0b1 >> 0;
        let a2 = self.0 & 0b10 >> 1;
        let a3 = self.0 & 0b100 >> 2;
        let b1 = self.0 & 0b1000 >> 3;
        let b2 = self.0 & 0b10000 >> 4;
        let b3 = self.0 & 0b100000 >> 5;
        let c1 = self.0 & 0b1000000 >> 6;
        let c2 = self.0 & 0b10000000 >> 7;
        let c3 = self.0 & 0b100000000 >> 8;

        self
    }
}

fn main() {
    let b = 0b1010;
    dbg!((b & 1 << 0) >> 0);
    dbg!((b & 1 << 1) >> 1);
    dbg!((b & 1 << 2) >> 2);
    dbg!((b & 1 << 3) >> 3);
    dbg!((b & 1 << 4) >> 4);
    dbg!((b & 1 << 5) >> 5);
    dbg!((b & 1 << 6) >> 6);
    dbg!((b & 1 << 7) >> 7);
    dbg!((b & 1 << 8) >> 8);
    let b = BitMatrix(8);
    b.mul(b);
    b.mul(b);
    Mul::mul(b, b);
    fn x() {
        fn sdf() {}
    }
}
