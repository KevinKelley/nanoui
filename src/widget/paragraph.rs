
use nanovg::*;
use util::*;
use draw::draw::Draw;
use draw::geom::Rect;


pub struct Paragraph {
	pub text: String,
	pub bounds: Rect,
	pub mx: f32,
	pub my: f32
}
impl Draw for Paragraph {
	fn draw(&self, vg: &Ctx)
	{
		let text = self.text.as_slice();
	    //let text = "This is longer chunk of text.\n  \n  Would have used lorem ipsum but she    was busy jumping over the lazy dog with the fox and all the men who came to the aid of the party.";

	    let (x,y) = (self.bounds.x(), self.bounds.y());
	    let width = self.bounds.w();
	    let (mx,my) = (self.mx, self.my);

	    let mut y:f32 = y;
	    let mut asc: f32 = 0.0;
	    let mut desc: f32 = 0.0;
	    let mut lineh: f32 = 0.0;
	    let mut gx: f32 = 0.0;
	    let mut gy: f32 = 0.0;
	    let mut gutter:i32 = 0;
	    let mut bounds: [f32, ..4] = [0.0, ..4];

	    vg.save();

	    vg.font_size(18.0);
	    vg.font_face("sans");
	    vg.text_align(LEFT|TOP);
	    vg.text_metrics(&mut asc, &mut desc, &mut lineh);

	    // The text break API can be used to fill a large buffer of rows,
	    // or to iterate over the text just few lines (or just one) at a time.
	    // The "next" variable of the last returned item tells where to continue.
	    let mut start: uint = 0;    // byte pos in utf8 'text' str
	    let end: uint = text.len(); // exclusive
	    let mut lnum = 0;
	    'chunks: loop {
	        let text = text.slice(start, end);
	        let rows = vg.text_break_lines(text, width, 3);
	        let nrows = rows.len();
	        if nrows == 0 { break 'chunks; }

	        for i in range(0, nrows) {
	            let row = &rows[i];
	            let hit: bool = mx > x && mx < (x+width) && my >= y && my < (y+lineh);

	            vg.begin_path();
	            vg.fill_color(rgba(255,255,255, if hit {64} else {16}));
	            vg.rect(x, y, row.width(), lineh);
	            vg.fill();

	            vg.fill_color(rgba(255,255,255,255));
	            let line = text.slice(row.start_index(), row.end_index());
	            vg.text(x, y, line);

	            if hit { // test for mouse-hit and display cursor
	                let mut caretx = if mx < x+row.width()/2.0 { x } else { x+row.width() };
	                let mut px = x;
	                let glyphs = vg.text_glyph_positions(x, y, line);
	                let nglyphs = glyphs.len();
	                for j in range(0, nglyphs) {
	                    let x0 = glyphs[j].x();
	                    let x1 = if j+1 < nglyphs { glyphs[j+1].x() } else { x+row.width() };
	                    let gx = x0 * 0.3 + x1 * 0.7;
	                    if mx >= px && mx < gx {
	                        caretx = glyphs[j].x();
	                    }
	                    px = gx;
	                }
	                vg.begin_path();
	                vg.fill_color(rgba(255,192,0,255));
	                vg.rect(caretx, y, 1.0, lineh);
	                vg.fill();

	                gutter = lnum+1;
	                gx = x - 10.0;
	                gy = y + lineh/2.0;
	            }
	            lnum += 1;
	            y += lineh;
	        }
	        // Keep going...
	        start += rows[nrows-1].next_index();
	    }

	    if gutter > 0 {
	        //char txt[16]; snprintf(txt, sizeof(txt), "%d", gutter);
	        let txt = format!("{}", gutter);
	        vg.font_size(13.0);
	        vg.text_align(RIGHT|MIDDLE);

	        vg.text_bounds(gx,gy, txt.as_slice(), &mut bounds);

	        vg.begin_path();
	        vg.fill_color(rgba(255,192,0,255));
	        vg.rounded_rect(
	            floor(bounds[0]) - 4.0,
	            floor(bounds[1]) - 2.0,
	            floor(bounds[2]-bounds[0]) + 8.0,
	            floor(bounds[3]-bounds[1]) + 4.0,
	           (floor(bounds[3]-bounds[1]) + 4.0) / 2.0 - 1.0);
	        vg.fill();

	        vg.fill_color(rgba(32,32,32,255));
	        vg.text(gx,gy, txt.as_slice());
	    }

	    y += 20.0;

	    vg.font_size(13.0);
	    vg.text_align(LEFT|TOP);
	    vg.text_line_height(1.2);

	    vg.text_box_bounds(x,y,
	        150.0, "Hover your mouse over the text to see calculated caret position.",
	        &mut bounds);

	    // Fade the tooltip out when close to it.
	    gx = abs((mx - (bounds[0]+bounds[2])*0.5) / (bounds[0] - bounds[2]));
	    gy = abs((my - (bounds[1]+bounds[3])*0.5) / (bounds[1] - bounds[3]));
	    let a = clamp( max(gx, gy) - 0.5,  0.0, 1.0);
	    vg.global_alpha(a);

	    vg.begin_path();
	    vg.fill_color(rgba(220,220,220,255));
	    vg.rounded_rect(
	        bounds[0]-2.0,
	        bounds[1]-2.0,
	        floor(bounds[2]-bounds[0])+4.0,
	        floor(bounds[3]-bounds[1])+4.0,
	        3.0);
	    let px = floor((bounds[2]+bounds[0])/2.0);
	    vg.move_to(px,bounds[1] - 10.0);
	    vg.line_to(px+7.0,bounds[1]+1.0);
	    vg.line_to(px-7.0,bounds[1]+1.0);
	    vg.fill();

	    vg.fill_color(rgba(0,0,0,220));
	    vg.text_box(x,y, 150.0, "Hover your mouse over the text to see calculated caret position.");

	    vg.restore();
	}
}
