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
/// You have to make a function that emulates the cpu. Given a list of instructions, it should return a Vec\<i64>, a list of all output that the program has produced.
///
/// Some more details:
///
/// - If the CPU ever reaches an instruction that is out of bounds, it should stop executing and return the output
/// - If any instruction is executed that does not have a defined id, the emulator should panic
/// - If any arithmetic overflow occurs during the execution of the program, the emulator should panic
/// - All registers are initialized are 0
/// - The first instruction that is executed is the instruction at index 0
///
/// The following example program should output \[17]:
/// (HEX representation | Decoded instruction | Explanation)
///
/// ```
/// 0: HEX 0000 00 00 00000011  |  id=0 rega=0 regb=0 const=17  |  Add 17 to register 0
/// 1: HEX 0002 00 00 00000000  |  id=2 rega=0 regb=0 const=0   |  Output the value in register 0
/// ```
///
/// The following example program should output \[5,4,3,2,1]:
///
/// ```
/// 0: HEX 0000 00 00 00000005  |  id=0 rega=0 regb=0 const=5  |  Add 5 to register 0
/// 1: HEX 0002 00 00 00000000  |  id=2 rega=0 regb=0 const=0  |  Output the value in register 0
/// 2: HEX 0000 00 01 FFFFFFFF  |  id=0 rega=0 regb=0 const=-1 |  Subtract 1 (add -1) from register 0
/// 3: HEX 0001 00 00 00000005  |  id=1 rega=0 regb=0 const=5  |  If register 0 is 0, end the program
/// 4: HEX 0001 01 00 00000001  |  id=1 rega=1 regb=0 const=2  |  If register 1 is 0, repeat the loop
///
/// Both example programs are available in the "tests" tab of weblab.
/// ```
#[weblab(title = "Simple CPU")]
mod assignment {
    #[weblab(solution)]
    mod solution {
        pub fn emulate(instructions: &Vec<u64>) -> Vec<i64> {
            let mut pc = 0;
            let mut registers = [0i64; 256];
            let mut output = vec![];

            while pc < instructions.len() {
                let instr = instructions[pc];
                let instr: [u8; 8] = instr.to_be_bytes();

                let id = u16::from_be_bytes([instr[0], instr[1]]);
                let reg_a = u8::from_be_bytes([instr[2]]);
                let reg_b = u8::from_be_bytes([instr[3]]);
                let constant = i32::from_be_bytes([instr[4], instr[5], instr[6], instr[7]]);

                match id {
                    0 => {
                        registers[reg_a as usize] += registers[reg_b as usize] + constant as i64;
                        pc += 1;
                    }
                    1 => {
                        if registers[reg_a as usize] == 0 {
                            pc = constant as usize;
                        } else {
                            pc += 1;
                        }
                    }
                    2 => {
                        output.push(registers[reg_a as usize]);
                        pc += 1;
                    }
                    _ => unreachable!(),
                }
            }

            output
        }
    }

    #[allow(unused_variables)]
    #[weblab(solution_template)]
    mod solution_template {
        pub fn emulate(instructions: &Vec<u64>) -> Vec<i64> {
            todo!()
        }
    }

    #[weblab(test)]
    mod test {
        use super::solution::*;
        use weblab::{solution_only, template_only};

        #[test]
        fn test_example1() {
            let instructions = vec![
                0x0000_00_00_00000011, // Add 17 to register 0
                0x0002_00_00_00000000, // Output the value in register 0
            ];
            let expected_output = vec![17];
            assert_eq!(emulate(&instructions), expected_output);
        }

        #[test]
        fn test_example2() {
            let instructions = vec![
                0x0000_00_00_00000005, // Add 5 to register 0
                0x0002_00_00_00000000, // Output the value in register 0
                0x0000_00_01_FFFFFFFF, // Subtract 1 from register 0
                0x0001_00_00_00000005, // If register 0 is 0, end the program
                0x0001_01_00_00000001, // If register 1 is 0, repeat the loop
            ];
            let expected_output = vec![5,4,3,2,1];
            assert_eq!(emulate(&instructions), expected_output);
        }

