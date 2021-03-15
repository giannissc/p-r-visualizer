
use druid::{WidgetExt, Env, Widget, EventCtx };
use druid::widget::{Button, Flex, Label, MainAxisAlignment, CrossAxisAlignment, Slider, Checkbox,};
use druid_widget_nursery::{DropdownSelect};
use druid::im::vector;

use super::grid_axis_widget::{GridWidget, RESET, LOCK_DRAWING, UNLOCK_DRAWING};
use crate::gui::controllers::TimerController;
use crate::gui::grid_axis_widget::GridWidgetData;
use crate::data::*;
use crate::pathfinding_types::*;
use crate::PathAlgorithms;
use crate::MazeAlgorithms;

////////////////////////////////////////////////////////////////////////////////////////////////
/// UI functions
////////////////////////////////////////////////////////////////////////////////////////////////

pub fn make_ui() -> impl Widget<AppData> {
    let grid = Flex::column().with_child(GridWidget::new(COLOR, GRID_ROWS, GRID_COLUMNS).with_id(GRID_ID).lens(AppData::grid_data));
    //let grid = Flex::column().with_child(GridWidget::new(COLOR, GRID_ROWS, GRID_COLUMNS).lens(AppData::grid_data).debug_invalidation());

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
                                    .with_flex_child(make_tool_dropdown(), 1.0)
                                    .with_flex_child(make_path_dropdown(), 1.0)
                                    .with_flex_child(make_maze_dropdown(), 1.0)
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
                                .with_range(0.2, 1000.0)
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
        if *data {
            ctx.submit_command(LOCK_DRAWING.to(GRID_ID));
        } else {
            ctx.submit_command(UNLOCK_DRAWING.to(GRID_ID));
            ctx.submit_command(RESET);
        }
        
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
        
        if *data {
            ctx.submit_command(UNLOCK_DRAWING.to(GRID_ID));
        } else {
            ctx.submit_command(LOCK_DRAWING.to(GRID_ID));
        }

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
    .on_click(|ctx, data: &mut GridWidgetData, _: &Env| {
        data.grid.clear_all();
        ctx.submit_command(RESET);
        ctx.request_paint();
    }).lens(AppData::grid_data).padding((5., 5.))
}

fn make_tool_button() -> impl Widget<AppData> {
    Button::new(|data: &GridWidgetData, _: &Env| match data.selected_tool {
        GridNodeType::Wall => "Wall".into(),
        GridNodeType::Empty => "Erase".into(),
        GridNodeType::StartNode(1) => "StartNode".into(),
        GridNodeType::TargetNode(1) => "EndNode".into(),
        _ => "".into(),
    })
    .on_click(|ctx, data: &mut GridWidgetData, _: &Env| {
        match data.selected_tool{
            GridNodeType::Wall => {
                data.selected_tool = GridNodeType::Empty;
            },
            GridNodeType::Empty => {
                data.selected_tool = GridNodeType::StartNode(1);
            },
            GridNodeType::StartNode(1) => {
                data.selected_tool = GridNodeType::TargetNode(1);
            },
            GridNodeType::TargetNode(1) => {
                data.selected_tool = GridNodeType::Wall;
            },
            _ => ()
        };
        ctx.request_layout();
    }).lens(AppData::grid_data).padding((5., 5.))
}

fn make_tool_dropdown() -> impl Widget<AppData> {
    DropdownSelect::new(vector![
        ("Wall", GridNodeType::Wall),
        ("Erase", GridNodeType::Empty),
        ("Start Node", GridNodeType::StartNode(1)),
        ("End Node", GridNodeType::TargetNode(1)),
    ]).lens(GridWidgetData::selected_tool).lens(AppData::grid_data).padding((5., 5.))
}

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

fn make_path_dropdown() -> impl Widget<AppData> {
    DropdownSelect::new(vector![
        ("A star", PathAlgorithms::Astar),
        ("Dijkstra", PathAlgorithms::Dijkstra),
        ("Swarm", PathAlgorithms::Swarm),
        ("Jump Point", PathAlgorithms::JumpPoint),
    ]).lens(AppData::path_tool).padding((5., 5.))
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

fn make_maze_dropdown() -> impl Widget<AppData> {
    DropdownSelect::new(vector![
        ("Random", MazeAlgorithms::Random),
        ("Recusrive", MazeAlgorithms::Recursive),
        ("Backtrace", MazeAlgorithms::Backtrace),
    ]).lens(AppData::maze_tool).padding((5., 5.))
}

fn make_grid_lines_button() -> impl Widget<AppData> {
    Checkbox::new("Grid Axis").on_click(|ctx: &mut EventCtx, data: &mut bool, _: &Env| {
        *data = !*data;        
    }).lens(GridWidgetData::show_grid_axis).lens(AppData::grid_data).padding((5., 5.)) 
}