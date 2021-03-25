use super::pathfinding_types::*;
use crate::gui::grid_widget::square_grid_widget_data::*;
use druid::im::{HashSet, Vector};
use druid::Data;

#[derive(Data, Clone, Eq, PartialEq, Debug)]
pub struct JumpPoint {
    algorithm_state: PathAlgorithmState,
    open_list: HashSet<PathNodes>,
    closed_list: HashSet<PathNodes>,
    path_list: Vector<PathNodes>,
    current_path_node: PathNodes,
}

impl JumpPoint {
    pub fn new() -> Self {
        JumpPoint {
            algorithm_state: PathAlgorithmState::Initialization,
            open_list: HashSet::new(),
            closed_list: HashSet::new(),
            path_list: Vector::new(),
            current_path_node: PathNodes::empty(),
        }
    }
}

impl PathFinderAlgorithm for JumpPoint {
    fn run(
        &mut self,
        grid: &mut crate::gui::grid_widget::square_grid_widget_data::Grid,
        config: &mut PathfinderConfig,
        net: Net,
    ) {
        todo!()
    }

    fn next_step(
        &mut self,
        grid: &mut crate::gui::grid_widget::square_grid_widget_data::Grid,
        config: &mut PathfinderConfig,
        net: Net,
    ) -> PathAlgorithmState {
        todo!()
    }

    fn previous_step(
        &mut self,
        grid: &mut crate::gui::grid_widget::square_grid_widget_data::Grid,
        config: &mut PathfinderConfig,
        net: Net,
    ) {
        todo!()
    }

    fn reset(&mut self) {
        todo!()
    }

    fn construct_path(&mut self, grid: &mut Grid, net: Net) {
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
