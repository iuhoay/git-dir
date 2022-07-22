# git-dir

Run git command in a directory.

## Installation

Install from source.

```
$ git clone https://github.com/iuhoay/git-dir.git
$ cd git-dir
$ cargo install --path .
```

## Usage

```
$ git-dir -h
git-ls 0.1.0
iuhoay <iuhoay@gmail.com>
Run git command in the directory.

USAGE:
    git-dir [OPTIONS]

OPTIONS:
    -h, --help           Print help information
    -p, --path <PATH>    [default: .]
    -V, --version        Print version information
```

### List all branches

```
$ git-dir
world - [main]
hello - [main]
```
