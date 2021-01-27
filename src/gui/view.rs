
use druid::{WidgetExt, Env, Widget, EventCtx };
use druid::widget::{Button, Flex, Label, MainAxisAlignment, CrossAxisAlignment, Slider, Checkbox,};
//use druid_widget_nursery::{DropdownSelect};

use super::grid_axis_widget::{GridWidget, WALL_TOOL, END_NODE_TOOL, START_NODE_TOOL, TOGGLE_GRID_AXIS, TOGGLE_DRAWING};
use crate::gui::controllers::TimerController;
use crate::data::*;
use crate::pathfinding_types::*;
use crate::PathAlgorithms;
use crate::MazeAlgorithms;

////////////////////////////////////////////////////////////////////////////////////////////////
/// UI functions
////////////////////////////////////////////////////////////////////////////////////////////////

pub fn make_ui() -> impl Widget<AppData> {
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
                                    .with_flex_child(make_run_button(), 1.0,)
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

fn make_run_button() -> impl Widget<AppData> {
    Button::new(|data: &bool, _: &Env| match data {
        true => "⏹️".into(),
        false => "▶️".into(),
    })
    .on_click(|ctx, data: &mut bool, _: &Env| {
        *data = !*data;
        ctx.submit_command(TOGGLE_DRAWING.to(ID_ONE));
        ctx.request_layout();
    }).lens(AppData::is_running).padding((5., 5.))
}

fn make_pause_button() -> impl Widget<AppData> {
    Button::new(|data: &bool, _: &Env| match data {
        true => "⏯".into(),
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
    }).lens(AppData::path_tool).padding((5., 5.))
}

fn make_maze_button() -> impl Widget<AppData> {
    Button::new(|data: &MazeAlgorithms, _: &Env| match data {
        MazeAlgorithms::Random => "Random".into(),
        MazeAlgorithms::Recursive => "Recursive".into(),
        MazeAlgorithms::Backtrace => "Backtrace".into(),
    })
    .on_click(|ctx, data: &mut MazeAlgorithms, _: &Env| {
        match data{
            MazeAlgorithms::Random => *data = MazeAlgorithms::Recursive,
            MazeAlgorithms::Recursive => *data = MazeAlgorithms::Backtrace,
            MazeAlgorithms::Backtrace => *data = MazeAlgorithms::Random,
        };
        ctx.request_layout();
    }).lens(AppData::maze_tool).padding((5., 5.)) 
}

fn make_grid_lines_button() -> impl Widget<AppData> {
    Checkbox::new("Grid Axis").on_click(|ctx: &mut EventCtx, data: &mut bool, _: &Env| {
        *data = !*data;
        ctx.submit_command(TOGGLE_GRID_AXIS.to(ID_ONE))
        
    }).lens(AppData::show_grid_lines).padding((5., 5.)) 
}