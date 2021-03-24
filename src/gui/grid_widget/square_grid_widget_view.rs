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

use druid::{BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, Lens, LifeCycle, LifeCycleCtx, MouseButton, PaintCtx, RenderContext, UpdateCtx, Widget, im::Vector};
use druid::{Color, Point, Rect, Size};
use log::{debug, info};
use super::square_grid_widget_data::*;
use druid_color_thesaurus::*;

//////////////////////////////////////////////////////////////////////////////////////
//
// Grid Widget
//
//////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, PartialEq, Data, Lens)]
pub struct GridWidget {
    max_rows: usize,
    max_columns: usize,
    min_cell_size: Size,
    visible_rows: usize,
    visible_columns: usize,
    chosen_cell_size: Size,
    left_corner_point: GridNodePosition,
    color: Color,
}

impl GridWidget {
    pub fn new(color: Color, rows:usize, columns:usize, cell_size: Size) -> Self {
        GridWidget {
            max_rows: rows,
            max_columns: columns,
            min_cell_size: cell_size,
            visible_columns: columns,
            visible_rows: rows,
            chosen_cell_size: Size {
                width: 0.0,
                height: 0.0,
            },
            left_corner_point: GridNodePosition{row:0, col:0},
            color: color, // TODO Need color array
        }
    }

    fn grid_pos(&self, p: Point) -> Option<GridNodePosition> {
        let w0 = self.chosen_cell_size.width;
        let h0 = self.chosen_cell_size.height;
        if p.x < 0.0 || p.y < 0.0 || w0 == 0.0 || h0 == 0.0 {
            return None;
        }
        let col = (p.x / w0) as usize;
        let row = (p.y / h0) as usize;
        if col >= self.max_columns || row >= self.max_rows {
            return None;
        }
        Some(GridNodePosition { row, col })
    }

    pub fn invalidation_area (&self, pos: GridNodePosition) -> Rect{
        let point = Point {
            x: self.chosen_cell_size.width * pos.col as f64,
            y: self.chosen_cell_size.height * pos.row as f64,
        };
        Rect::from_origin_size(point, self.chosen_cell_size)
    }
}

