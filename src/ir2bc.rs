use std::vec;

use crate::ir::*;
use crate::ir::Match::*;
use crate::bytecodes::*;
use crate::bytecodes::SFbytecode::*;


pub trait Ir2bc {
  fn gen_bc(&self, buf: &mut Vec<u8>);
}

impl Ir2bc for Match {
  fn gen_bc(&self, buf: &mut Vec<u8>) {
    match self {
      MatchString(str, next) => {
        let str = str.as_bytes();
        match str.len() {
          0 => buf.push(skip as u8),
          // 1 => buf.push(Match1u(str.try_into().unwrap())),
          // 2 => buf.push(Match2u(str.try_into().unwrap())),
          // 3 => buf.push(Match3u(str.try_into().unwrap())),
          // 4 => buf.push(Match4u(str.try_into().unwrap())),
          _ => {
            buf.push(matchn as u8);
            buf.push(str.len().try_into().expect("match string oversized (> 256)"));
            buf.extend(str)
          },
        };
        if let Some(x) = next {
          x.gen_bc(buf);
        }
      }
      Fork(matchs) => {
        assert!(matchs.len() == 2, "fork branch is not 2");
        let mut b1_code = vec![];
        matchs[0].gen_bc(&mut b1_code);

        let b1_len: u8 = b1_code.len().try_into().expect("match string oversized (> 256)");
        assert!(b1_len > 0, "branch is not zero");

        let mut b2_code = vec![];
        matchs[0].gen_bc(&mut b1_code);

        let b2_len: u8 = b1_code.len().try_into().expect("match string oversized (> 256)");
        assert!(b2_len > 0, "branch is not zero");

        buf.push(fork2 as u8);
        buf.push(b1_len-1);
        buf.append(&mut b1_code);

        buf.push(jump_b as u8);
        buf.push(b2_len);

        buf.append(&mut b1_code);

        buf.push(pop as u8);
      }
      _ => todo!()
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
