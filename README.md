# Garden

A CLI tool for the creations and maintenance of Garden.

## Commands

### path

```shell
    GARDEN_PATH=~/github/garden garden write
    garden -p ~/github/garden write
    garden --garden_path ~/github/garden write
```

### Write

Open a new file to write. Give a option to add a title from start or late.

```shell
    garden write
    garden write -t "Some title"
```

#### learning

To run watch: 

```shell
    cargo watch -x check -x test
```