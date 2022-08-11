use weblab::weblab;

#[weblab(programming_assignment)]
/// 
#[weblab(title = "Vec Deque")]
mod assignment {
    #[weblab(solution)]
    mod solution {
        use weblab::solution_only;

        solution_only! {

        }
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
