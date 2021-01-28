use druid::{Data, Lens};
use druid::im::HashMap;
use std::hash::{Hash, Hasher};
use crate::data::distance_heuristics::Heuristics;

#[derive(Data, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum MazeAlgorithms {
    Random,
    Backtrace,
    Recursive,
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

// Add wight and bomb nodes?
type Net = i32;
//type Weight = i32;
#[derive(Data, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]

pub enum GridNodeType {
    
    Wall,
    Empty,
    //WeightedNode(Weight),
    StartNode(Net),
    TargetNode(Net),
    //SteinerNode(Net),
    UnexploredNodes(Net), //Rename to visitedNodes
    ExploredNodes(Net), //Rename to visitedNodes
    ChosenPath(Net),
    
}


//////////////////////////////////////////////////////////////////////////////////////
//
// Grid
//
//////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, PartialEq, Data, Lens)]
pub struct Grid {
    pub storage: HashMap<GridNodePosition, GridNodeType>,
    pub start_node: GridNodePosition,
    pub end_node: GridNodePosition,
}

impl Grid {
    pub fn new(start_node: GridNodePosition, end_node: GridNodePosition) -> Grid {
        let mut storage = HashMap::new();
        storage.insert(start_node, GridNodeType::StartNode(1));
        storage.insert(end_node, GridNodeType::TargetNode(1));
        Grid {
            storage: storage,
            start_node: start_node,
            end_node: end_node,
        }
    }

    pub fn clear_all(&mut self){
        self.storage.clear();
        self.storage.insert(self.start_node, GridNodeType::StartNode(1));
        self.storage.insert(self.end_node, GridNodeType::TargetNode(1));
    }

    pub fn clear_paths(&mut self){
        unimplemented!()
    }

    pub fn add_node(&mut self, pos: &GridNodePosition, tool: GridNodeType){
        match tool {
            GridNodeType::Empty => (),
            GridNodeType::Wall => {
                if self.storage.contains_key(pos)  {
                    let item = self.storage.get(pos);
                    if item != Some(&GridNodeType::StartNode(1)) && item != Some(&GridNodeType::TargetNode(1)) {
                        self.storage.insert(*pos, tool);
                    }
                } else {
                    self.storage.insert(*pos, tool);
                }
            },
            GridNodeType::StartNode(_) => {
                if *pos != self.end_node {
                    self.storage.remove(&self.start_node);
                    self.start_node = *pos;
                    self.storage.insert(*pos, tool);
                }
                
            },
            GridNodeType::TargetNode(_) => {
                if *pos != self.start_node{
                    self.storage.remove(&self.end_node);
                    self.end_node = *pos;
                    self.storage.insert(*pos, tool);
                }
                
            },
            GridNodeType::ExploredNodes(_) => {
                let item = self.storage.get(pos); 
                if item != Some(&GridNodeType::TargetNode(1)) && item != Some(&GridNodeType::StartNode(1)){
                    self.storage.insert(*pos, tool);
                }
            },
            GridNodeType::UnexploredNodes(_) => {
                let item = self.storage.get(pos); 
                if item != Some(&GridNodeType::TargetNode(1)) && item != Some(&GridNodeType::StartNode(1)){
                    self.storage.insert(*pos, tool);
                }
            },
            GridNodeType::ChosenPath(_) => {
                let item = self.storage.get(pos); 
                if item != Some(&GridNodeType::TargetNode(1)) && item != Some(&GridNodeType::StartNode(1)){
                    self.storage.insert(*pos, tool);
                }
            },
        }
    }

    pub fn remove_node(&mut self, pos: &GridNodePosition){
        let item = self.storage.get(pos);
        if item == Some(&GridNodeType::Wall) {
            self.storage.remove(pos);
        }
    }

    pub fn add_path(&mut self, pos:GridNodePosition){
        unimplemented!()
    }

    pub fn remove_path(&mut self, pos:GridNodePosition){
        unimplemented!()
    }

    pub fn add_node_area(&mut self, pos: GridNodePosition, row_n: usize, column_n: usize, tool: GridNodeType){
        unimplemented!()
    }

    pub fn remove_node_area(&mut self, pos: GridNodePosition, row_n: usize, column_n: usize, tool: GridNodeType){
        unimplemented!()
    }

    pub fn add_node_perimeter(&mut self, pos: GridNodePosition, row_n: usize, column_n: usize, tool: GridNodeType){
        unimplemented!()
    }

    pub fn remove_node_perimeter(&mut self, pos: GridNodePosition, row_n: usize, column_n: usize, tool: GridNodeType){
        unimplemented!()
    }

    pub fn neighbours_rectilinear(&self, pos: GridNodePosition) -> [Option<GridNodePosition>; 4]{
        let mut result: [Option<GridNodePosition>; 4] = [None; 4];
        for (index, node) in pos.neighbors_rectilinear().iter().enumerate(){
            if !self.storage.contains_key(node) || self.storage.get(node) != Some(&GridNodeType::Wall){
                result[index] = Some(*node);
            }
        }
        result
    }

    pub fn neighbours_octilinear(&self, pos: GridNodePosition) -> [Option<GridNodePosition>; 8]{
        let mut result: [Option<GridNodePosition>; 8] = [None; 8];
        for (index, node) in pos.neighbors_octilinear().iter().enumerate(){
            if !self.storage.contains_key(node) || self.storage.get(node) != Some(&GridNodeType::Wall){
                result[index] = Some(*node);
            }
        }
        result
    }
}


//////////////////////////////////////////////////////////////////////////////////////
//
// GridPos Implementations
//
//////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Data, Copy, PartialEq, Debug, Hash, Eq)]
pub struct GridNodePosition {   
    pub row: usize,
    pub col: usize,
}

impl GridNodePosition {
    pub fn above(self) -> GridNodePosition {
        GridNodePosition {
            row: self.row - 1,
            col: self.col,
        }
    }

    pub fn below(self) -> GridNodePosition {
        GridNodePosition {
            row: self.row + 1,
            col: self.col,
        }
    }

    pub fn left(self) -> GridNodePosition {
        GridNodePosition {
            row: self.row,
            col: self.col - 1,
        }
    }

    pub fn right(self) -> GridNodePosition {
        GridNodePosition {
            row: self.row,
            col: self.col + 1,
        }
    }

    // Also known in vlsi as the Manhattan Architecture
    pub fn neighbors_rectilinear(self) -> [GridNodePosition; 4] {
        let above = self.above();
        let below = self.below();
        let left = self.left();
        let right = self.right();
        [
            above,
            below,
            left,
            right,
        ]
    }

    // Also known in vlsi as the X Architecture
    pub fn neighbors_octilinear(self) -> [GridNodePosition; 8] {
        let above = self.above();
        let below = self.below();
        let left = self.left();
        let right = self.right();
        let above_left = above.left();
        let above_right = above.right();
        let below_left = below.left();
        let below_right = below.right();
        [
            above,
            below,
            left,
            right,
            above_left,
            above_right,
            below_left,
            below_right,
        ]
    }
}