# rebox
Simple table-based storage for remora-investigator


## Getting Started

```sh
$ cd rebox

$ nix develop

$ cargo run --example crud

Table [departments] created.

| row_id | id | name |
|      1 | 1 | Marketing |

Table [users] created.

| row_id | created_at | full_name | id | is_active | login |
|      1 | 1689820596 | Charlie Root | 1 | true | root |


Tables
======

 - departments
 - users

```

#### Checking saved payload
```sh
$ hexyl --panels=4 rebox_data/example_crud/data.safe.bin
```
![](docs/img/rebox-hexdump.png?raw=true)
