use weblab::weblab;

#[weblab(programming_assignment)]
/// For this exercise, design the `Increment` trait.
/// It has two methods:
/// `increment`, that should mutate the number and
#[weblab(title = "Addable")]
mod assignment {
    #[weblab(solution)]
    mod solution {
        pub trait Addable {
            fn add(self, other: Self) -> Self;
        }
    }

    #[allow(unused_variables)]
    #[weblab(solution_template)]
    mod solution_template {
        /// TODO
        pub trait Addable {

        }
    }

    #[weblab(test)]
    mod test {
        use super::solution::*;
        use weblab::{solution_only, template_only};

        solution_only! {
        }
    }
}
