use std::io::{self, Read};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "tojson", about = "Convert from differnt formats to json")]
struct Opts {
    #[structopt(short, long, default_value = "auto", possible_values = &["auto", "yaml", "toml"])]
    from: String,
    #[structopt(short, long)]
    pretty: bool,
    filename: Option<String>,
}

fn get_file_content(filename: &str) -> String {
    let path_string = format!("{}", filename);
    std::fs::read_to_string(path_string).expect(&format!("Could not read file {}.", filename))
}

fn from_toml(text: &str, pretty: bool) -> String {
    use toml::Value;
    let value = text.parse::<Value>().unwrap();
    if pretty {
        serde_json::to_string_pretty(&value).expect("Unable to convert")
    } else {
        serde_json::to_string(&value).expect("Unable to convert")
    }
}

fn from_yaml(text: &str, pretty: bool) -> String {
    use serde_yaml::Value;
    let value: Value = serde_yaml::from_str(&text).unwrap();
    if pretty {
        serde_json::to_string_pretty(&value).expect("Unable to convert")
    } else {
        serde_json::to_string(&value).expect("Unable to convert")
    }
}

fn main() {
    let opt = Opts::from_args();
    if let Some(file) = &opt.filename {
        if opt.from == "auto" {
            if file.ends_with("yaml") {
                println!("{}", from_yaml(&get_file_content(&file), opt.pretty));
            } else if file.ends_with("toml") {
                println!("{}", from_toml(&get_file_content(&file), opt.pretty));
            } else {
                eprintln!("Unknown format for {}", file);
            }
        } else if opt.from == "yaml" {
            println!("{}", from_yaml(&get_file_content(&file), opt.pretty));
        } else if opt.from == "toml" {
            println!("{}", from_toml(&get_file_content(&file), opt.pretty));
        } else {
            eprintln!("Unknown format {}.", opt.from);
            std::process::exit(1);
        }
    } else {
        // Use stdin if filename not available
        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        handle.read_to_string(&mut buffer).unwrap();

        if opt.from == "yaml" {
            println!("{}", from_yaml(&buffer, opt.pretty));
        } else if opt.from == "toml" {
            println!("{}", from_toml(&buffer, opt.pretty));
        } else {
            eprintln!("Need to specify a format for stdin");
            std::process::exit(1);
        }
    }
}

#[test]
fn test_toml_to_json() {
    assert_eq!(
        from_toml(&get_file_content("testfiles/input.toml"), false) + "\n",
        get_file_content("testfiles/output.toml.json")
    )
}

#[test]
fn test_yaml_to_json() {
    assert_eq!(
        from_yaml(&get_file_content("testfiles/input.yaml"), false) + "\n",
        get_file_content("testfiles/output.yaml.json")
    )
}

#[test]
fn test_toml_to_json_pretty() {
    assert_eq!(
        from_toml(&get_file_content("testfiles/input.toml"), true) + "\n",
        get_file_content("testfiles/output.toml.pretty.json")
    )
}

#[test]
fn test_yaml_to_json_pretty() {
    assert_eq!(
        from_yaml(&get_file_content("testfiles/input.yaml"), true) + "\n",
        get_file_content("testfiles/output.yaml.pretty.json")
    )
}
