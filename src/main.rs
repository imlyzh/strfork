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

  for (size, i) in codes.iter().enumerate() {
    println!("{:4}: {:8b} {}", size, i, i);
  }

  let state = State::new();

  let src = "bb".as_bytes();
  let result = state.direct_interpret(&codes, src);
  println!("out: {}", result);
}
