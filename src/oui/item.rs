
use std::mem::zeroed;
use super::{
    Tag,
    Handler,
    EventFlags,
    LayoutFlags
};
use super::geom::{
    Vec2,
    Rect
};


#[deriving(Eq, PartialEq, Clone, Show)]
pub struct Item {
    pub itemid: i32
}

impl Item {
    pub fn wrap(itemid: i32) -> Item { Item { itemid: itemid } }
    pub fn none() -> Item { Item::wrap(-1) }

    pub fn valid(&self) -> bool { self.itemid != -1 }
    pub fn invalid(&self) -> bool { !self.valid() }
}

pub struct ItemImp<Wgt> {
    // declaration independent opaque tag (for persistence)
    pub tag: Tag,

    // event handler
    pub handler: Handler<Wgt>,

    // container structure

    // number of kids
    pub numkids: i32,
    // index of first kid
    pub firstkid: Item,
    // index of last kid
    pub lastkid: Item,

    // child structure

    // parent item
    pub parent: Item,
    // index of kid (this item) relative to parent (what number child am I?)
    pub kidid: i32,
    // index of next sibling with same parent
    pub nextitem: Item,
    // index of previous sibling with same parent
    pub previtem: Item,

    // one or multiple of UIlayoutFlags
    pub layout_flags: LayoutFlags,
    // size
    pub size: Vec2,
    // visited flags for layouting
    pub visited: i32,
    // margin offsets, interpretation depends on flags
    pub margins: [i32, ..4],
    // neighbors to position borders to
    pub relto: [Item, ..4],

    // computed size
    pub computed_size: Vec2,
    // relative rect
    pub rect: Rect,

    // attributes

    pub frozen: bool,

    pub widget: Wgt,

    // a combination of Events
    pub event_flags: EventFlags,
}

impl<Wgt> ItemImp<Wgt> {
    pub fn new(wgt:Wgt) -> ItemImp<Wgt> {
        let mut item: ItemImp<Wgt> = unsafe { zeroed() };
        item.parent = Item::none();
        item.firstkid = Item::none();
        item.lastkid = Item::none();
        item.nextitem = Item::none();
        item.previtem = Item::none();

        item.widget = wgt;

        for i in range(0u, 4u) {
            item.relto[i] = Item::none();
        }
        item
    }
}
