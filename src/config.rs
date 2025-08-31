use gtk4_layer_shell::Edge;

pub const BAR_HEIGHT: i32 = 400;
pub const BAR_WIDTH: i32 = 700;
pub const PEEK: i32 = 2;
pub const ANIM_MS: u32 = 120;

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum BarPos {
    Top,
    Bottom,
}

pub const BAR_POS: BarPos = BarPos::Bottom;

pub fn bar_edge() -> Edge {
    match BAR_POS {
        BarPos::Top => Edge::Top,
        BarPos::Bottom => Edge::Bottom,
    }
}

