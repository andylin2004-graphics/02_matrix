use crate::image::Image;

impl Image{
    pub fn draw_line(&mut self, mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32, color: Color){
        if x0 > x1{
            let mut tmp = x0;
            x0 = x1;
            x1 = tmp;
            tmp = y0;
            y0 = y1;
            y1 = tmp;
        }
        let slope: f32 = (y1-y0) as f32 / (x1-x0) as f32;
        if slope > 1.0{
            // octant 2
            let mut x = x0;
            let mut y = y0;
            let a = 2*(y1-y0);
            let b = -2*(x1-x0);
            let mut d = 1/2*a + b; // emphasis on controlling y
            while y <= y1{
                self.plot(x, y, color);
                if d < 0{ // as b dominates a, and we need to hit 0
                    x += 1;
                    d += a;
                }
                y += 1;
                d += b;
            }
        }else if slope >= 0.0{
            // octant 1
            let mut x = x0;
            let mut y = y0;
            let a = 2*(y1-y0);
            let b = -2*(x1-x0);
            let mut d = a + 1/2*b; // emphasis on controlling x
            while x <= x1{
                self.plot(x, y, color);
                if d > 0{ // as a dominates b, and we need to hit 0
                    y += 1;
                    d += b;
                }
                x += 1;
                d += a;
            }
        }else if slope < -1.0{
            // octant 7
            let mut x = x0;
            let mut y = y0;
            let a = 2*(y1-y0); // since this is negative, you dont need to make the next part negative
            let b = 2*(x1-x0);
            let mut d = 1/2*a + b; // emphasis on controlling x
            while y >= y1{
                self.plot(x, y, color);
                if d < 0{ // as a dominates b, and we need to hit 0
                    x += 1;
                    d -= a; // basically adding
                }
                y -= 1;
                d -= b; // basically adding
            }
        }else{
            // octant 8
            let mut x = x0;
            let mut y = y0;
            let a = 2*(y1-y0); // since this is negative, you dont need to make the next part negative
            let b = 2*(x1-x0);
            let mut d = a + 1/2*b; // emphasis on controlling y
            while x <= x1{
                self.plot(x, y, color);
                if d > 0{ // as b dominates a, and we need to hit 0
                    y -= 1;
                    d -= b; // basically adding
                }
                x += 1;
                d -= a; // basically adding
            }
        }
    }
}