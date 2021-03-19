
use druid::{Env, EventCtx, Widget, WidgetExt, TextLayout};
use druid::widget::{Button, Flex, Label, MainAxisAlignment, CrossAxisAlignment, Slider, Checkbox, Switch, LensWrap};
use druid_widget_nursery::{DropdownSelect};
use druid::im::vector;

use super::grid_widget::square_grid_widget_data::*;
use super::grid_widget::square_grid_widget_view::GridWidget;
use crate::gui::controllers::{PathfinderController};
use crate::data::app_data::*;
use crate::pathfinding_algorithms::pathfinding_types::*;
use crate::pathfinding_algorithms::{astar::Astar, dijkstra::Dijkstra, bfs::BFS, dfs::DFS, swarm::Swarm};
use crate::maze_generation_algorithms::maze_generation_types::*;
use crate::maze_generation_algorithms::{random::Random, recursive_subdivision::RecursiveSubdivision, recursive_backtrace::RecursiveBacktrace};

////////////////////////////////////////////////////////////////////////////////////////////////
/// UI functions
////////////////////////////////////////////////////////////////////////////////////////////////

pub fn make_ui() -> impl Widget<AppData> {
    let grid = Flex::column().with_child(GridWidget::new(COLOR, GRID_ROWS, GRID_COLUMNS).with_id(GRID_ID).lens(AppData::grid_data));
    //let grid = Flex::column().with_child(GridWidget::new(COLOR, GRID_ROWS, GRID_COLUMNS).lens(AppData::grid_data).debug_invalidation());

    let switch = LensWrap::new(Switch::new(), AppData::pathfinder_mode);
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
                                    //.with_flex_child(make_path_dropdown(), 1.0)
                                    .with_flex_child(switch, 1.0)
                                    //.with_flex_child(make_maze_dropdown(), 1.0)
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
                                .with_range(0.5, 1000.0)
                                .expand_width()
                                .lens(AppData::updates_per_second),
                            1.,
                        )
                        .padding(8.0),
                ).background(BACKGROUND),     
            ).main_axis_alignment(MainAxisAlignment::SpaceBetween).cross_axis_alignment(CrossAxisAlignment::Center)
            .controller(PathfinderController::new())
            
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
    }).lens(AppData::grid_data).padding((5., 5.))
}

fn make_tool_dropdown() -> impl Widget<AppData> {
    DropdownSelect::new(vector![
        ("Wall", GridNodeType::Wall),
        ("Erase", GridNodeType::Empty),
        ("Start Node", GridNodeType::StartNode(1)), // It doesn't matter which number you have here because when adding node both selected_tool and selected_net will be used
        ("End Node", GridNodeType::TargetNode(1)),
    ]).lens(GridWidgetData::selected_tool).lens(AppData::grid_data).padding((5., 5.))
}

fn make_path_dropdown() -> impl Widget<AppData> {
    println!("Calling path dropdown");
    DropdownSelect::new(vector![
        ("A star", PathAlgorithms::Astar(Astar::new())),
        ("Dijkstra", PathAlgorithms::Dijkstra(Dijkstra::new())),
        ("BFS", PathAlgorithms::BFS(BFS::new())),
        ("DFS", PathAlgorithms::DFS(DFS::new())),
        ("Swarm", PathAlgorithms::Swarm(Swarm::new())),
    ]).lens(AppData::path_tool).padding((5., 5.))
}

fn make_maze_dropdown() -> impl Widget<AppData> {
    DropdownSelect::new(vector![
        ("Recursive Backtrace", MazeAlgorithms::RecursiveBacktrace(RecursiveBacktrace::new())),
        ("Recusrive Subdivision", MazeAlgorithms::RecursiveSubdivision(RecursiveSubdivision::new())),
        ("Random", MazeAlgorithms::Random(Random::new())),
    ]).lens(AppData::maze_tool).padding((5., 5.))
}

fn make_grid_lines_button() -> impl Widget<AppData> {
    Checkbox::new("Grid Axis").on_click(|_ctx: &mut EventCtx, data: &mut bool, _: &Env| {
        *data = !*data;        
    }).lens(GridWidgetData::show_grid_axis).lens(AppData::grid_data).padding((5., 5.)) 
}