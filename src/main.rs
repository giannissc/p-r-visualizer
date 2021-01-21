// TODO Configure rustfmt and clippy

mod grid_axis;
mod pathfinding_algorithms;
mod maze_algorithms;

use grid_axis::{
    GridWidget, Grid, GridNodes
};


// Druid imports
use druid::widget::{Button, Flex, Label, MainAxisAlignment, CrossAxisAlignment, Slider, Checkbox};
use druid::{
    theme, AppLauncher, Data, Lens, LocalizedString, WidgetExt,
    WindowDesc, Env, Widget, Color,
};
use maze_algorithms::MazeAlgorithms;
use pathfinding_algorithms::PathAlgorithms;

//////////////////////////////////////////////////////////////////////////////////////
// Constants
//////////////////////////////////////////////////////////////////////////////////////
const GRID_COLUMNS: usize = 160; 
const GRID_ROWS: usize = 100; 
const COLOR: Color = Color::BLACK;
const BACKGROUND: Color = Color::grey8(23);

//////////////////////////////////////////////////////////////////////////////////////
// Structs
//////////////////////////////////////////////////////////////////////////////////////
// Application State
#[derive(Clone, Data, Lens)]
struct AppData {
    pause_algo: bool,
    start_algo: bool,
    updates_per_second: f64,
    grid: Grid,
    tool: GridNodes,
    path_algo: PathAlgorithms,
    maze_algo: MazeAlgorithms,
}

//////////////////////////////////////////////////////////////////////////////////////
//
// Main
//
//////////////////////////////////////////////////////////////////////////////////////

fn main() {
    // TODO Arrange for window size to be set so that it fits the number of row, columns, cell_size
    let main_window = WindowDesc::new(make_ui_simple)
        .window_size((1000.0, 500.0))
        .title(LocalizedString::new("Placement & Routing Experiments"));
    let data = AppData {
        pause_algo: true,
        start_algo: false,
        updates_per_second: 20.0,
        grid: Grid::new(),
        tool: GridNodes::Wall,
        path_algo: PathAlgorithms::Astar,
        maze_algo: MazeAlgorithms::Random,
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

fn make_ui_simple() -> impl Widget<AppData> {
    let grid = Flex::column().with_child(GridWidget::new(COLOR, GRID_ROWS, GRID_COLUMNS).lens(AppData::grid));
    //let grid = Flex::column().with_child(GridWidget::new(COLOR, GRID_ROWS, GRID_COLUMNS).lens(AppData::grid).debug_invalidation());
    
    let start_button = Button::new(|data: &bool, _: &Env| match data {
        true => "Stop".into(),
        false => "Start".into(),
    })
    .on_click(|ctx, data: &mut bool, _: &Env| {
        *data = !*data;
        ctx.request_layout();
    }).lens(AppData::start_algo).padding((5., 5.));

    let pause_button = Button::new(|data: &bool, _: &Env| match data {
        true => "Resume".into(),
        false => "Pause".into(),
    })
    .on_click(|ctx, data: &mut bool, _: &Env| {
        *data = !*data;
        ctx.request_layout();
    }).lens(AppData::pause_algo).padding((5., 5.));

    let clear_button = Button::new("Clear")
    .on_click(|ctx, data: &mut Grid, _: &Env| {
        data.clear();
        ctx.request_paint();
    }).lens(AppData::grid).padding((5., 5.));

    let tool_button = Button::new(|data: &GridNodes, _: &Env| match data {
        GridNodes::Wall => "Wall".into(),
        GridNodes::StartNode(1) => "StartNode".into(),
        GridNodes::EndNode(1) => "EndNode".into(),
        _ => "".into(),
    })
    .on_click(|ctx, data: &mut GridNodes, _: &Env| {
        match data{
            GridNodes::Wall => *data = GridNodes::StartNode(1),
            GridNodes::StartNode(1) => *data = GridNodes::EndNode(1),
            GridNodes::EndNode(1) => *data = GridNodes::Wall,
            _ => ()
        };
        ctx.request_layout();
    }).lens(AppData::tool).padding((5., 5.));

    let path_button = Button::new(|data: &PathAlgorithms, _: &Env| match data {
        PathAlgorithms::Astar => "A star".into(),
        PathAlgorithms::Dijkstra => "Dijkstra".into(),
        PathAlgorithms::BidirectionalDijkstra => "Bidirectional Dijkstra".into(),
        _ => "".into(),
    })
    .on_click(|ctx, data: &mut PathAlgorithms, _: &Env| {
        match data{
            PathAlgorithms::Astar => *data = PathAlgorithms::Dijkstra,
            PathAlgorithms::Dijkstra => *data = PathAlgorithms::BidirectionalDijkstra,
            PathAlgorithms::BidirectionalDijkstra => *data = PathAlgorithms::Astar,
            _ => ()
        };
        ctx.request_layout();
    }).lens(AppData::path_algo).padding((5., 5.));

    let maze_button = Button::new(|data: &MazeAlgorithms, _: &Env| match data {
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
    }).lens(AppData::maze_algo).padding((5., 5.));

    Flex::column()
        .with_flex_child(grid,1.0)
        .with_child(
            Flex::column()
                .with_child(
                    // a row with two buttons
                    Flex::row()
                        .with_flex_child(start_button, 1.0,)
                        .with_flex_child(pause_button, 1.0,)
                        .with_flex_child(clear_button, 1.0,)
                        .with_flex_child(tool_button, 1.0)
                        .with_flex_child(path_button, 1.0)
                        .with_flex_child(maze_button, 1.0)
                        .padding(8.0),
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
}
