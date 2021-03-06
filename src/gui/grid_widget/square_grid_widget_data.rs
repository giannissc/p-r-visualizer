use druid::im::hashmap::Iter;
use druid::im::{HashMap, HashSet};
use druid::{Data, Lens, Selector};
use log::{debug, info};
use std::rc::Rc;

pub const LOCK_DRAWING: Selector = Selector::new("lock-drawing");
pub const UNLOCK_DRAWING: Selector = Selector::new("unlock-drawing");
pub const RESET: Selector = Selector::new("RESET");
pub const CLEAR_STORE: Selector = Selector::new("CLEAR");

//////////////////////////////////////////////////////////////////////////////////////
//
// GridWidgetData
//
//////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, PartialEq, Data, Lens)]
pub struct GridWidgetData {
    pub grid: Grid,
    pub interaction_state: Interaction,
    pub selected_tool: GridNodeType<Net>,
    pub show_grid_axis: bool,
    pub selected_net: Net,
}

impl GridWidgetData {
    pub fn new(grid: Grid) -> Self {
        GridWidgetData {
            grid: grid,
            interaction_state: Interaction::None,
            show_grid_axis: true,
            selected_tool: GridNodeType::Wall,
            selected_net: 1,
        }
    }
}
//////////////////////////////////////////////////////////////////////////////////////
//
// Interaction
//
//////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, PartialEq, Data, Debug)]
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
    storage: HashMap<GridNodePosition, GridNodeType<Net>>,
    addition_storage: HashSet<GridNodePosition>,
    deletion_storage: HashSet<GridNodePosition>,
    pub start_node: GridNodePosition,
    pub end_node: GridNodePosition,
}

impl Grid {
    pub fn new(start_node: GridNodePosition, end_node: GridNodePosition) -> Grid {
        let mut storage = HashMap::new();
        let mut addition_storage = HashSet::new();
        let deletion_storage = HashSet::new();
        storage.insert(start_node, GridNodeType::StartNode(1));
        storage.insert(end_node, GridNodeType::TargetNode(1));
        addition_storage.insert(start_node);
        addition_storage.insert(end_node);
        Grid {
            storage: storage,
            addition_storage,
            deletion_storage,
            start_node: start_node,
            end_node: end_node,
        }
    }

    pub fn get_item(&self, key: &GridNodePosition) -> Option<&GridNodeType<Net>> {
        self.storage.get(key)
    }

    pub fn get_additions(&self) -> HashSet<GridNodePosition> {
        self.addition_storage.clone()
    }

    pub fn get_deletions(&self) -> HashSet<GridNodePosition> {
        self.deletion_storage.clone()
    }

    pub fn clear_store(&mut self) {
        self.addition_storage.clear();
        self.deletion_storage.clear();
    }

    pub fn clear_all(&mut self) {
        let mut temp_list: HashSet<GridNodePosition> = HashSet::new();
        for (node_pos, node_type) in self.storage.iter() {
            if !matches!(node_type, &GridNodeType::StartNode(_))
                && !matches!(node_type, &GridNodeType::TargetNode(_))
            {
                temp_list.insert(*node_pos);
            }
        }

        for node in temp_list.iter() {
            self.remove_node(node);
        }
    }

    pub fn clear_paths(&mut self) {
        let mut temp_list: HashSet<GridNodePosition> = HashSet::new();
        for (node_pos, node_type) in self.storage.iter() {
            if matches!(node_type, &GridNodeType::ExploredNodes(_))
                || matches!(node_type, &GridNodeType::UnexploredNodes(_))
                || matches!(node_type, &GridNodeType::ChosenPath(_))
            {
                temp_list.insert(*node_pos);
            }
        }

        for node in temp_list.iter() {
            self.remove_node(node);
        }
    }

