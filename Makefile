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
# cargo new
# $ make cargo-new-collection
################################################################################
.PHONY: cargo-new-collection
cargo-new-collection:
	ls collection \
	|| cargo new collection --bin

################################################################################
# cargo new
# $ make cargo-new-panic-practice
################################################################################
.PHONY: cargo-new-panic-practice
cargo-new-panic-practice:
	ls panic-practice \
	|| cargo new panic-practice --bin

################################################################################
# cargo new
# $ make cargo-new-generic-trait-lifetime
################################################################################
.PHONY: cargo-new-generic-trait-lifetime
cargo-new-generic-trait-lifetime:
	ls generic-trait-lifetime \
	|| cargo new generic-trait-lifetime --bin

################################################################################
# cargo new
# $ make cargo-new-adder
################################################################################
.PHONY: cargo-new-adder
cargo-new-adder:
	ls adder \
	|| cargo new adder --lib

################################################################################
# cargo new
# $ make cargo-new-minigrep
################################################################################
.PHONY: cargo-new-minigrep
cargo-new-minigrep:
	ls minigrep \
	|| cargo new minigrep --lib

################################################################################
# cargo new
# $ make cargo-new-iterator-closure
################################################################################
.PHONY: cargo-new-iterator-closure
cargo-new-iterator-closure:
	ls iterator-closure \
	|| cargo new iterator-closure --lib

################################################################################
# cargo new
# $ make cargo-new-iterator-closure
################################################################################
.PHONY: cargo-new-iterator-closure
cargo-new-iterator-closure:
	ls iterator-closure \
	|| cargo new iterator-closure --lib

################################################################################
# cargo new
# $ make cargo-new-smart-pointer
################################################################################
.PHONY: cargo-new-smart-pointer
cargo-new-smart-pointer:
	ls smart-pointer \
	|| cargo new smart-pointer --lib

################################################################################
# cargo new
# $ make cargo-new-messanger
################################################################################
.PHONY: cargo-new-messanger
cargo-new-messanger:
	ls messanger \
	|| cargo new messanger --lib

################################################################################
# Distribute Makefile.cargo-template as Makefile
# $ make distribute-makefile
################################################################################
.PHONY: distribute-makefile
distribute-makefile:
	ls -1 ./*/Cargo.toml \
	| xargs -I {path} dirname {path} \
	| xargs -I {path} cp ./Makefile.cargo-template {path}/Makefile
