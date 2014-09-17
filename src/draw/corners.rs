
use oui::{Context, Item};
use blendish::widget::{Widget, Row, Column};

/// flags indicating which corners are sharp (for grouping widgets)
bitflags!(
    flags CornerFlags: u32 {
        // all corners are round
        static CORNER_NONE         = 0,
        // sharp top left corner
        static CORNER_TOP_LEFT     = 1,
        // sharp top right corner
        static CORNER_TOP_RIGHT    = 2,
        // sharp bottom right corner
        static CORNER_DOWN_RIGHT   = 4,
        // sharp bottom left corner
        static CORNER_DOWN_LEFT    = 8,
        // all corners are sharp;
        // you can invert a set of flags using ^= BND_CORNER_ALL
        static CORNER_ALL          = 0xF,
        // top border is sharp
        static CORNER_TOP          = 3,
        // bottom border is sharp
        static CORNER_DOWN         = 0xC,
        // left border is sharp
        static CORNER_LEFT         = 9,
        // right border is sharp
        static CORNER_RIGHT        = 6
    }
)

////////////////////////////////////////////////////////////////////////////////
// calculate which corners are sharp for an item, depending on whether
// the container the item is in has negative spacing, and the item
// is first or last element in a sequence of 2 or more elements.
pub fn corner_flags(ui: &mut Context<Widget>, item: Item) -> CornerFlags {
    let parent = ui.parent(item);
    if parent.invalid() { return CORNER_NONE };
    let numsibs = ui.get_child_count(parent);
    if numsibs < 2 { return CORNER_NONE; }
    let kidid = ui.get_child_id(item);
    let widget = ui.get_widget(parent);
    match *widget {
        Column { unused:_ } => {
            // first child, sharp corners down
            if kidid == 0 { return CORNER_DOWN; }
            // last child, sharp corners up
            else if kidid == numsibs-1 { return CORNER_TOP; }
            // middle child, sharp everywhere
            else { return CORNER_ALL; }
        }
        Row { unused: _ } => {
            // first child, sharp right
            if kidid == 0 { return CORNER_RIGHT; }
            // last child, sharp left
            else if kidid == numsibs-1 { return CORNER_LEFT; }
            // middle child, sharp all
            else { return CORNER_ALL; }
        }
        _ => {}
    };
    return CORNER_NONE;
}
