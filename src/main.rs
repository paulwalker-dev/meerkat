use clap::Parser;
use std::fs;
use std::io;
use std::path::Path;

#[derive(Parser, Debug)]
struct Args {
  /// Input TOML file
  input: String,

  /// Output JSON file
  #[clap(short, long)]
  out: Option<String>,
}

fn main() -> Result<(), io::Error> {
  let args = Args::parse();
  let toml_contents = parse_toml_file(&args.input)?;
  let output_path = match args.out {
    Some(path) => path,
    None => basename(&args.input) + ".json",
  };
  write_json_file(output_path, toml_contents)?;
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
  if path == "-" {
    print!("{}", json_string);
    return Ok(());
  }
  fs::write(path, json_string)?;
  Ok(())
}
