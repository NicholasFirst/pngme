use std::path::PathBuf;

enum Commands {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

pub struct EncodeArgs {
    pub path: PathBuf,
    pub chunk_type: String,
    pub chunk_data: String,
}

pub struct DecodeArgs {
    pub path: PathBuf,
    pub chunk_type: String,
}

pub struct RemoveArgs {
    pub path: PathBuf,
    pub chunk_type: String,
}

pub struct PrintArgs {
    pub path: PathBuf,
}
