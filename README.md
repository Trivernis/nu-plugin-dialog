# nu-plugin-dialog

[![asciicast](https://asciinema.org/a/2ZvPHpTgkZFmeId3z01IwQQ6F.svg)](https://asciinema.org/a/2ZvPHpTgkZFmeId3z01IwQQ6F)

This plugin adds more options to prompt for user input to nushell.
It mainly uses the [dialoguer](https://github.com/console-rs/dialoguer) crate for prompts.


## Example Usage

```nu
# select a file
let selected_file = ( ls / | get name | ask select $in --fuzzy --prompt "Select a file" )

# ask for confirmation
let quit = ( ask confirm "Are you sure that you want to quit?" --default false )

if $quit {
  exit
}
```

For more information run `ask --help` or pass `--` help to the subcommands.

## License

CNPLv7+
