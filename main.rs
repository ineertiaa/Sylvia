use std::process::Command;
use std::io::Write;
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

use serde_json::Value;
use console::{Key, Term};

use std::io;

use colored::Colorize;

// dear programmer (or user) reading this,
// there may be inconsistencies or imperfections within,
// but the real code is the friends we made along the way
// good luck.

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

    let _ = homescreen();

    Ok(())
}

fn homescreen() -> Result<(), Box<dyn std::error::Error>>{
    clearscreen::clear().expect("couldn't clear.");

    typewriter(TITLE, (204, 71, 184), 3);

    let nixver = Command::new("nix")
        .arg("--version")
        .output()
        .expect("Couldn't get nix version.");

    println!("nixver: {}", String::from_utf8_lossy(&nixver.stdout));

    if internetcheck(){
        println!("{}", "Sylvia is ready to go. \n".magenta().bold());
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
    print!("{} - {}", "Q".black().on_red(), "Quit \n \n".red());

    std::io::stdout().flush().expect("output flushing didn't work.");

    let term = Term::stdout();

    loop {
        match term.read_key()? {
            Key::Char('s') | Key::Char('S') => {
                search();
                break;
            }

            Key::Char('l') | Key::Char('L') => {
                let _ = listpkgs();
                break;
            }

            Key::Char('q') | Key::Char('Q') => {
                println!("{}", "Quitting, goodbye!".bold());
                std::thread::sleep(std::time::Duration::from_millis(1000));
                std::process::exit(0);
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
    print!(" ");
    std::io::stdout().flush().expect("output flushing didn't work.");

    let mut buffer = String::new();
    let _input = io::stdin().read_line(&mut buffer);
    println!("{}", "searching...".bold());
    let _ = display_pkginfo(buffer.trim());
}

fn listpkgs() -> Result<(), Box<dyn std::error::Error>>{
    clearscreen::clear().expect("couldn't clear.");

    let _output = Command::new("nix")
        .args(["profile", "list", "--json"])
        .output()
        .expect("couldn't list installed packages.");

    let json: Value = serde_json::from_str(&String::from_utf8_lossy(&_output.stdout)).expect("couldn't convert package string to json :(");

    for name in json["elements"].as_object().expect("missing elements :(").keys() {
        let nameandnewline = format!("{} \n", name); //sorry for bad variable name im tired

        typewriter(&nameandnewline, (255, 255, 255), 10);
    }

    println!("\n{}", "Press any key to return home. \n".bold());

    let term = Term::stdout();
    let _key = term.read_key()?;

    let _ = homescreen();

    Ok(())
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
    }

    println!("\n{}", "Press any key to return home. \n".bold());

    let term = Term::stdout();
    let _key = term.read_key()?;
    
    let _ = homescreen();

    Ok(())
}

fn test(pkgname: &str) -> Result<(), Box<dyn std::error::Error>>{
    clearscreen::clear().expect("couldn't clear.");

    println!("testing package...");
    println!("{}", "press Q to quit the command.".yellow().bold());

    std::thread::sleep(std::time::Duration::from_millis(1000));

    clearscreen::clear().expect("couldn't clear.");

    let pkgquery = format!("nixpkgs#{}", pkgname);

    let _output = Command::new("nix")
        .args(["run", &pkgquery])
        .status()
        .expect("couldn't test package.");

    print!("\x1b[?25h"); 
    std::io::stdout().flush().unwrap();

    println!("\n{}", "Press any key to go back. \n".bold());

    let term = Term::stdout();
    let _key = term.read_key()?;

    let _ = display_pkginfo(&pkgname);

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
    let json = get_pkginfo(pkgname).unwrap();

    clearscreen::clear().expect("couldn't clear.");

    let name = json.get("mainProgram").and_then(Value::as_str).unwrap();
    let description = json.get("description").and_then(Value::as_str).unwrap();
    let website = json.get("homepage").and_then(Value::as_str).unwrap();

    println!("{} \n", name.bold());

    println!("Description: {:?}", description);
    println!("Homepage: {} \n \n", website);

    print!("{} - {}", "I".black().on_green(), "Install this package  ".green());
    print!("{} - {}", "S".black().on_blue(), "Search for another package  ".blue());
    print!("{} - {}", "T".black().on_yellow(), "Test this package \n \n".yellow());

    if checkinstalled(&pkgname){
        print!("{} - {}", "U".black().on_red(), "Uninstall this package  ".red());
    }

    std::io::stdout().flush().expect("output flushing didn't work.");

    let term = Term::stdout();

    loop {
        match term.read_key()? {
            Key::Char('i') | Key::Char('I') => {
                println!();
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

            Key::Char('u') | Key::Char('U') => {
                if checkinstalled(&pkgname){
                    let _ = uninstallpkg(&pkgname);
                }
            }

            _ => {}
        }
    }

    Ok(())
}

fn uninstallpkg(pkg: &str) -> Result<(), Box<dyn std::error::Error>>{
    let pkgquery = format!("{}", pkg);

    let output = Command::new("nix")
        .args(["profile", "remove", &pkgquery])
        .output()
        .expect("couldn't remove package.");

    // not needed, it just adds clunkiness to the terminal that people don't care about.
    // println!("\n{:?}", String::from_utf8_lossy(&outpout.stderr));

    if output.status.success(){
        println!("\nremoved package!");
    }
    else{
        println!("{}", "package removing failed..".red());
    }

    println!("\n{}", "Press any key to return home.".bold());

    let term = Term::stdout();
    let _key = term.read_key()?;

    let _ = homescreen();

    Ok(())
}

fn checkinstalled(pkg: &str) -> bool {
    Command::new("nix")
        .args(["profile", "list"])
        .output()
        .map(|o| {
            let out = String::from_utf8_lossy(&o.stdout);

            out.lines().any(|line| {
                line.trim() == format!("Name: {pkg}")
                    || line.trim().ends_with(&format!(".{pkg}"))
            })
        })
        .unwrap_or(false)
}

fn typewriter(target: &str, colour: (u8, u8, u8), speed_milis: u64){
    let (r, g, b) = colour;

    for cha in target.chars() {
        print!("{}", cha.to_string().truecolor(r, g, b));
        std::io::stdout().flush().expect("output flushing didn't work.");
        std::thread::sleep(std::time::Duration::from_millis(speed_milis));
    }
}
