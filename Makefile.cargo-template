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
# cargo run
# $ make run
################################################################################
.PHONY: run
run:
	cargo run

################################################################################
# cargo check
# $ make check
################################################################################
.PHONY: check
check:
	cargo check

################################################################################
# cargo update
# $ make update
################################################################################
.PHONY: update
update:
	cargo update

################################################################################
# cargo doc
# $ make doc
################################################################################
.PHONY: doc
doc:
	cargo doc

################################################################################
# cargo docしたやつをnginx dockerで見えるようにする
# $ make doc-docker
################################################################################
DOCKER_NGINX_TAG  := stable
DOCKER_NGINX_HOST := nginx
.PHONY: doc-docker
doc-docker: doc
	( docker container ls | grep $(DOCKER_NGINX_HOST) ) \
	|| docker run \
		--detach \
		--rm \
		--interactive \
		--tty \
		--name $(DOCKER_NGINX_HOST) \
		--hostname $(DOCKER_NGINX_HOST) \
		--user $(id -u):$(id -g) \
		--publish 80:80 \
		--mount type=bind,source=$(PWD)/target/doc/,target=/usr/share/nginx/html/ \
		nginx:$(DOCKER_NGINX_TAG)
	echo Please open http://HOST/$(shell basename $(PWD))/index.html

################################################################################
# nginxを落とす
# $ make clean
################################################################################
.PHONY: clean
clean:
	docker container ls --quiet --filter 'name=$(DOCKER_NGINX_HOST)' \
	| xargs -I {container-id} docker container stop {container-id}

################################################################################
# rustup component add rustfmt
# $ make add-rustfmt
################################################################################
.PHONY: add-rustfmt
add-rustfmt:
	rustup component add rustfmt

################################################################################
# cargo rustfmt
# $ make fmt
################################################################################
.PHONY: fmt
fmt:
	cargo fmt

################################################################################
# cargo test
# $ make test
################################################################################
.PHONY: test
test:
	cargo test
