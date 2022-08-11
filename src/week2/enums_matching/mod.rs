use weblab::weblab_folder;

mod creating_patterns;
mod enum_definition;
mod errors;
mod matching;
mod safe_division;
mod structural_matching;
mod tic_tac_toe;

weblab_folder!(
    "enums and matching",
    enum_definition,
    safe_division,
    matching,
    structural_matching,
    creating_patterns,
    tic_tac_toe
);
