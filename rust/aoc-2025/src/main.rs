use std::io;

mod day1;
mod day2;
mod day3;
mod shared;

fn main() -> io::Result<()> {
  day1::run()?;
  //   day2::run(); // takes too long
  day3::run();
  Ok(())
}
