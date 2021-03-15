pub mod distance_heuristics;
pub mod pathfinding_types;

use druid::{Lens, Data, Color, WidgetId};

use crate::{data::pathfinding_types::*, };
use crate::gui::grid_axis_widget::GridWidgetData;

//////////////////////////////////////////////////////////////////////////////////////
// Constants
//////////////////////////////////////////////////////////////////////////////////////
pub const GRID_COLUMNS: usize = 80; 
pub const GRID_ROWS: usize = 30; 
pub const COLOR: Color = Color::BLACK;
pub const BACKGROUND: Color = Color::grey8(23);
pub const GRID_ID: WidgetId = WidgetId::reserved(1);


//////////////////////////////////////////////////////////////////////////////////////
// Structs
//////////////////////////////////////////////////////////////////////////////////////
// Application State
#[derive(Clone, Data, Lens)]
pub struct AppData {
    pub is_paused: bool,
    pub is_running: bool,
    pub updates_per_second: f64,
    pub grid_data: GridWidgetData,    
    pub path_tool: PathAlgorithms,
    pub maze_tool: MazeAlgorithms,
}

impl AppData {
    pub fn iter_interval(&self) -> u64 {
        (1000. / self.updates_per_second) as u64
    }
}