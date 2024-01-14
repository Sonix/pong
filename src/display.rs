use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;

pub trait Display {
    fn display(&self, canvas: &mut WindowCanvas);
}

pub struct VoidDisplay {}

impl Display for VoidDisplay {
    fn display(&self, _: &mut WindowCanvas) {}
}

pub struct ScreenFill {
    pub color: Color,
}

impl Display for ScreenFill {
    fn display(&self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(self.color);
        canvas.clear();
    }
}

pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub color: Color,
}

impl Display for Rect {
    fn display(&self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(self.color);
        canvas.fill_rect(sdl2::rect::Rect::new(self.x, self.y, self.width, self.height)).unwrap();
    }
}

pub struct Circle {
    pub x: i32,
    pub y: i32,
    pub radius: u32,
    pub filled: bool,
    pub color: Color,
}

impl Display for Circle {
    fn display(&self, canvas: &mut WindowCanvas) {
        if !self.filled {
            canvas.set_draw_color(self.color);
            let diameter: i32 = (self.radius * 2) as i32;
            let mut x: i32 = (self.radius - 1) as i32;
            let mut y = 0;
            let mut tx = 1;
            let mut ty = 1;
            let mut error = tx - diameter;

            while x >= y {
                canvas.draw_point(Point::new(self.x + x, self.y - y)).unwrap();
                canvas.draw_point(Point::new(self.x + x, self.y + y)).unwrap();
                canvas.draw_point(Point::new(self.x - x, self.y - y)).unwrap();
                canvas.draw_point(Point::new(self.x - x, self.y + y)).unwrap();
                canvas.draw_point(Point::new(self.x + y, self.y - x)).unwrap();
                canvas.draw_point(Point::new(self.x + y, self.y + x)).unwrap();
                canvas.draw_point(Point::new(self.x - y, self.y - x)).unwrap();
                canvas.draw_point(Point::new(self.x - y, self.y + x)).unwrap();

                if error <= 0 {
                    y += 1;
                    error += ty;
                    ty += 2;
                }

                if error > 0 {
                    x -= 1;
                    tx += 2;
                    error += tx - diameter;
                }
            }
        } else {
            canvas.set_draw_color(self.color);
            for w in 0..self.radius as i32 * 2
            {
                for h in 0..self.radius as i32 * 2
                {
                    let dx: i32 = self.radius as i32 - w; // horizontal offset
                    let dy: i32 = self.radius as i32 - h; // vertical offset
                    if (dx*dx + dy*dy) <= (self.radius * self.radius) as i32
                    {
                        canvas.draw_point(Point::new(self.x + dx,self.y + dy)).unwrap();
                    }
                }
            }
        }
    }
}

