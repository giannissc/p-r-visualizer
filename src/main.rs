// TODO Configure rustfmt and clippy
// TODO Implement custom patterns
// TODO Refactor app structure

mod controllers;
mod data;
mod pathfinding_algorithms;
mod maze_algorithms;

mod gui {
    pub mod grid_axis_widget;
    pub mod view;
}


use crate::data::pathfinding_types::*;
use crate::data::*;
use crate::gui::view::make_ui;

// Druid imports

use druid::{ theme, AppLauncher, LocalizedString, WindowDesc,Color, };



//////////////////////////////////////////////////////////////////////////////////////
//
// Main
//
//////////////////////////////////////////////////////////////////////////////////////

fn main() {
    let main_window = WindowDesc::new(make_ui)
        .window_size((1000.0, 500.0))
        .title(LocalizedString::new("Placement & Routing Experiments"));
    let data = AppData {
        is_paused: true,
        updates_per_second: 20.0,
        grid: Grid::new(GridNodePosition{row: 20, col: 10}, GridNodePosition{row:20, col:50}),
        selected_tool: GridNodeType::Wall,
        path_algo: PathAlgorithms::Astar,
        maze_algo: MazeAlgorithms::Random,
        show_grid_lines: true,
    };
    AppLauncher::with_window(main_window)
        .configure_env(|env, _| {
            env.set(theme::SELECTION_COLOR, Color::rgb8(0xA6, 0xCC, 0xFF));
            env.set(theme::WINDOW_BACKGROUND_COLOR, Color::WHITE);
            env.set(theme::LABEL_COLOR, Color::WHITE);
            env.set(theme::CURSOR_COLOR, Color::BLACK);
            env.set(theme::BACKGROUND_LIGHT, Color::rgb8(230, 230, 230));
        })
        .use_simple_logger()
        .launch(data)
        .expect("launch failed");
}