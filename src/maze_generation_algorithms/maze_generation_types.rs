use super::{
    random::Random, recursive_backtrace::RecursiveBacktrace,
    recursive_subdivision::RecursiveSubdivision,
};
use crate::gui::grid_widget::square_grid_widget_data::*;
use druid::im::{HashSet, Vector};
use druid::Data;
use std::hash::{Hash, Hasher};

#[derive(Data, Clone, Eq, PartialEq, Debug)]
pub enum MazeAlgorithms {
    Random(Random),
    RecursiveBacktrace(RecursiveBacktrace),
    RecursiveSubdivision(RecursiveSubdivision),
}

impl MazeAlgorithms {
    pub fn get_inner(&mut self) -> Box<&mut dyn MazeGenerationAlgorithm> {
        match self {
            MazeAlgorithms::Random(inner) => Box::new(inner),
            MazeAlgorithms::RecursiveBacktrace(inner) => Box::new(inner),
            MazeAlgorithms::RecursiveSubdivision(inner) => Box::new(inner),
        }
    }
}

pub trait MazeGenerationAlgorithm {
    fn run(&mut self, grid: &mut Grid);
    fn next_step(&mut self, grid: &mut Grid) -> MazeAlgorithmState;
    fn previous_step(&mut self);
    fn reset(&mut self);
    fn get_next_node(&mut self, grid: &mut Grid) -> Option<MazeNodes>;
    fn get_closed_nodes(&self) -> &HashSet<MazeNodes>;
    fn get_algorithm_state(&self) -> &MazeAlgorithmState;
}

#[derive(Data, Clone, Eq, PartialEq, Debug)]
pub struct MazeGenerationConfig {
    pub algorithm_state: MazeAlgorithmState,
    pub open_list: HashSet<MazeNodes>,
    pub closed_list: HashSet<MazeNodes>,
    pub current_path_node: MazeNodes,
}

impl MazeGenerationConfig {
    pub fn new() -> Self {
        MazeGenerationConfig {
            algorithm_state: MazeAlgorithmState::Initialization,
            open_list: HashSet::new(),
            closed_list: HashSet::new(),
            current_path_node: MazeNodes::empty(),
        }
    }
}
#[derive(Data, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Debug)]
pub enum MazeAlgorithmState {
    Initialization,
    Running,
    Finished,
    Failed,
}
#[derive(Data, Copy, Clone, Debug, Eq)]
pub struct MazeNodes {
    pub position: GridNodePosition,
    pub parent: Option<GridNodePosition>,
}

impl MazeNodes {
    pub fn new(current_pos: GridNodePosition, parent: Option<GridNodePosition>) -> Self {
        MazeNodes {
            position: current_pos,
            parent: parent,
        }
    }

    pub fn empty() -> Self {
        MazeNodes {
            position: GridNodePosition { row: 0, col: 0 },
            parent: None,
        }
    }
}

impl PartialEq for MazeNodes {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

impl Hash for MazeNodes {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.position.hash(hasher);
    }
}
