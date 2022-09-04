<p align="center" style="text-align: center" >
  <img src="./logo.png" /><br/>
</p>

# Themer
Update all of your config files to match same theme with just one line in terminal.

# Installation

To install latest version of Themer, execute this command inside of your terminal
```bash
bash <(curl https://github.com/uwumouse/themer/releases/latest/download/install.sh -L -s)
```
Script will prompt you before installation will begin.

Any other version can be installed with similar command by swapping `<release tag>` with the desired version prefixed by `v`
```bash
bash <(curl https://github.com/uwumouse/themer/releases/download/<release tag>/install.sh -L -s)
```

# Documentation
You can go through project's [Wiki](https://github.com/uwumouse/themer/wiki) on Themer's Wiki page to get started.

## What Themer can do?
- [X] Automatically set/swap color scheme variables (practically any variables) defined in your configuration file
- [X] Custom format: define your own code to be injected when you swap a theme.
- [X] Ignoring variables you don't need for each file
- [X] Specify command to reload your environment automatically
- [X] Aliasing vars for some custom names 
- [X] Import files inside custom block
