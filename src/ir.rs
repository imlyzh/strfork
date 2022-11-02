


// #[derive(Debug)]
// pub struct Matchs(pub Vec<StringMatch>);


#[derive(Debug)]
pub enum Match {
  MatchString(String, Option<Box<Match>>),
  Fork(Vec<Match>),
  Loop(LoopType, Box<Match>),
}

#[derive(Debug)]
pub enum LoopType {
  ZeroOrMore,
  OneOrMore,
  // CycleNTimes(usize)
}
