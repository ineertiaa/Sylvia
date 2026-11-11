use std::process::Command;
use serde_json::Value;
use console::{Key, Term};

use std::io;

use colored::Colorize;
use figlet_rs::{FIGlet, Toilet};

fn main() -> Result<(), Box<dyn std::error::Error>>{
    clearscreen::clear().expect("couldn't clear.");

    let font = Toilet::mono9().unwrap();
    let title = font.convert("Sylvia").expect("couldn't make title ascii :(");
    println!("{}", title.to_string().truecolor(212, 123, 198));
    println!("{}", "press any key to search.".truecolor(204, 71, 184));

    let term = Term::stdout();
    let _ = term.read_key()?;

    search();

    Ok(())
}

fn search(){
    clearscreen::clear().expect("couldn't clear.");
    println!("Search for a package:");
    let mut buffer = String::new();
    let _input = io::stdin().read_line(&mut buffer);
    let _ = display_pkginfo(buffer.trim());
}

fn install(pkgname: &str) -> Result<(), Box<dyn std::error::Error>>{
    let pkgquery = format!("nixpkgs#{}", pkgname);

    let output = Command::new("nix")
        .args(["profile", "add", &pkgquery])
        .output()
        .expect("Couldn't install package");

    if output.status.success(){
        println!("{}", String::from_utf8_lossy(&output.stderr));
        println!("Installed!");

        let term = Term::stdout();
        println!("Quit? (Y/n)");

        loop {
            match term.read_key()? {
                Key::Char('y') | Key::Char('Y') => {
                    clearscreen::clear().expect("couldn't clear.");
                    std::process::exit(0);
                }

                Key::Char('n') | Key::Char('N') => {
                    search();
                    break;
                }

                _ => {}
            }
        }

    }

    Ok(())
}

fn test(pkgname: &str) -> Result<(), Box<dyn std::error::Error>>{
    println!("testing package...");

    let pkgquery = format!("nixpkgs#{}", pkgname);

    let output = Command::new("nix")
        .args(["run", &pkgquery])
        .status()
        .expect("couldn't test package.");

    println!("Quit? (Y/n)");

    let term = Term::stdout();

    loop {
        match term.read_key()? {
            Key::Char('y') | Key::Char('Y') => {
                clearscreen::clear().expect("couldn't clear.");
                std::process::exit(0);
            }

            Key::Char('n') | Key::Char('N') => {
                search();
                break;
            }

            _ => {}
        }
    }

    Ok(())
}
fn get_pkginfo(pkgname: &str) -> Result<Value, Box<dyn std::error::Error>>{
    let namequery = format!("nixpkgs#{}.meta", pkgname);

    let output = Command::new("nix")
        .args(["eval", "--json", &namequery])
        .output()
        .expect("Couldn't get info on package");

    if !output.status.success(){
        search();
    }

    let parsed: Value = serde_json::from_slice(&output.stdout)?;

    Ok(parsed.to_owned())
}

fn display_pkginfo(pkgname: &str) -> Result<(), Box<dyn std::error::Error>>{
    clearscreen::clear().expect("couldn't clear.");

    let json = get_pkginfo(pkgname).unwrap();

    let name = json.get("mainProgram").and_then(Value::as_str).unwrap();
    let description = json.get("description").and_then(Value::as_str).unwrap();
    let website = json.get("homepage").and_then(Value::as_str).unwrap();

    println!("{} \n", name.bold());

    println!("Description: {:?}", description);
    println!("Homepage: {} \n \n", website);

    println!("{} - {}", "I".black().on_green(), "Install this package".green());
    println!("{} - {}", "S".black().on_blue(), "Search for another package".blue());
    println!("{} - {}", "T".black().on_yellow(), "Test this package".yellow());

    println!();

    let term = Term::stdout();

    loop {
        match term.read_key()? {
            Key::Char('i') | Key::Char('I') => {
                println!("installing...");
                let _ = install(&pkgname);
            }

            Key::Char('s') | Key::Char('S') => {
                search();
                break;
            }

            Key::Char('t') | Key::Char('T') => {
                let _ = test(&pkgname);
                break;
            }

            _ => {}
        }
    }

    Ok(())
}
