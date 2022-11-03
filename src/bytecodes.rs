


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

/*
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum SFbytecode {
  accept  = 0b0000_0010,
  reject  = 0b0000_0100,
  skip    = 0b0000_0110,  // idx(index) = idx+1
  push    = 0b0000_1000,  // push(idx)

  jump    = 0b0000_0001,  // pc(program counter) = pc+load(pc+1)
  // jump_b  = 0b1001_0001,  // pc = pc - n

  match1u = 0b0000_1001,  // if load1(idx) != load1(pc+1) { idx = pop(); jump(top()); } else { idx=idx+1; }

  range1u = 0b0000_0011,

  fork2   = 0b0000_1011,  // push(load(pc+2)); push(idx); jump(load(pc+1));
}
 */

pub const accept: u8  = 0b0000_0010;
pub const reject: u8  = 0b0000_0100;
pub const skip: u8    = 0b0000_0110;  // idx(index) = idx+1
pub const push: u8    = 0b0000_1000;  // push(idx)
pub const jump: u8    = 0b0000_0001;  // pc(program counter) = pc+load(pc+1)
pub const match1u: u8 = 0b0000_1001;  // if load1(idx) != load1(pc+1) { idx = pop(); jump(top()); } else {
pub const range1u: u8 = 0b0000_0011;
pub const fork2: u8   = 0b0000_1011;