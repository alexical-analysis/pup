use std::path::PathBuf;

use crate::compiler::str_store::MStr;

pub struct Module {
    name: MStr,
    dir: PathBuf,
}
