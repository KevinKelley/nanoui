
use {
    ItemState,
        COLD,HOT,ACTIVE,FROZEN,
};
use util::{max};
use oui::geom::{Vec2, Rect};
use oui::item::{Item, ItemImp};
use oui::{
    Tag,
    Handler,
    EventFlags,
        BUTTON0_UP, BUTTON0_DOWN, BUTTON0_HOT_UP,
        BUTTON0_CAPTURE,
        APPEND,
    LayoutFlags,
        LEFT, RIGHT, HFILL,
};


enum MouseCapture {
    IDLE,
    CAPTURE,
}


/// this 'Context' is really "Container with Layout"
/// (in self/smalltalk Morphic there's a World, root container)
/// here context is owner of all contained items; hierarchy is okay
/// but grandchildren and deeper still "live" here.  The 'Item'
/// handle is the identity carrier; ownership is in this Context,
/// references are for short term use.  Linkage between parents and
/// siblings and children are through the 'Item' handle.
///
/// As to the basic UI traits:
/// - Container -- add, remove, iterate children...
/// - Layout -- flags for horz/vert flow, and for rel-to T,R,B,L.
/// - Eventing -- button-1 clicks and mouse-moves are translated into
///   a notion of cold/hot/active/frozen.  There's only ever 1 "hot" item,
///   which is the leaf-node under the cursor (like 'hover').
/// - Paint -- not handled here; simply iterate thru child-hierarchy and
///   and draw each, at its location.
pub struct Context<Wgt> {
    // button state in this frame
    buttons: u64,
    // button state in the previous frame
    last_buttons: u64,

    // where the cursor was at the beginning of the active state
    start_cursor: Vec2,
    // where the cursor was last frame
    last_cursor: Vec2,
    // where the cursor is currently
    cursor: Vec2,

    hot_tag: Tag,
    active_tag: Tag,
    hot_item: Item,
    active_item: Item,
    hot_rect: Rect,
    active_rect: Rect,

    capture: MouseCapture,

    items: Vec<ItemImp<Wgt>>,
}


impl<Wgt> Context<Wgt> {

    pub fn create_context() -> Context<Wgt> {
        Context {
            // button state in this frame
            buttons: 0,
            // button state in the previous frame
            last_buttons: 0,

            // where the cursor was at the beginning of the active state
            start_cursor: Vec2::zero(),
            // where the cursor was last frame
            last_cursor: Vec2::zero(),
            // where the cursor is currently
            cursor: Vec2::zero(),

            hot_tag: -1,
            active_tag: -1,
            hot_item: Item::none(),
            active_item: Item::none(),
            hot_rect: Rect::zero(),
            active_rect: Rect::zero(),

            capture: IDLE,

            items: Vec::new(),
        }
    }

    pub fn set_button(&mut self, button: u64, enabled: bool) {
        let mask = 1u64<<button as uint;
        // set new bit
        self.buttons = if enabled
                {self.buttons | mask}
            else {self.buttons & !mask};
    }

    pub fn get_last_button(&self, button: u64) -> bool {
        self.last_buttons & (1u64<<button as uint) != 0
    }

    pub fn get_button(&self, button: u64) -> bool {
        self.buttons & (1u64<<button as uint) != 0
    }

    pub fn button_pressed(&self, button: u64) -> bool {
        !self.get_last_button(button) && self.get_button(button)
    }

    pub fn button_released(&self, button: u64) -> bool {
        self.get_last_button(button) && !self.get_button(button)
    }

    pub fn set_cursor(&mut self, x: i32, y: i32) {
        self.cursor.x = x;
        self.cursor.y = y;
    }

    pub fn get_cursor(&self) -> Vec2 {
        self.cursor
    }

    pub fn get_cursor_start(&self) -> Vec2 {
        self.start_cursor
    }

    pub fn get_cursor_delta(&self) -> Vec2 {
        Vec2 {
            x: self.cursor.x - self.last_cursor.x,
            y: self.cursor.y - self.last_cursor.y
        }
    }

    pub fn get_cursor_start_delta(&self) -> Vec2 {
        Vec2 {
            x: self.cursor.x - self.start_cursor.x,
            y: self.cursor.y - self.start_cursor.y
        }
    }

    pub fn root(&mut self) -> Item {
        if self.count() == 0 { return Item::none() }
        Item::wrap(0)
    }

