mod all_about_vecs;
mod integer_sizes;
mod ownership;
mod references;
mod vector3;

use weblab::weblab_folder;

weblab_folder!(
    "types",
    integer_sizes,
    vector3,
    ownership,
    references,
    all_about_vecs
);
