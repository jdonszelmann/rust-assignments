use weblab::weblab_folder;

mod enum_definition;
mod safe_division;
mod tic_tac_toe;
mod structural_matching;
mod creating_patterns;
mod matching;
mod errors;

weblab_folder!("enums and matching", enum_definition, safe_division, matching, structural_matching, creating_patterns, tic_tac_toe);
