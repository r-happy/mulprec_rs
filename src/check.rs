use std::time::Instant;

use crate::mulprec;

pub fn check() {
    print!("<<<<<< check >>>>>>\n");
    is_zero();
    set_int();
    copy_number();
    num_comp();
    add();
    add_fibo();
}

pub fn set_int() {
    print!("\nset_int\n");
    let mut a = mulprec::NUMBER::new();
    a.set_int(123456789123456789);
    print!("a: ");
    a.display();
    print!("\n");
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
