// TODO Configure rustfmt and clippy
// TODO Implement custom patterns
// TODO Refactor app structure

mod grid_axis;
mod pathfinding_algorithms;
mod maze_algorithms;
mod distance_heuristics;
mod pathfinding_types;
mod controllers;

use grid_axis::{GridWidget, WALL_TOOL, END_NODE_TOOL, START_NODE_TOOL, TOGGLE_GRID_AXIS};
use crate::pathfinding_types::*;
use crate::controllers::TimerController;

// Druid imports
use druid::{EventCtx, widget::{Button, Flex, Label, MainAxisAlignment, CrossAxisAlignment, Slider, Checkbox, WidgetId}};
use druid::{
    theme, AppLauncher, Data, Lens, LocalizedString, WidgetExt,
    WindowDesc, Env, Widget, Color,
};
use maze_algorithms::MazeAlgorithms;
use druid_widget_nursery::{DropdownSelect};
use pathfinding_algorithms::PathAlgorithms;

//////////////////////////////////////////////////////////////////////////////////////
// Constants
//////////////////////////////////////////////////////////////////////////////////////
const GRID_COLUMNS: usize = 160; 
const GRID_ROWS: usize = 100; 
const COLOR: Color = Color::BLACK;
const BACKGROUND: Color = Color::grey8(23);
const ID_ONE: WidgetId = WidgetId::reserved(1);


//////////////////////////////////////////////////////////////////////////////////////
// Structs
//////////////////////////////////////////////////////////////////////////////////////
// Application State
#[derive(Clone, Data, Lens)]
pub struct AppData {
    pub is_paused: bool,
    updates_per_second: f64,
    pub grid: Grid,
    selected_tool: GridNodeType,
    path_algo: PathAlgorithms,
    maze_algo: MazeAlgorithms,
    show_grid_lines: bool
}

impl AppData {
    pub fn iter_interval(&self) -> u64 {
        (1000. / self.updates_per_second) as u64
    }
}

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

////////////////////////////////////////////////////////////////////////////////////////////////
/// UI functions
////////////////////////////////////////////////////////////////////////////////////////////////

fn make_ui() -> impl Widget<AppData> {
    let grid = Flex::column().with_child(GridWidget::new(COLOR, GRID_ROWS, GRID_COLUMNS).with_id(ID_ONE).lens(AppData::grid));
    //let grid = Flex::column().with_child(GridWidget::new(COLOR, GRID_ROWS, GRID_COLUMNS).lens(AppData::grid).debug_invalidation());

    Flex::column()
        .with_flex_child(grid,1.0)
        .with_child(
            Flex::column()
                .with_child(
                    // a row with two buttons
                    Flex::row()
                        .with_flex_child(
                            Flex::row()
                                    .with_flex_child(make_stop_button(), 1.0,)
                                    .with_flex_child(make_pause_button(), 1.0,)
                                    .with_flex_child(make_previous_button(), 1.0)
                                    .with_flex_child(make_next_button(), 1.0)
                                    .with_flex_child(make_clear_button(), 1.0,)
                                    .with_flex_child(make_tool_button(), 1.0)
                                    .with_flex_child(make_path_button(), 1.0)
                                    .with_flex_child(make_maze_button(), 1.0)
                                    .padding(8.0), 
                            1.0)
                        .with_child(make_grid_lines_button())
                    
                )
                .with_child(
                    Flex::row()
                        .with_child(
                            Label::new(|data: &AppData, _env: &_| {
                                format!("{:.2} updates/s", data.updates_per_second)
                            })
                            .padding(3.0),
                        )
                        .with_flex_child(
                            Slider::new()
                                .with_range(0.2, 20.0)
                                .expand_width()
                                .lens(AppData::updates_per_second),
                            1.,
                        )
                        .padding(8.0),
                ).background(BACKGROUND),     
            ).main_axis_alignment(MainAxisAlignment::SpaceBetween).cross_axis_alignment(CrossAxisAlignment::Center)
            .controller(TimerController::new())
}

fn make_stop_button() -> impl Widget<AppData> {
    Button::new("⏹")
    .on_click(|_ctx, _data: &mut AppData, _: &Env| {
        //ctx.request_layout();
    }).padding((5., 5.))
}

fn make_pause_button() -> impl Widget<AppData> {
    Button::new(|data: &bool, _: &Env| match data {
        true => "▶".into(),
        false => "⏸".into(),
    })
    .on_click(|ctx, data: &mut bool, _: &Env| {
        *data = !*data;
        ctx.request_layout();
    }).lens(AppData::is_paused).padding((5., 5.))
}

