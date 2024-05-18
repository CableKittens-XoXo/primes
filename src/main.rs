#[tokio::main]
async fn main() {
  const N: usize = 10000000;
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
