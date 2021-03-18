use druid::Data;
use druid::im::HashSet;
use crate::gui::grid_widget::square_grid_widget_data::*;
use std::hash::{Hash, Hasher, };

#[derive(Data, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum MazeAlgorithms {
    Random,
    RecursiveBacktrace,
    RecursiveSubdivision,
}

pub trait MazeGenerationAlgorithm{
    fn run(&mut self, grid: &mut Grid);
    fn next_step(&mut self, grid: &mut Grid)  -> MazeAlgorithmState;
    fn previous_step(&mut self);
    fn reset(&mut self);
    fn get_next_node(&self) -> Option<MazeNodes>;
    fn get_open_nodes(&self) -> &HashSet<MazeNodes>;
    fn get_closed_nodes(&self) -> &HashSet<MazeNodes>;
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
#[derive(Data, Copy, Clone, Debug, Eq, )]
pub struct MazeNodes {
    pub position: GridNodePosition,
    pub parent: Option<GridNodePosition>,
}

impl MazeNodes {

    pub fn new(current_pos:GridNodePosition, parent: Option<GridNodePosition>) -> Self {
        MazeNodes {
            position: current_pos,
            parent: parent,
        }
    }

    pub fn empty() -> Self {
        MazeNodes {
            position: GridNodePosition{row: 0, col: 0},
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