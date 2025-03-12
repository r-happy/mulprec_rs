// const
const BASE: i64 = 1000000000;
const KETA: usize = 800;
const SHIFT: usize = 20;

// struct
pub struct NUMBER {
    pub n: [i64; KETA],
    pub sign: i64,
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
    pub fn set_int(&mut self, x: i64) {
        self.clear();

        let mut x = x;
        let mut r: i64;
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

    /*
        現在の桁を取得
    */
    pub fn get_keta(&self) -> usize {
        let mut i = KETA;
        let mut r_v: usize = 0;
        while i > 0 {
            i -= 1;
            if self.n[i] != 0 {
                r_v = i + 1;
                break;
            }
        }
        return r_v;
    }

    /*
        左にシフト
    */
    pub fn shift_left(&mut self, n: usize) {
        if n >= KETA {
            self.clear();
            return;
        }
        for i in (0..KETA - n).rev() {
            self.n[i + n] = self.n[i];
        }
        for i in 0..n {
            self.n[i] = 0;
        }
    }

    /*
        右にシフト
    */
    pub fn shift_right(&mut self, n: usize) {
        if n >= KETA {
            self.clear();
            return;
        }
        for i in 0..KETA - n {
            self.n[i] = self.n[i + n];
        }
        for i in (KETA - n..KETA).rev() {
            self.n[i] = 0;
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
pub fn num_comp(s1: &NUMBER, s2: &NUMBER) -> i64 {
    if s1.sign > s2.sign {
        return 1;
    }

    if s1.sign < s2.sign {
        return -1;
    }

    let mut r_v: i64 = 0;
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
    let mut d: i64;
    let mut e: i64 = 0;

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
    let mut h: i64 = 0;
    let mut s1i: i64;
    let mut s2i: i64;

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
    }

    if h != 0 {
        target.clear();
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

/*
    s1 * s2 = target
*/
pub fn multiple(s1: &NUMBER, s2: &NUMBER, target: &mut NUMBER) {
    simple_multiple(s1, s2, target);
}

pub fn simple_multiple(s1: &NUMBER, s2: &NUMBER, target: &mut NUMBER) {
    target.clear();
    let s1_keta = s1.get_keta();
    let s2_keta = s2.get_keta();

    for i in 0..s2_keta {
        if s2.n[i] == 0 {
            continue;
        }

        for j in 0..s1_keta {
            if j + i >= s1_keta + s2_keta {
                break;
            }

            if s1.n[j] == 0 {
                continue;
            }

            target.n[j + i] += s1.n[j] * s2.n[i];

            if target.n[j + i] >= BASE {
                target.n[j + i + 1] += target.n[j + i] / BASE;
                target.n[j + i] %= BASE;
            }
        }
    }
}

/*
    inverse(s1) = target
*/
pub fn inverse(s: &NUMBER, target: &mut NUMBER, n: usize) {
    let mut x = NUMBER::new();
    let mut y = NUMBER::new();
    let mut h = NUMBER::new();
    let mut one = NUMBER::new();
    let mut t1 = NUMBER::new();
    let mut t2 = NUMBER::new();
    let keta: usize = s.get_keta();

    one.set_int(1);
    one.shift_left(n);

    // x <- 2 * 10 ** n - keta
    x.set_int(2);
    x.shift_left(n - keta);

    loop {
        copy_number(&x, &mut y);
        multiple(s, &y, &mut t1);
        sub(&one, &t1, &mut h);

        multiple(&h, &h, &mut t1);
        t1.shift_right(n);
        add(&t1, &h, &mut t2);
        add(&t2, &one, &mut t1);
        multiple(&y, &t1, &mut x);
        x.shift_right(n);

        if (n - h.get_keta()) * 3 >= n {
            break;
        }
    }

    copy_number(&x, target);
}

/*
    s1 / s2 = target
*/
pub fn divide(s1: &NUMBER, s2: &NUMBER, target: &mut NUMBER) {
    if s2.is_zero() {
        panic!("divide by zero");
    }

    if s2.get_keta() > 1 {
        divide_w_inverse(s1, s2, target);
    } else {
        one_divide(s1, s2, target);
    }
}

fn one_divide(s1: &NUMBER, s2: &NUMBER, target: &mut NUMBER) {
    let mut t: i64;
    let mut h: i64;
    target.clear();

    h = 0;

    for i in (0..s1.get_keta()).rev() {
        t = BASE * h + s1.n[i];
        h = t % s2.n[0];
        target.n[i] = (t - h) / s2.n[0];
    }
}

fn divide_w_inverse(s1: &NUMBER, s2: &NUMBER, target: &mut NUMBER) {
    let mut inversed: NUMBER = NUMBER::new();
    let mut t1: NUMBER = NUMBER::new();
    let mut t2: NUMBER = NUMBER::new();
    let num_comped: i64;

    target.clear();
    let n: usize = s1.get_keta() + 1;

    inverse(s2, &mut inversed, n);
    multiple(s1, &inversed, &mut t1);
    t1.shift_right(n);

    increment(&t1, &mut t2);
    multiple(&s2, &t2, &mut t1);
    num_comped = num_comp(&t1, s1);
    if num_comped == 1 {
        decrement(&t2, target);
    } else {
        copy_number(&t2, target);
    }
}
