use nanovg::*;
use util::*;


fn draw_graph(vg: &Ctx, x: f32,
             y: f32, w: f32,
             h: f32, t: f32)
{
	let mut samples: [f32, ..6] = [0.0, ..6];
	let mut sx: [f32, ..6] = [0.0, ..6];
	let mut sy: [f32, ..6] = [0.0, ..6];
	let dx = w/5.0;

	samples[0] = (1.0+sin(t*1.2345+cos(t*0.33457)*0.44))*0.5;
	samples[1] = (1.0+sin(t*0.68363+cos(t*1.3)*1.55))*0.5;
	samples[2] = (1.0+sin(t*1.1642+cos(t*0.33457)*1.24))*0.5;
	samples[3] = (1.0+sin(t*0.56345+cos(t*1.63)*0.14))*0.5;
	samples[4] = (1.0+sin(t*1.6245+cos(t*0.254)*0.3))*0.5;
	samples[5] = (1.0+sin(t*0.345+cos(t*0.03)*0.6))*0.5;

	for i in range(0, 6u) {
		sx[i] = x+ (i as f32)*dx;
		sy[i] = y+h*samples[i]*0.8;
	}

	// Graph background
	let bg = vg.linear_gradient(x,y,x,y+h, rgba(0,160,192,0), rgba(0,160,192,64));
	vg.begin_path();
	vg.move_to(sx[0], sy[0]);
	for i in range(1, 6u) {
		vg.bezier_to(sx[i-1]+dx*0.5,sy[i-1], sx[i]-dx*0.5,sy[i], sx[i],sy[i]);
	}
	vg.line_to(x+w, y+h);
	vg.line_to(x, y+h);
	vg.fill_paint(bg);
	vg.fill();

	// Graph line
	vg.begin_path();
	vg.move_to(sx[0], sy[0]+2.0);
	for i in range(1, 6u) {
		vg.bezier_to(sx[i-1]+dx*0.5,sy[i-1]+2.0, sx[i]-dx*0.5,sy[i]+2.0, sx[i],sy[i]+2.0);
	}
	vg.stroke_color(rgba(0,0,0,32));
	vg.stroke_width(3.0);
	vg.stroke();

	vg.begin_path();
	vg.move_to(sx[0], sy[0]);
	for i in range(1, 6u) {
		vg.bezier_to(sx[i-1]+dx*0.5,sy[i-1], sx[i]-dx*0.5,sy[i], sx[i],sy[i]);
	}
	vg.stroke_color(rgba(0,160,192,255));
	vg.stroke_width(3.0);
	vg.stroke();

	// Graph sample pos
	for i in range(0, 6u) {
		let bg = vg.radial_gradient(sx[i],sy[i]+2.0, 3.0,8.0, rgba(0,0,0,32), rgba(0,0,0,0));
		vg.begin_path();
		vg.rect(sx[i]-10.0, sy[i]-10.0+2.0, 20.0,20.0);
		vg.fill_paint(bg);
		vg.fill();
	}

	vg.begin_path();
	for i in range(0, 6u) {
		vg.circle(sx[i], sy[i], 4.0);
	}
	vg.fill_color(rgba(0,160,192,255));
	vg.fill();
	vg.begin_path();
	for i in range(0, 6u) {
		vg.circle(sx[i], sy[i], 2.0);
	}
	vg.fill_color(rgba(220,220,220,255));
	vg.fill();

	vg.stroke_width(1.0);
}
