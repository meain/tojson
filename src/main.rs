use std::io::{self, Read};
use structopt::StructOpt;

use serde_json;
use serde_yaml;
use toml;

#[derive(StructOpt, Debug)]
#[structopt(name = "tojson", about = "Convert from differnt formats to json")]
struct Opts {
    #[structopt(short, long, default_value = "auto", possible_values = &["auto", "yaml", "toml", "json"])]
    from: String,
    #[structopt(short, long, default_value = "json", possible_values = &["json", "yaml", "toml"])]
    to: String,
    #[structopt(short, long)]
    pretty: bool,
    filename: Option<String>,
}

fn get_file_content(filename: &str) -> String {
    let path_string = format!("{}", filename);
    std::fs::read_to_string(path_string).expect(&format!("Could not read file {}.", filename))
}

fn from_toml<T>(text: &str) -> T
where
    T: for<'de> serde::Deserialize<'de> + serde::ser::Serialize,
{
    toml::from_str(text).expect("Unable to parse file.")
}

fn from_yaml<T>(text: &str) -> T
where
    T: for<'de> serde::Deserialize<'de>,
{
    serde_yaml::from_str(&text).expect("Unable to parse file.")
}

fn from_json<T>(text: &str) -> T
where
    T: for<'de> serde::Deserialize<'de> + serde::ser::Serialize,
{
    serde_json::from_str(&text).expect("Unable to parse file.")
}

fn to_json<T>(value: T, pretty: bool) -> String
where
    T: for<'de> serde::Deserialize<'de> + serde::ser::Serialize,
{
    if pretty {
        serde_json::to_string_pretty(&value).expect("Unable to convert to json.")
    } else {
        serde_json::to_string(&value).expect("Unable to convert to json.")
    }
}

fn to_toml<T>(value: T, pretty: bool) -> String
where
    T: for<'de> serde::Deserialize<'de> + serde::ser::Serialize,
{
    if pretty {
        toml::to_string_pretty(&value).expect("Unable to convert to toml.")
    } else {
        toml::to_string(&value).expect("Unable to convert to toml.")
    }
}

fn to_yaml<T>(value: T, _pretty: bool) -> String
where
    T: for<'de> serde::Deserialize<'de> + serde::ser::Serialize,
{
    serde_yaml::to_string(&value).expect("Unable to convert to yaml.")
}

#[derive(Debug)]
enum Format {
    Toml,
    Yaml,
    Json,
}

impl Format {
    fn to_string(&self) -> String {
        match &self {
            Format::Toml => "toml".to_string(),
            Format::Yaml => "yaml".to_string(),
            Format::Json => "json".to_string(),
        }
    }
}

fn convert(content: &str, from: &Format, to: &Format, pretty: bool) -> String {
    match (from, to) {
        (Format::Json, Format::Json) => to_json(from_json::<serde_json::Value>(&content), pretty),
        (Format::Json, Format::Toml) => to_toml(from_json::<toml::Value>(&content), pretty),
        (Format::Json, Format::Yaml) => to_yaml(from_json::<serde_yaml::Value>(&content), pretty),
        (Format::Toml, Format::Json) => to_json(from_toml::<serde_json::Value>(&content), pretty),
        (Format::Toml, Format::Toml) => to_toml(from_toml::<toml::Value>(&content), pretty),
        (Format::Toml, Format::Yaml) => to_yaml(from_toml::<serde_yaml::Value>(&content), pretty),
        (Format::Yaml, Format::Json) => to_json(from_yaml::<serde_json::Value>(&content), pretty),
        (Format::Yaml, Format::Toml) => to_toml(from_yaml::<toml::Value>(&content), pretty),
        (Format::Yaml, Format::Yaml) => to_yaml(from_yaml::<serde_yaml::Value>(&content), pretty),
    }
}

fn main() {
    let opt = Opts::from_args();
    let from: Format = match opt.from.as_ref() {
        "auto" => match &opt.filename {
            Some(filename) => {
                if filename.ends_with("yaml") || filename.ends_with("yml") {
                    Format::Yaml
                } else if filename.ends_with("toml") {
                    Format::Toml
                } else if filename.ends_with("json") {
                    Format::Json
                } else {
                    panic!("Unable to identify input format.")
                }
            }
            None => panic!("Please specify input format when reading from stdin"),
        },
        "yaml" => Format::Yaml,
        "toml" => Format::Toml,
        "json" => Format::Json,
        _ => unreachable!(),
    };

    let to: Format = match opt.to.as_ref() {
        "yaml" => Format::Yaml,
        "toml" => Format::Toml,
        "json" => Format::Json,
        _ => unreachable!(),
    };

    let content = match &opt.filename {
        Some(filename) => get_file_content(&filename),
        None => {
            let mut buffer = String::new();
            let stdin = io::stdin();
            let mut handle = stdin.lock();
            handle
                .read_to_string(&mut buffer)
                .expect("Unable to read stdin.");
            buffer
        }
    };

    println!("{}", convert(&content, &from, &to, opt.pretty))
}

mod test {
    use crate::Format;

    fn ts(from: Format, to: Format) {
        let mut input_file = "testfiles/file.".to_string();
        input_file.push_str(&from.to_string());
        let mut output_file = "testfiles/file.".to_string();
        output_file.push_str(&to.to_string());
        assert_eq!(
            crate::convert(&crate::get_file_content(&input_file), &from, &to, false) + "\n",
            crate::get_file_content(&output_file)
        );

        let mut output_file = "testfiles/file.".to_string();
        output_file.push_str("pretty.");
        output_file.push_str(&to.to_string());
        println!("output_file: {:?}", output_file);
        assert_eq!(
            crate::convert(&crate::get_file_content(&input_file), &from, &to, true) + "\n",
            crate::get_file_content(&output_file)
        );
    }

    #[test]
    fn test_json_json() {
        ts(Format::Json, Format::Json);
    }
    #[test]
    fn test_json_yaml() {
        ts(Format::Json, Format::Yaml);
    }
    #[test]
    fn test_json_toml() {
        ts(Format::Json, Format::Toml);
    }

    #[test]
    fn test_toml_json() {
        ts(Format::Toml, Format::Json);
    }
    #[test]
    fn test_toml_yaml() {
        ts(Format::Toml, Format::Yaml);
    }
    #[test]
    fn test_toml_toml() {
        ts(Format::Toml, Format::Toml);
    }

    #[test]
    fn test_yaml_json() {
        ts(Format::Yaml, Format::Json);
    }
    #[test]
    fn test_yaml_yaml() {
        ts(Format::Yaml, Format::Yaml);
    }
    #[test]
    fn test_yaml_toml() {
        ts(Format::Yaml, Format::Toml);
    }
}
