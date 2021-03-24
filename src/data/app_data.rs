use druid::{Lens, Data, Color, WidgetId};

use crate::pathfinding_algorithms::pathfinding_types::{PathAlgorithms, PathfinderConfig};
use crate::maze_generation_algorithms::maze_generation_types::MazeAlgorithms;
use crate::gui::grid_widget::square_grid_widget_data::GridWidgetData;

//////////////////////////////////////////////////////////////////////////////////////
// Constants
//////////////////////////////////////////////////////////////////////////////////////
pub const GRID_COLUMNS: usize = 81; 
pub const GRID_ROWS: usize = 31; 
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
    pub path_config: PathfinderConfig,
    pub maze_tool: MazeAlgorithms,
    pub pathfinder_mode: bool,
}

impl AppData {
    pub fn to_period_milli(&self) -> u64 {
        (1000. / self.updates_per_second) as u64
    }
}