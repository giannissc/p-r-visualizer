// TODO Implement infinite zoom and pan functionallity. See scroll example and clipBox documentation
// FIXME Partial repaint
// Handle Tool Chain using commands

// Refactor GridNodeTypes to be more generic. Locked, Weighted, Color, Single/Multiple
/*
pub enum NodeTypes {
    SingleInstanceLocked<Color, Net, String, Weight>,
    MultipleLocked<Color, Net, String, Weight>,
    MultipleUnlocked<Color, Net, String, Weight>
}

pub struct NodeTypesInternal {
    color: COlor,
    net: Net,
    desc: String,
    Weight: u32,
}
*/

use druid::{BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, Lens, LifeCycle, LifeCycleCtx, MouseButton, PaintCtx, RenderContext, Selector, UpdateCtx, Widget, text::Selection};
use druid::{Color, Point, Rect, Size};
use crate::pathfinding_types::*;


pub const TOGGLE_GRID_AXIS: Selector = Selector::new("toggle-grid");
pub const WALL_TOOL: Selector = Selector::new("wall-tool");
pub const START_NODE_TOOL: Selector = Selector::new("start-node-tool");
pub const END_NODE_TOOL: Selector = Selector::new("end-node-tool");
pub const TOGGLE_DRAWING: Selector =  Selector::new("toggle-drawing");

#[derive(Clone, PartialEq, Data)]
enum Interaction {
    None,
    Drawing,
    Erasing,
    //Panning,
    LockedUI,
}


//////////////////////////////////////////////////////////////////////////////////////
//
// Grid Widget
//
//////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, PartialEq, Data, Lens)]
pub struct GridWidget {
    rows: usize,
    columns: usize,
    cell_size: Size,
    drawing: Interaction,
    show_grid_axis: bool,
    color: Color,
    selected_tool: GridNodeType
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
            show_grid_axis: true, // Need to handle this with Commands and Widget ID
            color: color, // TODO Need color array
            selected_tool: GridNodeType::Wall,
        }
    }

    fn grid_pos(&self, p: Point) -> Option<GridNodePosition> {
        let w0 = self.cell_size.width;
        let h0 = self.cell_size.height;
        if p.x < 0.0 || p.y < 0.0 || w0 == 0.0 || h0 == 0.0 {
            return None;
        }
        let col = (p.x / w0) as usize;
        let row = (p.y / h0) as usize;
        if col >= self.columns || row >= self.rows {
            return None;
        }
        Some(GridNodePosition { row, col })
    }

    fn invalidation_area (&self, pos: GridNodePosition) -> Rect{
        let point = Point {
            x: self.cell_size.width * pos.col as f64,
            y: self.cell_size.height * pos.row as f64,
        };
        Rect::from_origin_size(point, self.cell_size)
    }
}

