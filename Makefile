.PHONY: build-release build-debug clean playtest run run-release test update
.PRECIOUS: deps-installed tests-passing
.SUFFIXES:

PROJNAME:=$(shell pwd | rev | cut -d '/' -f 1 | rev)
RELEASE_COND=$(shell if [ "release" = "$*" ]; then echo '-r'; else if [ "debug" = "$*" ]; then :; else echo 'Unrecognized target (`release` or `debug`)'; exit 1; fi; fi)

build-debug: tests-passing target/debug/${PROJNAME}
build-release: tests-passing target/release/${PROJNAME}

target/%/${PROJNAME}: tests-passing
	cargo build ${RELEASE_COND}

clean:
	cargo $@
	rm -fr ext src/spl error.txt tests-passing # deps-installed

tests-passing: Cargo.toml deps-installed spl-headers.h $(shell find . -name robocup-rs.d -type f -print -quit | xargs cat | cut -d ':' -f 2- | tr ' ' '\n' | grep -v 'spl/' | tr '\n' ' ')
	make update # sneak this in here so dependencies aren't constantly out of date
	cargo fmt
	cargo clippy --all-targets --all-features -- -D warnings
	cargo test
	touch $@

update: | ext/all
	rustup $@
	cargo $@

run run-release: tests-passing
	cargo run $(shell if [ "run-release" = "$@" ]; then echo '-r'; fi)

push: tests-passing
	git config advice.addIgnoredFile true
	git add .gitignore -A
	git commit -m '$(shell cd ~ && pwd | rev | cut -d '/' -f 1 | rev) used `make push`'
	git push

ext:
	mkdir -p $@

deps-installed: install-dependencies.sh
	rm -f $@
	sh $<
	touch $@

ext/GameController/bin/%: ext/GameController
	cd $< && ant

ext/%: | ext
	git submodule update --init --remote --recursive

spl-headers.h: ext/GameController/examples/c
	echo '#ifndef SPL_HEADERS_H /* NOLINT(llvm-header-guard) */' > $@
	echo '#define SPL_HEADERS_H' >> $@
	for file in $$(find $< -type f); do echo '#include "'$${file}'"' >> $@; done
	echo '#endif /* SPL_HEADERS_H */' >> $@

playtest: ext/GameController/bin/GameController.jar target/debug/${PROJNAME}
	cd ext/GameController/bin && java -jar GameController.jar &
	cd target/debug && GAMECONTROLLER_IP?=$(shell ifconfig | grep 'inet\ ' | grep -v '127\.0\.0\.1' | head -n 1) ./${PROJNAME} &

# target/%/${PROJNAME}.d:
# 	if [ ! -f $@ ]; then make $*; fi

# include target/debug/${PROJNAME}.d target/release/${PROJNAME}.d
