WEBAPP_DIST_DIR = webapp/dist
TARGET_DIR = target
DB_FILES = db.sqlite*


.PHONY: webapp
webapp:
	wasm-pack build --target web --out-name wasm --out-dir ./dist --release --no-typescript webapp
	cp webapp/dist/{wasm.js,wasm_bg.wasm} public/


.PHONY: serve
serve:
	cargo watch -w server -x 'run -p server -- server'


.PHONY: init
init:
	cargo run -p server -- init


.PHONY: webapp_debug
webapp_debug:
	wasm-pack build --target web --out-name wasm --out-dir ./dist --dev --no-typescript webapp
	cp webapp/dist/{wasm.js,wasm_bg.wasm} public/


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
	rm -rf $(WEBAPP_DIST_DIR) $(TARGET_DIR) $(DB_FILES)
