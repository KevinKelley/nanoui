
use nanovg::{Ctx, Image, NOREPEAT};

/// width of icon sheet
pub static ICON_SHEET_WIDTH: u32 = 602;
/// height of icon sheet
pub static ICON_SHEET_HEIGHT: u32 = 640;
/// gridsize of icon sheet in both dimensions
pub static ICON_SHEET_GRID: u32 = 21;
/// offset of first icon tile relative to left border
pub static ICON_SHEET_OFFSET_X: u32 = 5;
/// offset of first icon tile relative to top border
pub static ICON_SHEET_OFFSET_Y: u32 = 10;
/// resolution of single icon
pub static ICON_SHEET_RES: u32 = 16;


// build an icon ID from two coordinates into the icon sheet, where
// (0,0) designates the upper-leftmost icon, (1,0) the one right next to it,
// and so on.
pub fn ICONID(x: u8, y: u8) -> u16 { x as u16 | (y as u16 << 8) }


// FIXME need Some<iconid> apparently (seems to use -1 for no-icon)
pub fn icon_id(x:u8, y:u8) -> i32 { ICONID(x, y) as i32 }
pub fn no_icon() -> i32 { -1 }
//struct IconID(i32);
// let iconid = IconID(icon_id(x, y)); let IconID(id) = iconid;


    /// Draw an icon with (x, y) as its upper left coordinate; the iconid selects
    /// the icon from the sheet; use the ICONID macro to build icon IDs.
    pub fn draw_icon(vg: &mut Ctx, x: f32, y: f32, icons: &Image, iconid: u32)
    {
        //let icons = self.theme().icon_image;
        //if (icons < 0) {return}  // no icons loaded

        if iconid == -1 as u32 { return; }

        let ix = iconid & 0xff;
        let iy = (iconid>>8) & 0xff;
        let u = (ICON_SHEET_OFFSET_X + ix*ICON_SHEET_GRID) as f32;
        let v = (ICON_SHEET_OFFSET_Y + iy*ICON_SHEET_GRID) as f32;

        let res = ICON_SHEET_RES as f32;
        vg.begin_path();
        vg.rect(x, y, res, res);
        vg.fill_paint(
            vg.image_pattern(x-u, y-v,
                ICON_SHEET_WIDTH as f32,
                ICON_SHEET_HEIGHT as f32,
                0.0, icons, NOREPEAT, 1.0)
        );
        vg.fill();
    }
