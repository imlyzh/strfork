
use crate::bytecodes::*;

#[derive(Debug)]
pub struct State {
  pub pc: usize,
  pub idx: usize,
  // pub stack: VecDeque<u8>,
  pub history: Option<Box<State>>,
}

impl State {
  pub fn new() -> Self {
    State { pc: 0, idx: 0, history: None }
  }

  pub fn push(self) -> Self {
    State {
      pc: self.pc.clone(),
      idx: self.idx.clone(),
      history: Some(Box::new(self)),
    }
  }

  pub fn push_backpoint(mut self, pc: usize, idx: usize) -> Self {
    let old_pc = self.pc.clone();
    let old_idx = self.idx.clone();
    self.pc = pc;
    self.idx = idx;
    State {
      pc: old_pc,
      idx: old_idx,
      history: Some(Box::new(self)),
    }
  }

  pub fn pop(self) -> Option<State> {
    self.history.map(|x| *x)
  }
}

impl State {
  #[inline]
  pub fn direct_interpret(mut self, codes: &[u8], src: &[u8]) -> bool {
    loop {
      println!("eval");
      let pc = dbg!(self.pc);
      let idx = dbg!(self.idx);
      let code = dbg!(codes[pc]);
      match code {
        accept => return true,
        skip => {
          self.idx += 1;
          self.pc += 1;
        },
        jump => {
          let offset = codes[self.pc+1] as i8 as isize;
          self.pc = (self.pc as isize + offset) as usize;
        }
        match1u => {
          if codes[self.pc+1] != src[self.idx] {
            if let Some(x) = self.pop() {
              self = x;
              continue;
            } else {
              return false;
            }
          } else {
            self.idx+=1;
            self.pc+=2;
          }
        }
        fork2 => {
          let br1 = codes[self.pc+1] as i8 as isize;
          let br2 = codes[self.pc+2] as i8 as isize;
          let br1pc = (self.pc as isize + br1) as usize;
          let br2pc = (self.pc as isize + br2) as usize;
          let idx = self.idx;
          println!("fork2:");
          dbg!(br2pc);
          dbg!(idx);
          self = self.push_backpoint(br2pc, idx);
          self.pc = br1pc;
          continue;
        }
        code => {
          unimplemented!("code: {}", code);
        }
      }
    }
  }
}

/*
pub extern "C" fn wrapper_direct_interpret(&mut State, codes: ) {

}
 */