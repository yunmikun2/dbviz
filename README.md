# dbviz

Visualize your database schema.

The tool loads database schema and draws it as a graph.

## Usage

```sh
$ dbviz -d database_name | dot -Tpng > schema.png
```

### Adapters

It's supposed to support different loaders (databases) and drawers
(formats for output).

Currently the only loader supported is `postgresql`, as for drawers,
it's `dot`:

```sh
$ dbviz --loader postgresql --drawer dot -d database_name > schema.dot
```

## Installation

### Arch linux

You can install the package from
[AUR](https://aur.archlinux.org/packages/dbviz-git).

```sh
$ paru -S dbviz-git
```

### Build from source

You need [cargo](https://doc.rust-lang.org/cargo/) installed.

```sh
$ git clone 'https://github.com/yunmikun2/dbviz'
$ cd dbviz
$ cargo build
$ cp ./target/release/dbviz ~/.local/bin
```

After this you've got the binary executable in `~/.local/bin/dbviz`.
