use std::time::Instant;

use crate::fft::{self, multiply_using_fft};
use crate::mulprec::{self, KETA, NUMBER};

pub fn check() {
    print!("<<<<<< check >>>>>>\n");
    is_zero();
    set_int();
    get_keta();
    copy_number();
    num_comp();
    add();
    // add_fibo();
    sub();
    increment();
    decrement();
    multiple();
    one_divide();
    shift_left();
    shift_right();
    inverse();
    fft();
}

pub fn set_int() {
    print!("\nset_int\n");
    let mut a = mulprec::NUMBER::new();
    a.set_int(123456789123456789);
    print!("a: ");
    a.display();
    print!("\n");
}

pub fn get_keta() {
    print!("\nget_keta\n");
    let mut a = mulprec::NUMBER::new();
    a.n[10] = 123;
    print!("a: ");
    a.display();
    print!("\n");
    print!("a: {}\n", a.get_keta());
}

pub fn is_zero() {
    print!("\nis_zero\n");
    let mut a = mulprec::NUMBER::new();
    let mut b = mulprec::NUMBER::new();
    a.set_int(12345678);
    b.clear();
    print!("a: ");
    a.display();
    print!("\n");
    print!("b: ");
    b.display();
    print!("\n");
    print!("a: {}\n", a.is_zero());
    print!("b: {}\n", b.is_zero());
}

pub fn copy_number() {
    print!("\ncopy_number\n");
    let mut a = mulprec::NUMBER::new();
    let mut b = mulprec::NUMBER::new();
    a.set_int(12345678);
    print!("a: ");
    a.display();
    print!("\n");
    print!("b: ");
    b.display();
    print!("\n");
    mulprec::copy_number(&a, &mut b);
    print!("a: ");
    a.display();
    print!("\n");
    print!("b: ");
    b.display();
    print!("\n");
}

pub fn num_comp() {
    print!("\nnum_comp\n");
    let mut a = mulprec::NUMBER::new();
    let mut b = mulprec::NUMBER::new();
    a.set_int(12345678);
    b.set_int(12345679);

    print!("a: ");
    a.display();
    print!("\n");
    print!("b: ");
    b.display();
    print!("\n");

    print!("{}", mulprec::num_comp(&a, &b));
    print!("\n");
}

pub fn add() {
    print!("\nadd\n");
    let mut a = mulprec::NUMBER::new();
    let mut b = mulprec::NUMBER::new();
    let mut c = mulprec::NUMBER::new();
    a.set_int(12345678);
    b.set_int(12345679);
    print!("a: ");
    a.display();
    print!("\n");
    print!("b: ");
    b.display();
    print!("\n");
    mulprec::add(&a, &b, &mut c);
    print!("c: ");
    c.display();
    print!("\n");
}

pub fn add_fibo() {
    print!("\nadd_fibo\n");
    let fmax = 100000; // 計算するフィボナッチ数列の最大インデックス
    let mut f0 = mulprec::NUMBER::new();
    let mut f1 = mulprec::NUMBER::new();
    let mut tmp = mulprec::NUMBER::new();

    f0.set_int(0);
    f1.set_int(1);

    let start = Instant::now(); // 計測開始

    for _ in 2..=fmax {
        mulprec::add(&f0, &f1, &mut tmp); // tmp = f0 + f1
        mulprec::copy_number(&f1, &mut f0); // f0 = f1
        mulprec::copy_number(&tmp, &mut f1); // f1 = tmp
    }

    let duration = start.elapsed(); // 計測終了

    // 結果を表示
    print!("F{} = ", fmax);
    f1.display();
    println!();

    // 経過時間を表示
    println!("Elapsed time: {:?}", duration);
}

pub fn sub() {
    print!("\nsub\n");
    let mut a = mulprec::NUMBER::new();
    let mut b = mulprec::NUMBER::new();
    let mut c = mulprec::NUMBER::new();
    a.set_int(12345678);
    b.set_int(12345676);
    print!("a: ");
    a.display();
    print!("\n");
    print!("b: ");
    b.display();
    print!("\n");
    mulprec::sub(&a, &b, &mut c);
    print!("c: ");
    c.display();
    print!("\n");
}

pub fn increment() {
    print!("\nincrement\n");
    let mut a = mulprec::NUMBER::new();
    let mut b = mulprec::NUMBER::new();
    a.set_int(12345678);
    print!("a: ");
    a.display();
    print!("\n");
    mulprec::increment(&mut a, &mut b);
    print!("b: ");
    b.display();
    print!("\n");
}

pub fn decrement() {
    print!("\ndecrement\n");
    let mut a = mulprec::NUMBER::new();
    let mut b = mulprec::NUMBER::new();
    a.set_int(12345678);
    print!("a: ");
    a.display();
    print!("\n");
    mulprec::decrement(&mut a, &mut b);
    print!("b: ");
    b.display();
    print!("\n");
}

pub fn one_divide() {
    print!("\none_divide\n");
    let mut a = mulprec::NUMBER::new();
    let mut b = mulprec::NUMBER::new();
    let mut c = mulprec::NUMBER::new();
    a.set_int(121932631112635269);
    b.set_int(123456789);
    print!("a: ");
    a.display();
    print!("\n");
    print!("b: ");
    b.display();
    print!("\n");
    mulprec::divide(&a, &b, &mut c);
    print!("c: ");
    c.display();
    print!("\n");
}

pub fn multiple() {
    print!("\nmultiple\n");
    let mut a = mulprec::NUMBER::new();
    let mut b = mulprec::NUMBER::new();
    let mut c = mulprec::NUMBER::new();
    a.set_int(12345678);
    b.set_int(12345678);
    print!("a: ");
    a.display();
    print!("\n");
    print!("b: ");
    b.display();
    print!("\n");
    mulprec::multiple(&a, &b, &mut c);
    print!("c: ");
    c.display();
    print!("\n");
}

pub fn shift_left() {
    print!("\nshift_left\n");
    let mut a = mulprec::NUMBER::new();
    a.set_int(12345678);
    print!("a: ");
    a.display();
    print!("\n");
    a.shift_left(1);
    print!("a: ");
    a.display();
    print!("\n");
}

pub fn shift_right() {
    print!("\nshift_right\n");
    let mut a = mulprec::NUMBER::new();
    a.set_int(12345678);
    a.shift_left(2);
    print!("a: ");
    a.display();
    print!("\n");
    a.shift_right(1);
    print!("a: ");
    a.display();
    print!("\n");
}

pub fn inverse() {
    print!("\ninverse\n");
    let mut a = mulprec::NUMBER::new();
    let mut b = mulprec::NUMBER::new();
    a.set_int(123456789);
    print!("a: ");
    a.display();
    print!("\n");
    mulprec::inverse(&a, &mut b, 50);
    print!("b: ");
    b.display();
    print!("\n");
}