        solution_only! {
            #[test]
            fn test_more_values_example2() {
                for i in 1..40 {
                    let instructions = vec![
                        i as u64, // Add 5 to register 0
                        0x0002_00_00_00000000, // Output the value in register 0
                        0x0000_00_01_FFFFFFFF, // Subtract 1 from register 0
                        0x0001_00_00_00000005, // If register 0 is 0, end the program
                        0x0001_01_00_00000001, // If register 1 is 0, repeat the loop
                    ];
                    let expected_output: Vec<i64> = (1..=i).rev().collect();
                    assert_eq!(emulate(&instructions), expected_output);
                }
            }

            #[test]
            fn test_series_of_n() {
                for n in 1..40 {
                    let instructions = vec![
                        0x0000_00_00_0000000A, // Do 10 iterations
                        0x0002_02_00_00000000, // Start loop, Output the value in register 2
                        0x0000_00_01_FFFFFFFF, // Subtract 1 from register 0
                        0x0000_02_01_00000000 + n, //Add n to register 2
                        0x0001_00_00_FFFFFFFF, // If register 0 is 0, end the program
                        0x0001_01_00_00000001, // If register 1 is 0, repeat the loop
                    ];
                    let expected_output: Vec<i64> = (0..10).map(|x| x * n as i64).collect();
                    assert_eq!(emulate(&instructions), expected_output);
                }
            }

            #[test]
            fn powers_of_2() {
                let instructions = vec![
                    0x0000_02_00_00000002, // [2] = 2
                    0x0000_00_00_0000000A, // Do 10 iterations
                    0x0002_02_00_00000000, // Start loop, Output the value in register 2
                    0x0000_00_01_FFFFFFFF, // Subtract 1 from register 0
                    0x0000_02_02_00000000, // Add [2] to itself
                    0x0001_00_00_FFFFFFFF, // If register 0 is 0, end the program
                    0x0001_01_00_00000002, // If register 1 is 0, repeat the loop
                ];
                let expected_output: Vec<i64> = vec![2,4,8,16,32,64,128,256,512,1024];
                assert_eq!(emulate(&instructions), expected_output);
            }

            #[test]
            fn powers_of_2_plus_1() {
                let instructions = vec![
                    0x0000_02_00_00000002, // [2] = 2
                    0x0000_00_00_0000000A, // Do 10 iterations
                    0x0002_02_00_00000000, // Start loop, Output the value in register 2
                    0x0000_00_01_FFFFFFFF, // Subtract 1 from register 0
                    0x0000_02_02_00000001, // Add [2] to itself + 1
                    0x0001_00_00_FFFFFFFF, // If register 0 is 0, end the program
                    0x0001_01_00_00000002, // If register 1 is 0, repeat the loop
                ];
                let expected_output: Vec<i64> = vec![2,5,11,23, 47, 95, 191, 383, 767, 1535];
                assert_eq!(emulate(&instructions), expected_output);
            }

            #[test]
            #[should_panic]
            fn panic_on_overflow_number() {
                let instructions = vec![
                    0x0000_02_00_00000002, // [2] = 2
                    0x0000_00_00_00000040, // Do 64 iterations
                    0x0002_02_00_00000000, // Start loop, Output the value in register 2
                    0x0000_00_01_FFFFFFFF, // Subtract 1 from register 0
                    0x0000_02_02_00000000, // Add [2] to itself
                    0x0001_00_00_FFFFFFFF, // If register 0 is 0, end the program
                    0x0001_01_00_00000002, // If register 1 is 0, repeat the loop
                ];
                emulate(&instructions);
            }

            #[test]
            fn not_panic_on_high_number() {
                let instructions = vec![
                    0x0000_02_00_00000002, // [2] = 2
                    0x0000_00_00_00000030, // Do 65 iterations
                    0x0002_02_00_00000000, // Start loop, Output the value in register 2
                    0x0000_00_01_FFFFFFFF, // Subtract 1 from register 0
                    0x0000_02_02_00000000, // Add [2] to itself
                    0x0001_00_00_FFFFFFFF, // If register 0 is 0, end the program
                    0x0001_01_00_00000002, // If register 1 is 0, repeat the loop
                ];
                emulate(&instructions);
            }

            #[test]
            #[should_panic]
            fn panic_on_invalid_instruction() {
                let instructions = vec![
                    0x0003_00_00_00000000
                ];
                emulate(&instructions);
            }
        }
    }
}
