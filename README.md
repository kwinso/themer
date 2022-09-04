![image](https://user-images.githubusercontent.com/61386270/188315348-5b3979f5-1f18-4ee6-ad3f-0bbca8bdba63.png)

# About
Themer allows you to update themes for your desktop environment with just one command by swapping blocks of code that define color theme (and more!) variables inside configuration files. 

# Documentation
You can read documentation at [Wiki](https://github.com/uwumouse/themer/wiki) page to get started.

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

## What Themer can do?
- [X] Automatically set/swap color scheme variables (practically any variables) defined in your configuration file
- [X] Custom format: define your own code to be injected when you swap a theme.
- [X] Ignoring variables you don't need for each file
- [X] Specify command to reload your environment automatically
- [X] Aliasing vars for some custom names 
- [X] Import files inside custom block
