use weblab::weblab;

#[weblab(programming_assignment)]
/// The definitions of two shapes (Rectangle, Circle) and the `Shape` trait are given.
/// Implement the `Shape` trait for the shapes.
///
/// You may need the "pi" constant. There's one in the standard library you can use.
/// Search for "PI" on [https://docs.rs/std](https://docs.rs/std)
///
/// Note that tests will assume that all traits are implemented. Since the tests and your code are
/// compiled together, the tests won't compile if you didn't implement the required traits. In this case,
/// your score will be 0/0. Read the assignment carefully. To let tests *compile but fail* you can use the
/// `todo!()` macro in function bodies, so you don't need to finish all implementations before you can test
/// the ones you do have.
#[weblab(title = "Shapes")]
#[weblab(weight = 2)]
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

        fn almost_eq(a: f64, b: f64) -> bool {
            (a - b).abs() < 0.0000001
        }

        #[test]
        pub fn test() {
            let c1 = Circle::new(3.3f64);

            assert!(almost_eq(c1.area(), 3.3f64 * 3.3f64 * PI));
            assert!(almost_eq(c1.perimeter(), 2f64 * 3.3f64 * PI));
        }

        solution_only! {
            #[test]
            fn test_rectangle1() {
                let rectangle = Rectangle::new(5.0, 3.0);
                assert!(almost_eq(rectangle.area(), 15.0));
                assert!(almost_eq(rectangle.perimeter(), 16.0));
            }

            #[test]
            fn test_rectangle2() {
                let rectangle = Rectangle::new(0.3, 1982.3);
                assert!(almost_eq(rectangle.area(), 594.69));
                assert!(almost_eq(rectangle.perimeter(), 3965.2));
            }

            #[test]
            fn test_rectangle3() {
                let rectangle = Rectangle::new(0.0, 1.0);
                assert!(almost_eq(rectangle.area(), 0.0));
                assert!(almost_eq(rectangle.perimeter(), 2.0));
            }

            #[test]
            fn test_circle1() {
                let rectangle = Circle::new(5.0);
                assert!(almost_eq(rectangle.area(), 25.0 * PI));
                assert!(almost_eq(rectangle.perimeter(), 10.0 * PI));
            }

            #[test]
            fn test_circle2() {
                let rectangle = Circle::new(122038.12);
                assert!(almost_eq(rectangle.area(), 46788690454.10381));
                assert!(almost_eq(rectangle.perimeter(), 766788.1224998191));
            }

            #[test]
            fn test_circle3() {
                let rectangle = Circle::new(0.0);
                assert!(almost_eq(rectangle.area(), 0.0));
                assert!(almost_eq(rectangle.perimeter(), 0.0));
            }
        }
    }
}
