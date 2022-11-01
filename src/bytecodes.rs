

#[repr(u8)]

#[derive(Debug)]
pub enum SFbytecode {
  Accept,
  Reject,
  Match1c(u8),
  Match2c([u8; 2]),
  Match3c([u8; 3]),
  Match4c([u8; 4]),
  TryCycle(usize),          // loop end address(label)
  Fork2(usize),             // branch 2 address(label)
  // Fork(Vec<usize>),
  // StartCountCycle(usize),   // cycle count
  // ComplatedOnceCycle(usize),// loop start address(label)
}

#[derive(Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum UnwarpedFbytecode {
  accept,
  reject,
  match1c,
  match2c,
  match3c,
  match4c,
  try_cycle,
  fork2,
  // fork,
  // start_count_cycle,
  // complated_once_cycle,
}
