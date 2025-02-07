use clap::Parser;
use cli::Cli;
use color_eyre::Result;

use crate::ui::app::App;

mod cli;
mod config;
mod errors;
mod logging;
mod repositories;
mod ui;

#[tokio::main]
async fn main() -> Result<()> {
    crate::errors::init()?;
    crate::logging::init()?;

    let args = Cli::parse();
    let mut app = App::new(args.tick_rate, args.frame_rate)?;
    app.run().await?;
    Ok(())
}

// use std::fs::{self, File};
// use std::io::{self, BufRead};
// use std::path::Path;
// // use std::process::Command;
// use regex::Regex;

// mod repositories;
// use crate::repositories::Repository;

// fn is_privileged() -> bool {
//     let euid = unsafe { libc::geteuid() };

//     match euid {
//         0 => true,
//         _ => false,
//     }
// }

// // fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
// // where
// //     P: AsRef<Path>,
// // {
// //     let file = File::open(filename)?;
// //     Ok(io::BufReader::new(file).lines())
// // }

// fn main() {
//     println!("Hello, world!");

//     if is_privileged() {
//         println!("Privileged");
//     } else {
//         println!("Not privileged");
//     }

//     let mut apt_repositories = repositories::apt::AptRepositories::default();
//     let apt_load_result = apt_repositories.load_repository_list();
//     match apt_load_result {
//         Ok(count) => println!("{} Apt repositories loaded", count),
//         Err(e) => println!("Error loading apt repositories: {}", e),
//     }

//     // let apt_list_d = fs::read_dir("/etc/apt/sources.list.d").unwrap();
//     // for path in apt_list_d {
//     //     println!("Name: {}", path.unwrap().path().display());
//     // }
//     // // let apt_list = fs::read_dir("/etc/apt/sources.list").unwrap();
//     // // for path in apt_list {
//     // //     println!("Name: {}", path.unwrap().path().display());
//     // // }

//     // let contents =
//     //     fs::read_to_string("/etc/apt/sources.list.d/fish-shell-ubuntu-beta-4-jammy.list").unwrap();

//     // println!("{}", contents);

//     // if let Ok(lines) = read_lines("/etc/apt/sources.list.d/vscode.list") {
//     //     // Consumes the iterator, returns an (Optional) String
//     //     // for line in lines.map_while(Result::ok) {
//     //     //     println!("{}", line);
//     //     // }
//     //     let regex = Regex::new("^\\s{0,}#{0,}\\s{0,}deb").unwrap();
//     //     let filtered_lines = lines.filter(|x| match x {
//     //         Ok(line) => regex.is_match(&line),
//     //         Err(_) => false,
//     //     });

//     //     for line in filtered_lines {
//     //         println!("{}", line.unwrap());
//     //     }
//     // }

//     // let cmd = Command::new("docker")
//     //     .arg("ps")
//     //     .output()
//     //     .expect("ls command failed to start");

//     // let result = match str::from_utf8(&cmd.stdout) {
//     //     Ok(v) => v.to_string(),
//     //     Err(e) => format!("Error converting stdout to string: {}", e),
//     // };

//     // println!("{}", result);

//     // let ls_cmd = Command::new("ls")
//     //     .arg("-l")
//     //     .output()
//     //     .expect("ls command failed to start");

//     // let ls_cmd_result = match str::from_utf8(&ls_cmd.stdout) {
//     //     Ok(v) => v.to_string(),
//     //     Err(e) => format!("Error converting stdout to string: {}", e),
//     // };

//     // println!("{}", ls_cmd_result);

//     // let flatpak_cmd = Command::new("flatpak")
//     //     .arg("list")
//     //     .output()
//     //     .expect("flatpak command failed to start");

//     // let flatpak_cmd_result = match str::from_utf8(&flatpak_cmd.stdout) {
//     //     Ok(v) => v.to_string(),
//     //     Err(e) => format!("Error converting stdout to string: {}", e),
//     // };

//     // println!("{}", flatpak_cmd_result);

//     // let which_cmd = Command::new("which")
//     //     .arg("qwert")
//     //     .output()
//     //     .expect("which command failed to start");

//     // let which_cmd_result = match str::from_utf8(&which_cmd.stdout) {
//     //     Ok(v) => v.to_string(),
//     //     Err(e) => format!("Error converting stdout to string: {}", e),
//     // };

//     // println!("{}", which_cmd_result);
//     // let sudo = Command::new("sudo ls")
//     //     .arg("/")
//     //     .output()
//     //     .expect("which command failed to start");

//     // let sudo_result = match str::from_utf8(&sudo.stdout) {
//     //     Ok(v) => v.to_string(),
//     //     Err(e) => format!("Error converting stdout to string: {}", e),
//     // };

//     // println!("{}", sudo_result);

//     // let complex_cmd = Command::new("dpkg")
//     //     .arg("-l")
//     //     .output()
//     //     .expect("which command failed to start");

//     // let complex_cmd_result = match str::from_utf8(&complex_cmd.stdout) {
//     //     Ok(v) => v.to_string(),
//     //     Err(e) => format!("Error converting stdout to string: {}", e),
//     // };

//     // println!("{}", complex_cmd_result);
// }
