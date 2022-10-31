

#[repr(u8)]

#[derive(Debug)]
pub enum SFbytecode {
  Matchc(u8),
  Accept,
  Reject
}


#[derive(Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum UnwarpedFbytecode {
  matchc,
  accept,
  reject
}
