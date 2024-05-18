#[tokio::main]
async fn main() {
  const N: usize = 1000000;
  let mut ip: Vec<bool> = vec![true; N+1];
  let mut primes: Vec<usize> = vec![];
  ip[1] = false;
  for i in 2..=N{
    if ip[i]{
      primes.push(i);
      for j in (i*i..=N).step_by(i){
        ip[j] = false;
      }
    }
  }
  print!("Number Of Primes: {}",primes.len())
}
