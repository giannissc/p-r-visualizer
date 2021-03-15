use druid::{Data, Lens, Selector};
use druid::im::{HashMap, HashSet};


pub const LOCK_DRAWING: Selector =  Selector::new("lock-drawing");
pub const UNLOCK_DRAWING: Selector =  Selector::new("unlock-drawing");
pub const RESET: Selector =  Selector::new("RESET");

//////////////////////////////////////////////////////////////////////////////////////
//
// GridWidgetData
//
//////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, PartialEq, Data, Lens)]
pub struct GridWidgetData {
    pub grid: Grid,
    pub interaction_state: Interaction,
    pub selected_tool: GridNodeType,
    pub show_grid_axis: bool,   
}



impl GridWidgetData {
    pub fn new(grid: Grid) -> Self {
        GridWidgetData {
            grid: grid,
            interaction_state: Interaction::None,
            show_grid_axis: true,
            selected_tool: GridNodeType::Wall,
        }
    } 
}
//////////////////////////////////////////////////////////////////////////////////////
//
// Interaction
//
//////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, PartialEq, Data)]
pub enum Interaction {
    None,
    Drawing,
    //Panning,
    LockedUI,
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
        let mut temp_list: HashSet<GridNodePosition> = HashSet::new();
        for (node_pos , node_type) in self.storage.iter(){
            if node_type == &GridNodeType::ExploredNodes(1) || node_type == &GridNodeType::UnexploredNodes(1) || node_type == &GridNodeType::ChosenPath(1) {
                temp_list.insert(*node_pos);
            }
        }

        for node in temp_list.iter(){
            self.remove_node(node);
        }
    }

    pub fn add_node(&mut self, pos: &GridNodePosition, tool: GridNodeType){
        match tool {
            GridNodeType::Empty => (),
            GridNodeType::Wall => {
                if self.storage.contains_key(pos)  {
                    
                    if self.storage.get(pos) == Some(&GridNodeType::ChosenPath(1)) {
                        self.clear_paths();
                    }

                    let item = self.storage.get(pos);
                    
                    if item != Some(&GridNodeType::StartNode(1)) && item != Some(&GridNodeType::TargetNode(1)) {
                        self.storage.insert(*pos, tool);
                        // if a wall node interferes with a chosenPath node reset algorithm and clear board
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
                    // When either goalpoast is moved you need to reset the algorithm and clear the board from all the algorithm nodes
                    self.clear_paths();
                }
                
            },
            GridNodeType::TargetNode(_) => {
                if *pos != self.start_node{
                    self.storage.remove(&self.end_node);
                    self.end_node = *pos;
                    self.storage.insert(*pos, tool);
                    // When either goalpoast is moved you need to reset the algorithm and clear the board from all the algorithm nodes
                    self.clear_paths();
                }
                
            },
            GridNodeType::ExploredNodes(_) => {
                let item = self.storage.get(pos); 
                if item != Some(&GridNodeType::TargetNode(1)) && item != Some(&GridNodeType::StartNode(1)) && item != Some(&GridNodeType::Wall){
                    self.storage.insert(*pos, tool);
                }
            },
            GridNodeType::UnexploredNodes(_) => {
                let item = self.storage.get(pos); 
                if item != Some(&GridNodeType::TargetNode(1)) && item != Some(&GridNodeType::StartNode(1))  && item != Some(&GridNodeType::Wall){
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
        if item != Some(&GridNodeType::StartNode(1))  || item != Some(&GridNodeType::TargetNode(1)) {
            self.storage.remove(pos);
        }
    }

    pub fn add_path(&mut self, _pos:GridNodePosition){
        unimplemented!()
    }

    pub fn remove_path(&mut self, _pos:GridNodePosition){
        unimplemented!()
    }

    pub fn add_node_area(&mut self, _pos: GridNodePosition, _row_n: usize, _column_n: usize, _tool: GridNodeType){
        unimplemented!()
    }

    pub fn remove_node_area(&mut self, _pos: GridNodePosition, _row_n: usize, _column_n: usize, _tool: GridNodeType){
        unimplemented!()
    }

    pub fn add_node_perimeter(&mut self, pos: GridNodePosition, row_n: usize, column_n: usize, tool: GridNodeType){
        for row in pos.row..pos.row+row_n {
            println!("Row: {:?}", row);
            if row == pos.row || row == pos.row + row_n -1 {
                // Top and Bottom Boundaries
                println!("Printing top/bottom boundary");
                for  column in pos.col..pos.col+column_n {
                    self.add_node(&GridNodePosition{row:row, col:column}, tool);
                }
            } else {  
                // Left Boundary
                self.add_node(&GridNodePosition{row:row, col:pos.col}, tool);
                // Right Boundary
                self.add_node(&GridNodePosition{row:row, col:pos.col+column_n-1}, tool);
            }
        }
    }

    pub fn remove_node_perimeter(&mut self, _pos: GridNodePosition, _row_n: usize, _column_n: usize, _tool: GridNodeType){
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
// GridNodePosition
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

//////////////////////////////////////////////////////////////////////////////////////
//
// GridNodeType
//
//////////////////////////////////////////////////////////////////////////////////////
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