fn make_previous_button() -> impl Widget<AppData> {
    Button::new("⏮")
    .on_click(|_ctx, _data: &mut AppData, _: &Env| {
        //ctx.request_layout();
    }).padding((5., 5.))
}

fn make_next_button() -> impl Widget<AppData> {
    Button::new("⏭")
    .on_click(|_ctx, _data: &mut AppData, _: &Env| {
        //ctx.request_layout();
    }).padding((5., 5.))
}

fn make_clear_button() -> impl Widget<AppData> {
    Button::new("Clear")
    .on_click(|ctx, data: &mut Grid, _: &Env| {
        data.clear_all();
        ctx.request_paint();
    }).lens(AppData::grid).padding((5., 5.))
}

fn make_tool_button() -> impl Widget<AppData> {
    Button::new(|data: &GridNodeType, _: &Env| match data {
        GridNodeType::Wall => "Wall".into(),
        GridNodeType::StartNode(1) => "StartNode".into(),
        GridNodeType::TargetNode(1) => "EndNode".into(),
        _ => "".into(),
    })
    .on_click(|ctx, data: &mut GridNodeType, _: &Env| {
        match data{
            GridNodeType::Wall => {
                *data = GridNodeType::StartNode(1);
                ctx.submit_command(START_NODE_TOOL.to(ID_ONE))
            },
            GridNodeType::StartNode(1) => {
                *data = GridNodeType::TargetNode(1);
                ctx.submit_command(END_NODE_TOOL.to(ID_ONE))
            },
            GridNodeType::TargetNode(1) => {
                *data = GridNodeType::Wall;
                ctx.submit_command(WALL_TOOL.to(ID_ONE))
            },
            _ => ()
        };
        ctx.request_layout();
    }).lens(AppData::selected_tool).padding((5., 5.))
}

/* 
fn make_path_dropdown() -> impl Widget<AppData> {
    DropdownSelect::build_widget(vec![
        ("A star", PathAlgorithms::Astar),
        ("Dijkstra", PathAlgorithms::Dijkstra),
        ("Swarm", PathAlgorithms::Swarm),
        ("Jump Point", PathAlgorithms::JumpPoint),
    ])
}

*/

fn make_path_button() -> impl Widget<AppData> {
    Button::new(|data: &PathAlgorithms, _: &Env| match data {
        PathAlgorithms::Astar => "A star".into(),
        PathAlgorithms::Dijkstra => "Dijkstra".into(),
        PathAlgorithms::Swarm => "Swarm".into(),
        _ => "".into(),
    })
    .on_click(|ctx, data: &mut PathAlgorithms, _: &Env| {
        match data{
            PathAlgorithms::Astar => *data = PathAlgorithms::Dijkstra,
            PathAlgorithms::Dijkstra => *data = PathAlgorithms::Swarm,
            PathAlgorithms::Swarm => *data = PathAlgorithms::Astar,
            _ => ()
        };
        ctx.request_layout();
    }).lens(AppData::path_algo).padding((5., 5.))
}

fn make_maze_button() -> impl Widget<AppData> {
    Button::new(|data: &MazeAlgorithms, _: &Env| match data {
        MazeAlgorithms::Random => "Random".into(),
        MazeAlgorithms::Recursive => "Recursive".into(),
        MazeAlgorithms::Backtrace => "Backtrace".into(),
        _ => "".into(),
    })
    .on_click(|ctx, data: &mut MazeAlgorithms, _: &Env| {
        match data{
            MazeAlgorithms::Random => *data = MazeAlgorithms::Recursive,
            MazeAlgorithms::Recursive => *data = MazeAlgorithms::Backtrace,
            MazeAlgorithms::Backtrace => *data = MazeAlgorithms::Random,
            _ => ()
        };
        ctx.request_layout();
    }).lens(AppData::maze_algo).padding((5., 5.)) 
}

fn make_grid_lines_button() -> impl Widget<AppData> {
    Checkbox::new("Grid Axis").on_click(|ctx: &mut EventCtx, data: &mut bool, _: &Env| {
        *data = !*data;
        ctx.submit_command(TOGGLE_GRID_AXIS.to(ID_ONE))
        
    }).lens(AppData::show_grid_lines).padding((5., 5.)) 
}
