// TODO Implement infinite zoom and pan functionallity. See scroll example and clipBox documentation
// FIXME Partial repaint
use druid::{
    Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, MouseButton, PaintCtx, RenderContext,
    UpdateCtx, Widget, BoxConstraints, Data, Lens
};

use druid::{Color, Point, Rect, Size, im::HashMap};

#[derive(Clone, PartialEq, Data)]
pub enum Interaction {
    None,
    Drawing,
    Erasing,
    //Panning,
    //Locked,
}

#[derive(Clone, Data, Copy, PartialEq, Debug, Hash, Eq)]
struct GridPos {   
    row: usize,
    col: usize,
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

#[derive(Clone, PartialEq, Data, Lens)]
pub struct Grid {
    storage: HashMap<GridPos, GridNodes>,
}

#[derive(Clone, PartialEq, Data, Lens)]
pub struct GridWidget {
    rows: usize,
    columns: usize,
    cell_size: Size,
    drawing: Interaction,
    show_grid_axis: bool,
    color: Color,    
}

//////////////////////////////////////////////////////////////////////////////////////
//
// Implementations
//
//////////////////////////////////////////////////////////////////////////////////////
// GridWidget Implementations
//////////////////////////////////////////////////////////////////////////////////////
impl Grid {
    pub fn new() -> Grid {
        Grid {
            storage: HashMap::new(),
        }
    }

    pub fn clear(&mut self){
        self.storage.clear();
    }
}
impl GridWidget {
    pub fn new(color: Color, rows:usize, columns:usize) -> Self {
        GridWidget {
            rows: rows,
            columns: columns,
            cell_size: Size {
                width: 0.0,
                height: 0.0,
            },
            drawing: Interaction::None,
            show_grid_axis: true,
            color: color,
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
        if row >= self.columns || col >= self.rows {
            return None;
        }
        Some(GridPos { row, col })
    }
}

impl Widget<Grid> for GridWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut Grid, _env: &Env) {
        match event {
            Event::WindowConnected => {
                ctx.request_paint();
            }

            Event::MouseDown(e) => {
                if e.button == MouseButton::Left {
                    let grid_pos_opt = self.grid_pos(e.pos);
                    grid_pos_opt.iter().for_each(|pos| {
                        if self.drawing == Interaction::None {
                            if data.storage.contains_key(pos) {
                                data.storage.remove(pos);
                                self.drawing = Interaction::Erasing
                            } else {
                                data.storage.insert(*pos, GridNodes::Wall);
                                self.drawing = Interaction::Drawing
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
                    self.drawing = Interaction::None;
                }
            }
            Event::MouseMove(e) => {
                if self.drawing != Interaction::None {
                    let grid_pos_opt = self.grid_pos(e.pos);
                    grid_pos_opt.iter().for_each(|pos| {
                        //println!("Event Move: {:?}", *pos);
                        if self.drawing == Interaction::Drawing {
                            data.storage.insert(*pos, GridNodes::Wall);
                        } else if self.drawing == Interaction::Erasing {
                            data.storage.remove(pos);
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

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &Grid, _env: &Env, ) {
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &Grid, _data: &Grid, _env: &Env) {
        //ctx.request_paint();
    }

    // Maybe the issue when drawing a non square grid
    fn layout(&mut self, _layout_ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &Grid, _env: &Env,) -> Size {
        let width = bc.max().width;

        Size {
            width: width,
            height: (self.rows as f64 * width) / self.columns as f64,
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &Grid, _env: &Env) {
        //Update cell size
        let grid_size: Size = ctx.size();
        
        let cell_size = Size {
            width: grid_size.width.max(grid_size.height) / self.columns.max(self.rows) as f64,
            height: grid_size.width.max(grid_size.height) / self.columns.max(self.rows) as f64,
        };
        self.cell_size = cell_size;

        //println!("Cell size: {:?}", cell_size);
        
        // Draw grid cells
        for (cell_pos, cell_type) in data.storage.iter(){
            let point = Point {
                x: cell_size.width * cell_pos.row as f64,
                y: cell_size.height * cell_pos.col as f64,
            };

            let rect = Rect::from_origin_size(point, cell_size);
            // Keep in mind that stroke get added to the size of the existing rectangle
            //ctx.stroke(rect, &Color::AQUA, 5.0);

            if cell_type == &GridNodes::Wall {
                ctx.fill(rect, &self.color);
            }
        }

        // Draw grid axis

        if self.show_grid_axis {
            for row in 0..=self.rows {
                let from_point = Point {
                    x: 0.0,
                    y: cell_size.height * row as f64,
                };
    
                let size = Size::new(ctx.size().width, self.cell_size.height * 0.05);
                let rect = Rect::from_origin_size(from_point, size);
                ctx.fill(rect, &Color::GRAY);
            }
    
            for column in 0..=self.columns {
                let from_point = Point {
                    x: cell_size.width * column as f64,
                    y: 0.0,
                };
    
                let size = Size::new( self.cell_size.width * 0.05, ctx.size().height);
                let rect = Rect::from_origin_size(from_point, size);
                ctx.fill(rect, &Color::GRAY);  
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