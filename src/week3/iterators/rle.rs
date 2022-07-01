use weblab::weblab;

#[weblab(programming_assignment)]
/// Run-length encoding
/// TODO description
#[weblab(title = "Run-length encoding")]
mod assignment {
    #[weblab(solution)]
    mod solution {
        use std::cmp::Ordering;

        pub fn run_length_encoding(input_iter: impl Iterator<Item=char>) -> impl Iterator<Item=(char, usize)> {
        }
    }

    #[allow(unused_variables)]
    #[weblab(solution_template)]
    mod solution_template {
    }

    #[weblab(test)]
    mod test {
        use super::solution::*;
        use weblab::{solution_only, template_only};

        template_only! {

        }

        solution_only! {

        }
    }
}
