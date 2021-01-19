// TODO Implement infinite zoom and pan functionallity. See scroll example and clipBox documentation
// TODO Configure rustfmt and clippy
// TODO Merge Grid and GridWidget into one struct
// TODO Implement Grid axis lines.
// FIXME Removing ctx.stroke shows grid lines on windowed mode but not maximized.

use std::ffi::IntoStringError;
// Druid imports
use druid::widget::{
    Align, Button, Container, CrossAxisAlignment, Flex, Label, List, Padding, RadioGroup, Scroll,
    SizedBox, Split,
};
use druid::{
    theme, AppLauncher, BoxConstraints, Data, Lens, LocalizedString, WidgetExt,
    WindowDesc,
};
use druid::{Color, Point, Rect, Size, im::HashMap, kurbo::Line};
use druid::{
    Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, MouseButton, PaintCtx, RenderContext,
    UpdateCtx, Widget,
};

//////////////////////////////////////////////////////////////////////////////////////
// Constants
//////////////////////////////////////////////////////////////////////////////////////
const GRID_COLUMNS: usize = 500; 
const GRID_ROWS: usize = 250; 
const LIGHT_BLUE: Color = Color::from_rgba32_u32(0xA4CBFA); 

//////////////////////////////////////////////////////////////////////////////////////
// Structs
//////////////////////////////////////////////////////////////////////////////////////
// Application State
#[derive(Clone, Data, Lens)]
struct State {
    grid: Grid,
    drawing: Interaction,
    paused: bool,
}

// Drawing Structs
#[derive(Clone, Data)]
struct Grid {
    //storage: Arc<Vec<bool>>,
    //storage: Arc<Vec<Option<GridNodes>>>,
    storage: HashMap<GridPos, GridNodes>,
    show_grid_axis: bool,
}

// Application Custom Widgets
struct GridWidget {
    cell_size: Size,
    color: Color,
}

#[derive(Clone, PartialEq, Data)]
enum GridNodes {
    Wall,
    StartNode(i32),
    EndNode(i32),
    OpenPath(i32),
    ClosedPath(i32),
    ChosenPath(i32),
}

#[derive(Clone, PartialEq, Data)]
enum Interaction {
    None,
    Drawing,
    Erasing,
}

#[derive(Clone, Data, Copy, PartialEq, Debug, Hash, Eq)]
struct GridPos {   
    row: usize,
    col: usize,
}

