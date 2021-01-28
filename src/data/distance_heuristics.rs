use crate::pathfinding_types::*;
pub enum Heuristics {
    Manhattan,
    Euclidean,
    Octile,
    Chebyshev,
}

impl Heuristics {
    pub fn target_cost(current: GridNodePosition, target: GridNodePosition) -> i64 {
        Heuristics::euclidean_cost(current, target)
    }
    
    fn manhattan_cost(current: GridNodePosition, target: GridNodePosition) -> i64{
        let current_x = current.col as i64;
        let current_y = current.row as i64;
    
        let target_x = target.col as i64;
        let target_y = target.row as i64;
    
        (current_x - target_x).abs() + (current_y - target_y).abs()
    }
    
    fn octile_cost(current: GridNodePosition, target: GridNodePosition) -> i64{
        let current_x = current.col as i64;
        let current_y = current.row as i64;
    
        let target_x = target.col as i64;
        let target_y = target.row as i64;
    
        (current_x - target_x).abs().max((current_y - target_y).abs()) 
    }

    fn chebyshev_cost(current: GridNodePosition, target: GridNodePosition) -> i64{
        let current_x = current.col as i64;
        let current_y = current.row as i64;
    
        let target_x = target.col as i64;
        let target_y = target.row as i64;
    
        (current_x - target_x).abs().max((current_y - target_y).abs())
    }
    
    fn euclidean_cost(current: GridNodePosition, target: GridNodePosition) -> i64{
        let current_x = current.col as i64;
        let current_y = current.row as i64;
    
        let target_x = target.col as i64;
        let target_y = target.row as i64;
    
        (current_x - target_x).abs().pow(2) + (current_y - target_y).abs().pow(2)
    }

    fn hamming_cost(current: GridNodePosition, target: GridNodePosition) -> i64{
        let current_x = current.col as i64;
        let current_y = current.row as i64;
    
        let target_x = target.col as i64;
        let target_y = target.row as i64;
    
        (current_x - target_x).abs().pow(2) + (current_y - target_y).abs().pow(2)
    }

}

