


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
  Range(Vec<char>),
}

#[derive(Debug)]
pub enum LoopType {
  ZeroOrMore,
  OneOrMore,
  // CycleNTimes(usize)
}
