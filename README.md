# Dotypasta

Dotypasta let you easily fetch, save and apply dotfiles.  
Inspired by [chezmoi](https://chezmoi.io)

## Installation

**dotypasta is still in early development**  
If you want to give it a shot, clone the repository and `cargo build`

## Convention & Configuration

Dotypasta clone a repository named `dotfiles` from a specified user at `$HOME/.local/share/dotypasta`.  
This directory must contains the `dotypasta.toml` file, and respect a structure commonly used among `dotfiles` repository (the root of the directory is supposed to be your home).

```toml
zsh = [".zshrc", ".oh-my-zsh/custom/themes/nizil.zsh"]
i3 = [".config/i3/config", ".config/i3blocks/config"]
vi = [".vimrc", ".config/nvim/init.vim"]
```

With such config file, dotypasta has 3 *apps* defined (zsh, i3 and vi) with their respective files.  
It expect to find this files in the repository, and will copy them in your HOME directory (while preserving the relative path).

## Features roadmap

- [x] `dotypasta load <username>` clone a `dotfiles` repository
  - [x] option `--hub [github | gitlab | bitbucket ... ]` to select the git host 
  - [x] option `-t <reference>` to load a specific tag or branch :warning: detached head
  - [x] option `--ssh` to clone the repository using ssh
  - [ ] option `--apply` to directly apply the configuration
- [x] `dotypasta clear` to delete `$HOME/.local/share/dotypasta`
- [x] `dotypasta app <appname>` show you the files copied for `appname`
  - [x] `dotypasta app <appname> -a <file> -a <file2>...` add all files to the `appname` configuration
  - [x] `dotypasta app <appname> -d <file> -d <file2>...` delete all files from the `appname` configuration
- [ ] `dotypasta diff` show you the difference between the loaded dotfiles and your current configuration
  - [ ] option `--app <appname> --app <appname2>...` to see the difference for specific apps
- [ ] `dotypasta apply` copy all the dotfiles from the loaded configuration to the host
  - [ ] option `--app <appname> --app <appname2>...` to apply the configuration for specific apps
- [ ] `dotypasta save` copy all the dotfiles from the host to the local repository
  - [ ] option `--app <appname> --app <appname2>...` to backup dotfiles for specific apps
