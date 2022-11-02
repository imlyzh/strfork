


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
  skip    = 0b0000_1000,
  pop     = 0b0000_0001,
  jump_f  = 0b0001_0001,
  jump_b  = 0b1001_0001,
  match1u = 0b0001_1001,
  match2u = 0b0010_1001,
  match3u = 0b0011_1001,
  match4u = 0b0100_1001,
  match5u = 0b0101_1001,
  match6u = 0b0110_1001,
  match7u = 0b0111_1001,
  match8u = 0b1000_1001,

  try_cycle = 0b0000_1101,
  fork2     = 0b0001_1101,
  // fork,
  // start_count_cycle,
  // complated_once_cycle,
}
