use std::io::{stdout, Write};
use clap::ArgMatches;
use clap::App;
use clap::AppSettings;
use curl::easy::Easy;

fn main() {
    let app = App::new(env!("CARGO_CRATE_NAME"))
        .author("OiranCage Team")
        .version("0.0.1")
        .about("plugin manager for pocketmine-mp")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(App::new("init").about("initialize plugins folder"))
        .subcommand(App::new("list").about("show list of installed plugins"))
        .subcommand(App::new("install").about("install plugin from repository"))
        .subcommand(App::new("update").about("update plugin from repository"))
        .subcommand(App::new("remove").about("remove plugin"));

    let matches = app.get_matches();

    match matches.subcommand() {
        Some(("init", _sub_matches)) => {initialize()},
        Some(("list", _sub_matches)) => {},
        Some(("install", _sub_matches)) => {},
        Some(("update", sub_matches)) => {update(sub_matches)}
        Some(("remove", _sub_matches)) => {},
        _ => {}
    }
}

fn initialize(){
    
}

fn update(sub_matches: &ArgMatches){
    let url = "https://poggit.pmmp.io/releases.json";
    let mut easy = Easy::new();
    easy.url(url).unwrap();
    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();

    println!("{}", easy.response_code().unwrap());
}