impl Widget<Grid> for GridWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut Grid, _env: &Env) {
        match event {
            Event::WindowConnected => {
                ctx.request_paint();
            }

            Event::Command(cmd) => {
                if cmd.is(WALL_TOOL) {
                    self.selected_tool = GridNodeType::Wall;
                } else if cmd.is(END_NODE_TOOL) {
                    self.selected_tool = GridNodeType::TargetNode(1);
                } else if cmd.is(START_NODE_TOOL){
                    self.selected_tool = GridNodeType::StartNode(1);
                } else if cmd.is(TOGGLE_GRID_AXIS) {
                    self.show_grid_axis = !self.show_grid_axis;
                    ctx.request_paint();
                } else if cmd.is(TOGGLE_DRAWING) {
                    if self.drawing == Interaction::None {
                        self.drawing = Interaction::LockedUI
                    } else if self.drawing == Interaction::LockedUI {
                        self.drawing = Interaction::None
                    }
                }
            }

            Event::MouseDown(e) => {
                if e.button == MouseButton::Left {
                    let grid_pos_opt = self.grid_pos(e.pos);
                    grid_pos_opt.iter().for_each(|pos| {
                        if self.drawing == Interaction::None {
                            if data.storage.contains_key(pos) && data.storage.get(pos) == Some(&GridNodeType::Wall) {
                                data.storage.remove(pos);
                                self.drawing = Interaction::Erasing
                            } else if !data.storage.contains_key(pos) {
                                if self.selected_tool == GridNodeType::Wall {
                                    data.storage.insert(*pos, GridNodeType::Wall);
                                } else if self.selected_tool == GridNodeType::TargetNode(1) {
                                    data.storage.remove(&data.end_node);
                                    ctx.request_paint_rect(self.invalidation_area(data.end_node));
                                    data.end_node = *pos;
                                    data.storage.insert(*pos, GridNodeType::TargetNode(1));
                                } else if self.selected_tool == GridNodeType::StartNode(1) {
                                    data.storage.remove(&data.start_node);
                                    ctx.request_paint_rect(self.invalidation_area(data.start_node));
                                    data.start_node = *pos;
                                    data.storage.insert(*pos, GridNodeType::StartNode(1));
                                }
                                self.drawing = Interaction::Drawing
                            }
                            ctx.request_paint_rect(self.invalidation_area(*pos));
                        }
                    });
                }
            }
            Event::MouseUp(e) => {
                if e.button == MouseButton::Left && self.drawing != Interaction::LockedUI {
                    self.drawing = Interaction::None;
                }
            }
            Event::MouseMove(e) => {
                if self.drawing != Interaction::LockedUI {
                    let grid_pos_opt = self.grid_pos(e.pos);
                    grid_pos_opt.iter().for_each(|pos| {
                        //println!("Event Move: {:?}", *pos);
                        if self.drawing == Interaction::Drawing && !data.storage.contains_key(pos){
                            if self.selected_tool == GridNodeType::Wall {
                                data.storage.insert(*pos, GridNodeType::Wall);
                            } else if self.selected_tool == GridNodeType::TargetNode(1) {
                                data.storage.remove(&data.end_node);
                                ctx.request_paint_rect(self.invalidation_area(data.end_node));
                                data.end_node = *pos;
                                data.storage.insert(*pos, GridNodeType::TargetNode(1));
                            } else if self.selected_tool == GridNodeType::StartNode(1) {
                                data.storage.remove(&data.start_node);
                                ctx.request_paint_rect(self.invalidation_area(data.start_node));
                                data.start_node = *pos;
                                data.storage.insert(*pos, GridNodeType::StartNode(1));
                            }
                        } else if self.drawing == Interaction::Erasing && data.storage.get(pos) == Some(&GridNodeType::Wall) {
                            data.storage.remove(pos);
                        } 

                        ctx.request_paint_rect(self.invalidation_area(*pos));
                    });
                }
            }
            _ => {}
        }
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &Grid, _env: &Env, ) {
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &Grid, data: &Grid, _env: &Env) {
        if data.storage.len() == 2{
            ctx.request_paint();
        }    
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
                x: cell_size.width * cell_pos.col as f64,
                y: cell_size.height * cell_pos.row as f64,
            };

            let rect = Rect::from_origin_size(point, cell_size);
            // Keep in mind that stroke get added to the size of the existing rectangle
            //ctx.stroke(rect, &Color::AQUA, 5.0);

            match cell_type {
                GridNodeType::Wall => ctx.fill(rect, &self.color),
                GridNodeType::StartNode(_) => ctx.fill(rect, &Color::AQUA),
                GridNodeType::TargetNode(_) => ctx.fill(rect, &Color::PURPLE),
                GridNodeType::UnexploredNodes(_) => ctx.fill(rect, &Color::rgb8(255, 191, 0)),
                GridNodeType::ExploredNodes(_) => ctx.fill(rect, &Color::MAROON),
                GridNodeType::ChosenPath(_) => ctx.fill(rect, &Color::GREEN),
                _ => (),
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




