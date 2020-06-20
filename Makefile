include Makefile.rustup

################################################################################
# download .gitignore
# $ make gitignore
################################################################################
.PHONY: gitignore
gitignore:
	curl --output .gitignore https://www.toptal.com/developers/gitignore/api/rust,intellij+all,linux,macos,windows,vim,emacs

################################################################################
# cargo new
# $ make cargo-new-hello
################################################################################
.PHONY: cargo-new-hello
cargo-new-hello:
	cargo new cargo_hello --bin

################################################################################
# cargo new
# $ make cargo-new-guessing
################################################################################
.PHONY: cargo-new-guessing
cargo-new-guessing:
	cargo new gussing_game --bin

################################################################################
# cargo new
# $ make cargo-new-variables
################################################################################
.PHONY: cargo-new-variables
cargo-new-variables:
	cargo new variables --bin

################################################################################
# cargo new
# $ make cargo-new-ownership
################################################################################
.PHONY: cargo-new-ownership
cargo-new-ownership:
	cargo new ownership --bin

################################################################################
# cargo new
# $ make cargo-new-structure
################################################################################
.PHONY: cargo-new-structure
cargo-new-structure:
	cargo new structure --bin

################################################################################
# cargo new
# $ make cargo-new-enum
################################################################################
.PHONY: cargo-new-enum-practice
cargo-new-enum-practice:
	cargo new enum-practice --bin
