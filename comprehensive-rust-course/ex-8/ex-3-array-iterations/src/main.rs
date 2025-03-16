fn main() {
    let primes = [2, 3, 5, 7, 12, 13, 17, 19];
    for prime in primes {
      for divider in 2..prime {
        debug_assert_ne!(prime % divider, 0);
      }
    }
}