include tools/docker/docker.mk

DOCKER := $(foreach target,$(DOCKER_TARGETS),docker.$(target))

.PHONY: all build check clean codecov docs examples libtpms package publish tests $(DOCKER)


all: clean check build

check:
	cargo check --release --locked --all-targets

tests:
	CARGO_PROFILE_RELEASE_DEBUG=true \
	RUST_BACKTRACE=1 \
	cargo test --release --tests --locked -- --test-threads=1

build:
	cargo build --release --locked

docs:
	cargo doc --no-deps --locked

examples:
	set -e; for i in examples/*.rs; do cargo run --example $$(basename $$i .rs); done

format:
	cargo fmt --all $(if $(APPLY_FMT),,--check)

package:
	cargo package --locked

publish:
	cargo publish --locked

clean:
	rm -rf target $(if $(CARGO_TARGET_DIR),"$(CARGO_TARGET_DIR)")

libtpms:
	./tools/libtpms/libtpms-test-runner.sh --include-ignored

codecov:
	./tools/codecov/code-coverage.sh ccov target/llvm-cov/codecov-output.json

$(DOCKER): docker.%:
	$(MAKE) -C tools/docker $*
