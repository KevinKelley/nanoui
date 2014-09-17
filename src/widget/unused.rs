use std::num::{pow};
use nanovg::*;
use util::*;
use draw::draw::Draw;
use draw::icons::*;
use draw::geom::{Rect};

fn draw_eyes(vg: &Ctx, x: f32,
            y: f32, w: f32,
            h: f32, mx: f32,
            my: f32, t: f32)
{
	let ex = w *0.23;
	let ey = h * 0.5;
	let lx = x + ex;
	let ly = y + ey;
	let rx = x + w - ex;
	let ry = y + ey;
	let br = min(ex, ey) * 0.5;
	let blink: f32 = 1.0 - pow(sin(t*0.5),200)*0.8;

	let bg = vg.linear_gradient(x,y+h*0.5,x+w*0.1,y+h, rgba(0,0,0,32), rgba(0,0,0,16));
	vg.begin_path();
	vg.ellipse(lx+3.0,ly+16.0, ex,ey);
	vg.ellipse(rx+3.0,ry+16.0, ex,ey);
	vg.fill_paint(bg);
	vg.fill();

	let shadow = vg.linear_gradient(x,y+h*0.25,x+w*0.1,y+h, rgba(220,220,220,255), rgba(128,128,128,255));
	vg.begin_path();
	vg.ellipse(lx,ly, ex,ey);
	vg.ellipse(rx,ry, ex,ey);
	vg.fill_paint(shadow);
	vg.fill();

	let mut dx = (mx - rx) / (ex * 10.0);
	let mut dy = (my - ry) / (ey * 10.0);
	let mut d = sqrt(dx*dx+dy*dy);
	if d > 1.0 {
		dx /= d; dy /= d;
	}
	dx *= ex*0.4;
	dy *= ey*0.5;
	vg.begin_path();
	vg.ellipse(lx+dx,ly+dy+ey*0.25*(1.0-blink), br,br*blink);
	vg.fill_color(rgba(32,32,32,255));
	vg.fill();

	dx = (mx - rx) / (ex * 10.0);
	dy = (my - ry) / (ey * 10.0);
	d = sqrt(dx*dx+dy*dy);
	if d > 1.0 {
		dx /= d; dy /= d;
	}
	dx *= ex*0.4;
	dy *= ey*0.5;
	vg.begin_path();
	vg.ellipse(rx+dx,ry+dy+ey*0.25*(1.0-blink), br,br*blink);
	vg.fill_color(rgba(32,32,32,255));
	vg.fill();

	let lgloss = vg.radial_gradient(lx-ex*0.25,ly-ey*0.5, ex*0.1,ex*0.75, rgba(255,255,255,128), rgba(255,255,255,0));
	vg.begin_path();
	vg.ellipse(lx,ly, ex,ey);
	vg.fill_paint(lgloss);
	vg.fill();

	let rgloss = vg.radial_gradient(rx-ex*0.25,ry-ey*0.5, ex*0.1,ex*0.75, rgba(255,255,255,128), rgba(255,255,255,0));
	vg.begin_path();
	vg.ellipse(rx,ry, ex,ey);
	vg.fill_paint(rgloss);
	vg.fill();
}


pub struct SearchBox {
	pub text: String,
	pub bounds: Rect
}
impl Draw for SearchBox {
	fn draw(&self, vg: &Ctx)
	{
		let text = self.text.as_slice();
		let x = self.bounds.x();
		let y = self.bounds.y();
		let w = self.bounds.w();
		let h = self.bounds.h();

		let cornerRadius = h/2.0 - 1.0;

		// Edit
		let bg = vg.box_gradient(x,y+1.5, w,h, h/2.0,5.0, rgba(0,0,0,16), rgba(0,0,0,92));
		vg.begin_path();
		vg.rounded_rect(x,y, w,h, cornerRadius);
		vg.fill_paint(bg);
		vg.fill();

	  /*	vg.begin_path();
		vg.rounded_rect(x+0.5,y+0.5, w-1,h-1, cornerRadius-0.5);
		vg.stroke_color(rgba(0,0,0,48));
		vg.stroke();*/

		vg.font_size(h*1.3);
		vg.font_face("icons");
		vg.fill_color(rgba(255,255,255,64));
		vg.text_align(CENTER|MIDDLE);
		vg.text(x+h*0.55, y+h*0.55, ICON_SEARCH.as_slice());

		vg.font_size(20.0);
		vg.font_face("sans");
		vg.fill_color(rgba(255,255,255,32));

		vg.text_align(LEFT|MIDDLE);
		vg.text(x+h*1.05,y+h*0.5,text);

		vg.font_size(h*1.3);
		vg.font_face("icons");
		vg.fill_color(rgba(255,255,255,32));
		vg.text_align(CENTER|MIDDLE);
		vg.text(x+w-h*0.55, y+h*0.55, ICON_CIRCLED_CROSS.as_slice());
	}
}



