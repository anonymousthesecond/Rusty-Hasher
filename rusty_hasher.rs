use std::io::{self, Read};
use ring::{digest, Digest};

fn hash_data(data: &[u8]) -> Result<Vec<u8>, ring::Error> {
  let mut hasher = digest::Context::new(digest::SHA512_ALG);
  hasher.update(data);
  let result = hasher.finish();
  Ok(result.as_ref().to_vec())
}

fn hash_from_file(path: &str) -> Result<Vec<u8>, ring::Error> {
  let mut file = std::fs::File::open(path)?;
  let mut buffer = Vec::new();
  file.read_to_end(&mut buffer)?;
  hash_data(&buffer)
}

fn hash_from_stdin() -> Result<Vec<u8>, ring::Error> {
  let mut stdin = io::stdin();
  let mut buffer = Vec::new();
  stdin.read_to_end(&mut buffer)?;
  hash_data(&buffer)
}

fn main() {
  let mut args = std::env::args();
  args.next(); // Skip the program name

  if let Some(arg) = args.next() {
    // Hashing from a file
    if arg == "-" {
      println!("Reading data from standard input...");
      let hash_value = hash_from_stdin()?;
      println!("SHA-512 hash: {:?}", hash_value);
    } else {
      println!("Reading data from file: {}", arg);
      let hash_value = hash_from_file(&arg)?;
      println!("SHA-512 hash: {:?}", hash_value);
    }
  } else {
    println!("Usage: sha512_hasher <file_path> OR - (for standard input)");
  }
}
