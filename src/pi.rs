use std::io::{self, Write};

use crate::mulprec::{self, KETA, NUMBER, SHIFT, add, copy_number, divide, multiple, num_comp};

/*
    target <- pi
    ニュートン無平方根法
*/
pub fn get_pi(target: &mut mulprec::NUMBER) {
    let mut denominator1 = NUMBER::new();
    let mut denominator2 = NUMBER::new();
    let mut numerator = NUMBER::new();
    let mut pi = NUMBER::new();
    let mut t_pi = NUMBER::new();
    let zero = NUMBER::new();
    let mut twoi = NUMBER::new();
    let mut t1 = NUMBER::new();
    let mut t2 = NUMBER::new();
    let mut i: usize = 0;

    t_pi.n[SHIFT] = 1;
    pi.n[SHIFT] = 1;

    loop {
        if i % 200 == 0 {
            print!("#");
            io::stdout().flush().unwrap();
        }

        // numerator
        twoi.set_int((2 * i + 3) as i64);
        t1.set_int((2 * i + 1) as i64);
        multiple(&t_pi, &t1, &mut numerator);

        // denominators
        denominator1.set_int((8 * (i + 1)) as i64);
        denominator2.set_int((2 * i + 3) as i64);

        if num_comp(&numerator, &zero) == 0 {
            break;
        }

        divide(&numerator, &denominator1, &mut t1);
        divide(&t1, &denominator2, &mut t2);

        add(&pi, &t2, &mut t1);
        copy_number(&t1, &mut pi);

        multiple(&t2, &twoi, &mut t_pi);

        i += 1;
    }

    t1.set_int(3);
    multiple(&pi, &t1, target);
    print!("\n");
}

pub fn check_pi(a: &NUMBER) -> i32 {
    let mut file = vec![0; KETA * 9];

    let file_result = std::fs::File::open("pi.txt");
    let file_handle = match file_result {
        Ok(file) => file,
        Err(_) => return -1,
    };

    let mut i = 0;
    let mut reader = std::io::BufReader::new(file_handle);
    use std::io::Read;

    // Read digits from file
    let mut buffer = [0u8; 1];
    while i < KETA * 9 {
        match reader.read_exact(&mut buffer) {
            Ok(_) => {
                let r = buffer[0] as char;
                if r >= '0' && r <= '9' {
                    file[i] = (r as u8 - b'0') as i32;
                    i += 1;
                }
            }
            Err(_) => break,
        }
    }

    let mut r_value = 0;
    for j in 0..a.get_keta() - 1 {
        let mut digit = a.n[a.get_keta() - j - 1];
        let mut cnt = 0;
        r_value += 1;
        while digit != 0 {
            if digit % 10 != file[(j * 9 - cnt) as usize] as i64 {
                break;
            }
            digit /= 10;
            cnt += 1;
        }
    }

    r_value * 9
}
