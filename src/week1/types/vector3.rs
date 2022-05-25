use weblab::weblab;

#[weblab(programming_assignment)]
/// In math, vectors are a commonly used tool (not to be confused with Rust `Vec`!).
/// For this assignment, implement a struct that represents a 3D vector.
/// A 3D vector can be imagined as an arrow in 3D space, starting at the origin, going to a certain point.
/// This point can be represented using 3 numbers. For this assignment, give these numbers the type `f64`, a 64-bit floating point number.
///
/// For this assignment:
///
/// * Define the fields of the Vector3 struct. Make sure the names of the fields are `x`, `y` and `z`, that the fields are `f64`, and **that all fields are pub**.
/// * Define the global function `new`, which constructs a new Vector3
/// * Define the global function `add`, which adds two Vector3s
/// * Bonus: Make a test, in which you make 2 vectors, and add them
///
#[weblab(title = "3D Vectors")]
mod assignment {
    #[weblab(solution)]
    mod solution {
        use weblab::{solution_only, template_only};

        pub struct Vector3 {
            pub x: f64,
            pub y: f64,
            pub z: f64,
        }

        pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
            Vector3 { x, y, z }
        }

        pub fn add(a: Vector3, b: Vector3) -> Vector3 {
            Vector3 {
                x: a.x + b.x,
                y: a.y + b.y,
                z: a.z + b.z,
            }
        }
    }

    #[allow(unused_variables)]
    #[weblab(solution_template)]
    mod solution_template {
        use weblab::{solution_only, template_only};

        /// TODO add fields
        pub struct Vector3 {}

        pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
            todo!()
        }

        pub fn add(a: Vector3, b: Vector3) -> Vector3 {
            todo!()
        }
    }

    #[weblab(test)]
    mod test {
        use super::solution::*;
        use weblab::{solution_only, template_only};

        /// This test tests the new method, it is already complete
        #[test]
        fn test_new() {
            let v1 = new(1.2, 3.5, 6.0);
            assert_eq!(v1.x, 1.2);
            assert_eq!(v1.y, 3.5);
            assert_eq!(v1.z, 6.0);
        }

        solution_only! {
            #[test]
            fn test_more_news() {
                let v1 = new(11131819312897.2, -0.000000005, 6.0);
                assert_eq!(v1.x, 11131819312897.2);
                assert_eq!(v1.y, -0.000000005);
                assert_eq!(v1.z, 6.0);
            }

            #[test]
            fn test_struct_add1() {
                let v1 = new(11131819312897.2, -0.000000005, 6.0);
                let v2 = new(-2311231.2, 21321.22, -2202.2);

                let v3 = new(11131817001666.0, 21321.219999995003, -2196.2);
                let v4 = add(v1, v2);
                assert_eq!(v3.x, v4.x);
                assert_eq!(v3.y, v4.y);
                assert_eq!(v3.z, v4.z);
            }

            #[test]
            fn test_struct_add2() {
                let v1 = new(0.0, 1.0, 2.0);
                let v2 = new(-2.3, 2.3, 1.3);

                let v3 = new(-2.3, 3.3, 3.3);
                let v4 = add(v1, v2);
                assert_eq!(v3.x, v4.x);
                assert_eq!(v3.y, v4.y);
                assert_eq!(v3.z, v4.z);
            }
        }
    }
}
