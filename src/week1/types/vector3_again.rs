use weblab::weblab;

#[weblab(programming_assignment)]
/// In the last assignment, you've implemented a 3D Vector and written some functions to make the struct useful.
/// These functions were global, which is not ideal. The better way of doing this is to use *method syntax*.
/// Then, we could use our functions like this:
/// ```
/// let v1 = Vector3::new(1.0, 2.0, 3.0);
/// let v1 = Vector3::new(4.3, 3.4, 1.3);
/// let v3 = v1.add(v2);
/// ```
///
/// Define the `new` and `add` functions from the previous assignment using method syntax.
#[weblab(title = "3D Vectors, again")]
mod assignment {
    #[weblab(solution)]
    mod solution {
        use weblab::{solution_only, template_only};

        pub struct Vector3 {
            pub x: f64,
            pub y: f64,
            pub z: f64,
        }

        impl Vector3 {
            pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
                Vector3 { x, y, z }
            }

            pub fn add(self, other: Vector3) -> Vector3 {
                Vector3 {
                    x: self.x + other.x,
                    y: self.y + other.y,
                    z: self.z + other.z,
                }
            }
        }
    }

    #[weblab(solution_template)]
    mod solution_template {
        use weblab::{solution_only, template_only};

        /// TODO add fields
        pub struct Vector3 {}

        //TODO, add the new() method and the add() method
        impl Vector3 {}
    }

    #[weblab(test)]
    mod test {
        use super::solution::*;
        use weblab::{solution_only, template_only};

        /// This test tests the new method, it is already complete
        #[test]
        fn test_new() {
            let v1 = Vector3::new(1.2, 3.5, 6.0);
            assert_eq!(v1.x, 1.2);
            assert_eq!(v1.y, 3.5);
            assert_eq!(v1.z, 6.0);
        }

        solution_only! {
            #[test]
            fn test_more_news() {
                let v1 = Vector3::new(11131819312897.2, -0.000000005, 6.0);
                assert_eq!(v1.x, 11131819312897.2);
                assert_eq!(v1.y, -0.000000005);
                assert_eq!(v1.z, 6.0);
            }

            #[test]
            fn test_struct_add1() {
                let v1 = Vector3::new(11131819312897.2, -0.000000005, 6.0);
                let v2 = Vector3::new(-2311231.2, 21321.22, -2202.2);

                let v3 = Vector3::new(11131817001666.0, 21321.219999995003, -2196.2);
                let v4 = v1.add(v2);
                assert_eq!(v3.x, v4.x);
                assert_eq!(v3.y, v4.y);
                assert_eq!(v3.z, v4.z);
            }

            #[test]
            fn test_struct_add2() {
                let v1 = Vector3::new(0.0, 1.0, 2.0);
                let v2 = Vector3::new(-2.3, 2.3, 1.3);

                let v3 = Vector3::new(-2.3, 3.3, 3.3);
                let v4 = v1.add(v2);
                assert_eq!(v3.x, v4.x);
                assert_eq!(v3.y, v4.y);
                assert_eq!(v3.z, v4.z);
            }
        }
    }
}
