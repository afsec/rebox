# rebox
Simple table-based storage

## Getting Started

```sh
$ cd rebox

$ nix develop

$ cargo run --example crud
```
![](docs/img/rebox-crud-example-output.png?raw=true)

## Checking saved payload
```sh
$ hexyl --panels=4 rebox_data/example_crud/data.safe.bin
```
![](docs/img/rebox-hexdump.png?raw=true)
