use super::pathfinding_types::*;
use druid::Data;
use druid::im::{Vector, HashSet};

#[derive(Data, Clone, Eq, PartialEq, Debug)]
pub struct GreedyBestFirstSearch{
    algorithm_state: PathAlgorithmState,
    open_list: HashSet<PathNodes>,
    closed_list: HashSet<PathNodes>,
    path_list: Vector<PathNodes>,
    current_path_node: PathNodes, 
}

impl GreedyBestFirstSearch {
    pub fn new() -> Self {
        GreedyBestFirstSearch {
            algorithm_state: PathAlgorithmState::Initialization,
            open_list: HashSet::new(),
            closed_list: HashSet::new(),
            path_list: Vector::new(), 
            current_path_node: PathNodes::empty(),
        }
    }
}

impl PathFinderAlgorithm for GreedyBestFirstSearch {
    fn run(&mut self, grid: &mut crate::gui::grid_widget::square_grid_widget_data::Grid, config: &mut PathfinderConfig) {
        todo!()
    }

    fn next_step(&mut self, grid: &mut crate::gui::grid_widget::square_grid_widget_data::Grid, config: &mut PathfinderConfig)  -> PathAlgorithmState {
        todo!()
    }

    fn previous_step(&mut self, grid: &mut crate::gui::grid_widget::square_grid_widget_data::Grid, config: &mut PathfinderConfig) {
        todo!()
    }

    fn reset(&mut self) {
        todo!()
    }

    fn construct_path(&mut self) {
        todo!()
    }

    fn get_next_node(&self) -> Option<PathNodes> {
        todo!()
    }

    fn get_open_nodes(&self) -> &HashSet<PathNodes> {
        todo!()
    }

    fn get_closed_nodes(&self) -> &HashSet<PathNodes> {
        todo!()
    }

    fn get_path_nodes(&self) -> &Vector<PathNodes> {
        todo!()
    }

    fn get_algorithm_state(&self) -> &PathAlgorithmState {
        todo!()
    }
}