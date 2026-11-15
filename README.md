<img width="380" height="139" alt="Sylvia" src="https://github.com/user-attachments/assets/1632d0b5-8025-4b58-9793-dc773c580647" />

# Sylvia Package Manager Shell
> (originally built for Sylvan OS)

## Nix turned into an easily usable Terminal App
- No CLI experience needed
- Add, remove and test packages in a few key presses

https://github.com/user-attachments/assets/5b81c258-ec0b-42d0-8fe6-8adeec902d08

# How to use
## Requirements
- Cargo
- Rust

1. Clone the repo and enter the folder
2. Run `cargo install --path .`

## For BaSH
Add `export PATH="$HOME/.cargo/bin:$PATH` to the end of your .zshrc or run `echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc`
Then run `source ~/.bashrc`

## For ZSH
Add `export PATH="$HOME/.cargo/bin:$PATH` to the end of your .zshrc or run `echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc`
Then run `source ~/.zshrc`

## For FiSH
run `fish_add_path $HOME/.cargo/bin`
