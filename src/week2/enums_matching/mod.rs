use weblab::weblab_folder;

mod binary_tree;
mod creating_patterns;
mod enum_definition1;
mod enum_definition2;
mod errors;
mod matching;
mod result_exercise;
mod safe_division;
mod structural_matching;
mod tic_tac_toe;

weblab_folder!(
    "enums and matching",
    matching,
    enum_definition1,
    enum_definition2,
    safe_division,
    result_exercise,
    structural_matching,
    creating_patterns,
    binary_tree,
    tic_tac_toe
);
