pub mod Model{
    use std::ops::{Add, Mul, Sub};


    #[derive(Clone, Debug)]
    pub struct Point{
        pub x: f32,
        pub y: f32,
    }
    
    impl Default for Point{
        fn default() -> Self {
            Self { x: 0.0, y: 0.0 }
        }
    }

    impl Point{

        pub fn new(x:f32,y:f32) -> Self{
            Self { x, y }
        }

        pub fn dist(&self,p2:Point) -> f32{
            ((self.x - p2.x()).powf(2.0) + (self.y - p2.y()).powf(2.0)).sqrt()
        }

        pub fn x(&self) -> f32{
            self.x
        }

        pub fn y(&self) -> f32{
            self.y
        }

        pub fn from_pt(p: &Point) -> Self{
            Self { x: p.x(), y: p.y() }
        }

        pub fn lin_ip(&self,other: &Point,t: f32) -> Point{
            Self { x: self.x + (other.x - self.x) * t, y: self.y + (other.y - self.y) * t}
        }

    }

    impl Add for Point{
        type Output = Point;
        fn add(self, rhs: Self) -> Self::Output {            
            Self::new(self.x() + rhs.x(), self.y() + rhs.y())
        }
    }

    impl Sub for Point{
        type Output = Point;

        fn sub(self, rhs: Self) -> Self::Output {
            Self::new(self.x() - rhs.x(), self.y() - rhs.y())
        }

    }

    impl Mul<f32> for &Point{
        type Output = Point;

        fn mul(self, rhs: f32) -> Self::Output {
            Point::new(self.x() * rhs, self.y() * rhs)
        }
    }


    pub struct LinearInterpolate{
        start: Point,
        end: Point
    }

    impl LinearInterpolate{
        pub fn new(start: Point,end: Point) -> Self{
            Self { start, end }
        }

        pub fn at(&self,t: f32) -> Point{
            self.start.lin_ip(&self.end, t)
        }
    }


    pub struct QuadraticBezier{
        start: Point,
        control: Point,
        end:Point
    }

    impl QuadraticBezier{
        pub fn new(start: Point,control: Point,end:Point) -> Self{
            Self { start, control, end }
        }

        pub fn at(&self,t: f32) -> Point{
            let a = self.start.lin_ip(&self.control, t);
            let b = self.control.lin_ip(&self.end, t);
            a.lin_ip(&b, t)
        }

        pub fn flatten(&self,cnt: usize) -> Vec<Point>{
            let mut ret = Vec::with_capacity(cnt+1);
            for i in 0..=cnt{
                let t = i as f32 / cnt as f32;
                ret.push(self.at(t));
            }
            ret
        }

    }

    pub struct CubicBezier{
        start: Point,
        c1: Point,
        c2: Point,
        end: Point
    }

    impl CubicBezier{
        pub fn new(start: Point, c1: Point, c2: Point, end: Point) -> Self{
            Self { start, c1, c2, end }
        }

        pub fn at(&self, t: f32) -> Point{
            let a = self.start.lin_ip(&self.c1, t);
            let b = self.c1.lin_ip(&self.c2, t);
            let c = self.c2.lin_ip(&self.end, t);
            let d = a.lin_ip(&b, t);
            let e = b.lin_ip(&c, t);
            d.lin_ip(&e, t)
        }

        pub fn flatten(&self, cnt: usize) -> Vec<Point>{
            let mut ret = Vec::with_capacity(cnt+1);
            for i in 0..=cnt{
                let t = i as f32 / cnt as f32;
                ret.push(self.at(t));
            }
            ret
        }
    }

    pub struct RGB{
       pub r:u8,
       pub g:u8,
       pub b:u8
    }

    impl RGB{
        pub fn new(r: u8,g:u8,b:u8) -> Self{
            Self { r, g, b }
        }

        pub fn invert(&mut self){
            self.r = u8::MAX - self.r;
            self.g = u8::MAX - self.g;
            self.b = u8::MAX - self.b;
        }
    }


    pub struct MyStyle{
       pub point_color: RGB,
       pub line_color: RGB,
       pub fill_color: RGB,
       pub line_width: usize,
       pub point_chars: char,
       pub line_chars: char,
       pub fill_chars: char
    }

    impl Default for MyStyle{
        fn default() -> Self {
            Self { point_color: RGB::new(255, 0, 0), line_color: RGB::new(0, 255, 0), fill_color: RGB::new(0, 0, 255), line_width: 2, point_chars: 'x', line_chars: '.', fill_chars: '.' }
        }
    }

