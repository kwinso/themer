# 1.3.1
- [X] Optimized `set` subcommand speed: writing to file only once when using multiple blocks mode

# 1.3.0
- [X] Multiple themer blocks inside single file
- [X] `comment_end` variable to support comments like `/* comment */`

# 1.2.1
- [X] Do not exit after an error when applying changes

# 1.2.0 
- [X] Cover everything with tests 
  - [X] Getting custom block vars 
  - [X] Var formatting 
- [X] Add Documentation
- [X] `reload` variable to run any command after configs update
- [X] Aliasing
- [X] `only` as the opposite to `ignore`
- [X] Imports
  - [X] Expand variables inside import statement for dynamic files
- [X] Split `engine.rs` into `mod`  to avoid large file with all codebase
