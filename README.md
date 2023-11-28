# Dotypasta

Dotypasta let you easily fetch, save and apply dotfiles.  
Inspired by [chezmoi](https://chezmoi.io)

## Installation

**dotypasta is still in early development**  
If you want to give it a shot, clone the repository and `cargo build`

## Convention & Configuration

Dotypasta relies on two things:
- a `.dotypastarc` in your home directory, it tells dotypasta which files must be copied.
- a `dotypasta` folder at `~/.local/share/`, usually a git repostory, where all the dotfiles are stored.

##### ~/.dotypastarc

This is a simple flat toml file.
```toml
dotypasta = [".dotypastarc"]
zsh = [".zshrc", ".oh-my-zsh/custom/themes/nizil.zsh"]
i3 = [".config/i3/config", ".config/i3blocks/config"]
vi = [".vimrc", ".config/nvim/init.vim"]
```
With such config file, dotypasta has 4 *apps* defined (dotypasta, zsh, i3 and vi) with their respective files. It expects to find this files in the dotypasta directory (see below).

##### ~/.local/share/dotypasta/

Using the `load` subcommand, dotypasta clone a repository named `dotfiles` from a specified user at `$HOME/.local/share/dotypasta`.  
This directory must respect a structure commonly used among `dotfiles` repository, i.e. the root of the directory is supposed to be your home.

In the previous example, the repository must have `.dotypastarc`, `.zshrc`, `.vimrc` at the root of the repository, a `config` file in a directory `.config/i3/`, and so on.

## Features roadmap

- [x] `dotypasta load <username>` clone a `dotfiles` repository
  - [x] `--hub [github | gitlab | bitbucket ... ]` to select the git host 
  - [x] `-t <reference>` to load a specific tag or branch
    - [ ] fix the detached head issue
  - [x] `--ssh` to clone the repository using ssh
  - [x] `--apply` to directly apply the configuration
- [x] `dotypasta clear` to delete `$HOME/.local/share/dotypasta`
- [x] `dotypasta app <appname>` show you the files copied for `appname` as configured in `.dotypastarc`
  - [x] `dotypasta app <appname> -a <file> -a <file2>...` add all files to the `appname` configuration
  - [x] `dotypasta app <appname> -d <file> -d <file2>...` delete all files from the `appname` configuration
- [ ] `dotypasta diff` show you the difference between the loaded dotfiles and your current configuration
  - [ ] `--app <appname> --app <appname2>...` to see the difference for specific apps
- [x] `dotypasta apply` copy all the dotfiles from the loaded configuration to the host
  - [ ] `--app <appname> --app <appname2>...` to apply the configuration for specific apps
- [x] `dotypasta save` copy all the dotfiles from the host to the local repository
  - [ ] `--app <appname> --app <appname2>...` to backup dotfiles for specific apps
  - [ ] automatically commit & push
- [ ] better UI 
  - using [indicatif](https://docs.rs/indicatif/latest/indicatif/)
  - maybe a TUI framework ?
