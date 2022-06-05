use clap::Parser;
use std::fs;
use std::io;
use std::path::Path;

#[derive(Parser, Debug)]
struct Args {
  input: String,
}

fn main() -> Result<(), io::Error> {
  let args = Args::parse();
  let toml_contents = parse_toml_file(&args.input)?;
  write_json_file(basename(&args.input) + ".json", toml_contents)?;
  Ok(())
}

fn parse_toml_file(path: &str) -> Result<toml::Value, io::Error> {
  let file_raw = fs::read(path)?;
  let file_contents = String::from_utf8(file_raw).unwrap();
  Ok(toml::from_str(&file_contents)?)
}

fn basename(path: &str) -> String {
  match Path::new(path).file_stem() {
    Some(stem) => stem.to_str().expect("Unable to convert OsStr to &str"),
    None => path,
  }
  .to_string()
}

fn write_json_file(path: String, value: toml::Value) -> Result<(), io::Error> {
  let json_string = serde_json::to_string(&value)?;
  fs::write(path, json_string)?;
  Ok(())
}
