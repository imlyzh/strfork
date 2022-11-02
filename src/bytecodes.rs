


/*
#[derive(Debug)]
pub enum HighlevelSFbytecode {
  Accept,
  Reject,
  Skip,
  Match1u(u8),
  Match2u([u8; 2]),
  Match3u([u8; 3]),
  Match4u([u8; 4]),
  MatchN(Vec<u8>),
  TryCycle(usize),          // loop end address(label)
  Fork2(usize),             // branch 2 address(label)
  // Fork(Vec<usize>),
  // StartCountCycle(usize),   // cycle count
  // ComplatedOnceCycle(usize),// loop start address(label)
}
// */

#[derive(Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum SFbytecode {
  accept  = 0b0000_0010,
  reject  = 0b0000_0100,
  skip    = 0b0000_0110,  // idx(index) = idx+1
  push    = 0b0000_1000,  // push(idx)

  jump_f  = 0b0001_0001,  // pc(program counter) = pc+1+n
  jump_b  = 0b1001_0001,  // pc = pc - n

  match1u = 0b0000_1001,  // if load1(idx) != load1(pc+1) { idx = pop(); jump(top()); } else { idx=idx+1; }
  // match2u = 0b0001_1001,  // if load1(idx) != load2(pc+1) { idx = pop(); jump(top()); } else { idx=idx+1; }
  // match3u = 0b0010_1001,
  // match4u = 0b0011_1001,

  range1u = 0b0000_0011,
  // range2u = 0b0001_0011,
  // range3u = 0b0010_0011,
  // range4u = 0b0011_0011,

  fork2   = 0b0000_1011,  // push(load(pc+2)); jump(load(pc+1));
}
