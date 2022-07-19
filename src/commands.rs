use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use crate::Result;
use clap::{arg, AppSettings, Arg, ArgMatches, Command};
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

pub struct PngMeCmd {
    arg_matchs: ArgMatches,
}

impl PngMeCmd {
    pub fn new() -> Self {
        let matchs = Command::new("pngme")
            .about("A png secret message Cli")
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .setting(AppSettings::AllowExternalSubcommands)
            .setting(AppSettings::AllowInvalidUtf8ForExternalSubcommands)
            .subcommand(
                Command::new("encode")
                .about("encode png secert message")
                .setting(AppSettings::ArgRequiredElseHelp)
                .arg(arg!(<PATH> "png path").allow_invalid_utf8(true))
                .arg(Arg::new("CHUNK_TYPE").required(true).help("chunk type (the first character must be uppercase for png can normal display)"))
                .arg(Arg::new("DATA").required(true).help("secret message")),
            )
            .subcommand(
                Command::new("decode")
                .about("decode png secert message")
                .setting(AppSettings::ArgRequiredElseHelp)
                .arg(arg!(<PATH> "png path").allow_invalid_utf8(true))
                .arg(Arg::new("CHUNK_TYPE").required(true).help("chunk type (the first character must be uppercase for png can normal display)"))
            ).subcommand(
                Command::new("remove")
                .about("移除加密内容")
                .setting(AppSettings::ArgRequiredElseHelp)
                .arg(arg!(<PATH> "png path").allow_invalid_utf8(true))
                .arg(Arg::new("CHUNK_TYPE").required(true).help("chunk type (the first character must be uppercase for png can normal display)"))
            ).subcommand(
                Command::new("print")
                .about("打印")
                .setting(AppSettings::ArgRequiredElseHelp)
                .arg(arg!(<PATH> "png path").allow_invalid_utf8(true))
            )
            .get_matches();
        PngMeCmd { arg_matchs: matchs }
    }

    pub fn match_handler(&self) {
        match self.arg_matchs.subcommand() {
            Some(("encode", sub_matches)) => {
                let paths = sub_matches.value_of_os("PATH").unwrap_or_default();
                let data = sub_matches.value_of("DATA").unwrap();
                let chunk_type = sub_matches.value_of("CHUNK_TYPE").unwrap();
                let p = PathBuf::from(paths);
                let arg = EncodeArgs {
                    path: p,
                    chunk_type: chunk_type.to_string(),
                    chunk_data: data.to_string(),
                };
                if let Err(e) = encode(arg) {
                    println!("{:?}", e.to_string());
                }
            }
            Some(("decode", sub_matches)) => {
                let paths = sub_matches.value_of_os("PATH").unwrap_or_default();
                let chunk_type = sub_matches.value_of("CHUNK_TYPE").unwrap();
                let p = PathBuf::from(paths);
                let arg = DecodeArgs {
                    path: p,
                    chunk_type: chunk_type.to_string(),
                };
                if let Err(e) = decode(arg) {
                    println!("{:?}", e.to_string())
                }
            }
            Some(("remove", sub_matches)) => {
                let paths = sub_matches.value_of_os("PATH").unwrap_or_default();
                let chunk_type = sub_matches.value_of("CHUNK_TYPE").unwrap();
                let p = PathBuf::from(paths);
                let arg = RemoveArgs {
                    path: p,
                    chunk_type: chunk_type.to_string(),
                };
                if let Err(e) = remove(arg) {
                    println!("{:?}", e.to_string());
                }
            }
            Some(("print", sub_marches)) => {
                let paths = sub_marches.value_of_os("PATH").unwrap_or_default();
                let p = PathBuf::from(paths);
                let arg = PrintArgs { path: p };
                if let Err(e) = print_chunks(arg) {
                    println!("{:?}", e.to_string());
                }
            }
            _ => unreachable!(),
        }
    }
}

/// Encodes a message into a PNG file and saves the result
pub fn encode(args: EncodeArgs) -> Result<()> {
    let mut png = Png::from_file(&args.path)?;

    let chunk_type = ChunkType::from_str(args.chunk_type.as_str())?;

    let chunk_data = args.chunk_data.as_bytes();
    let chunk = Chunk::new(chunk_type, chunk_data.to_vec());
    png.append_chunk(chunk);
    fs::write(args.path, png.as_bytes())?;
    Ok(())
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: DecodeArgs) -> Result<()> {
    let png = Png::from_file(&args.path)?;
    let res = png.chunk_by_type(&args.chunk_type);
    match res {
        Some(res) => println!("data: {}", std::str::from_utf8(res.chunk_data.as_slice())?),
        None => println!("not exists"),
    }
    Ok(())
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: RemoveArgs) -> Result<()> {
    let mut png = Png::from_file(&args.path)?;
    let chunk_type = args.chunk_type.as_str();
    png.remove_chunk(chunk_type)?;
    fs::write(args.path, png.as_bytes())?;
    Ok(())
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks(args: PrintArgs) -> Result<()> {
    let png = Png::from_file(&args.path)?;
    println!("{}", png.to_string());
    Ok(())
}
