use weblab::weblab;

#[weblab(programming_assignment)]
/// Given is the definition of two shapes (Rectangle, Circle) and the definition of the `Shape` trait.
/// Implement the `Shape` trait for the shapes.
#[weblab(title = "Shapes")]
mod assignment {
    #[weblab(solution)]
    mod solution {
        use std::f64::consts::PI;
        use weblab::solution_only;

        pub trait Shape {
            fn area(&self) -> f64;
            fn perimeter(&self) -> f64;
        }
        pub struct Rectangle {
            width: f64,
            height: f64,
        }
        impl Rectangle {
            pub fn new(width: f64, height: f64) -> Self {
                Rectangle { width, height }
            }
        }
        pub struct Circle {
            radius: f64,
        }
        impl Circle {
            pub fn new(radius: f64) -> Self {
                Circle { radius }
            }
        }

        solution_only! {
            impl Shape for Rectangle {
                fn area(&self) -> f64 {
                    self.width * self.height
                }
                fn perimeter(&self) -> f64 {
                    self.width * 2.0 + self.height * 2.0
                }
            }

            impl Shape for Circle {
                fn area(&self) -> f64 {
                    self.radius * self.radius * PI
                }
                fn perimeter(&self) -> f64 {
                    2.0 * PI * self.radius
                }
            }
        }
    }

    #[weblab(test)]
    mod test {
        use super::solution::*;
        use std::f64::consts::PI;
        use weblab::{solution_only, template_only};

        solution_only! {
            fn assert_almost_eq(a: f64, b: f64) {
                if (a-b).abs() > 0.0000001 {
                    panic!("Expected {} to equal {}", a, b)
                }
            }

            #[test]
            fn test_rectangle1() {
                let rectangle = Rectangle::new(5.0, 3.0);
                assert_almost_eq(rectangle.area(), 15.0);
                assert_almost_eq(rectangle.perimeter(), 16.0);
            }

            #[test]
            fn test_rectangle2() {
                let rectangle = Rectangle::new(0.3, 1982.3);
                assert_almost_eq(rectangle.area(), 594.69);
                assert_almost_eq(rectangle.perimeter(), 3965.2);
            }

            #[test]
            fn test_rectangle3() {
                let rectangle = Rectangle::new(0.0, 1.0);
                assert_almost_eq(rectangle.area(), 0.0);
                assert_almost_eq(rectangle.perimeter(), 2.0);
            }

            #[test]
            fn test_circle1() {
                let rectangle = Circle::new(5.0);
                assert_almost_eq(rectangle.area(), 25.0 * PI);
                assert_almost_eq(rectangle.perimeter(), 10.0 * PI);
            }

            #[test]
            fn test_circle2() {
                let rectangle = Circle::new(122038.12);
                assert_almost_eq(rectangle.area(), 46788690454.10381);
                assert_almost_eq(rectangle.perimeter(), 766788.1224998191);
            }

            #[test]
            fn test_circle3() {
                let rectangle = Circle::new(0.0);
                assert_almost_eq(rectangle.area(), 0.0);
                assert_almost_eq(rectangle.perimeter(), 0.0);
            }
        }
    }
}
