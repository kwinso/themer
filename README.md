# Themer [IN DEVELOPMENT]
A tool that allows you to update your desktop theme with just one command.

## TODO
- [X] Parse configuration file
  - [X] Themes (color name + color value)
  - [X] Files (comment str, key-value format (e.g. `<key> = <value>`))
- [X] Writing default key-value pairs to files (finding the Themer block via comments)
- [X] Ignoring values in files
- [X] Custom file format
  - [X] `<colors>` directive: puts default key-value assignment
  - [X] `<color (any key from color set)>` to use defined values in custom format
  - [X] `<name>` to represent current theme name. Usefull in scenarios like `coloscheme` name for Nvim.
- [ ] Aliasing colors for some custom names 
- [ ] `<import:path/to/file>` statement for importing files into `custom` block