//fn draw_thumbnails(vg: &Ctx, x: f32, y: f32, w: f32, h: f32,
//                  images: [i32, ..12], nimages: uint, t: f32)
//{
//	let cornerRadius = 3.0;
//
//	let thumb: f32 = 60.0;
//	let arry : f32 = 30.5;
//	let stackh: f32 = (nimages/2) as f32 * (thumb+10.0) + 10.0;
//	let u : f32 = (1.0+cos(t*0.5))*0.5;
//	let u2: f32 = (1.0-cos(t*0.2))*0.5;
//
//	vg.save();
//	//	vg.clear_state();
//
//	// Drop shadow
//	let mut shadowPaint = vg.box_gradient(x,y+4.0, w,h, cornerRadius*2.0, 20.0, rgba(0,0,0,128), rgba(0,0,0,0));
//	vg.begin_path();
//	vg.rect(x-10.0,y-10.0, w+20.0,h+30.0);
//	vg.rounded_rect(x,y, w,h, cornerRadius);
//	vg.path_winding(HOLE);
//	vg.fill_paint(shadowPaint);
//	vg.fill();
//
//	// Window
//	vg.begin_path();
//	vg.rounded_rect(x,y, w,h, cornerRadius);
//	vg.move_to(x-10.0,y+arry);
//	vg.line_to(x+1.0,y+arry-11.0);
//	vg.line_to(x+1.0,y+arry+11.0);
//	vg.fill_color(rgba(200,200,200,255));
//	vg.fill();
//
//	vg.save();
//	vg.scissor(x,y,w,h);
//	vg.translate(0.0, -(stackh-h)*u);
//
//	let dv = 1.0 / (nimages as f32 - 1.0);
//
//	for i in range(0, nimages) {
//		let mut tx = x+10.0;
//		let mut ty = y+10.0;
//		tx += (i%2) as f32 * (thumb+10.0);
//		ty += (i/2) as f32 * (thumb+10.0);
//		let mut imgw: i32 = 0;
//		let mut imgh: i32 = 0;
//		let ix: f32;
//		let iy: f32;
//		let iw: f32;
//		let ih: f32;
//		vg.image_size(images[i], &mut imgw, &mut imgh);
//		if imgw < imgh {
//			iw = thumb;
//			ih = iw * (imgh as f32) / (imgw as f32);
//			ix = 0.0;
//			iy = -(ih-thumb)*0.5;
//		} else {
//			ih = thumb;
//			iw = ih * (imgw as f32) / (imgh as f32);
//			ix = -(iw-thumb)*0.5;
//			iy = 0.0;
//		}
//
//		let v = i as f32 * dv;
//		let a = clamp((u2-v) / dv, 0.0, 1.0);
//
//		if a < 1.0 {
//			draw_spinner(vg, tx+thumb/2.0,ty+thumb/2.0, thumb*0.25, t);
//		}
//
//		let imgPaint = vg.image_pattern(tx+ix, ty+iy, iw,ih, 0.0/180.0*PI, images[i], NOREPEAT, a);
//		vg.begin_path();
//		vg.rounded_rect(tx,ty, thumb,thumb, 5.0);
//		vg.fill_paint(imgPaint);
//		vg.fill();
//
//		shadowPaint = vg.box_gradient(tx-1.0,ty, thumb+2.0,thumb+2.0, 5.0, 3.0, rgba(0,0,0,128), rgba(0,0,0,0));
//		vg.begin_path();
//		vg.rect(tx-5.0,ty-5.0, thumb+10.0,thumb+10.0);
//		vg.rounded_rect(tx,ty, thumb,thumb, 6.0);
//		vg.path_winding(HOLE);
//		vg.fill_paint(shadowPaint);
//		vg.fill();
//
//		vg.begin_path();
//		vg.rounded_rect(tx+0.5,ty+0.5, thumb-1.0,thumb-1.0, 4.0-0.5);
//		vg.stroke_width(1.0);
//		vg.stroke_color(rgba(255,255,255,192));
//		vg.stroke();
//	}
//	vg.restore();
//
//	// Hide fades
//	let mut fadePaint = vg.linear_gradient(x,y,x,y+6.0, rgba(200,200,200,255), rgba(200,200,200,0));
//	vg.begin_path();
//	vg.rect(x+0.4,y,w-8.0,6.0);
//	vg.fill_paint(fadePaint);
//	vg.fill();
//
//	fadePaint = vg.linear_gradient(x,y+h,x,y+h-6.0, rgba(200,200,200,255), rgba(200,200,200,0));
//	vg.begin_path();
//	vg.rect(x+4.0,y+h-6.0,w-8.0,6.0);
//	vg.fill_paint(fadePaint);
//	vg.fill();
//
//	// Scroll bar
//	shadowPaint = vg.box_gradient(x+w-12.0+1.0,y+4.0+1.0, 8.0,h-8.0, 3.0,4.0, rgba(0,0,0,32), rgba(0,0,0,92));
//	vg.begin_path();
//	vg.rounded_rect(x+w-12.0,y+4.0, 8.0,h-8.0, 3.0);
//	vg.fill_paint(shadowPaint);
//	//	vg.fill_color(rgba(255,0,0,128));
//	vg.fill();
//
//	let scrollh = (h/stackh) * (h-8.0);
//	shadowPaint = vg.box_gradient(x+w-12.0-1.0,y+4.0+(h-8.0-scrollh)*u-1.0, 8.0,scrollh, 3.0,4.0, rgba(220,220,220,255), rgba(128,128,128,255));
//	vg.begin_path();
//	vg.rounded_rect(x+w-12.0+1.0,y+4.0+1.0 + (h-8.0-scrollh)*u, 8.0-2.0,scrollh-2.0, 2.0);
//	vg.fill_paint(shadowPaint);
//	//	vg.fill_color(rgba(0,0,0,128));
//	vg.fill();
//
//	vg.restore();
//}

