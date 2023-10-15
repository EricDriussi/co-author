# co-author

Give credit to your teammates when pairing or mob-programming.

[co-author](https://github.com/EricDriussi/co-author/assets/46979145/0946f729-e970-4e23-afcf-008d4d05e6a8)

## Use Case

When working within a team, it's useful to know who to ask when struggling with
a piece of code.

Git blame is awesome, but only the committer is mentioned.

This is a shame since we often are (should be) working with others.
It helps to know who else was involved (maybe the committer is not available
at the moment).

There are plenty of editor plugins that follow [GitHub's guidelines](https://docs.github.com/en/enterprise-cloud@latest/pull-requests/committing-changes-to-your-project/creating-and-editing-commits/creating-a-commit-with-multiple-authors#creating-co-authored-commits-on-the-command-line)
for co-authoring commits.

This is a simple CLI tool that achieves the same thing, while being editor independent
and easy to integrate into your existing workflows.

## Installation

```sh
cargo install co-author
```

## Usage

The tool expects a CSV `authors` file in `$XDG_CONFIG_HOME/coa/authors`
(defaults to `$HOME/.config/coa/authors`).

This file should follow the structure `alias,name,email`, as in the example:

```csv
a,Name Surname,someone@users.noreply.github.com
b,username,something@gmail.com
cd,Another Surname,someone@something.hi
```

If no options are passed, it will prompt you for a space-separated list of
aliases and then for a commit message.

It will produce a commit message with the formatted as follows:

```txt
a commit message


Co-Authored-by: Name Surname <someone@users.noreply.github.com>
Co-Authored-by: username <something@gmail.com>
```

If you group multiple users under the same alias, they will all be co-authors.

This is especially useful if you jump between various teams.

So for a file like:

```csv
a,Name Surname,someone@users.noreply.github.com
a,username,something@gmail.com
```

When given the alias `a`, it will add both users as co-authors.

## Options

You can modify the behavior in a number of ways, most will bypass the prompt:

```sh
co-author -h
Co-Author your git commits from the command line

Usage: co-author [OPTIONS]

Options:
  -f, --file <FILE>        File containing a csv formatted list of authors (alias,name,email)
  -l, --list <LIST>        List of comma separated author aliases
  -a, --all                Use all available authors
  -m, --message <MESSAGE>  Specify commit message
  -e, --editor             Open default editor for commit message
  -p, --pre-populate       Pre-populate prompt/editor with (first line of) last commit message
  -s, --sort               Sort authors signatures when adding to commit message
  -h, --help               Print help
  -V, --version            Print version
```

### --file

Use a specific file path.

You might want one per-project.

### --list

Use a pre-defined alias list.

Something like

```sh
alias coa_proj_a="co-author --list a,b,c"
alias coa_proj_x="co-author --list x,y,z"
```

Might be useful.

### --all

Use all the aliases in the file.

Conflicts with `--list`.

### --message

Just like git's `-m`: Specify a commit message.

### --editor

Just like git's default behavior: Fill in the commit message in a text editor.

It will look for the `editor` config in your git setup, defaulting to
`$EDITOR`, `vim` and `vi` in that order.

### --pre-populate

Pre-populate either the prompt or the editor with the last commit message
(not considering possible co-authors).

If you use [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/)
or other standards you might want the same general format, just with a different
type.

Conflicts with `--message`.

### --sort

Sort authors alphabetically by signature (`username <email>`).

If not used it will respect the order in the `authors` file.
