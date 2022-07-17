# Themer [IN DEVELOPMENT]
A tool that allows you to update your desktop theme with just one command.

## TODO
- [X] Parse configuration file
  - [X] Themes (color name + color value)
  - [X] Files (comment str, key-value format (e.g. `<key> = <value>`))
- [ ] Writing default key-value pairs to files (finding the Themer block via comments)
- [ ] Ignoring values in files
- [ ] Custom file format
  - [ ] `<COLORS>` directive: puts default key-value assignment
  - [ ] `<COLOR (any key from color set)>` to use defined values in custom format
  - [ ] `<NAME>` to represent current theme name. Usefull in scenarios like `coloscheme` name for Nvim.
- [ ] Aliasing colors for some custom names 