fn draw_lines(vg: &Ctx, x: f32, y: f32, w: f32, _h: f32, t: f32)
{
	let pad = 5.0;
	let s = w/9.0 - pad*2.0;
	let mut pts: [f32, ..4*2] = [0.0, ..4*2];
	let joins: [LineCap, ..3] = [MITER, ROUND, BEVEL];
	let caps: [LineCap, ..3] = [BUTT, ROUND, SQUARE];

	vg.save();
	pts[0] = -s*0.25 + cos(t*0.3) * s*0.5;
	pts[1] = sin(t*0.3) * s*0.5;
	pts[2] = -s*0.25;
	pts[3] = 0.0;
	pts[4] = s*0.25;
	pts[5] = 0.0;
	pts[6] = s*0.25 + cos(-t*0.3) * s*0.5;
	pts[7] = sin(-t*0.3) * s*0.5;

	for i in range(0, 3u) {
		for j in range(0, 3u) {
			let fx = x + s*0.5 + ((i as f32)*3.0+(j as f32))/9.0*w + pad;
			let fy = y - s*0.5 + pad;

			vg.line_cap(caps[i]);
			vg.line_join(joins[j]);

			vg.stroke_width(s*0.3);
			vg.stroke_color(rgba(0,0,0,160));
			vg.begin_path();
			vg.move_to(fx+pts[0], fy+pts[1]);
			vg.line_to(fx+pts[2], fy+pts[3]);
			vg.line_to(fx+pts[4], fy+pts[5]);
			vg.line_to(fx+pts[6], fy+pts[7]);
			vg.stroke();

			vg.line_cap(BUTT);
			vg.line_join(BEVEL);

			vg.stroke_width(1.0);
			vg.stroke_color(rgba(0,192,255,255));
			vg.begin_path();
			vg.move_to(fx+pts[0], fy+pts[1]);
			vg.line_to(fx+pts[2], fy+pts[3]);
			vg.line_to(fx+pts[4], fy+pts[5]);
			vg.line_to(fx+pts[6], fy+pts[7]);
			vg.stroke();
		}
	}


	vg.restore();
}

fn draw_widths(vg: &Ctx, x: f32,
              y: f32, width: f32)
{
	vg.save();
	let mut y = y;

	vg.stroke_color(rgba(0,0,0,255));

	for i in range(0, 20u) {
		let w = ((i as f32)+0.5)*0.1;
		vg.stroke_width(w);
		vg.begin_path();
		vg.move_to(x,y);
		vg.line_to(x+width,y+width*0.3);
		vg.stroke();
		y += 10.0;
	}

	vg.restore();
}

fn draw_caps(vg: &Ctx, x: f32,
            y: f32, width: f32)
{
	let caps: [LineCap, ..3] = [BUTT, ROUND, SQUARE];
	let lineWidth = 8.0;

	vg.save();

	vg.begin_path();
	vg.rect(x-lineWidth/2.0, y, width+lineWidth, 40.0);
	vg.fill_color(rgba(255,255,255,32));
	vg.fill();

	vg.begin_path();
	vg.rect(x, y, width, 40.0);
	vg.fill_color(rgba(255,255,255,32));
	vg.fill();

	vg.stroke_width(lineWidth);
	for i in range(0, 3u) {
		vg.line_cap(caps[i]);
		vg.stroke_color(rgba(0,0,0,255));
		vg.begin_path();
		vg.move_to(x, y + (i as f32)*10.0 + 5.0);
		vg.line_to(x+width, y + (i as f32)*10.0 + 5.0);
		vg.stroke();
	}

	vg.restore();
}
