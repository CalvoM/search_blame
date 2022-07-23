# Search Blame

This is a cli tool that can perform text search and git blame at the same time.

## Functionality

- [x] Perform search of text in files.
- [x] Perform git blame for lines in a file.
- [x] Combine the two parts above to perform search and filtering using git names.

## How to use it

```
search_blame --help
USAGE:
    search_blame [OPTIONS] --files <FILES> --text <TEXT>

OPTIONS:
        --blame <BLAME>    Name of the person to blame (Optional). If not provided, it uses the
                           current user name
        --files <FILES>    Path to the file(s) we search in.
    -h, --help             Print help information
        --root <ROOT>      Directory of the git root. This should point to a git repo root
                           directory.
        --text <TEXT>      Content to search in the files
    -V, --version          Print version information

```
