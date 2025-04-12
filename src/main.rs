use pi::{check_pi, get_pi};

// mod check;
mod mulprec;
mod pi;

fn main() -> std::io::Result<()> {
   let mut pi = mulprec::Number::new();

   let start = std::time::Instant::now();
   get_pi(&mut pi);
   let end = std::time::Instant::now();

   println!("pi");
   pi.display();
   println!();
   println!("time: {}s", (end - start).as_secs_f64());

   let matches = check_pi(&pi);
   println!("match: {}", matches);
   
    // check::check();

    Ok(())
}
