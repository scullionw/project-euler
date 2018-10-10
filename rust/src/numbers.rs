pub struct Fibonnaci {
    prev: u64,
    current: u64
}

impl Fibonnaci {
    pub fn new() -> Fibonnaci {
        Fibonnaci { prev: 0, current: 1 }
    }
}

impl Iterator for Fibonnaci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.prev + self.current;
        self.prev = self.current;
        self.current = next;
        Some(next)
    }
}


pub fn div_mod(n: u64, divisor: u64) -> (u64, u64) {
    let quotient = n / divisor;
    (quotient, n - divisor * quotient)
}


pub struct Infinity {
    val: u64
}

impl Infinity {
    pub fn naturals() -> Infinity {
        Infinity { val: 1 }
    }

    pub fn wholes() -> Infinity {
        Infinity { val: 0 }
    }

    pub fn from(n: u64) -> Infinity {
        Infinity { val: n }
    }
}

impl Iterator for Infinity {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.val;
        self.val += 1;
        Some(current)
    }
}