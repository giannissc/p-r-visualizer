use druid::im::{Vector, HashSet};
use crate::data::pathfinding_types::*;

#[derive(PartialEq, Copy, Clone)]
pub enum AlgorithmState {
    INITIALIZATION,
    RUNNING,
    PATH_CONSTRUCTION,
    FINISHED,
    FAILED,
}

pub struct PathAlgo {
    selected_algorithm: PathAlgorithms,
    is_bidirectional: bool,
    allow_diagonal: bool,
    algorithm_state: AlgorithmState,
    open_list: HashSet<PathNodes>,
    closed_list: HashSet<PathNodes>,
    path_list: Vector<PathNodes>,
    current_path_node: PathNodes, 

}


impl PathAlgo {
    pub fn new() -> Self {
        PathAlgo {
            selected_algorithm: PathAlgorithms::Astar,
            is_bidirectional: false,
            allow_diagonal: false,
            algorithm_state: AlgorithmState::INITIALIZATION,
            open_list: HashSet::new(),
            closed_list: HashSet::new(),
            path_list: Vector::new(), 
            current_path_node: PathNodes::empty(), 
        }
    }

    pub fn run(&self) {
        unimplemented!()
    }


    pub fn next_step(&mut self, grid: &Grid) -> AlgorithmState{
        if self.algorithm_state == AlgorithmState::INITIALIZATION {
            self.open_list.insert(PathNodes::new(0, grid.end_node, grid.start_node, None)); // Step 1: Add the starting node to the open list
            self.algorithm_state = AlgorithmState::RUNNING;           
        } else if self.algorithm_state == AlgorithmState::RUNNING {
            match self.get_min_cost_node(){
                None => {self.algorithm_state = AlgorithmState::FAILED},
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
                                    self.algorithm_state = AlgorithmState::PATH_CONSTRUCTION;
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

        } else if self.algorithm_state == AlgorithmState::PATH_CONSTRUCTION {
            self.construct_path();
        }
        self.algorithm_state
    }

    pub fn previous_step(&mut self) {
        unimplemented!()
    }

    pub fn cancel(&mut self) {
        self.open_list.clear();
        self.closed_list.clear();
        self.algorithm_state = AlgorithmState::INITIALIZATION;
    }

    fn get_min_cost_node(&self) -> Option<PathNodes> {
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

    fn construct_path(&mut self) {
        println!("Constructing Path");
        let mut current_node = self.current_path_node;
        self.path_list.push_front(current_node);
        //println!("Current node: {:?}", current_node);         
        let parent_node = self.closed_list.remove(&PathNodes::reduced(current_node.parent.unwrap())).unwrap();
        //println!("Parent node: {:?}", parent_node); 
        
        if parent_node.parent == None {
            self.algorithm_state = AlgorithmState::FINISHED;
        } else {
            self.current_path_node = parent_node;
        }
    }


    pub fn get_open_nodes(&self) -> &HashSet<PathNodes> {
        return &self.open_list;

    }

    pub fn get_closed_nodes(&self) -> &HashSet<PathNodes> {
        return &self.closed_list;

    }

    pub fn get_path_nodes(&self) -> &Vector<PathNodes> {
        return &self.path_list;
    }

}