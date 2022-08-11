use weblab::weblab;

#[weblab(programming_assignment)]
/// Sometimes, a program (or part of a program) can fail. In
/// that case it would be very nice if the user of the program
/// can see what went wrong.
///
/// Unlike other programming languages, in Rust we have a philosophy of making errors
/// both readable by users *and* by programs. The latter is useful so the program can
/// decide based on the error whether to fail completely, or maybe recover.
///
///
#[weblab(title = "Structural Matching")]
mod assignment {
    // #[weblab(library)]
    // mod library {
    // }

    #[weblab(solution)]
    mod solution {
        use weblab::{solution_only, template_only};
    }

    #[weblab(test_template)]
    mod test_template {}

    #[weblab(test)]
    mod test {
        use super::solution::*;
    }
}
