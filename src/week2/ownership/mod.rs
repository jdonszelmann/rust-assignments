use weblab::weblab_folder;

mod fizzbuzz;
mod fizzbuzzwoofyum;
mod is_it_sized;
mod stack;
mod string_theory;
mod strings;
mod vector3_again;

weblab_folder!(
    "ownership",
    vector3_again,
    stack,
    is_it_sized,
    strings,
    string_theory,
    fizzbuzz,
    fizzbuzzwoofyum
);
