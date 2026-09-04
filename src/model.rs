pub mod Model{
    use std::ops::{Add, Mul, Sub};


    pub struct Point{
        x: f32,
        y: f32,
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
            Self { point_color: RGB::new(255, 0, 0), line_color: RGB::new(255, 0, 0), fill_color: RGB::new(0, 0, 255), line_width: 1, point_chars: 'x', line_chars: '*', fill_chars: '.' }
        }
    }




    pub struct Rasterizer{
        style: MyStyle
    }

    impl Rasterizer{
        pub fn new(style: MyStyle) -> Self{
            Self { style }
        }


        pub fn draw_point(&self,buff: &mut [u32],buff_width: usize,pnt: &Point){
            let idx = buff_width * pnt.x() as usize + pnt.y() as usize;
            buff[idx] = 
            
        }



    }




}