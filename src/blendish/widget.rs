
use std::gc::Gc;
use std::cell::Cell;  // shared refs to ui-updatable data

pub use {
    ItemState,
        COLD,HOT,ACTIVE,FROZEN,
};

use blendish::*;
use blendish::themed_draw::ThemedDraw;
use blendish::lowlevel_draw::LowLevelDraw;
use draw::corners::corner_flags;
use oui::*;


pub enum Widget {
    Label { iconid:i32, text:String },
    Button { iconid:i32, text:String },
    Check { text:String, option: Gc<Cell<bool>> },
    Radio { iconid:i32, text:String, index: Gc<Cell<i32>> },
    Slider { text:String, progress: Gc<Cell<f32>> },
    Row { pub unused:i8 /*compiler doesn't support empty struct variants*/},
    Column { pub unused:i8 },
    Panel { unused:i8 }
}


// draw item and recurse for its children
pub fn draw_ui(ui: &mut Context<Widget>, vg: &mut ThemedContext, item: Item, x: i32, y: i32) {
    let (x,y,w,h) = {
        let rect = ui.get_rect(item);
        ((rect.x + x) as f32, (rect.y + y) as f32, rect.w as f32, rect.h as f32)
    };

    // OUI extends state, adding a "frozen" which gets dimmed
    let (item_state, frozen) = match ui.get_state(item) {
        COLD => (COLD, false),
        HOT => (HOT, false),
        ACTIVE => (ACTIVE, false),
        _ => (COLD, true)
    };
    if frozen {
        vg.nvg().global_alpha(DISABLED_ALPHA);
    }

    let kidid = ui.get_child_id(item);

    let cornerflags = corner_flags(ui, item);

    match *ui.get_widget(item) {
        Panel { unused:_ } => {
            vg.draw_bevel(x, y, w, h);
        }
        Label { iconid:iconid, text:ref label } => {
            vg.draw_label(x, y, w, h, iconid as u32, label.as_slice());
        }
        Button { iconid:iconid, text:ref label } => {
            vg.draw_tool_button(x, y, w, h,
                cornerflags, item_state,
                iconid as u32, label.as_slice());
        }
        Check { text:ref label, option:option } => {
            let state =
                if option.get() { ACTIVE }
                else { item_state };
            vg.draw_option_button(x, y, w, h, state, label.as_slice());
        }
        Radio { iconid:iconid, text:ref label, index:index } => {
            let state =
                if (*index).get() == kidid { ACTIVE }
                else { item_state };
            vg.draw_radio_button(x, y, w, h,
                cornerflags, state,
                iconid as u32, label.as_slice());
        }
        Slider { text:ref label, progress:progress } => {
            let val = progress.get();
            let val_str = format!("{}", val*100.0);
            vg.draw_slider(x, y, w, h,
                cornerflags, item_state,
                val, label.as_slice(), val_str.as_slice());
        }
        _ => {}
    }

    let mut kid = ui.first_child(item);
    while kid.valid() { // was, > 0 meaning valid and not root ?
        draw_ui(ui, vg, kid, x as i32, y as i32);
        kid = ui.next_sibling(kid);
    }

    if frozen {
        vg.nvg().global_alpha(1.0);  // this item was frozen: restore full alpha
    }
}

///////////////////////////////////////////////////////////////////////
// widget constructors

pub fn label(ui:&mut Context<Widget>, parent: Item, iconid: i32, label: &str)
-> Item
{
    let lbl = Label { iconid:iconid, text:label.to_string() };
    let item = ui.item(lbl);
    ui.set_size(item, 0, WIDGET_HEIGHT);
    ui.append(parent, item);
    return item;
}

pub fn button(ui:&mut Context<Widget>, parent: Item, tag: Tag, iconid: i32, label: &str,
    handler: Handler<Widget>)
-> Item
{
    let btn = Button { iconid:iconid, text:label.to_string() };
    let item = ui.item(btn);
    // set persistent tag for item that is used
    // to track activity over time
    ui.set_tag(item, tag);
    // set size of wiget; horizontal size is dynamic, vertical is fixed
    ui.set_size(item, 0, WIDGET_HEIGHT);
    // attach event handler e.g. demohandler above
    ui.set_handler(item, handler, BUTTON0_DOWN); // HOT_UP
    ui.append(parent, item);
    return item;
}

