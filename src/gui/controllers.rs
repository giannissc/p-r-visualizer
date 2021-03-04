use druid::{widget::Controller, Env, EventCtx, Widget, TimerToken, Event,};
use std::time::{Duration, Instant};

use crate::{AppData};
use crate::pathfinding_algorithms::*;
use crate::data::pathfinding_types::*;
use crate::gui::grid_axis_widget::{UNLOCK_DRAWING, RESET};
use crate::data::GRID_ID;




pub struct TimerController {
    pub timer_id: TimerToken,
    pub last_update: Instant,
    path_algo: PathAlgo,
}

impl TimerController {
    pub fn new() -> Self {
        TimerController {
            timer_id: TimerToken::INVALID,
            last_update: Instant::now(),
            path_algo: PathAlgo::new(),
        }
    }
}

impl <W: Widget<AppData>> Controller<AppData, W> for TimerController {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut AppData, env: &Env) {
        match event {
            Event::WindowConnected => {
                let deadline = Duration::from_millis(data.iter_interval());
                self.last_update = Instant::now();
                self.timer_id = ctx.request_timer(deadline);
                ctx.request_paint();                
            }
    
            Event::Timer(id) => {
                if *id == self.timer_id {

                    if !data.is_paused && data.is_running  && self.path_algo.algorithm_state != AlgorithmState::Finished { // Run the algorithm
                        
                        //println!("Algorithm running");
                        if self.path_algo.next_step(&mut data.grid) == AlgorithmState::Finished {
                            //data.is_running = false;
                            ctx.submit_command(UNLOCK_DRAWING.to(GRID_ID));
                        }

                        for node in self.path_algo.get_open_nodes().iter(){
                            data.grid.add_node(&node.position, GridNodeType::UnexploredNodes(1));
                            
                        }

                        for node in self.path_algo.get_closed_nodes().iter(){
                            data.grid.add_node(&node.position, GridNodeType::ExploredNodes(1));                            
                        }

                        for node in self.path_algo.get_path_nodes().iter(){
                            data.grid.add_node(&node.position, GridNodeType::ChosenPath(1));                            
                        }

                        ctx.request_paint(); // Change to partial paint?
                    }

                    let deadline = Duration::from_millis(data.iter_interval());
                    self.last_update = Instant::now();
                    self.timer_id = ctx.request_timer(deadline);
                }
            }

            Event::Command(cmd) => {
                if cmd.is(RESET) {
                    println!("Resetting algorithm");
                    self.path_algo.reset();
                }

                child.event(ctx, event, data, env)

            }

            _ =>  child.event(ctx, event, data, env),

        }
    }
}