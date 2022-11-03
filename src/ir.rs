


// #[derive(Debug)]
// pub struct Matchs(pub Vec<StringMatch>);


#[derive(Debug)]
pub struct Match(pub RawMatch, pub Option<Box<Match>>);

#[derive(Debug)]
pub enum RawMatch {
  MatchString(LineMatch),
  Fork(Vec<Match>),
  Loop(LoopType, Box<Match>),
}

#[derive(Debug)]
pub enum LineMatch {
  Str(String),
  Range(CharRange),
}

#[derive(Debug)]
pub enum CharRange {
  U1(u8, u8),
  U2(u16, u16),
  U4(u32, u32),
}

#[derive(Debug)]
pub enum LoopType {
  ZeroOrMore,
  OneOrMore,
  // CycleNTimes(usize)
}
