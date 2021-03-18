use druid::{Data};
use std::hash::{Hash, Hasher, };
use crate::data::distance_heuristics::Heuristics;
use crate::gui::grid_widget::square_grid_widget_data::*;
use druid::im::{HashSet, Vector};
use super::{astar::Astar, dfs::DFS, bfs::BFS, dijkstra::Dijkstra, swarm::Swarm, jump_point::JumpPoint, greedy_best_first::GreedyBestFirstSearch};


#[derive(Data, Clone, Eq, PartialEq, Debug)]
pub enum PathAlgorithms {
    Astar(Astar),
    Dijkstra(Dijkstra),
    GreedyBestFirstSearch(GreedyBestFirstSearch),
    BFS(BFS),
    DFS(DFS),
    Swarm(Swarm),
    JumpPoint(JumpPoint),
}

impl PathAlgorithms {
    pub fn get_inner(&mut self) -> Box<&mut dyn PathfinderAlgorithm>{
        match self {
            PathAlgorithms::Astar(inner ) => {Box::new(inner)}
            PathAlgorithms::Dijkstra(inner) => {Box::new(inner)}
            PathAlgorithms::GreedyBestFirstSearch(inner) => {Box::new(inner)}
            PathAlgorithms::BFS(inner) => {Box::new(inner)}
            PathAlgorithms::DFS(inner) => {Box::new(inner)}
            PathAlgorithms::Swarm(inner) => {Box::new(inner)}
            PathAlgorithms:: JumpPoint(inner) => {Box::new(inner)}
        }
    }
}

#[derive(Data, Clone, Eq, PartialEq, Debug)]
pub struct PathfinderConfig {
    pub is_bidirectional: bool,
    pub allow_diagonal: bool,
}

impl PathfinderConfig {
    pub fn new() -> Self {
        PathfinderConfig {
            is_bidirectional: false,
            allow_diagonal: false,
        }
    }
}


//////////////////////////////////////////////////////////////////////////////////////
//
// SquareGridAlgorithm
//
//////////////////////////////////////////////////////////////////////////////////////
pub trait PathfinderAlgorithm{
    fn run(&mut self, grid: &mut Grid, config: &mut PathfinderConfig);
    fn next_step(&mut self, grid: &mut Grid, config: &mut PathfinderConfig)  -> PathAlgorithmState;
    fn previous_step(&mut self);
    fn reset(&mut self, config: &mut PathfinderConfig);
    fn construct_path(&mut self, config: &mut PathfinderConfig);
    fn get_next_node(&self, config: &PathfinderConfig) -> Option<PathNodes>;
    fn get_open_nodes(&self) -> &HashSet<PathNodes>;
    fn get_closed_nodes(&self) -> &HashSet<PathNodes>;
    fn get_path_nodes(&self) -> &Vector<PathNodes>;
    fn get_algorithm_state(&self) -> &PathAlgorithmState;
}

#[derive(Data, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Debug)]
pub enum PathAlgorithmState {
    Initialization,
    Running,
    PathConstruction,
    Finished,
    Failed,
}


#[derive(Data, Copy, Clone, Debug, Eq, )]
pub struct PathNodes {
    pub cost_from_start: i64,
    pub cost_to_target: i64,
    pub total_cost: i64,
    pub position: GridNodePosition,
    pub parent: Option<GridNodePosition>,
}

impl PathNodes {

    pub fn new(cost_start:i64, target_pos: GridNodePosition, current_pos:GridNodePosition, parent: Option<GridNodePosition>) -> Self {
        PathNodes {
            cost_from_start: cost_start,
            cost_to_target: Heuristics::target_cost(current_pos, target_pos),
            total_cost: cost_start + Heuristics::target_cost(current_pos, target_pos),
            position: current_pos,
            parent: parent,
        }
    }

    pub fn reduced(current_pos:GridNodePosition) -> Self {
        PathNodes {
            cost_from_start: 0,
            cost_to_target: 0,
            total_cost: 0,
            position: current_pos,
            parent: None,
        }
    }

    pub fn empty() -> Self {
        PathNodes {
            cost_from_start: 0,
            cost_to_target: 0,
            total_cost: 0,
            position: GridNodePosition{row: 0, col: 0},
            parent: None,
        }

    }
}

impl PartialEq for PathNodes {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

impl Hash for PathNodes {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.position.hash(hasher);
    }
}


