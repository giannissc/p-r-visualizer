use druid::{widget::Controller, Env, EventCtx, Widget, TimerToken, Event};
use std::time::{Duration, Instant};

use crate::AppData;

pub struct TimerController {
    timer_id: TimerToken,
    last_update: Instant,
}

impl TimerController {
    pub fn new() -> Self {
        TimerController {
            timer_id: TimerToken::INVALID,
            last_update: Instant::now(),
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
                
            }
    
            Event::Timer(id) => {
                if *id == self.timer_id {
                    if !data.is_paused {
                        ctx.request_paint(); // Change to partial paint?
                        println!("Not paused")
                    }
                    let deadline = Duration::from_millis(data.iter_interval());
                    self.last_update = Instant::now();
                    self.timer_id = ctx.request_timer(deadline);
                    println!("Deadline: {:?}", deadline);
                }
            }

            _ =>  child.event(ctx, event, data, env),

        }
    }
}