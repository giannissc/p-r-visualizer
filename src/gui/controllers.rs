use druid::{widget::Controller, Env, EventCtx, Widget, TimerToken, Event,};
use std::{time::{Duration, Instant}};

use crate::data::app_data::{AppData, GRID_COLUMNS, GRID_ROWS};
use crate::maze_generation_algorithms::maze_generation_types::*;
use crate::pathfinding_algorithms::pathfinding_types::{PathAlgorithmState};
use crate::gui::grid_widget::square_grid_widget_data::*;

pub struct PathfinderController {
    pub timer_id: TimerToken,
    pub last_update: Instant,
}

impl PathfinderController {
    pub fn new() -> Self {
        PathfinderController {
            timer_id: TimerToken::INVALID,
            last_update: Instant::now(),
        }
    }
}

impl <W: Widget<AppData>> Controller<AppData, W> for PathfinderController {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut AppData, env: &Env) {
        match event {
            Event::WindowConnected => {
                let deadline = Duration::from_micros(data.iter_interval());
                self.last_update = Instant::now();
                self.timer_id = ctx.request_timer(deadline);           
            }
    
            Event::Timer(id) => {
                
                if *id == self.timer_id {
                    
                    if !data.is_paused && data.is_running {
                        if data.pathfinder_mode {
                            let algorithm = data.path_tool.get_inner();
                            let init_state = &PathAlgorithmState::Initialization;
                            let failed_state = &PathAlgorithmState::Failed;
                            let finished_state =  &PathAlgorithmState::Finished;

                            if algorithm.get_algorithm_state() != failed_state && algorithm.get_algorithm_state() != finished_state { // Run the algorithm
                                println!("Pathfinding algorithm running");

                                if algorithm.next_step(&mut data.grid_data.grid, &mut data.path_config) == *finished_state {
                                    data.grid_data.interaction_state = Interaction::None;
                                }
        
                                for node in algorithm.get_open_nodes().iter(){
                                    data.grid_data.grid.add_node(&node.position, GridNodeType::UnexploredNodes(data.grid_data.selected_net), data.grid_data.selected_net);
                                }
        
                                for node in algorithm.get_closed_nodes().iter(){
                                    data.grid_data.grid.add_node(&node.position, GridNodeType::ExploredNodes(data.grid_data.selected_net), data.grid_data.selected_net);                            
                                }
        
                                for node in algorithm.get_path_nodes().iter(){
                                    data.grid_data.grid.add_node(&node.position, GridNodeType::ChosenPath(data.grid_data.selected_net), data.grid_data.selected_net);                            
                                }

                                println!("Painting");
                                ctx.request_paint(); // Change to partial paint? Move to each for loop                                
                            }
    
                        } else {
                            let algorithm = data.maze_tool.get_inner();
                            let init_state = &MazeAlgorithmState::Initialization;
                            let failed_state = &MazeAlgorithmState::Failed;
                            let finished_state =  &MazeAlgorithmState::Finished;

                            if algorithm.get_algorithm_state() != failed_state && algorithm.get_algorithm_state() != finished_state {
                                println!("Maze generation algorithm running");
                                if algorithm.next_step(&mut data.grid_data.grid) == *finished_state {
                                    data.grid_data.interaction_state = Interaction::None;
                                }
                                ctx.request_paint(); // Change to partial paint? Move to each for loop                                
                            }
                            
                        }
                        
                    }
                    let deadline = Duration::from_micros(data.iter_interval());
                    self.last_update = Instant::now();
                    self.timer_id = ctx.request_timer(deadline);
                }
            }

            Event::Command(cmd) => {
                if cmd.is(RESET) {
                    //println!("Resetting algorithm");
                    let mut path_algorithm = data.path_tool.get_inner();
                    path_algorithm.reset();

                    let mut maze_algorithm = data.maze_tool.get_inner();
                    maze_algorithm.reset();
                }

                child.event(ctx, event, data, env)

            }

            _ =>  child.event(ctx, event, data, env),

        }
    }
}