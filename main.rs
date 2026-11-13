use std::process::Command;
use std::io::Write;
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

use serde_json::Value;
use console::{Key, Term};

use std::io;

use colored::Colorize;

const TITLE: &str = "
  ▄▄▄▄         ▀▀█             ▀          
 █▀   ▀ ▄   ▄    █    ▄   ▄  ▄▄▄     ▄▄▄  
 ▀█▄▄▄  ▀▄ ▄▀    █    ▀▄ ▄▀    █    ▀   █ 
     ▀█  █▄█     █     █▄█     █    ▄▀▀▀█ 
 ▀▄▄▄█▀  ▀█      ▀▄▄    █    ▄▄█▄▄  ▀▄▄▀█ 
         ▄▀                               

";

fn main() -> Result<(), Box<dyn std::error::Error>>{
    clearscreen::clear().expect("couldn't clear.");

    //let font = Toilet::mono9().unwrap();
    //let title = font.convert("Sylvia").expect("couldn't make title ascii :(");
    //println!("{}", title.to_string().truecolor(212, 123, 198));

    let _ = homescreen();

    Ok(())
}

fn homescreen() -> Result<(), Box<dyn std::error::Error>>{
    showtitle(TITLE);

    let nixver = Command::new("nix")
        .arg("--version")
        .output()
        .expect("Couldn't get nix version.");

    println!("nix: {}", String::from_utf8_lossy(&nixver.stdout));

    if internetcheck(){
        println!("{}", "Sylvia is ready to go. \n".truecolor(204, 71, 184).bold());
        let _ = homeoptions();
    }
    else{
        println!("{}", "Error: 8.8.8.8 DNS ping failed. Check your internet!".red().bold());
    }

    Ok(())
}

fn homeoptions() -> Result<(), Box<dyn std::error::Error>>{
    print!("{} - {}", "S".black().on_yellow(), "Search for Packages  ".yellow());
    print!("{} - {}", "L".black().on_cyan(), "List installed Packages  ".cyan());

    std::io::stdout().flush().expect("output flushing didn't work.");

    let term = Term::stdout();

    loop {
        match term.read_key()? {
            Key::Char('s') | Key::Char('S') => {
                search();
                break;
            }

            Key::Char('l') | Key::Char('L') => {
                listpkgs();
                break;
            }

            _ => {}
        }
    }

    Ok(())
}

fn internetcheck() -> bool{
    let addr = SocketAddr::from(([8, 8, 8, 8], 53));
    let timeout = Duration::from_secs(3);

    TcpStream::connect_timeout(&addr, timeout).is_ok()
}

fn search(){
    clearscreen::clear().expect("couldn't clear.");
    println!("Search for a package:");
    let mut buffer = String::new();
    let _input = io::stdin().read_line(&mut buffer);
    let _ = display_pkginfo(buffer.trim());
}

fn listpkgs(){
    clearscreen::clear().expect("couldn't clear.");

    let _output = Command::new("nix")
        .args(["profile", "list"])
        .output()
        .expect("couldn't list installed packages.");

    println!("{}", String::from_utf8_lossy(&_output.stdout));

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

fn test(pkgname: &str){
    println!("testing package...");

    let pkgquery = format!("nixpkgs#{}", pkgname);

    let _output = Command::new("nix")
        .args(["run", &pkgquery])
        .status()
        .expect("couldn't test package.");
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

    print!("{} - {}", "I".black().on_green(), "Install this package  ".green());
    print!("{} - {}", "S".black().on_blue(), "Search for another package  ".blue());
    print!("{} - {}", "T".black().on_yellow(), "Test this package  ".yellow());

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

fn showtitle(target: &str){
    for cha in target.chars() {
        print!("{}", cha.to_string().truecolor(212, 123, 198));
        std::io::stdout().flush().expect("output flushing didn't work.");
        std::thread::sleep(std::time::Duration::from_millis(3));
    }
}