impl Widget<GridWidgetData> for GridWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut GridWidgetData, _env: &Env) {
        match event {
            Event::Command(cmd) => {
                if cmd.is(LOCK_DRAWING) {
                    data.interaction_state = Interaction::LockedUI
                }else if cmd.is(UNLOCK_DRAWING) {
                    data.interaction_state = Interaction::None
                } else if cmd.is(RESET) {
                    data.grid.clear_paths();
                } else if cmd.is(CLEAR_STORE) {
                    data.grid.clear_store();
                }
            }

            Event::MouseDown(e) => {
                if e.button == MouseButton::Left {
                    let grid_pos_opt = self.grid_pos(e.pos);
                    grid_pos_opt.iter().for_each(|pos| {

                        if data.interaction_state == Interaction::None {
                            if data.selected_tool == GridNodeType::Empty {
                                data.grid.remove_node(pos);
                            } else {
                                if data.selected_tool == GridNodeType::TargetNode(data.selected_net) || data.selected_tool == GridNodeType::StartNode(data.selected_net) || data.grid.get_item(pos) == Some(&GridNodeType::ChosenPath(data.selected_net)) {
                                    ctx.submit_command(RESET);
                                }

                                data.grid.add_node(pos, data.selected_tool, data.selected_net);
                            }

                            data.interaction_state = Interaction::Drawing;
                        }
                    });
                }
            }
            Event::MouseUp(e) => {
                if e.button == MouseButton::Left && data.interaction_state != Interaction::LockedUI {
                    data.interaction_state = Interaction::None;
                }
            }
            Event::MouseMove(e) => {
                if data.interaction_state != Interaction::LockedUI && data.interaction_state != Interaction::None {
                    let grid_pos_opt = self.grid_pos(e.pos);
                    grid_pos_opt.iter().for_each(|pos| {
                       //debug!("Event Move: {:?}", *pos);

                        if data.interaction_state == Interaction::Drawing {
                            if data.selected_tool == GridNodeType::Empty {
                                data.grid.remove_node(pos);
                            } else {
                                if data.selected_tool == GridNodeType::TargetNode(data.selected_net) || data.selected_tool == GridNodeType::StartNode(data.selected_net) || data.grid.get_item(pos) == Some(&GridNodeType::ChosenPath(data.selected_net)) {
                                    ctx.submit_command(RESET);
                                }
                                
                                data.grid.add_node(pos, data.selected_tool, data.selected_net);
                            }   
                        }
                        //debug!("Request rectange repaint");
                    });
                }
            }
            _ => {}
        }
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &GridWidgetData, _env: &Env, ) {
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &GridWidgetData, data: &GridWidgetData, _env: &Env) {
        //debug!("Running grid widget update method");
        //debug!("Difference: {:?}", data.grid.get_storage().difference(old_data.grid.get_storage()));

        if data.show_grid_axis != old_data.show_grid_axis {
            //debug!("Painting the whole window on grid axis change");
            ctx.request_paint();
        } else {
            for cell in data.grid.get_additions().iter() {
                ctx.request_paint_rect(self.invalidation_area(*cell));

            }

            for cell in data.grid.get_deletions().iter() {
                ctx.request_paint_rect(self.invalidation_area(*cell));
            }

            ctx.submit_command(CLEAR_STORE);
        }
    }

    // Maybe the issue when drawing a non square grid
    fn layout(&mut self, _layout_ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &GridWidgetData, _env: &Env,) -> Size {
        let width = bc.max().width;
        let height = bc.max().height; 
        //debug!("Box constraints width: {:?}", bc.max().width);
        //debug!("Box constraints height: {:?}", bc.max().height);

        Size {
            width: width,
            height: height,
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &GridWidgetData, _env: &Env) {
        //debug!("Running paint method");
        //Update cell size
        let screen_space: Size = ctx.size();
        //debug!("Screen space: {:?}", ctx.size());

        let width_sized_cell = Size {
            width: screen_space.width/self.max_columns as f64,
            height: screen_space.width/self.max_columns as f64,
        };

        let height_sized_cell =Size {
            width: screen_space.height/self.max_rows as f64,
            height: screen_space.height/self.max_rows as f64,
        };
        
        self.visible_rows = (screen_space.height / width_sized_cell.height).ceil() as usize;
        self.visible_columns = (screen_space.width/ height_sized_cell.width).ceil() as usize;
        self.chosen_cell_size = self.min_cell_size;

        
        if self.visible_rows > self.max_rows || self.visible_columns > self.max_columns {
            let row_diff = self.visible_rows as i32 - self.max_rows as i32;
            let col_diff = self.visible_columns as i32 - self.max_columns as i32;
            
            if row_diff > col_diff {
                // Calculate minimum cell size to have all columns
                self.chosen_cell_size = height_sized_cell;
                self.visible_rows = self.max_rows;
                self.visible_columns = (screen_space.width / self.chosen_cell_size.width).ceil() as usize;
            } else {
                // Calculate minimum cell size to have all columns
                self.chosen_cell_size = width_sized_cell;
                self.visible_rows = (screen_space.height / self.chosen_cell_size.height).ceil() as usize;
                self.visible_columns = self.max_columns;
            }
        }
        
        if self.chosen_cell_size.height < self.min_cell_size.height {
            self.chosen_cell_size = self.min_cell_size;
        }
        
        //debug!("Visible rows: {:?}", self.visible_rows);
        //debug!("Max rows: {:?}", self.max_rows);
        //debug!("Visible columns: {:?}", self.visible_columns);
        //debug!("Max column:  {:?}", self.max_columns);
        //debug!("Chosen cell size: {:?}", self.chosen_cell_size);
        //debug!("Minimum cell size: {:?}", self.min_cell_size);        
        
        // Draw grid cells

        // Calculate area to render
        let mut paint_rectangles: Vector<Rect> = Vector::new();

        for paint_rect in ctx.region().rects().iter() {
            paint_rectangles.push_front(*paint_rect);
        }

        for paint_rect in paint_rectangles.iter(){
            let from_grid_pos: GridNodePosition = self.grid_pos(paint_rect.origin()).unwrap();
            let from_row = from_grid_pos.row;
            let from_col = from_grid_pos.col;
        
            let to_grid_pos = self
            .grid_pos(Point::new(paint_rect.max_x(), paint_rect.max_y()))
            .unwrap_or(GridNodePosition {
                col: self.visible_columns - 1,
                row: self.visible_rows - 1,
            });
            let to_row = to_grid_pos.row;
            let to_col = to_grid_pos.col;
            
            //debug!("Bounding box with origin {:?} and dimensions {:?} × {:?}", paint_rect.origin(), paint_rect.width(), paint_rect.height());

            let invalidation_rectangles = ctx.region().rects();
            //debug!("Paint from row: {:?} to row {:?}", from_row, to_row);
            //debug!("Paint from col: {:?} to col {:?}", from_col, to_col);

            // Partial Area Paint Logic

            for row in from_row..=to_row {
                for col in from_col..=to_col {
                    let point = Point {
                        x: self.chosen_cell_size.width * col as f64,
                        y: self.chosen_cell_size.height * row as f64,
                    };
                    let rect = Rect::from_origin_size(point, self.chosen_cell_size);

                    let grid_pos = GridNodePosition { row, col };

                    match data.grid.get_item(&grid_pos).unwrap_or(&GridNodeType::Empty) {
                        GridNodeType::Wall => ctx.fill(rect, &black::ONYX),
                        GridNodeType::StartNode(_) => ctx.fill(rect, &blue::ARGENTINIAN_BLUE),
                        GridNodeType::TargetNode(_) => ctx.fill(rect, &purple::PURPUREUS),
                        GridNodeType::UnexploredNodes(_) => ctx.fill(rect, &yellow::YELLOW_AMBER),
                        GridNodeType::ExploredNodes(_) => ctx.fill(rect, &brown::MAROON),
                        GridNodeType::ChosenPath(_) => ctx.fill(rect, &green::PERSIAN_GREEN),
                        _ => (),
                    }


                }
            }

            // Draw grid axis

            if data.show_grid_axis {
                for row in from_row..=to_row {
                    let from_point = Point {
                        x: 0.0,
                        y: self.chosen_cell_size.height * row as f64,
                    };
        
                    let size = Size::new(ctx.size().width, self.chosen_cell_size.height * 0.05);
                    let rect = Rect::from_origin_size(from_point, size);
                    ctx.fill(rect, &Color::GRAY);
                }
        
                for col in from_col..=to_col {
                    let from_point = Point {
                        x: self.chosen_cell_size.width * col as f64,
                        y: 0.0,
                    };

                    let height = self.visible_rows as f64 * self.chosen_cell_size.height;
        
                    let size = Size::new( self.chosen_cell_size.width * 0.05, height);
                    let rect = Rect::from_origin_size(from_point, size);
                    ctx.fill(rect, &Color::GRAY);  
                }
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