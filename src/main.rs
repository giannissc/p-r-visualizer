// TODO Configure rustfmt and clippy
// TODO Implement custom grid patterns
// Implement drop selection
// Add selection for heuristic
// Add bidirectional checkbox
// Add diagonal checkbox
// Implement Maze algorithm
// Have algorithms run in their own thread

mod data {
    pub mod app_data;
}

mod gui {
    pub mod view;
    pub mod controllers;

    pub mod grid_widget {
        pub mod square_grid_widget_data;
        pub mod square_grid_widget_view;
    }
}

mod pathfinding_algorithms {
    pub mod pathfinding_types;
    pub mod astar;
    pub mod dijkstra;
    pub mod bfs;
    pub mod dfs;
    pub mod swarm;
    pub mod jump_point;
    pub mod greedy_best_first;
    pub mod distance_heuristics;
}

mod maze_generation_algorithms {
    pub mod maze_generation_types;
    pub mod random;
    pub mod recursive_backtrace;
    pub mod recursive_subdivision;

}

use crate::data::app_data::*;
use crate::gui::view::make_ui;
use crate::gui::grid_widget::square_grid_widget_data::{GridWidgetData, GridNodePosition, Grid};
use crate::pathfinding_algorithms::pathfinding_types::*;
use crate::pathfinding_algorithms::{astar::Astar};
use crate::maze_generation_algorithms::{recursive_backtrace::RecursiveBacktrace};
use crate::maze_generation_algorithms::maze_generation_types::*;

// Druid imports

use druid::{ theme, AppLauncher, LocalizedString, WindowDesc,Color, };



//////////////////////////////////////////////////////////////////////////////////////
//
// Main
//
//////////////////////////////////////////////////////////////////////////////////////

fn main() {
    let main_window = WindowDesc::new(make_ui())
        .window_size((1000.0, 500.0))
        .title(LocalizedString::new("Placement & Routing Experiments"));
    let data = AppData {
        is_paused: false,
        is_running: false,
        updates_per_second: 10.0,
        grid_data: GridWidgetData::new(Grid::new(GridNodePosition{row: 20, col: 10}, GridNodePosition{row:20, col:50})),
        path_tool: PathAlgorithms::Astar(Astar::new()),
        path_config: PathfinderConfig::new(),
        maze_tool: MazeAlgorithms::RecursiveBacktrace(RecursiveBacktrace::new()),
        pathfinder_mode: true,       
    };
    AppLauncher::with_window(main_window)
        .configure_env(|env, _| {
            env.set(theme::SELECTION_TEXT_COLOR, Color::rgb8(0xA6, 0xCC, 0xFF));
            env.set(theme::WINDOW_BACKGROUND_COLOR, Color::WHITE);
            env.set(theme::LABEL_COLOR, Color::WHITE);
            env.set(theme::CURSOR_COLOR, Color::BLACK);
            env.set(theme::BACKGROUND_LIGHT, Color::rgb8(230, 230, 230));
        })
        .log_to_console()
        .launch(data)
        .expect("launch failed");
}