    /// return total number of items in this UI context.
    fn count(&self) -> uint {
        self.items.len()
    }

    /// find an Item, and return its Imp
    fn get(&mut self, item: Item) -> &mut ItemImp<Wgt> {
        assert!((item.itemid >= 0) && ((item.itemid as uint) < self.count()));
        self.items.get_mut(item.itemid as uint)
    }

    /// discard everything, reset to initial empty state
    pub fn clear(&mut self) {
        self.items.clear();
        self.hot_item = Item::none();
        self.active_item = Item::none();
    }

    /// create a UI Item, for given Wgt (take ownership of wgt).
    /// Item will still need to be 'append'ed, to some parent Item.
    pub fn item(&mut self, wgt: Wgt) -> Item {
        let idx = self.count();
        let it = Item::wrap(idx as i32);

        let item = ItemImp::new(wgt);
        self.items.push(item);

        return it;
    }

    pub fn notify_item(&mut self, item: Item, event: EventFlags) {
        if !self.get(item).event_flags.contains(event) { return; }
        let handler = self.get(item).handler;
        if handler.is_some() {
            (handler.unwrap())(self, item, event);
        }
    }

    /// add 'child' as last-child of 'item'.
    pub fn append(&mut self, item: Item, child: Item) -> Item {
        assert!(child.valid());
        assert!(self.parent(child).invalid());
        {
            let (new_kid_id, lastkid) = {
                let pparent = self.get(item);
                let nkids = pparent.numkids;
                pparent.numkids += 1;
                (nkids, pparent.lastkid)
            };
            {
                let pchild = self.get(child);
                pchild.parent = item;
                pchild.kidid = new_kid_id;
            }
            if lastkid.invalid() {
                let pparent = self.get(item);
                pparent.firstkid = child;
                pparent.lastkid = child;
            } else {
                self.get(child).previtem = lastkid;
                self.get(lastkid).nextitem = child;
                self.get(item).lastkid = child;
            }
        }
        self.notify_item(item, APPEND);
        return child;
    }

    pub fn set_frozen(&mut self, item: Item, enable: bool) {
        self.get(item).frozen = enable;
    }

    pub fn set_size(&mut self, item: Item, w: u32, h: u32) {
        let pitem = self.get(item);
        pitem.size.x = w as i32;
        pitem.size.y = h as i32;
    }

    pub fn get_width(&mut self, item: Item) -> u32 {
        return self.get(item).size.x as u32;
    }

    pub fn get_height(&mut self, item: Item) -> u32 {
        return self.get(item).size.y as u32;
    }

    pub fn set_layout(&mut self, item: Item, flags: LayoutFlags) {
        self.get(item).layout_flags = flags;
    }

    pub fn get_layout(&mut self, item: Item) -> LayoutFlags {
        return self.get(item).layout_flags;
    }

    pub fn set_margins(&mut self, item: Item, l: i32, t: i32, r: i32, b: i32) {
        let pitem = self.get(item);
        pitem.margins[0] = l;
        pitem.margins[1] = t;
        pitem.margins[2] = r;
        pitem.margins[3] = b;
    }

    pub fn get_margin_left(&mut self, item: Item) -> i32 {
        return self.get(item).margins[0];
    }
    pub fn get_margin_top(&mut self, item: Item) -> i32 {
        return self.get(item).margins[1];
    }
    pub fn get_margin_right(&mut self, item: Item) -> i32 {
        return self.get(item).margins[2];
    }
    pub fn get_margin_down(&mut self, item: Item) -> i32 {
        return self.get(item).margins[3];
    }


    pub fn set_rel_to_left(&mut self, item: Item, other: Item) {
        assert!(!other.valid() || (self.parent(other) == self.parent(item)));
        self.get(item).relto[0] = other;
    }

    pub fn get_rel_to_left(&mut self, item: Item) -> Item {
        return self.get(item).relto[0];
    }

    pub fn set_rel_to_top(&mut self, item: Item, other: Item) {
        assert!(!other.valid() || (self.parent(other) == self.parent(item)));
        self.get(item).relto[1] = other;
    }
    pub fn get_rel_to_top(&mut self, item: Item) -> Item {
        return self.get(item).relto[1];
    }

