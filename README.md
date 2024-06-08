# roundup

Recursively counts number of files and lines of text in a directory.

## Examples

For all filetypes: 

```shell
roundup /path/to/my/project/
```

For only files with the ".rs" extension:

```shell
roundup /path/to/my/project/
```

For files with the `.rs`, `.toml`, and `.md` extensions:

```shell
roundup /path/to/my/project/ rs,toml,md
```

