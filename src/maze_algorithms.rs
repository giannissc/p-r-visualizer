use crate::data::pathfinding_types::*;
#[derive(PartialEq, Copy, Clone)]
pub enum AlgorithmState {
    Initialization,
    Running,
    PathConstruction,
    Finished,
    Failed,
}

pub struct PathfindingConfig {
    selected_algorithm: MazeAlgorithms,
    pub algorithm_state: AlgorithmState,
    current_path_node: PathNodes, 

}