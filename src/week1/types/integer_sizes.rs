use weblab::inline_question_list;

inline_question_list! {
    title: "integer sizes",
    mc_question! {
        title: "1",
        question:
        /// Which integer type is the same size as a byte?
        explanation:
        /// A byte is 8 bits. The rust `u8` type corresponds to an integer with exactly 8 bits.
        option "`u8`" correct,
        option "`i16`",
        option "`usize`",
        option "`u32`",
        randomize
    }
    mc_question! {
        title: "2",
        question:
        /// Which integer type is the same size as a character?
        explanation:
        /// The C `char` type is only 1 byte large. However, this is not enough space to encode
        /// many of the characters used in languages other than english. Instead, a character
        /// is a 32 bit number, encoding a single **unicode** codepoint. This does, however, not
        /// mean that every single character in a string takes up 4 bytes. (for more info, see UTF-8)
        option "`u8`",
        option "`i16`",
        option "`usize`",
        option "`u32`" correct,
        randomize
    }
    mc_question! {
        title: "3",
        question:
        /// Which integer type can hold the largest numbers
        explanation:
        /// `u32` and `i32` have the same number of bits. Still, an i32 can't hold numbers as large
        /// as a `u32` since an `i32` also encodes negative numbers, taking up half the available space.
        option "`u8`",
        option "`i16`",
        option "`u32`" correct,
        option "`i32`",
        randomize
    }
    mc_question! {
        title: "4",
        question:
        /// Which integer type is best to use when you want to store the value of a pointer?
        explanation:
        /// A `usize` is defined as the size of a pointer on the system that the program is
        /// compiled on. That means that on a 32 bit system, a usize is 32 bits, while on a 64 bit
        /// system a usize is 64 bits. This makes them perfect to store lengths of things. The length
        /// of something can't be larger than memory is, and a pointer can also not be larger than memory is.
        /// That's where the name comes from. `isize` is like `usize`, but also stores negative numbers.
        option "`u64`",
        option "`u32`",
        option "`usize`" correct,
        option "`isize`",
        randomize
    }
    mc_question! {
        title: "5",
        question:
        /// Which integer type is best when you want to store numbers between 0 and 10000 most space efficiently?
        explanation:
        /// A `u16` can hold numbers up to 65535, and can thus comfortably hold numbers up to 10000.
        option "`u8`",
        option "`u16`" correct,
        option "`u32`",
        option "`u64`",
        randomize
    }
}
