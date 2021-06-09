/*                      square of Mersenne prime M61=(2^61-1)
  $ cargo run --release 5316911983139663487003542222693990401
      Finished release [optimized] target(s) in 0.00s
       Running `target/release/rhos 5316911983139663487003542222693990401`
  [2163790305] 5080385247327ns
  5316911983139663487003542222693990401: 2305843009213693951 2305843009213693951
$
*/
use std::env;
use ethnum::*;
use fine_grained::Stopwatch;

fn main() {
    let args: Vec<String> = env::args().collect();  assert!(args.len()==2);
    let n = args[1].parse::<u128>();                assert!(n.is_ok());

    pollard_rho_1st(n.unwrap());
}

fn f(x: U256, a: u128, n: u128) -> u128 {
  let r: u256 = (x*x + a) % n;
  return *r.low();
}

fn pollard_rho_1st(n: u128) {
    let mut c: u64 = 0;

    let mut a: u128 = 1;

    let mut g: u128;
    let mut x: u128;
    let mut y: u128;
    let mut d: u128;

    let mut stopwatch = Stopwatch::start_new();

    loop {
        x = 2;
        y = 2;
        g = 1;
        while g == 1 {
            c += 1;
            x = f(x.into(), a, n);
            y = f(f(y.into(), a, n).into(), a, n);
            if x<y { d = y-x; } else { d = x-y; }
            g = gcd(n, d);
        }
        if g<n {
            break;
        }
        a += 1;
    }

    println!("[{:}] {duration}", c, duration = stopwatch);
    stopwatch.stop();

    println!("{:}: {:} {:}", n, g, n/g);
}

// "gcd()" taken from Wikipedia; comments removed and u64 replaced with u128:
// https://en.wikipedia.org/wiki/Binary_GCD_algorithm#Iterative_version_in_Rust
pub fn gcd(mut u: u128, mut v: u128) -> u128 {
    use std::cmp::min;
    use std::mem::swap;

    if u == 0 {
        return v;
    } else if v == 0 {
        return u;
    }

    let i = u.trailing_zeros();  u >>= i;
    let j = v.trailing_zeros();  v >>= j;
    let k = min(i, j);

    loop {
        if u > v {
            swap(&mut u, &mut v);
        }

        v -= u;

        if v == 0 {
            return u << k;
        }

        v >>= v.trailing_zeros();
    }
}
