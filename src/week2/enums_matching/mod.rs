use weblab::weblab_folder;

mod binary_tree;
mod creating_patterns;
mod enum_definition1;
mod enum_definition2;
mod errors;
mod matching;
mod safe_division;
mod structural_matching;
mod tic_tac_toe;

weblab_folder!(
    "enums and matching",
    safe_division,
    matching,
    enum_definition1,
    enum_definition2,
    structural_matching,
    creating_patterns,
    binary_tree,
    tic_tac_toe
);
