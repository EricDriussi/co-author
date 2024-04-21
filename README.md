# co-author

Give credit to your teammates when pairing or mob-programming.

[co-author](https://github.com/EricDriussi/co-author/assets/46979145/0946f729-e970-4e23-afcf-008d4d05e6a8)

## Use Case

It's useful to know who to ask when struggling with
a piece of code.

Git blame is great, but it only mentions the committer, which leaves out the
pair/mob partners.

It helps to know who else was involved (maybe the committer is busy or left the
company long ago).

There are plenty of editor plugins ([VSCode](https://marketplace.visualstudio.com/items?itemName=megamegax.co-author), [JetBrains](https://plugins.jetbrains.com/plugin/10952-co-author)) that follow [GitHub's guidelines](https://docs.github.com/en/enterprise-cloud@latest/pull-requests/committing-changes-to-your-project/creating-and-editing-commits/creating-a-commit-with-multiple-authors#creating-co-authored-commits-on-the-command-line)
for co-authoring commits.

This is a simple CLI tool that achieves the same thing, while being editor independent
and easy to integrate into your existing workflows.

## Installation

```sh
cargo install co-author
```

## Usage

Co-author will look for an `authors.csv` file in your current working directory,
falling back to `$XDG_CONFIG_HOME/co-author/`, `$HOME/.config/co-author/`,
`$HOME/.co-author/` and `$HOME` in that order.

This file should follow the structure `alias,name,email`:

```csv
a,Name Surname,someone@users.noreply.github.com
b,username,something@gmail.com
cd,Another Person,someone@something.hi
```

If no options are passed, it will prompt you for a space-separated list of
aliases and then for a commit message.

It will produce a commit message with the following structure:

```txt
a commit message


Co-Authored-by: Name Surname <someone@users.noreply.github.com>
Co-Authored-by: username <something@gmail.com>
```

If you group multiple users under the same alias, they will all be retrieved at once.

This is especially useful if you jump between various teams and would rather pick
groups of people instead of individuals.

So for a file like:

```csv
a,Name Surname,someone@users.noreply.github.com
a,username,something@gmail.com
```

When given the alias `a`, it will add **both users** as co-authors.

## Options

You can modify the behavior in a number of ways, most will bypass the prompt:

```
co-author -h
Co-Author your git commits from the command line

Usage: co-author [OPTIONS]

Options:
  -f, --file <FILE>        CSV file containing a list of authors (alias,name,email)
  -l, --list <LIST>        List of comma separated author aliases
  -a, --all                Use all available authors
  -m, --message <MESSAGE>  Specify commit message
  -e, --editor             Open default editor for commit message
  -p, --pre-populate       Pre-populate prompt/editor with (first line of) last commit message
  -s, --sort               Sort authors signatures when adding to commit message
      --amend              Amend last commit, both message and authors will be overwritten
      --fzf                Use fzf for author selection
  -h, --help               Print help
  -V, --version            Print version
```

### --file

Specify a different authors file path.

### --list

Use a pre-defined alias list.

This makes the alias selection scriptable.
It also helps when working with different teams.

```sh
alias coa_proj_a="co-author --list a,b,c"
alias coa_proj_x="co-author --list x,y,z"
```

Omits the alias prompt.

### --all

Use all the aliases in the file.

Conflicts with `--list`.

Omits the alias prompt.

### --message

Just like git's `-m`: Specify a commit message.

Omits the message prompt.

### --editor

Just like git's default behavior: Open a text editor to write the commit message.

It will look for the `editor` config in your git setup, falling back to
`$EDITOR`, `vim` and `vi` in that order.

Omits the message prompt.

### --pre-populate

Pre-populate either the prompt or the editor with the **subject** of the last commit
message, so only the first line is recovered.

If you use [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/)
or other standards you might want the same general format, just with a different
type or description.

Conflicts with `--message`.

### --sort

Sort authors alphabetically by signature (`username <email>`).

If not used it will respect the order in the `authors.csv` file.

### --amend

Amends the last commit, overwriting message and authors with the newly provided ones.

Enables `--pre-populate` flag under the hood.

### --fzf

**Depends on `fzf` being installed.**

Presents a picker for the authors using your `fzf` install (and config).
Uses the `--multi` flag, so pressing `Tab` will select multiple authors.

Press enter when done do continue with the commit message.

Conflicts with `--all` and `--list`.
