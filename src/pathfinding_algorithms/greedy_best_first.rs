use super::pathfinding_types::*;
use druid::Data;
use druid::im::{Vector, HashSet};

#[derive(Data, Clone, Eq, PartialEq, Debug)]
pub struct GreedyBestFirstSearch{
    pub algorithm_state: PathAlgorithmState,
    pub open_list: HashSet<PathNodes>,
    pub closed_list: HashSet<PathNodes>,
    pub path_list: Vector<PathNodes>,
    pub current_path_node: PathNodes, 
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

impl PathfinderAlgorithm for GreedyBestFirstSearch {
    fn run(&mut self, grid: &mut crate::gui::grid_widget::square_grid_widget_data::Grid, config: &mut PathfinderConfig) {
        todo!()
    }

    fn next_step(&mut self, grid: &mut crate::gui::grid_widget::square_grid_widget_data::Grid, config: &mut PathfinderConfig)  -> PathAlgorithmState {
        todo!()
    }

    fn previous_step(&mut self) {
        todo!()
    }

    fn reset(&mut self, config: &mut PathfinderConfig) {
        todo!()
    }

    fn construct_path(&mut self, config: &mut PathfinderConfig) {
        todo!()
    }

    fn get_next_node(&self, config: &PathfinderConfig) -> Option<PathNodes> {
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