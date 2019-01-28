const ALLOWED_CHARS: &'static str =
  "abcdefghijklmnopqrstuwvxyzABCDEFGHIJKLMNOPQRSTUWVXYZ1234567890";

pub fn random_name(size: usize) -> String {
  use rand::{thread_rng, Rng};

  let mut random_string = String::with_capacity(size);

  let mut rng = thread_rng();

  loop {
    let n: usize = rng.gen_range(0, ALLOWED_CHARS.len());
    random_string.push(ALLOWED_CHARS.as_bytes()[n] as char);
    if random_string.len() >= size {
      break;
    }
  }

  random_string
}

#[cfg(test)]
mod tests {
  use super::random_name;

  #[test]
  fn test_random() {
    let t = random_name(10);
    assert_eq!(t.len(), 10);
  }
}
