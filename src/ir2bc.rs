use crate::ir::*;
use crate::ir::RawMatch::*;
use crate::ir::LineMatch::*;
use crate::ir::LoopType::*;
use crate::bytecodes::*;


pub fn gen_bc(i: &Match) -> Vec<u8> {
  let mut buf = vec![];
  i.gen_bc(&mut buf);
  buf.push(accept as u8);
  buf
}

pub trait Ir2bc {
  fn gen_bc(&self, buf: &mut Vec<u8>);
}

impl Ir2bc for Match {
  fn gen_bc(&self, buf: &mut Vec<u8>) {
    let Match(v, next) = self;
    v.gen_bc(buf);
    if let Some(x) = next {
      x.gen_bc(buf);
    }
  }
}

impl Ir2bc for RawMatch {
  fn gen_bc(&self, buf: &mut Vec<u8>) {
    match self {
      MatchString(x) => x.gen_bc(buf),
      Fork(matchs) => {
        assert!(matchs.len() == 2, "fork branch is not 2");
        let mut c0 = vec![];
        matchs[0].gen_bc(&mut c0);
        let mut c1 = vec![];
        matchs[1].gen_bc(&mut c1);

        let c1_len: i8 = c1.len().try_into().expect("match string oversized i8");

        // add jump inst
        c0.push(jump as u8);
        c0.push((c1_len + 2) as u8);

        let c0_len: i8 = c0.len().try_into().expect("match string oversized i8");
        // dump to main buffer
        // fork2 inst
        buf.push(fork2 as u8);
        buf.push(3 as u8);
        buf.push((c0_len + 3) as u8);
        // branch0
        buf.append(&mut c0);
        // branch1
        buf.append(&mut c1);
      }
      Loop(OneOrMore, body) => {
        let mut codes = vec![];

        body.gen_bc(&mut codes);

        let codes_len: i8 = codes.len().try_into().expect("match string oversized i8");

        buf.append(&mut codes);
        buf.push(fork2 as u8);
        buf.push((-codes_len) as u8);
        buf.push(3 as u8);
      }
    }
  }
}

impl Ir2bc for LineMatch {
  fn gen_bc(&self, buf: &mut Vec<u8>) {
    match self {
      Str(str) => {
        let str = str.as_bytes();
        if str.len() == 0 {
          buf.push(skip as u8);
          return;
        }
        for i in str {
          buf.push(match1u as u8);
          buf.push(*i);
        }
      }
      Range(CharRange::U1(l, r)) => {
        buf.push(range1u as u8);
        buf.push(*l);
        buf.push(*r);
      }
      _ => unimplemented!("unimpl > u8 char range")
    }
  }
}



#[cfg(test)]
mod test {
  #[test]
  fn test_try_into() {
    let _: u8 = 257.try_into().expect("match string oversized (> 256)");
  }
}