pub fn check(ui:&mut Context<Widget>, parent: Item, tag: Tag, label: &str,
    option: Gc<Cell<bool>>, handler: Handler<Widget>)
-> Item
{
    let chk = Check { text:label.to_string(), option:option };
    let item = ui.item(chk);
    ui.set_tag(item, tag);
    ui.set_size(item, 0, WIDGET_HEIGHT);
    ui.set_handler(item, handler, BUTTON0_DOWN);
    ui.append(parent, item);
    return item;
}

pub fn slider(ui:&mut Context<Widget>, parent: Item, tag: Tag, label: &str,
    progress: Gc<Cell<f32>>)
-> Item
{
    let sli = Slider { text:label.to_string(), progress:progress };
    let item = ui.item(sli);
    ui.set_tag(item, tag);
    ui.set_size(item, 0, WIDGET_HEIGHT);
    // attach our slider event handler and capture two classes of events
    ui.set_handler(item, Some(sliderhandler), BUTTON0_DOWN|BUTTON0_CAPTURE);
    ui.append(parent, item);
    return item;
}

pub fn radio(ui:&mut Context<Widget>, parent: Item, tag: Tag, iconid: i32, label: &str,
    index: Gc<Cell<i32>>)
-> Item
{
    let rad = Radio { iconid:iconid, text:label.to_string(), index:index };
    let item = ui.item(rad);
    ui.set_tag(item, tag);
    let w = if label.len() == 0 { TOOL_WIDTH } else { 0 };
    ui.set_size(item, w, WIDGET_HEIGHT);
    ui.set_handler(item, Some(radiohandler), BUTTON0_DOWN);
    ui.append(parent, item);
    return item;
}

pub fn panel(ui:&mut Context<Widget>) -> Item
{
    ui.item(Panel{unused:0})
}

pub fn column(ui:&mut Context<Widget>, parent: Item) -> Item
{
    let item = ui.item(Column{unused:0});
    ui.set_handler(item, Some(columnhandler), APPEND);
    ui.append(parent, item);
    return item;
}

pub fn row(ui: &mut Context<Widget>, parent: Item) -> Item
{
    let item = ui.item(Row{unused:0});
    ui.set_handler(item, Some(rowhandler), APPEND);
    ui.append(parent, item);
    return item;
}

pub fn vgroup(ui:&mut Context<Widget>, parent: Item) -> Item
{
    let item = ui.item(Column{unused:0});
    ui.set_handler(item, Some(vgrouphandler), APPEND);
    ui.append(parent, item);
    return item;
}

pub fn hgroup(ui:&mut Context<Widget>, parent: Item) -> Item
{
    let item = ui.item(Row{unused:0});
    ui.set_handler(item, Some(hgrouphandler), APPEND);
    ui.append(parent, item);
    return item;
}



///////////////////////////////////////////////////////////////////////
// handlers

pub fn demohandler(ui: &mut Context<Widget>, item: Item, _event: EventFlags) {
    let tag = ui.get_tag(item);
    let widget = ui.get_widget(item);
    match *widget {
        Button { text: ref mut label, iconid:_ } => {
            println!("clicked: #{} '{}'", tag, label);
        }
        _ => {}
    }
}
pub fn checkhandler(ui: &mut Context<Widget>, item: Item, _event: EventFlags) {
    let tag = ui.get_tag(item);
    let widget = ui.get_widget(item);
    match *widget {
        Check { text: ref mut label, option: option } => {
            println!("clicked: #{} '{}'", tag, label);
            let cell: Gc<Cell<bool>> = option;
            cell.set(!cell.get());
        }
        _ => {}
    }
}
pub fn freezehandler(ui: &mut Context<Widget>, item:Item, event:EventFlags) {
    // "super" call, handles default cell-update-on-click
    checkhandler(ui, item, event);
    // we didn't cache our target anywhere, so go find it.
    // (demo freezes the right column of the row that's above the
    // button that's above the "Freeze" checkbox)
    let tgt = {
        let it = ui.prev_sibling(item);
        let it = ui.prev_sibling(it);
        let tgt = ui.last_child(it);
        tgt
    };

    let tgt_id = ui.get_child_id(tgt);
    let friz = {
        let widget = ui.get_widget(item);
        match *widget {
            Check { text:_, option: option } => {
                option.get()
            }
            _ => { false }
        }
    };

    println!("freezing: #{} to '{}'", tgt_id, friz);
    ui.set_frozen(tgt, friz);
}
// simple logic for a radio button
pub fn radiohandler(ui: &mut Context<Widget>, item: Item, _event: EventFlags) {
    let tag = ui.get_tag(item);
    let kidid = ui.get_child_id(item);
    let widget = ui.get_widget(item);
    match *widget {
        Radio { iconid:_, text: ref mut label, index: index } => {
            println!("clicked: #{} '{}'", tag, label);
            let cell: Gc<Cell<i32>> = index;
            cell.set(kidid);
        }
        _ => {}
    }
}

