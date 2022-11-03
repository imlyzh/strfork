use strfork::ir::*;
use strfork::ir2bc::gen_bc;
use strfork::direct_interpreter::State;

fn main() {
  println!("strfork");
  let ir = Match(RawMatch::Fork(vec![
    Match(RawMatch::MatchString(LineMatch::Str("aa".to_string())), None),
    Match(RawMatch::MatchString(LineMatch::Str("bb".to_string())), None),
  ]), None);
  let codes = gen_bc(&ir);

  for i in codes.iter() {
    println!("{:8b} {}", i, i);
  }

  let state = State::new();

  let src = "ab".as_bytes();
  let result = state.direct_interpret(&codes, src);
  println!("out: {}", result);
}
