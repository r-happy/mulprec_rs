use pi::{check_pi, get_pi};

// mod check;
mod mulprec;
mod pi;

fn main() -> std::io::Result<()> {
   let mut pi = mulprec::NUMBER::new();

   let start = std::time::Instant::now();
   get_pi(&mut pi);
   let end = std::time::Instant::now();

   print!("pi\n");
   pi.display();
   print!("\n");
   print!("time: {}s\n", (end - start).as_secs_f64());

   let matches = check_pi(&pi);
   print!("match: {}\n", matches);
   
    // check::check();

    Ok(())
}
