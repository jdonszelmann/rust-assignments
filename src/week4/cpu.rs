use weblab::weblab;

#[weblab(programming_assignment)]
/// In this assignment, you will implement an emulator for a simple CPU.
///
/// The cpu has 256 registers, each register holding a 64 bit signed integer. The cpu has no memory.
///
/// The cpu will execute a list of instructions.
/// Each instruction is a 64 bit number (u64), with the following layout, from left to right:
///
/// - [0, 16) Instruction ID, which is an unsigned 16-bit integer.
/// - [16, 24) Register A, which is an unsigned 8-bit integer, indicating which of the 256 registers will be register A.
/// - [24, 32) Register B, which is an unsigned 8-bit integer, indicating which of the 256 registers will be register B.
/// - [32, 64) Constant, which is a signed 32-bit integer.
///
/// The cpu has the following instructions
///
/// - ADD with instruction id 0\
///   Add the value in register B and the constant to register A.\
///   \[A] = \[A] + \[B] + constant
/// - JUMPZ with instruction id 1\
///   If the value in register A is 0, the next instruction that is executed will be at the index in const.\
///   Otherwise, continue to the next instruction as usual.
/// - OUTPUT with instruction id 2\
///   Output the value in register A
///
/// Unless a jump occurs, the cpu should execute all instructions linearly,
///
/// You have to make a function that emulates the cpu. Given a list of instructions, it should return a Vec<i64>, a list of all output that the program has produced.
///
/// Some more details:
///
/// - If the CPU ever reaches an instruction that is out of bounds, it should stop executing and return the output
/// - If any instruction is executed that does not have a defined id, the cpu should panic
/// - If any arithmetic overflow occurs during the execution of the program, the cpu should panic
/// - All registers are initialized are 0
/// - The first instruction that is executed is the instruction at index 0
///
/// TODO: Maybe make it return result instead of panic?
#[weblab(title = "Simple CPU")]
mod assignment {
    #[weblab(solution)]
    mod solution {

    }

    #[allow(unused_variables)]
    #[weblab(solution_template)]
    mod solution_template {

    }

    #[weblab(test)]
    mod test {
        use super::solution::*;
        use weblab::{solution_only, template_only};

        solution_only! {

        }
    }
}
