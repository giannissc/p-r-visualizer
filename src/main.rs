// TODO Implement infinite zoom and pan functionallity. See scroll example and clipBox documentation
// TODO Configure rustfmt and clippy
// FIXME At grid dimensions (where width > height) a stack buffer overrun happens
// FIXME At gird dimensions (where height > width) at certain window sizes additional row are added to the grid (equal to the number of lines height is bigger than width), the pattern is the mirrored and shifted left by one.
// FIXME Removing ctx.stroke shows grid lines on windowed mode but not maximized. 
// FIXME Having ctx.stroke the same colour as ctx.fill still shows a hint of the grid lines
// FIXME Moving the window causes the app to redraw. File a bug for that

// Rust imports
use std::ops::{Index, IndexMut};
use std::sync::Arc;
use std::cmp;

// Druid imports
use druid::widget::{
    Align, Button, Container, CrossAxisAlignment, Flex, Label, List, Padding, RadioGroup, Scroll,
    SizedBox, Split,
};
use druid::{
    theme, AppLauncher, BoxConstraints, Data, Lens, LocalizedString, WidgetExt,
    WindowDesc,
};
use druid::{Color, Point, Rect, Size, im::HashSet};
use druid::{
    Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, MouseButton, PaintCtx, RenderContext,
    UpdateCtx, Widget,
};

//////////////////////////////////////////////////////////////////////////////////////
// Constants
//////////////////////////////////////////////////////////////////////////////////////
const GRID_WIDTH: usize = 50; 
const GRID_HEIGHT: usize = 50; 
const POOL_SIZE: usize = GRID_WIDTH * GRID_HEIGHT; 
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
    storage: Arc<Vec<Option<GridNodes>>>,
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

#[derive(Clone, Data, Copy, PartialEq, Debug)]
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
            storage: Arc::new(vec![None; POOL_SIZE]),
            //storage: Arc::new(vec![false; POOL_SIZE]),
        }
    }
}

impl Index<GridPos> for Grid {
    type Output = Option<GridNodes>;
    fn index(&self, pos: GridPos) -> &Self::Output {
        let idx = pos.row * GRID_WIDTH + pos.col;
        self.storage.index(idx)
    }
}

impl IndexMut<GridPos> for Grid {
    fn index_mut(&mut self, pos: GridPos) -> &mut Self::Output {
        let idx = pos.row * GRID_WIDTH + pos.col;
        Arc::make_mut(&mut self.storage).index_mut(idx)
    }
}

impl PartialEq for Grid {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..POOL_SIZE {
            if self.storage[i as usize] != other.storage[i as usize] {
                return false;
            }
        }
        return true;
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
        if row >= GRID_WIDTH || col >= GRID_HEIGHT {
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
                        println!("Event Down {:?}", pos);
                        let point = Point {
                            x: self.cell_size.width * pos.row as f64,
                            y: self.cell_size.height * pos.col as f64,
                        };
                        let rect = Rect::from_origin_size(point, self.cell_size);
                        if data.drawing == Interaction::None {
                            if data.grid[*pos] == None {
                                data.drawing = Interaction::Drawing
                            } else {
                                data.drawing = Interaction::Erasing
                            }

                        }
                        ctx.request_paint_rect(rect);
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
                        println!("Event Move: {:?}", *pos);
                        let point = Point {
                            x: self.cell_size.width * pos.row as f64,
                            y: self.cell_size.height * pos.col as f64,
                        };
                        let rect = Rect::from_origin_size(point, self.cell_size);
                        if data.drawing == Interaction::Drawing {
                            data.grid[*pos] = Some(GridNodes::Wall);
                        } else if data.drawing == Interaction::Erasing {
                            data.grid[*pos] = None;
                        }
                        
                        ctx.request_paint_rect(rect);
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

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &State, _data: &State, _env: &Env) {
        //ctx.request_paint();
    }

    // Maybe the issue when drawing a non square grid
    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &State,
        _env: &Env,
    ) -> Size {
        let max_size = bc.max();
        let min_side = max_size.height.min(max_size.width);
        Size {
            width: min_side,
            height: min_side,
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &State, _env: &Env) {
        //Update cell size
        let grid_size: Size = ctx.size();
        
        let cell_size = Size {
            width: grid_size.width.min(grid_size.height) / GRID_WIDTH.min(GRID_HEIGHT) as f64,
            height: grid_size.width.min(grid_size.height) / GRID_WIDTH.min(GRID_HEIGHT) as f64,
        };
        self.cell_size = cell_size;
        println!("Cell size: {:?}", cell_size);

        // Update drawing area size
        let paint_rect = ctx.region().to_rect();

        //Update row, columns ranges
        let grid_pos_opt: GridPos = self.grid_pos(paint_rect.origin()).unwrap();
        let from_row = grid_pos_opt.row;
        let from_col = grid_pos_opt.col;

        let to_grid_pos = self
            .grid_pos(Point::new(paint_rect.max_x(), paint_rect.max_y()))
            .unwrap_or(GridPos {
                col: GRID_WIDTH - 1,
                row: GRID_HEIGHT - 1,
            });
        let to_row = to_grid_pos.row;
        let to_col = to_grid_pos.col;

        //println!("Paint from row: {:?} to row {:?}", from_row, to_row);
        //println!("Paint from col: {:?} to col {:?}", from_col, to_col);

        // Draw grid

        for row in from_row..=to_row {
            for col in from_col..=to_col {
                let point = Point {
                    x: cell_size.width * row as f64,
                    y: cell_size.height * col as f64,
                };
                let rect = Rect::from_origin_size(point, cell_size);
                ctx.stroke(rect, &Color::BLACK, 1.0);

                //println!("Creating point ({:?},{:?})", row, col);

                let grid_pos_opt = GridPos { row, col };

                if data.grid[grid_pos_opt] == Some(GridNodes::Wall) {
                    ctx.fill(rect, &Color::BLACK);
                } else {
                    ctx.fill(rect, &LIGHT_BLUE);
                }

                //println!("Painting point ({:?},{:?})", row, col);
            }
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
    let main_window = WindowDesc::new(make_ui_simple)
        .window_size((1000.0, 600.0))
        .title(LocalizedString::new("Placement & Routing Experiments"));
    let mut grid = Grid::new();
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
    Flex::column().with_child(GridWidget::new().debug_invalidation())
    //Flex::column().with_child(GridWidget::new())
}