    pub fn set_rel_to_right(&mut self, item: Item, other: Item) {
        assert!(!other.valid() || (self.parent(other) == self.parent(item)));
        self.get(item).relto[2] = other;
    }
    pub fn get_rel_to_right(&mut self, item: Item) -> Item {
        return self.get(item).relto[2];
    }

    pub fn set_rel_to_down(&mut self, item: Item, other: Item) {
        assert!(!other.valid() || (self.parent(other) == self.parent(item)));
        self.get(item).relto[3] = other;
    }
    pub fn get_rel_to_down(&mut self, item: Item) -> Item {
        return self.get(item).relto[3];
    }

    pub fn get_rect(&mut self, item: Item) -> Rect {
        return self.get(item).rect;
    }

    pub fn get_active_rect(&self) -> Rect {
        return self.active_rect;
    }

    pub fn first_child(&mut self, item: Item) -> Item {
        return self.get(item).firstkid;
    }

    pub fn last_child(&mut self, item: Item) -> Item {
        return self.get(item).lastkid;
    }

    pub fn next_sibling(&mut self, item: Item) -> Item {
        return self.get(item).nextitem;
    }

    pub fn prev_sibling(&mut self, item: Item) -> Item {
        return self.get(item).previtem;
    }

    pub fn parent(&mut self, item: Item) -> Item {
        return self.get(item).parent;
    }


    pub fn get_widget(&mut self, item: Item) -> &mut Wgt {
        return &mut self.get(item).widget;
    }


    pub fn set_tag(&mut self, item: Item, tag: Tag) {
        self.get(item).tag = tag;
        if tag != -1 {
            if tag == self.hot_tag {
                self.hot_item = item;
            }
            if tag == self.active_tag {
                self.active_item = item;
            }
        }
    }

    pub fn get_tag(&mut self, item: Item) -> Tag {
        return self.get(item).tag;
    }

    pub fn set_handler(&mut self, item: Item, handler: Handler<Wgt>, flags: EventFlags) {
        let pitem =self. get(item);
        pitem.handler = handler;
        pitem.event_flags = flags;
    }

    pub fn get_handler(&mut self, item: Item) -> Handler<Wgt> {
        return self.get(item).handler;
    }

    pub fn get_handler_flags(&mut self, item: Item) -> EventFlags {
        return self.get(item).event_flags;
    }

    pub fn get_child_id(&mut self, item: Item) -> i32 {
        return self.get(item).kidid;
    }

    pub fn get_child_count(&mut self, item: Item) -> i32 {
        return self.get(item).numkids;
    }

    pub fn find_item(&mut self, item: Item, x: i32, y: i32, ox: i32, oy: i32) -> Item {
        let mut rect = {
            let pitem = self.get(item);
            if pitem.frozen { return Item::none(); }
            pitem.rect
        };
        let x = x - rect.x;
        let y = y - rect.y;
        let ox = ox + rect.x;
        let oy = oy + rect.y;
        if (x>=0)
        && (y>=0)
        && (x<rect.w)
        && (y<rect.h) {
            let mut kid = self.first_child(item);
            while kid.valid() {
                let best_hit = self.find_item(kid,x,y,ox,oy);
                if best_hit.valid() { return best_hit; }
                kid = self.next_sibling(kid);
            }
            rect.x += ox;
            rect.y += oy;
            self.hot_rect = rect;
            return item;
        }
        return Item::none();
    }

    //static
    pub fn is_active(&self, item: Item) -> bool {
        return self.active_item == item;
    }

    //static
    pub fn is_hot(&self, item: Item) -> bool {
        return self.hot_item == item;
    }

    pub fn get_state(&mut self, item: Item) -> ItemState {
        let hot = self.is_hot(item);
        let active = self.is_active(item);
        let pitem = self.get(item);
        if pitem.frozen {return FROZEN;}
        if active {
            if pitem.event_flags.contains(BUTTON0_CAPTURE|BUTTON0_UP) {return ACTIVE;}
            if pitem.event_flags.contains(BUTTON0_HOT_UP) && hot {
                return ACTIVE;
            }
            return COLD;
        } else if hot {
            return HOT;
        }
        return COLD;
    }


    ///////////////////////////////////////////////////////////////////
    // Internals