// simple logic for a slider
// event handler for slider (same handler for all sliders)
pub fn sliderhandler(ui: &mut Context<Widget>, item: Item, event: EventFlags) {
    println!("tag slider #{} event {}", ui.get_tag(item), event);

    // starting offset of the currently active slider
    static mut sliderstart: f32 = 0.0;
    let pos = ui.get_cursor_start_delta();
    let rc = ui.get_rect(item);
    let widget = ui.get_widget(item);
    match event {
        BUTTON0_DOWN => {
            println!("button0 down");
            match *widget {
                Slider { text:_, progress: currval } => {
                    unsafe { sliderstart = currval.get() };
                }
                _ => {}
            }
        }
        BUTTON0_CAPTURE => {
            println!("button0 capture");
            let val = unsafe { sliderstart + (pos.x as f32 / rc.w as f32) };
            let val = clamp(val, 0.0, 1.0);
            match *widget {
                Slider { text:_, progress: currval } => {
                    currval.set(val);
                }
                _ => {}
            }
        }
        _ => { println!("missed a slider event: {}", event) }
    }
}

pub fn columnhandler(ui: &mut Context<Widget>, parent: Item, _event: EventFlags) {
    let item = ui.last_child(parent);
    let last = ui.prev_sibling(item);
    // mark the new item as positioned under the previous item
    ui.set_rel_to_top(item, last);
    // fill parent horizontally, anchor to previous item vertically
    ui.set_layout(item, HFILL|TOP);
    // if not the first item, add a margin of 1
    let gap = if last.invalid() { 0 } else { 1 };
    ui.set_margins(item, 0,gap,0,0);
}
pub fn rowhandler(ui: &mut Context<Widget>, parent: Item, _event: EventFlags) {
    let item = ui.last_child(parent);
    let last = ui.prev_sibling(item);
    ui.set_rel_to_left(item, last);
    if last.valid() {
        ui.set_rel_to_right(last, item);
    }
    ui.set_layout(item, LEFT|RIGHT);
    let gap = if last.invalid() { 0 } else { 8 };
    ui.set_margins(item, gap,0,0,0);
}
pub fn vgrouphandler(ui: &mut Context<Widget>, parent: Item, _event: EventFlags) {
    let item = ui.last_child(parent);
    let last = ui.prev_sibling(item);
    // mark the new item as positioned under the previous item
    ui.set_rel_to_top(item, last);
    // fill horizontally, anchor to previous item vertically
    ui.set_layout(item, HFILL|TOP);
    // if not the first item, add a margin
    let gap = if last.invalid() { 0 } else { -2 };
    ui.set_margins(item, 0,gap,0,0);
}
pub fn hgrouphandler(ui: &mut Context<Widget>, parent: Item, _event: EventFlags) {
    let item = ui.last_child(parent);
    let last = ui.prev_sibling(item);
    ui.set_rel_to_left(item, last);
    if last.valid() {
        ui.set_rel_to_right(last, item);
    }
    ui.set_layout(item, LEFT|RIGHT);
    let gap = if last.invalid() { 0 } else { -1 };
    ui.set_margins(item, gap,0,0,0);
}
// end handlers
///////////////////////////////////////////////////////////////////////
