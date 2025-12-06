use std::io;

mod day1;
mod day2;

fn main() -> io::Result<()> {
  day1::run()?;
  day2::run();
  Ok(())
}
