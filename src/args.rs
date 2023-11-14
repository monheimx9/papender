use clap::{command, Arg, ArgAction, ArgMatches};
use config::{Config, File};
use std::collections::HashMap;
use std::env;
use std::path::Path;

pub struct LesOptions {
    pub input: Option<String>,
    pub output: Option<String>,
    pub no_resize: bool,
    pub resize: Option<u32>,
    pub flagos: Option<Vec<String>>,
}

pub fn config_load() -> LesOptions {
    let settings = Config::builder()
        .add_source(File::from(Path::new("./cfg.json")))
        .build()
        .unwrap()
        .try_deserialize::<HashMap<String, String>>()
        .unwrap();
    println!("\n{:?} ", &settings);
    {
        for (setty, villy) in &settings {
            println!("{:?}", &setty);
            println!("{:?}", &villy);
        }
    }
    let les_options = arg_parser_clap();
    LesOptions {
        flagos: match les_options.flagos {
            Some(a) => parse_flagos(a, settings),
            None => None,
        },
        ..les_options
    }
}

fn parse_flagos(
    los_flagos: Vec<String>,
    config_map: HashMap<String, String>,
) -> Option<Vec<String>> {
    let mut p = Vec::new();
    for flagos in los_flagos {
        let fla = config_map.get(&flagos);
        match fla {
            Some(fl) => p.push(fl.clone()),
            None => println!("Arg not found: {:?}", flagos),
        };
    }
    Some(p)
}

fn arg_parser_clap() -> LesOptions {
    let match_result: ArgMatches = command!()
        .arg(
            Arg::new("input")
                .required(true)
                .short('i')
                .help("Input image (*.png)"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .default_value("result.png")
                .help("Output name for the image (by default result.png)"),
        )
        .arg(
            Arg::new("no-resize")
                .conflicts_with("resize")
                .action(ArgAction::SetTrue)
                .short('R')
                .help("Disable resize of the input image (by default y:300 x:?)"),
        )
        .arg(
            Arg::new("resize")
                .conflicts_with("no-resize")
                .default_value("400")
                .short('r')
                .help("Set the resize height value only in pixels"),
        )
        .arg(
            Arg::new("flags")
                .short('f')
                .help("Pictures to be concatened, see cfg.json"),
        )
        .get_matches();
    let les_options = LesOptions {
        input: match_result
            .get_one::<String>("input")
            .map(|s| s.to_string()),
        output: match_result
            .get_one::<String>("output")
            .map(|s| s.to_string()),
        no_resize: match_result.get_flag("no-resize"),
        resize: match_result.get_one::<u32>("resize").copied(),
        flagos: {
            match match_result.get_one::<String>("flags") {
                Some(a) => parse_flags(a),
                None => None,
            }
        },
    };
    les_options
}

fn parse_flags(commands: &str) -> Option<Vec<String>> {
    Some(commands.split_whitespace().map(|s| s.to_string()).collect())
}
