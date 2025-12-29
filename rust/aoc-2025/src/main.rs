use std::io;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod shared;

fn main() -> io::Result<()> {
  day1::run()?;
  //   day2::run(); // takes too long
  let _ = day2::run; // get rid of warnings of unused code
  day3::run();
  day4::run()?;
  day5::run()?;
  day6::run();
  Ok(())
}
