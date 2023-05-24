fn main() {
  let args: Vec<String> = std::env::args().collect();
  if args.len() != 2 {
    println!("hikisuuga tigau");
    std::process::exit(1);
  }
  println!(".intel_syntax noprefix");
  println!(".globl main");
  println!("main:");
  println!("  mov rax, {}", args[1]);
  println!("  ret");

fn strtol(vec:& Vec<char>, mut i:usize, base:u32) -> (i64, usize) {
  let mut res = 0;
  let mut size = 0;
  while i < vec.len() {
    match vec[i].to_digit(base){
      Some(v) => res = res * (base as i64) + (v as i64),
      None => break,
    }
    i += 1;
    size += 1;
  }
  (res, size)
}