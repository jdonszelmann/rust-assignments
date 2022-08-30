use weblab::weblab_folder;

mod brackets;
mod fizzbuzz;
mod fizzbuzzwoofyum;
mod is_it_sized;
mod stack;
mod string_theory;
mod strings;
mod vector3_again;

weblab_folder!(
    "structs and ownership",
    sum,
    vector3_again,
    stack,
    // is_it_sized,
    brackets,
    strings,
    string_theory,
    // fizzbuzz,
    fizzbuzzwoofyum
);
