################################################################################
# download .gitignore
# $ make gitignore
################################################################################
.PHONY: gitignore
gitignore:
	curl --output .gitignore https://www.toptal.com/developers/gitignore/api/rust,intellij+all,linux,macos,windows,vim,emacs

################################################################################
# cargo build
# $ make build
################################################################################
.PHONY: build
build:
	cargo build

################################################################################
# run && cargo run
# $ make run
################################################################################
.PHONY: run
run:
	./target/debug/cargo_hello
	cargo run

################################################################################
# cargo check
# $ make check
################################################################################
.PHONY: check
check:
	cargo check
