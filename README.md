# goback

Go back to a previous git branch.

## Installation

```bash
npm install -g goback
```

## Usage

```bash
# Go back to the previous branch
goback

# Go back 2 branches ago
goback 2

# Print the previous branch name (don't switch)
goback --print
goback -p

# Print the branch name 2 steps back
goback 2 -p

# List recent 10 branches
goback --list
goback -l

# List recent 5 branches
goback -l 5
```

## Example

```bash
$ goback -l
╭───┬─────────────────────┬─────────╮
│ # │ Branch              │ Status  │
├───┼─────────────────────┼─────────┤
│ 0 │ feature/new-feature │ current │
├╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 1 │ develop             │         │
├╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 2 │ main                │         │
├╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 3 │ feature/old-feature │         │
╰───┴─────────────────────┴─────────╯

$ goback 2
Switched to branch 'main'
```

## How it works

`goback` parses your git reflog to extract branch checkout history, allowing you to quickly navigate back to previously visited branches.

## License

MIT
