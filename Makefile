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
	ls cargo_hello \
	|| cargo new cargo_hello --bin

################################################################################
# cargo new(typo発見したが気にしない)
# $ make cargo-new-guessing
################################################################################
.PHONY: cargo-new-guessing
cargo-new-guessing:
	ls gussing_game \
	|| cargo new gussing_game --bin

################################################################################
# cargo new
# $ make cargo-new-variables
################################################################################
.PHONY: cargo-new-variables
cargo-new-variables:
	ls variables \
	|| cargo new variables --bin

################################################################################
# cargo new
# $ make cargo-new-ownership
################################################################################
.PHONY: cargo-new-ownership
cargo-new-ownership:
	ls ownership \
	|| cargo new ownership --bin

################################################################################
# cargo new
# $ make cargo-new-structure
################################################################################
.PHONY: cargo-new-structure
cargo-new-structure:
	ls structure \
	|| cargo new structure --bin

################################################################################
# cargo new
# $ make cargo-new-enum
################################################################################
.PHONY: cargo-new-enum-practice
cargo-new-enum-practice:
	ls enum-practice \
	|| cargo new enum-practice --bin

################################################################################
# cargo new
# $ make cargo-new-communicator
################################################################################
.PHONY: cargo-new-communicator
cargo-new-communicator:
	ls communicator \
	|| cargo new communicator --lib

################################################################################
# Distribute Makefile.cargo-template as Makefile
# $ make distribute-makefile
################################################################################
.PHONY: distribute-makefile
distribute-makefile:
	ls -1 ./*/Cargo.toml \
	| xargs -I {path} dirname {path} \
	| xargs -I {path} cp ./Makefile.cargo-template {path}/Makefile
