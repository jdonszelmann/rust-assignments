use weblab::weblab_folder;

mod creating_patterns;
mod enum_definition1;
mod enum_definition2;
mod errors;
mod matching;
mod safe_division;
mod structural_matching;
mod tic_tac_toe;
mod binary_tree;

weblab_folder!(
    "enums and matching",
    matching,
    enum_definition1,
    enum_definition2,
    safe_division,
    structural_matching,
    creating_patterns,
    binary_tree,
    tic_tac_toe
);
