use weblab::weblab;

#[weblab(programming_assignment)]
/// Write a function called quadratic_solutions that takes in three arguments of type
/// f64 (a, b, and c) and calculates all the (real-valued) solutions of the quadratic equation
///
/// ```
/// ax2+bx+c=0.
/// ```
///
/// Of the real solutions, return the largest. If no real solutions exist, return the special float value `NaN` (Not A Number).
#[weblab(title = "quadratics")]
mod assignment {
    #[weblab(solution)]
    mod solution {
        #[allow(unused_imports)]
        use weblab::{solution_only, template_only};

        pub fn quadratic_solutions(a: f64, b: f64, c: f64) -> f64 {
            solution_only! {
                let d = b * b - (4.0 * a * c);
                if d < 0.0 {
                    return f64::NAN
                }

                let s1 = (-b + d.sqrt()) / 2.0 * a;
                let s2 = (-b - d.sqrt()) / 2.0 * a;

                s1.max(s2)
            }
            template_only! {
                todo!()
            }
        }
    }

    #[weblab(test_template)]
    mod test_template {
        use super::solution::quadratic_solutions;

        #[test]
        fn quadratic_test() {
            assert_eq!(quadratic_solutions(1.0, 0.0, -1.0), 1.0);
        }
    }

    #[weblab(test)]
    mod test {
        use super::solution::quadratic_solutions;

        macro_rules! assert_almost_eq {
            ($a: expr, $b: expr, $delta: expr) => {
                assert!(($b - $a).abs() < $delta);
            };
            ($a: expr, $b: expr) => {
                assert!(($b - $a).abs() < 0.001);
            };
        }

        fn quadratic_spec(a: f64, b: f64, c: f64) -> f64 {
            let d = b * b - (4.0 * a * c);
            if d < 0.0 {
                return f64::NAN;
            }

            let s1 = (-b + d.sqrt()) / 2.0 * a;
            let s2 = (-b - d.sqrt()) / 2.0 * a;

            s1.max(s2)
        }


        #[test]
        fn quadratic_test_nice_value() {
            assert_almost_eq!(quadratic_solutions(1.0, 0.0, -1.0), 1.0);
        }

        #[test]
        fn quadratic_test_no_solutions() {
            assert!(quadratic_solutions(1.0, 0.0, 1.0).is_nan());
        }

        #[test]
        fn quadratic_test_one_solution() {
            assert_almost_eq!(quadratic_spec(1.0, 0.0, 0.0), 0.0);
        }

        // #[test]
        // fn quadratic_test_spec() {
        //     assert!(quadratic_solutions(1.0, 0.0, 1.0).is_nan());
        // }
    }
}
