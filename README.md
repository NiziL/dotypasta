# Dotypasta

Dotypasta let you easily fetch, save and apply dotfiles.  
Inspired by [chezmoi](https://chezmoi.io)

## Features roadmap

- [x] `dotypasta load <username>` clone a `dotfiles` repository to `$HOME/.local/share/dotypasta` 
  - [x] option `--hub [github | gitlab | bitbucket ... ]` to select the git host 
  - [x] option `-t <reference>` to load a specific tag or branch :warning: detached head
  - [x] option `--ssh` to clone the repository using ssh
  - [ ] option `--apply` to directly apply the configuration
- [x] `dotypasta clear` to delete `$HOME/.local/share/dotypasta`
- [x] `dotypasta app <appname>` show you the files copied for `appname` as defined in `$HOME/.local/share/dotypasta/dotypasta.toml`
  - [x] `dotypasta app <appname> -a <file> -a <file2>...` add all files to the `appname` configuration
  - [x] `dotypasta app <appname> -d <file> -d <file2>...` delete all files from the `appname` configuration
- [ ] `dotypasta diff` show you the difference between the loaded dotfiles and your current configuration
  - [ ] option `--app <appname> --app <appname2>...` to see the difference for specific apps
- [ ] `dotypasta apply` apply the configuration from `$HOME/.local/share/dotypasta`
  - [ ] option `--app <appname> --app <appname2>...` to apply the configuration for specific apps
- [ ] `dotypasta save` copy all the dotfiles defined in `dotypasta.toml` from the host to `$HOME/.local/share/dotypasta`
  - [ ] option `--app <appname> --app <appname2>...` to backup dotfiles for specific apps
