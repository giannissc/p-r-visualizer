// TODO Configure rustfmt and clippy
// TODO Implement custom grid patterns
// Implement drop selection
// Add selection for heuristic
// Add bidirectional checkbox
// Add diagonal checkbox
// Implement Maze algorithm
// Have algorithms run in their own thread


mod pathfinding_algorithms;
mod maze_algorithms;

mod data {
    pub mod app_data;
    pub mod distance_heuristics;
    pub mod pathfinding_types;
}

mod gui {
    pub mod view;
    pub mod controllers;

    pub mod grid_widget {
        pub mod grid_widget_data;
        pub mod grid_widget_view;
    }
}


use crate::data::pathfinding_types::*;
use crate::data::app_data::*;
use crate::gui::view::make_ui;
use crate::gui::grid_widget::grid_widget_data::{GridWidgetData, GridNodePosition, Grid};

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
        path_tool: PathAlgorithms::Astar,
        maze_tool: MazeAlgorithms::Random,        
    };
    AppLauncher::with_window(main_window)
        .configure_env(|env, _| {
            env.set(theme::SELECTION_COLOR, Color::rgb8(0xA6, 0xCC, 0xFF));
            env.set(theme::WINDOW_BACKGROUND_COLOR, Color::WHITE);
            env.set(theme::LABEL_COLOR, Color::WHITE);
            env.set(theme::CURSOR_COLOR, Color::BLACK);
            env.set(theme::BACKGROUND_LIGHT, Color::rgb8(230, 230, 230));
        })
        .use_env_tracing()
        .launch(data)
        .expect("launch failed");
}