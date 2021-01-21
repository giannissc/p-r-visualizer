use druid::Data;
#[derive(Data, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum MazeAlgorithms {
    Random,
    Backtrace,
    Recursive,
}