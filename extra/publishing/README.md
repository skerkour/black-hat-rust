# [The tools and services I used to write, edit and self-publish my book](https://kerkour.com/book-self-publishing-pandoc)


Put your files in a `src` folder and edit the `Makefile` accordingly.


Then:
```shell
$ make docker
$ docker run -ti --rm -v `pwd`:/ebook localhost/skerkour/ebook
```

All your ebooks are now in the `ebooks` folder.


You can find other themes here: https://github.com/tajmone/pandoc-goodies/tree/master/skylighting/themes
