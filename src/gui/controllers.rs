use druid::{widget::Controller, Env, Event, EventCtx, TimerToken, Widget};
use log::*;
use std::time::{Duration, Instant};

use crate::data::app_data::{AppData, GRID_COLUMNS, GRID_ROWS};
use crate::gui::grid_widget::square_grid_widget_data::*;
use crate::maze_generation_algorithms::maze_generation_types::*;
use crate::pathfinding_algorithms::pathfinding_types::PathAlgorithmState;

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

impl<W: Widget<AppData>> Controller<AppData, W> for PathfinderController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut AppData,
        env: &Env,
    ) {
        match event {
            Event::WindowConnected => {
                let deadline = Duration::from_millis(data.to_period_milli());
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
                            let finished_state = &PathAlgorithmState::Finished;

                            if algorithm.get_algorithm_state() != failed_state
                                && algorithm.get_algorithm_state() != finished_state
                            {
                                // Run the algorithm
                                //debug!("Pathfinding algorithm running");

                                if algorithm.next_step(
                                    &mut data.grid_data.grid,
                                    &mut data.path_config,
                                    data.grid_data.selected_net,
                                ) == *finished_state
                                {
                                    data.grid_data.interaction_state = Interaction::None;
                                }
                            }
                        } else {
                            let algorithm = data.maze_tool.get_inner();
                            let init_state = &MazeAlgorithmState::Initialization;
                            let failed_state = &MazeAlgorithmState::Failed;
                            let finished_state = &MazeAlgorithmState::Finished;

                            if algorithm.get_algorithm_state() != failed_state
                                && algorithm.get_algorithm_state() != finished_state
                            {
                                //debug!("Maze generation algorithm running");
                                if algorithm.next_step(&mut data.grid_data.grid) == *finished_state
                                {
                                    data.grid_data.interaction_state = Interaction::None;
                                }
                            }
                        }
                    }
                    let deadline = Duration::from_millis(data.to_period_milli());
                    self.last_update = Instant::now();
                    self.timer_id = ctx.request_timer(deadline);
                }
            }

            Event::Command(cmd) => {
                if cmd.is(RESET) {
                    info!("Resetting algorithm");
                    let mut path_algorithm = data.path_tool.get_inner();
                    path_algorithm.reset();

                    let mut maze_algorithm = data.maze_tool.get_inner();
                    maze_algorithm.reset();
                }

                child.event(ctx, event, data, env)
            }

            _ => child.event(ctx, event, data, env),
        }
    }
}
