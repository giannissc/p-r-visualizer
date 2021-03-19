use super::maze_generation_types::*;
use druid::Data;
use druid::im::{HashSet, Vector};
use crate::gui::grid_widget::square_grid_widget_data::*;
use crate::data::app_data::{AppData, GRID_COLUMNS, GRID_ROWS};

#[derive(Data, Clone, Eq, PartialEq, Debug)]
pub struct RecursiveSubdivision {
    algorithm_state: MazeAlgorithmState,
    closed_list: HashSet<MazeNodes>,
    current_path_node: MazeNodes, 
}

impl RecursiveSubdivision {
    pub fn new() -> Self {
        RecursiveSubdivision {
            algorithm_state: MazeAlgorithmState::Initialization,
            closed_list: HashSet::new(),
            current_path_node: MazeNodes::empty(),
        }
    }
}

impl MazeGenerationAlgorithm for RecursiveSubdivision {
    fn run(&mut self, grid: &mut crate::gui::grid_widget::square_grid_widget_data::Grid) {
        todo!()
    }

    fn next_step(&mut self, grid: &mut crate::gui::grid_widget::square_grid_widget_data::Grid)  -> MazeAlgorithmState {
        todo!()
    }

    fn previous_step(&mut self) {
        todo!()
    }

    fn reset(&mut self) {
        self.closed_list.clear();
        self.algorithm_state = MazeAlgorithmState::Initialization;
    }

    fn get_next_node(&mut self, grid: &mut Grid) -> Option<MazeNodes> {
        todo!()
    }

    fn get_closed_nodes(&self) -> &HashSet<MazeNodes> {
        &self.closed_list
    }

    fn get_algorithm_state(&self) -> &MazeAlgorithmState {
        &self.algorithm_state
    }
}