# Themer [IN DEVELOPMENT]
A tool that allows you to update your desktop theme with just one command.

## TODO
- [X] Parse configuration file
  - [X] Themes (color name + color value)
  - [X] Files (comment str, key-value format (e.g. `<key> = <value>`))
- [ ] Writing default key-value pairs to files (finding the Themer block via comments)
- [ ] Ignoring values in files
- [ ] Custom file format
  - [ ] `<colors>` directive: puts default key-value assignment
  - [ ] `<color (any key from color set)>` to use defined values in custom format
  - [ ] `<name>` to represent current theme name. Usefull in scenarios like `coloscheme` name for Nvim.
- [ ] Aliasing colors for some custom names 
- [ ] `<import:path/to/file>` statement for importing files into `custom` block
