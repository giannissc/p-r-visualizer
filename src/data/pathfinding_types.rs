use druid::{Data};
use std::hash::{Hash, Hasher};
use crate::data::distance_heuristics::Heuristics;
use crate::gui::grid_widget::grid_widget_data::*;

#[derive(Data, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum MazeAlgorithms {
    Random,
    RecursiveBacktrace,
    RecursiveSubdivision,
}

#[derive(Data, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum PathAlgorithms {
    Astar,
    Dijkstra,
    GreedyBestFirstSearch,
    BFS,
    DFS,
    Swarm,
    JumpPoint,
}

#[derive(Copy, Clone, Debug, Eq)]
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