    fn compute_chain_size(&mut self, item: Item,
        need_size: &mut i32, hard_size: &mut i32, dim: uint
    ) {
        let wdim = dim+2;
        let mut size = {
            let pitem = self.get(item);
            pitem.rect[wdim] + pitem.margins[dim] + pitem.margins[wdim]
        };
        *need_size = size;
        *hard_size = if self.get(item).size[dim] > 0 {size} else {0};

        self.get(item).visited |= 1<<dim;
        // traverse along left neighbors
        let mut iter = 0u;
        let mut prev = item;
        // FIXME flags
        while ((self.get(prev).layout_flags.bits>>dim) & LEFT.bits) != 0 {
            prev = self.get(prev).relto[dim];
            if prev.invalid() { break };
            let pitem = self.get(prev);
            pitem.visited |= 1<<dim;
            size = pitem.rect[wdim] + pitem.margins[dim] + pitem.margins[wdim];
            *need_size = (*need_size) + size;
            *hard_size = (*hard_size) + (if pitem.size[dim] > 0 {size} else {0});
            iter += 1;
            assert!(iter<1000000); // infinite loop
        }
        // traverse along right neighbors
        let mut iter = 0u;
        let mut next = item;
        // FIXME flags
        while ((self.get(next).layout_flags.bits>>dim) & RIGHT.bits) != 0 {
            next = self.get(next).relto[wdim];
            if next.invalid() { break };
            let pitem = self.get(next);
            pitem.visited |= 1<<dim;    // are we gettin our dim's and wdim's mixed up? idono
            size = pitem.rect[wdim] + pitem.margins[dim] + pitem.margins[wdim];
            *need_size = (*need_size) + size;
            *hard_size = (*hard_size) + (if pitem.size[dim] > 0 {size} else {0});
            iter += 1;
            assert!(iter<1000000); // infinite loop
        }
    }

    fn compute_size_dim(&mut self, item: Item, dim: uint) {
        let wdim = dim+2;
        let mut need_size = 0;
        let mut hard_size = 0;
        let mut kid = self.get(item).firstkid;
        while kid.valid() {
            let visited = self.get(kid).visited;
            if visited & (1<<dim) == 0 {
                let mut ns: i32 = 0;
                let mut hs: i32 = 0;
                self.compute_chain_size(kid, &mut ns, &mut hs, dim);
                need_size = max(need_size, ns);
                hard_size = max(hard_size, hs);
            }
            kid = self.next_sibling(kid);
        }
        let pitem = self.get(item);
        pitem.computed_size[dim] = hard_size;

        if pitem.size[dim] > 0 {
            pitem.rect[wdim] = pitem.size[dim];
        } else {
            pitem.rect[wdim] = need_size;
        }
    }

    fn compute_best_size(&mut self, item: Item, dim: uint) {
        self.get(item).visited = 0;
        // children expand the size
        let mut kid = self.first_child(item);
        while kid.valid() {
            self.compute_best_size(kid, dim);
            kid = self.next_sibling(kid);
        }

        self.compute_size_dim(item, dim);
    }

