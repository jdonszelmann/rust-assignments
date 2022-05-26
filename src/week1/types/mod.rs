mod all_about_vecs;
mod integer_sizes;
mod references;
mod stack;
mod vector3;
mod vector3_again;

use weblab::weblab_folder;

weblab_folder!(
    "types",
    integer_sizes,
    vector3,
    vector3_again,
    references,
    all_about_vecs,
    stack
);