    pub fn add_node(&mut self, pos: &GridNodePosition, tool: GridNodeType<Net>, net: Net) {
        match tool {
            GridNodeType::Empty => (),
            GridNodeType::Wall => {
                if self.storage.contains_key(pos) {
                    match self.storage.get(pos) {
                        Some(&GridNodeType::ChosenPath(_)) => {
                            self.clear_paths();
                        }
                        _ => (),
                    }

                    let item = self.storage.get(pos);

                    if !matches!(item, Some(&GridNodeType::StartNode(_)))
                        && !matches!(item, Some(&GridNodeType::TargetNode(_)))
                    {
                        self.storage.insert(*pos, GridNodeType::Wall);
                        self.addition_storage.insert(*pos);
                        // if a wall node interferes with a chosenPath node reset algorithm and clear board
                    }
                } else {
                    self.storage.insert(*pos, GridNodeType::Wall);
                    self.addition_storage.insert(*pos);
                }
            }
            GridNodeType::StartNode(_) => {
                if *pos != self.end_node {
                    self.storage.remove(&self.start_node);
                    self.deletion_storage.insert(self.start_node);
                    self.start_node = *pos;
                    self.storage
                        .insert(self.start_node, GridNodeType::StartNode(net));
                    self.addition_storage.insert(self.start_node);
                    // When either goalpoast is moved you need to reset the algorithm and clear the board from all the algorithm nodes
                    self.clear_paths();
                }
            }
            GridNodeType::TargetNode(_) => {
                if *pos != self.start_node {
                    self.storage.remove(&self.end_node);
                    self.deletion_storage.insert(self.end_node);
                    self.end_node = *pos;
                    self.storage
                        .insert(self.end_node, GridNodeType::TargetNode(net));
                    self.addition_storage.insert(self.end_node);
                    // When either goalpoast is moved you need to reset the algorithm and clear the board from all the algorithm nodes
                    self.clear_paths();
                }
            }
            GridNodeType::ExploredNodes(_) => {
                let item = self.storage.get(pos);
                if !matches!(item, Some(&GridNodeType::StartNode(_)))
                    && !matches!(item, Some(&GridNodeType::TargetNode(_)))
                    && item != Some(&GridNodeType::Wall)
                {
                    self.storage.insert(*pos, GridNodeType::ExploredNodes(net));
                    self.addition_storage.insert(*pos);
                }
            }
            GridNodeType::UnexploredNodes(_) => {
                let item = self.storage.get(pos);
                if !matches!(item, Some(&GridNodeType::StartNode(_)))
                    && !matches!(item, Some(&GridNodeType::TargetNode(_)))
                    && item != Some(&GridNodeType::Wall)
                {
                    self.storage
                        .insert(*pos, GridNodeType::UnexploredNodes(net));
                    self.addition_storage.insert(*pos);
                }
            }
            GridNodeType::ChosenPath(_) => {
                let item = self.storage.get(pos);
                if !matches!(item, Some(&GridNodeType::StartNode(_)))
                    && !matches!(item, Some(&GridNodeType::TargetNode(_)))
                {
                    self.storage.insert(*pos, GridNodeType::ChosenPath(net));
                    self.addition_storage.insert(*pos);
                }
            }
        }
    }

    pub fn remove_node(&mut self, pos: &GridNodePosition) {
        let item = self.storage.get(pos);
        if !matches!(item, Some(&GridNodeType::StartNode(_)))
            && !matches!(item, Some(&GridNodeType::TargetNode(_)))
        {
            self.storage.remove(pos);
            self.deletion_storage.insert(*pos);
        }
    }

    pub fn add_path(&mut self, _pos: GridNodePosition) {
        unimplemented!()
    }

    pub fn remove_path(&mut self, _pos: GridNodePosition) {
        unimplemented!()
    }

    pub fn add_node_area(
        &mut self,
        pos: GridNodePosition,
        row_n: usize,
        column_n: usize,
        tool: GridNodeType<Net>,
        net: Net,
    ) {
        for row in pos.row..pos.row + row_n {
            for column in pos.col..pos.col + column_n {
                self.add_node(
                    &GridNodePosition {
                        row: row,
                        col: column,
                    },
                    tool,
                    net,
                );
            }
        }
    }

