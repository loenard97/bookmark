<div align="center">

# bookmark
A file path bookmarking tool

![](https://img.shields.io/github/last-commit/loenard97/bookmark?&style=for-the-badge&logo=github&color=F74C00)
![](https://img.shields.io/github/repo-size/loenard97/bookmark?&style=for-the-badge&logo=github&color=F74C00)

</div>


## Usage
Use this bash script:
```sh
# .bashrc

bm()
{
    $BMPATH=$(bookmark $1)
    if [[ -n "$BMPATH" ]]; then
        cd $BMPATH
    fi
}
```

Create a new bookmark for the current working directory with `bm mymark`.
Calling `bm mymark` a second time will switch the current working directory to this bookmark.

Additionally `bm -l` will list all bookmarks and `bm -r mymark` will remove mymark from the list of bookmarks.
The list of bookmarks is stored at `~/.cache/bookmark/bookmarks.json`.

