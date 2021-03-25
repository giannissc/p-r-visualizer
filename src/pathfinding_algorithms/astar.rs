use super::pathfinding_types::*;
use druid::Data;
use druid::im::{Vector, HashSet};
use log::{debug, info};
use crate::gui::grid_widget::square_grid_widget_data::*;
use crate::data::app_data::{GRID_COLUMNS, GRID_ROWS};

#[derive(Data, Clone, Eq, PartialEq, Debug)]
pub struct Astar{
    algorithm_state: PathAlgorithmState,
    open_list: HashSet<PathNodes>,
    closed_list: HashSet<PathNodes>,
    path_list: Vector<PathNodes>,
    current_path_node: PathNodes, 
}

impl Astar {
    pub fn new() -> Self {
        Astar {
            algorithm_state: PathAlgorithmState::Initialization,
            open_list: HashSet::new(),
            closed_list: HashSet::new(),
            path_list: Vector::new(), 
            current_path_node: PathNodes::empty(),
        }
    }
}

impl PathFinderAlgorithm for Astar {
    fn run(&mut self, grid: &mut Grid, config: &mut PathfinderConfig, net: Net) {
        todo!()
    }


    fn next_step(&mut self, grid: &mut Grid, config: &mut PathfinderConfig, net: Net) -> PathAlgorithmState{
        if self.algorithm_state == PathAlgorithmState::Initialization {
            info!("Setting up algorithm");
            self.current_path_node = PathNodes::new(0, grid.end_node, grid.start_node, None);
            self.open_list.insert(self.current_path_node); // Step 1: Add the starting node to the open list
            grid.add_node(&self.current_path_node.position, GridNodeType::UnexploredNodes(net), net);
            self.algorithm_state = PathAlgorithmState::Running;     
            grid.clear_paths();
            grid.add_node_perimeter(GridNodePosition{row:0, col:0}, GRID_ROWS, GRID_COLUMNS, GridNodeType::Wall, 1)
        } else if self.algorithm_state == PathAlgorithmState::Running {
            match self.get_next_node(){
                None => {self.algorithm_state = PathAlgorithmState::Failed},
                Some(current_node) => {
                    self.open_list.remove(&current_node); // Step 2: Remove lower cost node from the open list
                    grid.remove_node(&current_node.position);
                    self.closed_list.insert(current_node); // Step 3: Add current node to the closed list
                    grid.add_node(&current_node.position, GridNodeType::ExploredNodes(net), net);
                    for node in grid.available_neighbours_rectilinear(current_node.position).iter(){// Step 4: Generate list of neighbours
                        // Step 4.1: Ignore if it is not walkable (already checked that in neighbours function)
                        match node {
                            None => (), // Step 4.1: Node is not walkable
                            Some(neighbour_pos) => {
                                let neighbour_node = PathNodes::new(&current_node.cost_from_start + 1, grid.end_node, *neighbour_pos, Some(current_node.position));
                                if neighbour_node.position == grid.end_node {
                                    self.current_path_node = neighbour_node;
                                    self.algorithm_state = PathAlgorithmState::PathConstruction;
                                }

                                if !self.closed_list.contains(&neighbour_node) { // Step 4.1: Node is not in closed list either.
                                    if !self.open_list.contains(&neighbour_node){
                                        self.open_list.insert(neighbour_node); // Step 4.2: If node is not in open node add it to it
                                        grid.add_node(&neighbour_node.position, GridNodeType::UnexploredNodes(net), net);
                                    } else {
                                        let other_node = self.open_list.remove(&neighbour_node).unwrap();
                                        grid.remove_node(&neighbour_node.position);
                                        if neighbour_node.cost_from_start < other_node.cost_from_start { // Step 4.3: Keep the node with the lower G score
                                            self.open_list.insert(neighbour_node);
                                            grid.add_node(&neighbour_node.position, GridNodeType::UnexploredNodes(net), net);
                                        } else {
                                            self.open_list.insert(other_node);
                                            grid.add_node(&other_node.position, GridNodeType::UnexploredNodes(net), net);
                                        }
                                    }
                                }
                            },   
                        }
                    }
                }           
            }

        } else if self.algorithm_state == PathAlgorithmState::PathConstruction {
            self.construct_path(grid, net);
        }
        self.algorithm_state
    }

    fn previous_step(&mut self, grid: &mut Grid, config: &mut PathfinderConfig, net: Net) {
        todo!()
    }

    fn reset(&mut self) {
        self.open_list.clear();
        self.closed_list.clear();
        self.path_list.clear();
        self.algorithm_state = PathAlgorithmState::Initialization;
    }

    fn construct_path(&mut self, grid: &mut Grid, net: Net) {
        //debug!("Constructing Path");
        let current_node = self.current_path_node;
        self.path_list.push_front(current_node);
        grid.add_node(&current_node.position, GridNodeType::ChosenPath(net), net);
        //debug!("Current node: {:?}", current_node);         
        let parent_node = self.closed_list.remove(&PathNodes::reduced(current_node.parent.unwrap())).unwrap();
        grid.remove_node(&current_node.parent.unwrap());
        //debug!("Parent node: {:?}", parent_node); 
        
        if parent_node.parent == None {
            self.algorithm_state = PathAlgorithmState::Finished;
        } else {
            self.current_path_node = parent_node;
        }
    }

    fn get_next_node(&self) -> Option<PathNodes> {
        // Gen min cost node for A*
        let mut min_cost = std::i64::MAX;
        let mut min_node: Option<PathNodes> = None;

        if self.open_list.is_empty() {return None}

        for node in self.open_list.iter(){
            if node.total_cost < min_cost {
                min_cost = node.total_cost;
                min_node = Some(*node);
            }
        }
        min_node
    }

    fn get_open_nodes(&self) -> &HashSet<PathNodes> {
        &self.open_list
    }

    fn get_closed_nodes(&self) -> &HashSet<PathNodes> {
        &self.closed_list
    }

    fn get_path_nodes(&self) -> &Vector<PathNodes> {
        &self.path_list
    }

    fn get_algorithm_state(&self) -> &PathAlgorithmState {
        &self.algorithm_state
    }
}