    fn layout_child_item(&mut self, parent: Item, item: Item, dyncount: &mut i32, dim: uint) {
        //let pitem = self.get(item);

        if self.get(item).visited & (4<<dim) != 0 {return};
        self.get(item).visited |= 4<<dim;

        if self.get(item).size[dim] == 0 {
            *dyncount = (*dyncount)+1;
        }

        let wdim = dim+2;

        let mut x = 0;
        let mut s = self.get(parent).rect[wdim];

        let flags = self.get(item).layout_flags.bits>>dim;
        let flags = LayoutFlags::from_bits(flags).expect("bitfail");
        let hasl = flags.contains(LEFT) && self.get(item).relto[dim].valid();
        let hasr = flags.contains(RIGHT) && self.get(item).relto[wdim].valid();

        if hasl {
            let l = self.get(item).relto[dim];
            self.layout_child_item(parent, l, dyncount, dim);
            let pl = self.get(l);
            x = pl.rect[dim]+pl.rect[wdim]+pl.margins[wdim];
            s -= x;
        }
        if hasr {
            let r = self.get(item).relto[wdim];
            self.layout_child_item(parent, r, dyncount, dim);
            let pr = self.get(r);
            s = pr.rect[dim]-pr.margins[dim]-x;
        }

        match flags & HFILL {
            LEFT => {
                self.get(item).rect[dim] = x+self.get(item).margins[dim];
            }
            RIGHT => {
                self.get(item).rect[dim] = x+s-self.get(item).rect[wdim]-self.get(item).margins[wdim];
            }
            HFILL => {
                if self.get(item).size[dim] > 0 { // hard maximum size; can't stretch
                    if !hasl {
                        self.get(item).rect[dim] = x+self.get(item).margins[dim];
                    }
                    else {
                        self.get(item).rect[dim] = x+s-self.get(item).rect[wdim]-self.get(item).margins[wdim];
                    }
                } else {
                    if true { // !self.get(item).rect[wdim]) {
                        let width = self.get(parent).rect[wdim] - self.get(parent).computed_size[dim];
                        let space = width / (*dyncount);
                        //let rest = width - space*(*dyncount);
                        if !hasl {
                            self.get(item).rect[dim] = x+self.get(item).margins[dim];
                            self.get(item).rect[wdim] = s-self.get(item).margins[dim]-self.get(item).margins[wdim];
                        } else {
                            self.get(item).rect[wdim] = space-self.get(item).margins[dim]-self.get(item).margins[wdim];
                            self.get(item).rect[dim] = x+s-self.get(item).rect[wdim]-self.get(item).margins[wdim];
                        }
                    } else {
                        self.get(item).rect[dim] = x+self.get(item).margins[dim];
                        self.get(item).rect[wdim] = s-self.get(item).margins[dim]-self.get(item).margins[wdim];
                    }
                }
            }
            //default:
            _ /*HCENTER*/ => {
                self.get(item).rect[dim] = x+(s-self.get(item).rect[wdim])/2+self.get(item).margins[dim];
            }
        }
    }

    fn layout_item_dim(&mut self, item: Item, dim: uint) {
        let mut kid = self.get(item).firstkid;
        while kid.valid() {
            //let pkid = self.get(kid);
            let mut dyncount = 0;
            self.layout_child_item(item, kid, &mut dyncount, dim);
            kid = self.next_sibling(kid);
        }
    }

    fn layout_item(&mut self, item: Item, dim: uint) {
        self.layout_item_dim(item, dim);
        let mut kid = self.first_child(item);
        while kid.valid() {
            self.layout_item(kid, dim);
            kid = self.next_sibling(kid);
        }
    }


    pub fn layout(&mut self) {
        if self.count() == 0 { return; }
        let root = self.root();

        // compute widths
        self.compute_best_size(root,0);
        // position root element rect
        self.get(root).rect.x = self.get(root).margins[0];
        self.layout_item(root,0);

        // compute heights
        self.compute_best_size(root,1);
        // position root element rect
        self.get(root).rect.y = self.get(root).margins[1];
        self.layout_item(root,1);
    }

    pub fn process(&mut self) {
        if self.count() == 0 { return; }

        let cursor = self.cursor;
        let root = self.root();
        let hot = self.find_item(root, cursor.x, cursor.y, 0, 0);
        let active = self.active_item;

        match self.capture {
            IDLE => {
                self.start_cursor = cursor;
                if self.get_button(0) {
                    self.hot_item = Item::none();
                    self.active_rect = self.hot_rect;
                    self.active_item = hot;
                    if hot.valid() {
                        self.notify_item(hot, BUTTON0_DOWN);
                    }
                    self.capture = CAPTURE;
                } else {
                    self.hot_item = hot;
                }
            }
            CAPTURE => {
                if !self.get_button(0) {
                    if active.valid() {
                        self.notify_item(active, BUTTON0_UP);
                        if active == hot {
                            self.notify_item(active, BUTTON0_HOT_UP);
                        }
                    }
                    self.active_item = Item::none();
                    self.capture = IDLE;
                } else {
                    if active.valid() {
                        self.notify_item(active, BUTTON0_CAPTURE);
                    }
                    if hot == active {
                        self.hot_item = hot;
                    }
                    else {
                        self.hot_item = Item::none();
                    }
                }
            }
        }
        // self has changed, reset handles to match current state
        self.last_cursor = self.cursor;
        let active = self.active_item;
        let hot = self.hot_item;
        self.hot_tag = if hot.valid() {self.get_tag(hot)} else {0};
        self.active_tag = if active.valid() {self.get_tag(active)} else {0};
    }
}
