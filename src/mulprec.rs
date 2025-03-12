// const
const BASE: isize = 1000000000;
const KETA: usize = 2500;
const SHIFT: usize = 125;

// struct
pub struct NUMBER {
    pub n: [isize; KETA],
    pub sign: isize,
}

impl NUMBER {
    pub fn new() -> Self {
        let mut number = NUMBER {
            n: [0; KETA],
            sign: 0,
        };
        number.clear();
        number
    }

    // ゼロクリア
    pub fn clear(&mut self) {
        self.n = [0; KETA];
        self.sign = 0;
    }

    pub fn display(&self) {
        if self.sign == 1 {
            print!("+");
        } else if self.sign == -1 {
            print!("-");
        }

        let mut is_zero = true;
        for i in (0..KETA).rev() {
            let num = self.n[i];

            // 先行ゼロをスキップ
            if is_zero && num == 0 {
                continue;
            }
            is_zero = false;

            if i == SHIFT {
                print!("{}", num);
            } else {
                print!("{:09}", num);
            }
        }

        if is_zero {
            print!("0");
        }
    }

    /*
        ゼロかどうかの判定
    */
    pub fn is_zero(&self) -> bool {
        self.n.iter().all(|&x| x == 0)
    }

    /*
        xの値をセット
    */
    pub fn set_int(&mut self, x: isize) {
        self.clear();

        let mut x = x;
        let mut r: isize;
        let mut i = 0;

        while x != 0 {
            r = x % BASE;
            x -= r;
            x /= BASE;

            if i >= KETA {
                panic!("overflow");
            }

            if self.n[i] == 0 {
                self.n[i] = r;
            } else {
                panic!("overflow");
            }

            i += 1;
        }
    }
}

/*
    target = source
*/
pub fn copy_number(source: &NUMBER, target: &mut NUMBER) {
    target.sign = source.sign;
    target.n = source.n;
}

/*
   source1 > source2: 1
   source1 < source2: -1
   source1 = source2: 0
*/
pub fn num_comp(s1: &NUMBER, s2: &NUMBER) -> isize {
    if s1.sign > s2.sign {
        return 1;
    }

    if s1.sign < s2.sign {
        return -1;
    }

    let mut r_v: isize = 0;
    for i in (0..KETA - 1).rev() {
        if s1.n[i] > s2.n[i] {
            r_v = 1;
            break;
        }
        if s1.n[i] < s2.n[i] {
            r_v = -1;
            break;
        }
    }

    return r_v;
}

/*
    s1 + s2 = target
*/
pub fn add(s1: &NUMBER, st2: &NUMBER, target: &mut NUMBER) {
    let mut d: isize;
    let mut e: isize = 0;

    for i in 0..KETA - 1 {
        if s1.n[i] == 0 && st2.n[i] == 0 && e == 0 {
            target.n[i] = 0;
            continue;
        }

        d = s1.n[i] + st2.n[i] + e;
        target.n[i] = d % BASE;
        e = d / BASE;
    }
}

/*
    s1 - s2 = target
*/
pub fn sub(s1: &NUMBER, s2: &NUMBER, target: &mut NUMBER) {
    let mut h: isize = 0;
    let mut s1i: isize;
    let mut s2i: isize;

    for i in 0..KETA - 1 {
        s1i = s1.n[i];
        s2i = s2.n[i];
        s1i -= h;

        if s1i >= s2i {
            target.n[i] = s1i - s2i;
            h = 0;
        }

        if s1i < s2i {
            target.n[i] = s1i + BASE - s2i;
            h = 1;
        }

        if h != 0 {
            target.clear();
        }
    }
}

/*
    s1 + 1 = target
*/
pub fn increment(s1: &NUMBER, target: &mut NUMBER) {
    let mut one: NUMBER = NUMBER::new();
    one.set_int(1);
    add(s1, &one, target);
}

/*
    s1 - 1 = target
*/
pub fn decrement(s1: &NUMBER, target: &mut NUMBER) {
    let mut one: NUMBER = NUMBER::new();
    one.set_int(1);
    sub(s1, &one, target);
}