    pub struct Rasterizer{
        style: MyStyle
    }

    impl Rasterizer{
        pub fn new(style: MyStyle) -> Self{
            Self { style }
        }

        fn rgb_to_u32(&self, rgb: &RGB) -> u32{
            ((rgb.r as u32) << 16) | ((rgb.g as u32) << 8) | (rgb.b as u32)
        }

        pub fn draw_point(&self,buff: &mut [u32],cmap: &mut [char],buff_width: usize,pnt: &Point){
            let x = pnt.x as usize;
            let y = pnt.y as usize;
            if x < buff_width && y < buff.len() / buff_width{
                let idx = y * buff_width + x;
                buff[idx] = self.rgb_to_u32(&self.style.point_color);
                cmap[idx] = self.style.point_chars;
            }
        }

        pub fn draw_line(&self, buff: &mut [u32],cmap: &mut [char], buff_width: usize, p1: &Point, p2: &Point,is_curve:bool){
            let x0 = p1.x as i32;
            let y0 = p1.y as i32;
            let x1 = p2.x as i32;
            let y1 = p2.y as i32;
            
            let dx = (x1 - x0).abs();
            let dy = -(y1 - y0).abs();
            let sx = if x0 < x1 { 1 } else { -1 };
            let sy = if y0 < y1 { 1 } else { -1 };

            let mut err = dx + dy;
            let mut x = x0;
            let mut y = y0;
            let buff_height = buff.len() / buff_width;

            loop{
                if !is_curve && x == x0 && y == y0{
                    let idx = (y as usize) * buff_width + (x as usize);
                    cmap[idx] = self.style.point_chars;
                    buff[idx] = self.rgb_to_u32(&self.style.point_color);
                }
                else if x >= 0 && x < buff_width as i32 && y >= 0 && y < buff_height as i32{
                    let idx = (y as usize) * buff_width + (x as usize);
                    cmap[idx] = self.style.line_chars;
                    buff[idx] = self.rgb_to_u32(&self.style.line_color);
                }                
                if x == x1 && y == y1 { 
                    let idx = (y as usize) * buff_width + (x as usize);

                    if !is_curve{
                        cmap[idx] = self.style.point_chars;
                        buff[idx] = self.rgb_to_u32(&self.style.point_color);
                    }
                    break; 
                }
                let e2 = 2 * err;
                if e2 >= dy{
                    err += dy;
                    x += sx;
                }
                if e2 <= dx{
                    err += dx;
                    y += sy;
                }
            }
        }

        pub fn draw_spline(&self, buff: &mut [u32],cmap: &mut [char], buff_width: usize, points: &[Point], spline_type: SplineType){
            match spline_type{
                SplineType::Point => {
                    if !points.is_empty(){
                        for point in points{
                            self.draw_point(buff,cmap, buff_width, point);
                        }
                    }
                }
                SplineType::Line => {
                    if points.len() >= 2{
                        for i in 0..points.len()-1{
                            self.draw_line(buff,cmap, buff_width, &points[i], &points[i+1],false);
                        }
                    }
                }
                SplineType::QuadraticBezier => {
                    if points.len() >= 3{
                        let bezier = QuadraticBezier::new(points[0].clone(), points[1].clone(), points[2].clone());
                        let flattened = bezier.flatten(10);
                        for i in 0..flattened.len()-1{
                            self.draw_line(buff,cmap, buff_width, &flattened[i], &flattened[i+1],!(i <= 1) && !(i >= flattened.len()-2));
                        }
                        
                        
                    }
                }
                SplineType::CubicBezier => {
                    if points.len() >= 4{
                        let bezier = CubicBezier::new(points[0].clone(), points[1].clone(), points[2].clone(), points[3].clone());
                        let flattened = bezier.flatten(10);
                        for i in 0..flattened.len()-1{
                            self.draw_line(buff,cmap, buff_width, &flattened[i], &flattened[i+1],!(i <= 1) && !(i >= flattened.len()-2));
                        }
                    }
                }
            }
        }

        pub fn clear(&self, buff: &mut [u32]){
            for pixel in buff.iter_mut(){
                *pixel = 0;
            }
        }
    }

    #[derive(Clone, Copy, PartialEq)]
    pub enum SplineType{
        Point,
        Line,
        QuadraticBezier,
        CubicBezier
    }

}