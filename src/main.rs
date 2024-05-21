use std::{io::Write, io};
#[tokio::main]
async fn main() {
  let N = cargo()
  let mut ip: Vec<bool> = vec![true; N+1];
  let mut primes: Vec<usize> = vec![];
  for i in 2..=N{
    if ip[i]{
      primes.push(i);
      let mut sq = i*i;
      while sq <= N{
        ip[sq] = false;
        sq += i;
      }
    }
  }
  print!("Number Of Primes: {}",primes.len());
}

#[tokio::command]
async fn prompt(text:&str) -> usize {
  let mut input_line = String::new();
  print!("{}", text);
  std::io::stdout().flush().unwrap();
  io::stdin() // the rough equivalent of `std::cin`
    .read_line(&mut input_line) // actually read the line
    .expect("Failed to read line"); // which can fail, however
  let x: usize = input_line
    .trim() // ignore whitespace around input
    .parse() // convert to integers
    .expect("Input not an integer"); // which, again, can fail
  return x;
}

#[tokio::command]
async fn cargo() -> Vec<String> {
  let mut args: Vec<String> = std::env::args().collect();
  if &args[0] == "target/debug/fear-and-hunger" { args.remove(0); };
  if args.len() == 0 {
    let N: usize = prompt("primes up to ");
  } else if args.len() == 1 {
    let N: usize = args[0];
  } else {
    panic!("WHAT NOW?")
  };
  return N;
}
