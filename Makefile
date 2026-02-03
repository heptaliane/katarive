.PHONY: test dev

test:
	buf lint .

dev:
	cargo tauri dev
