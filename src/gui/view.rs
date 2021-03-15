
use druid::{WidgetExt, Env, Widget, EventCtx };
use druid::widget::{Button, Flex, Label, MainAxisAlignment, CrossAxisAlignment, Slider, Checkbox,};
use druid_widget_nursery::{DropdownSelect};
use druid::im::vector;

use super::grid_widget::grid_widget_data::*;
use super::grid_widget::grid_widget_view::GridWidget;
use crate::gui::controllers::TimerController;
use crate::data::app_data::*;
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
                                .with_range(0.2, 500.0)
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

fn make_tool_dropdown() -> impl Widget<AppData> {
    DropdownSelect::new(vector![
        ("Wall", GridNodeType::Wall),
        ("Erase", GridNodeType::Empty),
        ("Start Node", GridNodeType::StartNode(1)),
        ("End Node", GridNodeType::TargetNode(1)),
    ]).lens(GridWidgetData::selected_tool).lens(AppData::grid_data).padding((5., 5.))
}

fn make_path_dropdown() -> impl Widget<AppData> {
    DropdownSelect::new(vector![
        ("A star", PathAlgorithms::Astar),
        ("Dijkstra", PathAlgorithms::Dijkstra),
        ("Swarm", PathAlgorithms::Swarm),
        ("Jump Point", PathAlgorithms::JumpPoint),
    ]).lens(AppData::path_tool).padding((5., 5.))
}

fn make_maze_dropdown() -> impl Widget<AppData> {
    DropdownSelect::new(vector![
        ("Random", MazeAlgorithms::Random),
        ("Recusrive Subdivision", MazeAlgorithms::RecursiveSubdivision),
        ("Recursive Backtrace", MazeAlgorithms::RecursiveBacktrace),
    ]).lens(AppData::maze_tool).padding((5., 5.))
}

fn make_grid_lines_button() -> impl Widget<AppData> {
    Checkbox::new("Grid Axis").on_click(|_ctx: &mut EventCtx, data: &mut bool, _: &Env| {
        *data = !*data;        
    }).lens(GridWidgetData::show_grid_axis).lens(AppData::grid_data).padding((5., 5.)) 
}