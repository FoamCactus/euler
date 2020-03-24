pub fn problem16() {
    let mut a = PowerOfTwo::new();
    for _ in 1..1001 {
        a.next();
    }
    println!("the sum of the digits of 2^1000 is {}",  a.digit_sum());


}

struct PowerOfTwo {
    digits: [u8;350],
}

impl PowerOfTwo {
    fn new() -> PowerOfTwo {
        let mut d = [0;350];
        d[0] = 1;
        PowerOfTwo{digits: d}
    }

    fn next(& mut self){
        let length = self.digits.len();
        let mut co = 0;
        let mut counter = 0;
        while counter < length {
            let a = (self.digits[counter] * 2) + co;
            if a >= 10 {
               self.digits[counter] = a % 10;
               co = a / 10;
            } else {
               self.digits[counter] = a;
               co = 0;
            }
            counter = counter + 1;
        }
        if co > 0 {
            self.digits[counter] = co;
        }
    }

    fn digit_sum(&self) -> usize {
        let mut total = 0;
        for digit in self.digits.iter() {
            total = total + (*digit as usize);
        }
        total
    }
}


