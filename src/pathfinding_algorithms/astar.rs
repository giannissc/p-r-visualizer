use super::pathfinding_types::*;
use druid::Data;
use druid::im::{Vector, HashSet};
use crate::gui::grid_widget::square_grid_widget_data::*;

#[derive(Data, Clone, Eq, PartialEq, Debug)]
pub struct Astar{
    pub algorithm_state: PathAlgorithmState,
    pub open_list: HashSet<PathNodes>,
    pub closed_list: HashSet<PathNodes>,
    pub path_list: Vector<PathNodes>,
    pub current_path_node: PathNodes, 
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

impl PathfinderAlgorithm for Astar {
    fn run(&mut self, grid: &mut Grid, config: &mut PathfinderConfig) {
        todo!()
    }


    fn next_step(&mut self, grid: &mut Grid, config: &mut PathfinderConfig) -> PathAlgorithmState{
        if self.algorithm_state == PathAlgorithmState::Initialization {
            //println!("Setting up algorithm");
            self.open_list.insert(PathNodes::new(0, grid.end_node, grid.start_node, None)); // Step 1: Add the starting node to the open list
            self.algorithm_state = PathAlgorithmState::Running;     
            grid.clear_paths();      
        } else if self.algorithm_state == PathAlgorithmState::Running {
            match self.get_next_node(config){
                None => {self.algorithm_state = PathAlgorithmState::Failed},
                Some(current_node) => {
                    self.open_list.remove(&current_node); // Step 2: Remove lower cost node from the open list
                    self.closed_list.insert(current_node); // Step 3: Add current node to the closed list
                    for node in grid.neighbours_rectilinear(current_node.position).iter(){// Step 4: Generate list of neighbours
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
                                    } else {
                                        let other_node = self.open_list.remove(&neighbour_node).unwrap();
                                        if neighbour_node.cost_from_start < other_node.cost_from_start { // Step 4.3: Keep the node with the lower G score
                                            self.open_list.insert(neighbour_node);
                                        } else {
                                            self.open_list.insert(other_node);
                                        }
                                    }
                                }
                            },   
                        }
                    }
                }           
            }

        } else if self.algorithm_state == PathAlgorithmState::PathConstruction {
            self.construct_path(config);
        }
        self.algorithm_state
    }

    fn previous_step(&mut self) {
        todo!()
    }

    fn reset(&mut self, config: &mut PathfinderConfig) {
        self.open_list.clear();
        self.closed_list.clear();
        self.path_list.clear();
        self.algorithm_state = PathAlgorithmState::Initialization;
    }

    fn construct_path(&mut self, config: &mut PathfinderConfig) {
        //println!("Constructing Path");
        let current_node = self.current_path_node;
        self.path_list.push_front(current_node);
        //println!("Current node: {:?}", current_node);         
        let parent_node = self.closed_list.remove(&PathNodes::reduced(current_node.parent.unwrap())).unwrap();
        //println!("Parent node: {:?}", parent_node); 
        
        if parent_node.parent == None {
            self.algorithm_state = PathAlgorithmState::Finished;
        } else {
            self.current_path_node = parent_node;
        }
    }

    fn get_next_node(&self, config: &PathfinderConfig) -> Option<PathNodes> {
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