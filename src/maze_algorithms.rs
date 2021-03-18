use crate::data::pathfinding_types::*;
use druid::im::HashSet;

pub struct MazeAlgorithmConfig {
    selected_algorithm: MazeAlgorithms,
    pub algorithm_state: AlgorithmState,
    current_path_node: PathNodes,
    open_list: HashSet<PathNodes>,
    closed_list: HashSet<PathNodes>,
}

impl MazeAlgorithmConfig {
    pub fn new() -> Self {
        MazeAlgorithmConfig {
            selected_algorithm: MazeAlgorithms::RecursiveBacktrace,
            algorithm_state: AlgorithmState::Initialization,
            current_path_node: PathNodes::empty(),
            open_list: HashSet::new(),
            closed_list: HashSet::new(),
        }
    }
}

impl GridAlgorithm for MazeAlgorithmConfig {
    fn run(&mut self, grid: &mut crate::gui::grid_widget::square_grid_widget_data::Grid) {
        todo!()
    }

    fn next_step(&mut self, grid: &mut crate::gui::grid_widget::square_grid_widget_data::Grid)  -> AlgorithmState {
        todo!()
    }

    fn previous_step(&mut self) {
        todo!()
    }

    fn reset(&mut self) {
        todo!()
    }

    fn get_next_node(&self) -> Option<PathNodes> {
        todo!()
    }

    fn get_open_nodes(&self) -> &HashSet<PathNodes> {
        return &self.open_list;

    }

    fn get_closed_nodes(&self) -> &HashSet<PathNodes> {
        return &self.closed_list;

    }
}