    pub fn remove_node_area(
        &mut self,
        _pos: GridNodePosition,
        _row_n: usize,
        _column_n: usize,
        _tool: GridNodeType<Net>,
    ) {
        unimplemented!()
    }

    pub fn add_node_perimeter(
        &mut self,
        pos: GridNodePosition,
        row_n: usize,
        column_n: usize,
        tool: GridNodeType<Net>,
        net: Net,
    ) {
        for row in pos.row..pos.row + row_n {
            //debug!("Add node perimeter");
            //debug!("Row: {:?}", row);
            if row == pos.row || row == pos.row + row_n - 1 {
                // Top and Bottom Boundaries
                //debug!("Printing top/bottom boundary");
                for column in pos.col..pos.col + column_n {
                    self.add_node(
                        &GridNodePosition {
                            row: row,
                            col: column,
                        },
                        tool,
                        net,
                    );
                }
            } else {
                //debug!("Printing left/right boundary");
                // Left Boundary
                self.add_node(
                    &GridNodePosition {
                        row: row,
                        col: pos.col,
                    },
                    tool,
                    net,
                );
                // Right Boundary
                self.add_node(
                    &GridNodePosition {
                        row: row,
                        col: pos.col + column_n - 1,
                    },
                    tool,
                    net,
                );
            }
        }
    }

    pub fn remove_node_perimeter(
        &mut self,
        _pos: GridNodePosition,
        _row_n: usize,
        _column_n: usize,
        _tool: GridNodeType<Net>,
    ) {
        unimplemented!()
    }

    pub fn available_neighbours_rectilinear(
        &self,
        pos: GridNodePosition,
    ) -> [Option<GridNodePosition>; 4] {
        let mut result: [Option<GridNodePosition>; 4] = [None; 4];
        for (index, node) in pos.neighbors_rectilinear().iter().enumerate() {
            result[index] = self.check_if_wall(node)
        }
        result
    }

    pub fn available_neighbours_octilinear(
        &self,
        pos: GridNodePosition,
    ) -> [Option<GridNodePosition>; 8] {
        let mut result: [Option<GridNodePosition>; 8] = [None; 8];
        for (index, node) in pos.neighbors_octilinear().iter().enumerate() {
            result[index] = self.check_if_wall(node)
        }
        result
    }

    pub fn available_above(&self, pos: GridNodePosition) -> Option<GridNodePosition> {
        let node = &pos.above();
        self.check_if_wall(node)
    }

    pub fn available_below(&self, pos: GridNodePosition) -> Option<GridNodePosition> {
        let node = &pos.below();
        self.check_if_wall(node)
    }

    pub fn available_left(&self, pos: GridNodePosition) -> Option<GridNodePosition> {
        let node = &pos.left();
        self.check_if_wall(node)
    }

    pub fn available_right(&self, pos: GridNodePosition) -> Option<GridNodePosition> {
        let node = &pos.right();
        self.check_if_wall(node)
    }

    fn check_if_wall(&self, node: &GridNodePosition) -> Option<GridNodePosition> {
        if !self.storage.contains_key(&node) || self.storage.get(&node) != Some(&GridNodeType::Wall)
        {
            return Some(*node);
        }
        return None;
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
        [above, below, left, right]
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
pub type Net = i32;
//type Weight = i32;
#[derive(Data, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]

pub enum GridNodeType<Net> {
    Wall,
    Empty,
    //WeightedNode(Weight),
    StartNode(Net),
    TargetNode(Net),
    //SteinerNode(Net),
    UnexploredNodes(Net), //Rename to visitedNodes
    ExploredNodes(Net),   //Rename to visitedNodes
    ChosenPath(Net),
}