//////////////////////////////////////////////////////////////////////////////////////
//
// Implementations
//
//////////////////////////////////////////////////////////////////////////////////////
// Grid Implementations
//////////////////////////////////////////////////////////////////////////////////////
impl Grid {
    pub fn new() -> Grid {
        Grid {
            //storage: Arc::new(vec![false; POOL_SIZE]),
            //storage: Arc::new(vec![None; POOL_SIZE]),
            storage: HashMap::new(),
            show_grid_axis: true,
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////////
// GridWidget Implementations
//////////////////////////////////////////////////////////////////////////////////////
impl GridWidget {
    fn new() -> GridWidget {
        GridWidget {
            cell_size: Size {
                width: 0.0,
                height: 0.0,
            },
            color: LIGHT_BLUE,
        }
    }

    fn grid_pos(&self, p: Point) -> Option<GridPos> {
        let w0 = self.cell_size.width;
        let h0 = self.cell_size.height;
        if p.x < 0.0 || p.y < 0.0 || w0 == 0.0 || h0 == 0.0 {
            return None;
        }
        let row = (p.x / w0) as usize;
        let col = (p.y / h0) as usize;
        if row >= GRID_COLUMNS || col >= GRID_ROWS {
            return None;
        }
        Some(GridPos { row, col })
    }
}

impl Widget<State> for GridWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut State, _env: &Env) {
        match event {
            Event::WindowConnected => {
                ctx.request_paint();
            }

            Event::MouseDown(e) => {
                if e.button == MouseButton::Left {
                    let grid_pos_opt = self.grid_pos(e.pos);
                    grid_pos_opt.iter().for_each(|pos| {
                        if data.drawing == Interaction::None {
                            if data.grid.storage.contains_key(pos) {
                                data.grid.storage.remove(pos);
                                data.drawing = Interaction::Erasing
                            } else {
                                data.grid.storage.insert(*pos, GridNodes::Wall);
                                data.drawing = Interaction::Drawing
                            }
                        }

                        let point = Point {
                            x: self.cell_size.width * pos.row as f64,
                            y: self.cell_size.height * pos.col as f64,
                        };
                        let rect = Rect::from_origin_size(point, self.cell_size);
                        //println!("Event - Position - Invalidation Rectangle: {:?}", point);
                        //println!("Event - Size - Invalidation Rectangle: {:?}", rect.size());
                        //println!("Event - Size - Cell: {:?}\n================================", self.cell_size);
                        ctx.request_paint_rect(rect);
                        //ctx.request_paint();
                    });
                }
            }
            Event::MouseUp(e) => {
                if e.button == MouseButton::Left {
                    data.drawing = Interaction::None;
                }
            }
            Event::MouseMove(e) => {
                if data.drawing != Interaction::None {
                    let grid_pos_opt = self.grid_pos(e.pos);
                    grid_pos_opt.iter().for_each(|pos| {
                        //println!("Event Move: {:?}", *pos);
                        if data.drawing == Interaction::Drawing {
                            data.grid.storage.insert(*pos, GridNodes::Wall);
                        } else if data.drawing == Interaction::Erasing {
                            data.grid.storage.remove(pos);
                        }

                        let point = Point {
                            x: self.cell_size.width * pos.row as f64,
                            y: self.cell_size.height * pos.col as f64,
                        };
                        let rect = Rect::from_origin_size(point, self.cell_size);
                        //println!("Event - Position - Invalidation Rectangle: {:?}", point);
                        //println!("Event - Size - Invalidation Rectangle: {:?}", rect.size());
                        //println!("Event - Size - Cell: {:?}\n================================", self.cell_size);
                        ctx.request_paint_rect(rect);
                        //ctx.request_paint();
                    });
                }
            }
            _ => {}
        }
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &State,
        _env: &Env,
    ) {
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &State, _data: &State, _env: &Env) {
        //ctx.request_paint();
    }

    // Maybe the issue when drawing a non square grid
    fn layout(&mut self, _layout_ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &State, _env: &Env,) -> Size {
        let width = bc.max().width;

        Size {
            width: width,
            height: (GRID_ROWS as f64 * width) / GRID_COLUMNS as f64,
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &State, _env: &Env) {
        //Update cell size
        let grid_size: Size = ctx.size();
        
        let cell_size = Size {
            width: grid_size.width.max(grid_size.height) / GRID_COLUMNS.max(GRID_ROWS) as f64,
            height: grid_size.width.max(grid_size.height) / GRID_COLUMNS.max(GRID_ROWS) as f64,
        };
        self.cell_size = cell_size;

        //println!("Cell size: {:?}", cell_size);
        
        // Draw grid cells
        for (cell_pos, cell_type) in data.grid.storage.iter(){
            let point = Point {
                x: cell_size.width * cell_pos.row as f64,
                y: cell_size.height * cell_pos.col as f64,
            };

            let rect = Rect::from_origin_size(point, cell_size);
            // Keep in mind that stroke get added to the size of the existing rectangle
            //ctx.stroke(rect, &Color::AQUA, 5.0);

            if cell_type == &GridNodes::Wall {
                ctx.fill(rect, &Color::BLACK);
            }
        }

        // Draw grid axis
        for row in 1..=GRID_ROWS {
            let from_point = Point {
                x: 0.0,
                y: cell_size.height * row as f64,
            };

            let size = Size::new(ctx.size().width, self.cell_size.height * 0.05);
            let rect = Rect::from_origin_size(from_point, size);
            ctx.fill(rect, &Color::GRAY);
        }

        for column in 1..=GRID_COLUMNS {
            let from_point = Point {
                x: cell_size.width * column as f64,
                y: 0.0,
            };

            let size = Size::new( self.cell_size.width * 0.05, ctx.size().height);
            let rect = Rect::from_origin_size(from_point, size);
            ctx.fill(rect, &Color::GRAY);  
        }

    }

    fn id(&self) -> Option<druid::WidgetId> {
        None
    }

    fn type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
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
    let grid = Grid::new();
    let data = State {
        grid,
        drawing: Interaction::None,
        paused: true,
    };
    AppLauncher::with_window(main_window)
        .configure_env(|env, _| {
            env.set(theme::SELECTION_COLOR, Color::rgb8(0xA6, 0xCC, 0xFF));
            env.set(theme::WINDOW_BACKGROUND_COLOR, Color::WHITE);
            env.set(theme::LABEL_COLOR, Color::BLACK);
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

fn make_ui_simple() -> impl Widget<State> {
    //Flex::column().with_child(GridWidget::new().debug_invalidation())
    Flex::column().with_child(GridWidget::new())
}
