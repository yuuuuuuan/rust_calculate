use std::ops::Mul;
use std::f64::consts::PI;
use std::f64;

enum Graphics {
    Round,
    Triangle,
    Rectangle,
}

fn get_area_round<T>(r:T) -> f64
where
    T: Into<f64>
{
    let radius_f64: f64 = r.into();
    PI * radius_f64 * radius_f64
}

fn get_area_rectangle<T>(x:T,y:T) -> f64
where
    T: Into<f64>
{
    let x_f64: f64 = x.into();
    let y_f64: f64 = y.into();
    x_f64*y_f64
}

fn get_area_triangle<T>(a:T,b:T,c:T) -> f64
where
    T: Into<f64>
{
    let a: f64 = a.into();
    let b: f64 = b.into();
    let c: f64 = c.into();
    let s = (a + b + c) * 0.5;
    let area = f64::sqrt(s * (s - a) * (s - b) * (s - c));
    area
}
pub trait Area {
    fn get_area<T>(&self,input:&mut [T]) -> Option<f64>
        where
            T: Into<f64>,
    ;
}

impl Area for Graphics {
    fn get_area<T>(&self,input:&mut [T]) -> Option<f64>
    where
        T: Into<f64>,
    {
        match &self {
            Graphics::Round => {
                match input.len() {
                    1 => {
                        Some(4.4)
                    },
                    _ => None,
                }
            },
            Graphics::Triangle => {
                match input.len() {
                    3 => {
                        Some(4.4)
                    },
                    _ => None,
                }
            },
            Graphics::Rectangle => {
                match input.len() {
                    2 => {
                        Some(4.4)
                    },
                    _ => None,
                }
            },
        }
    }
}

fn main() {
    //let round1 = &mut vec![100];
    //let rectangle1 = &mut vec![100,100];
    //println!("{:?}",Graphics::Round.get_area(round1));
    //println!("{:?}",Graphics::Rectangle.get_area(rectangle1));
    println!("{:?}",get_area_round(4));
    println!("{:?}",get_area_rectangle(4,5));
    println!("{:?}",get_area_triangle(3,3,3));
}
