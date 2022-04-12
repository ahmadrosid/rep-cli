## Replace

Replace string in file.
```bash
rep 0.1.0
Batch replace text file

USAGE:
    rep.exe [OPTIONS] --from <from> --input <input> --to <to>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -e, --ext <ext>        File extensions use * to accept all ext [default: *]
    -f, --from <from>      Original string
    -i, --input <input>    File path to replace
    -t, --to <to>          Replaced string
```

## Example
```bash
rep -i public/wiki/ -e html -f 'link rel="stylesheet" href="' -t 'link rel="stylesheet" href="/wiki/'
```
