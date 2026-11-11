use std::process::Command;
use serde_json::Value;

fn main(){
    let output = Command::new("echo")
        .arg("Hello")
        .output()
        .expect("Couldn't execute");

    display_page(&String::from_utf8_lossy(&output.stdout), "hello");
}

fn display_page(title: &str, content: &str){
    println!("{} \n", title);

    let stuff = get_pkginfo("fastfetch").unwrap();

    println!("{:?}", stuff);
}

fn get_pkginfo(pkgname: &str) -> Result<String, Box<dyn std::error::Error>>{
    let namequery = format!("nixpkgs#{}.meta", pkgname);

    let output = Command::new("nix")
        .args(["eval", "--json", &namequery])
        .output()
        .expect("Couldn't get info on package");

    let parsed: Value = serde_json::from_slice(&output.stdout)?;

    let description = parsed
        .get("description")
        .and_then(Value::as_str)
        .unwrap_or("No description.");

    Ok(description.to_owned())
}
