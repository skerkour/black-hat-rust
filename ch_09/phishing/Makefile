WEBAPP_DIST_DIR = webapp/dist
TARGET_DIR = target
DB_FILES = db.sqlite*
DIST_DIR = dist


.PHONY: webapp
webapp:
	wasm-pack build --target web --out-name wasm --out-dir ./dist --release --no-typescript webapp
	cp webapp/dist/wasm* public/


.PHONY: serve
serve:
	cargo watch -w server -x 'run -p server'


.PHONY: webapp_debug
webapp_debug:
	wasm-pack build --target web --out-name wasm --out-dir ./dist --dev --no-typescript webapp
	cp webapp/dist/wasm* public/


.PHONY: dev
dev:
	cargo watch -w webapp/src -s 'make webapp_debug'


.PHONY: check
check:
	cargo check


.PHONY: fmt
fmt:
	cargo fmt


.PHONY: clean
clean:
	rm -rf $(WEBAPP_DIST_DIR) $(TARGET_DIR) $(DB_FILES) $(DIST_DIR)


.PHONY: build
build: webapp
	mkdir -p $(DIST_DIR)
	cargo build --release -p server
	strip -s target/release/server
	cp target/release/server $(DIST_DIR)
	cp -r public $(DIST_DIR)


.PHONY: rpi
rpi: webapp_debug
	mkdir -p $(DIST_DIR)
	cargo build --release -p server
	strip -s target/release/server
	cp target/release/server $(DIST_DIR)
	cp -r public $(DIST_DIR)
