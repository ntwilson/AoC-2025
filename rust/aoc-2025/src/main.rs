use std::io;
use std::time::Instant;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod shared;

fn main() -> io::Result<()> {
  day1::run()?;
  //   day2::run(); // takes too long
  let _ = day2::run; // get rid of warnings of unused code
  day3::run();
  day4::run()?;
  day5::run()?;
  day6::run();
  day7::run();
  let start = Instant::now();
  day8::run();
  let duration = start.elapsed();
  println!("day8::run() took: {:?}", duration);
  Ok(())
}
