# [The tools and services I used to write, edit and self-publish my book](https://kerkour.com/book-self-publishing-pandoc)


Put your files in a `src` folder and edit the `Makefile` accordingly.


Then:
```shell
$ make docker
$ docker run --ti --rm -v `pwd`:/ebook localhost/skerkour/ebook
```
