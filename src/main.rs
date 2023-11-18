use std::f64::consts::PI;
use std::f64;
use std::cmp::PartialOrd;
enum Graphics {
    Round,
    Triangle,
    Rectangle,
}
const NAN :f64 = 0.0/0.0;
fn get_area_round<T>(r:&T) -> f64
where
    T: PartialOrd + Copy,
    T: Into<f64>,
{
    let radius_f64: f64 = (*r).into();
    if radius_f64 < 0.0 {
        return NAN;
    }
    PI * radius_f64 * radius_f64
}

fn get_area_rectangle<T>(x:&T,y:&T) -> f64
where
    T: PartialOrd + Copy,
    T: Into<f64>,
{
    let x_f64: f64 = (*x).into();
    let y_f64: f64 = (*y).into();
    if x_f64 < 0.0 || y_f64 < 0.0 {
        return NAN;
    }
    x_f64*y_f64
}

fn get_area_triangle<T>(a:&T,b:&T,c:&T) -> f64
where
    T: PartialOrd + Copy,
    T: Into<f64>,
{
    let a: f64 = (*a).into();
    let b: f64 = (*b).into();
    let c: f64 = (*c).into();
    if a < 0.0 || b < 0.0 || c < 0.0 {
        return NAN;
    } else if a+b<c || b+c<a || b+c<a {
        return NAN;
    }
    let s = (a + b + c) * 0.5;
    let area = f64::sqrt(s * (s - a) * (s - b) * (s - c));
    area
}
pub trait Area {
    fn get_area<T>(&self,_:&[T]) -> Result<f64,&str>
        where
            T: PartialOrd + Copy,
            T: Into<f64>,
    ;
}

impl Area for Graphics {
    fn get_area<T>(&self, input: &[T]) -> Result<f64, &str>
        where
            T: PartialOrd + Copy,
            T: Into<f64>,
    {
        match self {
            Graphics::Round => {
                match input.len() {
                    1 => {
                        let r = input.get(0).unwrap();
                        Ok(get_area_round(r))
                    }

                    _ => Err("err input"),
                }
            }

            Graphics::Rectangle => {
                match input.len() {
                    2 => {
                        let x = input.get(0).unwrap();
                        let y = input.get(1).unwrap();
                        Ok(get_area_rectangle(x, y))
                    }
                    _ => Err("err input"),
                }
            }

            Graphics::Triangle => {
                match input.len() {
                    3 => {
                        let a = input.get(0).unwrap();
                        let b = input.get(1).unwrap();
                        let c = input.get(2).unwrap();
                        Ok(get_area_triangle(a, b, c))
                    }
                    _ => Err("err input"),
                }
            }
        }
    }
}
fn main() {
    let round1 = & mut vec![10];
    println!("round1:{:?}",round1);
    println!("{:?}",Graphics::Round.get_area(round1));
    let round2 = & mut vec![10.00];
    println!("round2:{:?}",round2);
    println!("{:?}",Graphics::Round.get_area(round2));
    let round3 = & mut vec![10,10];
    println!("round3:{:?}",round3);
    println!("{:?}",Graphics::Round.get_area(round3));
    let round4 = & mut vec![-10];
    println!("round4:{:?}",round4);
    println!("{:?}",Graphics::Round.get_area(round4));

    let rectangle1 = & mut vec![10,10];
    println!("rectangle1:{:?}",rectangle1);
    println!("{:?}",Graphics::Rectangle.get_area(rectangle1));
    let rectangle2 = & mut vec![10.00,10.00];
    println!("rectangle2:{:?}",rectangle2);
    println!("{:?}",Graphics::Rectangle.get_area(rectangle2));
    let rectangle3 = & mut vec![10];
    println!("rectangle3:{:?}",rectangle3);
    println!("{:?}",Graphics::Rectangle.get_area(rectangle3));
    let rectangle4 = & mut vec![-10,10];
    println!("rectangle3:{:?}",rectangle4);
    println!("{:?}",Graphics::Rectangle.get_area(rectangle4));

    let triangle1 = & mut vec![10,10,10];
    println!("triangle1:{:?}",triangle1);
    println!("{:?}",Graphics::Triangle.get_area(triangle1));
    let triangle2 = & mut vec![10.00,10.00,10.00];
    println!("triangle2:{:?}",triangle2);
    println!("{:?}",Graphics::Triangle.get_area(triangle2));
    let triangle3 = & mut vec![10.00,10.00];
    println!("triangle3:{:?}",triangle3);
    println!("{:?}",Graphics::Triangle.get_area(triangle3));
    let triangle4 = & mut vec![-10.00,10.00,10.00];
    println!("triangle4:{:?}",triangle4);
    println!("{:?}",Graphics::Triangle.get_area(triangle4));
}
