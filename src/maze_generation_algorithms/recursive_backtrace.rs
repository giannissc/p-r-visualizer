use std::ops::Add;

use log::info;
use rand::Rng;

use super::maze_generation_types::*;
use druid::Data;
use druid::im::{HashSet, Vector};
use crate::gui::grid_widget::square_grid_widget_data::*;
use crate::data::app_data::{GRID_COLUMNS, GRID_ROWS};

#[derive(Data, Clone, Eq, PartialEq, Debug)]
pub struct RecursiveBacktrace {
    algorithm_state: MazeAlgorithmState,
    closed_list: HashSet<MazeNodes>,
    current_maze_node: MazeNodes, 
}

impl RecursiveBacktrace {
    pub fn new() -> Self {
        RecursiveBacktrace {
            algorithm_state: MazeAlgorithmState::Initialization,
            closed_list: HashSet::new(),
            current_maze_node: MazeNodes::empty(),
        }
    }
}

impl MazeGenerationAlgorithm for RecursiveBacktrace {
    fn run(&mut self, grid: &mut crate::gui::grid_widget::square_grid_widget_data::Grid) {
        todo!()
    }

    fn next_step(&mut self, grid: &mut crate::gui::grid_widget::square_grid_widget_data::Grid)  -> MazeAlgorithmState {
        if self.algorithm_state == MazeAlgorithmState::Initialization {
            info!("Setting up algorithm");
            // Generate a random point as the starting point and set as current path node
            let mut rng = rand::thread_rng();
            let row = rng.gen_range(1..GRID_ROWS);
            let column = rng.gen_range(1..GRID_COLUMNS);
            let pos = GridNodePosition{row: 1, col: 1};
            self.current_maze_node = MazeNodes::new(pos, None);

            // Clean board and setup perimeter    
            grid.clear_all();
            info!("Adding node perimeter");
            grid.add_node_perimeter(GridNodePosition{row:0, col:0}, GRID_ROWS, GRID_COLUMNS, GridNodeType::Wall, 1);

            // Change state of algorithm to running
            self.algorithm_state = MazeAlgorithmState::Running; 
        } else if self.algorithm_state == MazeAlgorithmState::Running {
            
            match self.get_next_node(grid){
                None => {self.algorithm_state = MazeAlgorithmState::Finished},
                Some(next_node) => {
                    // Add current_node in closed_list
                    self.closed_list.insert(self.current_maze_node);
                    self.current_maze_node = next_node
                },
            }
        }

        self.algorithm_state
    }

    fn previous_step(&mut self) {
        todo!()
    }

    fn reset(&mut self) {
        self.closed_list.clear();
        self.algorithm_state = MazeAlgorithmState::Initialization;
    }

    fn get_next_node(&mut self, grid: &mut Grid) -> Option<MazeNodes> {
        // Get 1st order neighbours. Discard those already in closed list
        let mut first_order_list: Vector<MazeNodes> = Vector::new();
        let mut second_order_list: Vector<MazeNodes> = Vector::new();
        for (index, first__order_neighbour) in grid.available_neighbours_rectilinear(self.current_maze_node.position).iter().enumerate(){
            match first__order_neighbour {
                None => (),
                Some(first_order_neighbour_position) => {
                    let first_neighbour_node = MazeNodes::new(*first_order_neighbour_position, Some(self.current_maze_node.position));
                    // Get 1st order neighbours. Discard those already in closed list
                    if !self.closed_list.contains(&first_neighbour_node) {
                        let second_order_option: Option<GridNodePosition>;

                        if index == 0 || index == 1 {
                            // For each successful neighbour shade their sides (add wall)
                            // above and below
                            grid.add_node(&first_order_neighbour_position.left(), GridNodeType::Wall, 1);
                            grid.add_node(&first_order_neighbour_position.right(), GridNodeType::Wall, 1);

                            if index == 0 {
                                second_order_option = grid.available_above(*first_order_neighbour_position);
                            } else {
                                second_order_option = grid.available_below(*first_order_neighbour_position);
                            }
                        } else {
                            // For each successful neighbour shade their sides (add wall)
                            // left and right
                            grid.add_node(&first_order_neighbour_position.above(), GridNodeType::Wall, 1);
                            grid.add_node(&first_order_neighbour_position.below(), GridNodeType::Wall, 1);

                            if index == 2 {
                                second_order_option = grid.available_left(*first_order_neighbour_position);
                            } else {
                                second_order_option = grid.available_right(*first_order_neighbour_position);
                            }
                        }

                        match second_order_option{
                            None => {
                                // Else go to 1st order descendent and add wall
                                grid.add_node(&first_order_neighbour_position, GridNodeType::Wall, 1);
                            }
                            Some(second_order_neighbour_position) => {
                                // For each successful neighbour move to 2nd order neighbours
                                let second_neighbour_node = MazeNodes::new(second_order_neighbour_position, Some(self.current_maze_node.position));
                                if self.closed_list.contains(&second_neighbour_node) {
                                    // Else go to 1st order descendent and add wall
                                    grid.add_node(&first_order_neighbour_position, GridNodeType::Wall, 1)
                                } else {
                                    // If available and not on closed list put to random list
                                    first_order_list.push_back(first_neighbour_node);
                                    second_order_list.push_back(second_neighbour_node);
                                }
                            }
                        }
                    }
                }
            }
        }
        

        if second_order_list.len() == 0 {
            // If random list is empty set next node the parent of the current_node (search parent position in closed list)
            let parent  = self.current_maze_node.parent;

            match parent {
                // If parent node of the current_node is None return None to terminate the algorithm
                None => {return None}
                Some(pos) => {
                    let parent_node = self.closed_list.remove(&MazeNodes::new(pos, None)).unwrap();
                    self.closed_list.insert(parent_node);
                    return Some(parent_node);
                }
            }

        } else {
            // Based on len() of random list choose 1 random index.
            let mut rng = rand::thread_rng();
            let index = rng.gen_range(0..second_order_list.len());
            self.closed_list.insert(first_order_list.remove(index));
            let element = second_order_list.remove(index);
            Some(element)
        }        
    }

    fn get_closed_nodes(&self) -> &HashSet<MazeNodes> {
        &self.closed_list
    }

    fn get_algorithm_state(&self) -> &MazeAlgorithmState {
        &self.algorithm_state
    }
}