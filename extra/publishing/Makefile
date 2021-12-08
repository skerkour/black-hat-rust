BOOK_FILE = black_hat_rust
BOOK_TITLE = Black Hat Rust
CHAPTERS_FILES = settings.txt  \
	src/ch_01.md \
	src/ch_02.md \
	src/ch_03.md \
	src/ch_04.md \
	src/ch_05.md \
	src/ch_06.md \
	src/ch_07.md \
	src/ch_08.md \
	src/ch_09.md \
	src/ch_10.md \
	src/ch_11.md \
	src/ch_12.md \
	src/ch_13.md \
	src/ch_14.md


COVER = src/assets/black_hat_rust_cover.png
DATE := $(shell date "+%B %e, %Y")
VERSION = v2021.41
DIST_DIR = ebooks
DOCKER_IMAGE = localhost/skerkour/ebook


.PHONY: all
all: pdf epub mobi epubcheck


.PHONY: pdf
pdf:
	pandoc $(CHAPTERS_FILES) \
	--resource-path=src \
	--output=$(DIST_DIR)/$(BOOK_FILE)_content.pdf \
	--pdf-engine=xelatex \
	--table-of-contents --toc-depth=2 \
	--number-sections \
	--top-level-division=chapter \
	--include-in-header inline_code.tex \
	-V fontsize=12pt \
	-V documentclass=report \
	-V linkcolor:blue \
	--highlight-style tango_theme.json \
	-M date="$(VERSION)"
	pdftk src/assets/black_hat_rust_cover.pdf $(DIST_DIR)/$(BOOK_FILE)_content.pdf cat output $(DIST_DIR)/$(BOOK_FILE).pdf
	rm $(DIST_DIR)/$(BOOK_FILE)_content.pdf


.PHONY: epub
epub:
	pandoc $(CHAPTERS_FILES) \
	--resource-path=src \
	--output=$(DIST_DIR)/$(BOOK_FILE).epub \
	--table-of-contents --toc-depth=2 \
	--top-level-division=chapter \
	--number-sections \
	--listings \
	--standalone \
	--epub-cover-image=$(COVER) \
	--metadata title="$(BOOK_FILE)" \
	--highlight-style tango_theme.json \
	--css epub.css \
	-M date="$(DATE)"


.PHONY: epubcheck
epubcheck:
	java -jar /usr/bin/epubcheck ebooks/black_hat_rust.epub


.PHONY: mobi
mobi:
	ebook-convert $(DIST_DIR)/$(BOOK_FILE).epub $(DIST_DIR)/$(BOOK_FILE).mobi --cover $(COVER)


.PHONY: docker
docker:
	docker build -t $(DOCKER_IMAGE):latest .
