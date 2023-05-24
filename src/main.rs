fn main() {
  let args: Vec<String> = std::env::args().collect();
  if args.len() != 2 {
    eprintln!("hikisuuga tigau");
    std::process::exit(1);
  }
  let args = args[1].chars().collect();
  let mut i = 0;
  let (mut val, mut size) = strtol(&args, i, 10);
  i += size;
  println!(".intel_syntax noprefix");
  println!(".globl main");
  println!("main:");
  println!("  mov rax, {}",  val);
  while i < args.len(){
    match args[i] {
      '+' => {
        i += 1;
        (val, size) = strtol(&args, i, 10);
        i += size;
        println!("  add rax, {}", val);
      },
      '-' =>{
        i += 1;
        (val, size) = strtol(&args, i, 10);
        i+= size;
        println!("  sub rax, {}", val);
      }
      _ => {
        eprintln!("err");
        std::process::exit(1);
      },
    }
  }
  println!("  ret");
}

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