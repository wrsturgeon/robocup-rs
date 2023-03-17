.PHONY: check clean format playtest pr run run-release test format update
.PRECIOUS: deps-installed tests-passing
.SUFFIXES:

# https://doc.rust-lang.org/rustc/command-line-arguments.html
RUSTFLAGS=# -Zmove-size-limit=4
RUSTOPTFLAGS=-Copt-level=3 -Ctarget-cpu=native

TARGET:=$(shell rustc -vV | grep host | cut -d ' ' -f 2-)
cargo=cargo +nightly ${1} --target=${TARGET}
PROJNAME:=$(shell pwd | rev | cut -d '/' -f 1 | rev)
RELEASE_COND=$(shell echo "$@" | grep -q '.*release.*' && echo '-r' || :)
FMTFLAGS:=--config-path $(shell pwd)/rustfmt.toml --unstable-features --error-on-unformatted
format=cargo fmt $1 -- ${FMTFLAGS}
bindeps=update Cargo.toml rust-toolchain.toml .cargo/config rustfmt.toml deps-installed spl-headers.h $(shell find target -path '*/${1}/*' -name robocup-rs.d -type f -print -quit | xargs cat | cut -d ':' -f 2- | tr ' ' '\n' | grep -v 'spl/' | tr '\n' ' ')

release: target/${TARGET}/release/${PROJNAME} tests-passing
debug: target/${TARGET}/debug/${PROJNAME} tests-passing
run-debug: debug; target/${TARGET}/debug/${PROJNAME}
run-release: release; target/${TARGET}/release/${PROJNAME}
disassemble-debug: debug; TODO
disassemble-release: release; TODO

target/%/debug/${PROJNAME}: $(call bindeps,debug)
	make update # sneak this in here so dependencies aren't constantly out of date
	RUSTFLAGS='${RUSTFLAGS}' $(call cargo,rustc) ${RELEASE_COND} -Zunstable-options --keep-going -- $${RUSTFLAGS}
	
target/%/release/${PROJNAME}: $(call bindeps,release)
	make update # sneak this in here so dependencies aren't constantly out of date
	RUSTFLAGS='$(strip ${RUSTFLAGS} ${RUSTOPTFLAGS})' $(call cargo,rustc) ${RELEASE_COND} -- $${RUSTFLAGS}

clean:
	cargo $@
	rm -fr ext src/spl error.txt tests-passing # deps-installed

tests-passing: $(call bindeps,debug)
	$(call cargo,clippy) --all-targets --all-features -- -D warnings
	$(call cargo,test)
	rustfmt ${FMTFLAGS} --check $$(find src/spl -type f) # if this fails, the problem is in build.rs, which writes these files!!!
	$(call format,--check) # if this fails, just run `make format`
	touch $@

test: tests-passing
check: debug release test

format:
	$(call format)

update: | ext/all
	rustup $@
	cargo $@

ext:
	mkdir -p $@

deps-installed: install-dependencies.sh
	rm -f $@
	sh $<
	touch $@

ext/GameController/bin/%: $(shell find ext/GameController -type f ! -path '*/bin/*')
	cd ext/GameController && ant

ext/%: | ext
	git submodule update --init --remote --recursive

spl-headers.h: ext/GameController/examples/c
	echo '#ifndef SPL_HEADERS_H /* NOLINT(llvm-header-guard) */' > $@
	echo '#define SPL_HEADERS_H' >> $@
	for file in $$(find $< -type f); do echo '#include "'$${file}'"' >> $@; done
	echo '#endif /* SPL_HEADERS_H */' >> $@

run_jar_bg=ps aux | grep -v grep | grep -q '.*${1}.*' || { cd ext/GameController/bin && java -jar ${1}.jar & }
open-gc: ext/GameController/bin/GameController.jar
	# $(call run_jar_bg,TeamCommunicationMonitor)
	$(call run_jar_bg,GameController)

playtest: open-gc run-debug

pr: check
	git branch --show-current | grep -q main && git checkout -b dev || :
	git add -A
	if [ ! -z "$$(git status --porcelain)" ]; then \
	     echo "Please write a very brief (~5-word) description of your changes:" \
	  && git commit -m "$$(read -r line_read && echo "$${line_read}")" \
	  && git push -u origin $$(git branch --show-current) \
	  && gh pr create -t "$${line_read}" -b '$(shell cd ~ && pwd | rev | cut -d '/' -f 1 | rev) used `make pr`'; \
	  fi

# target/%/${PROJNAME}.d:
# 	if [ ! -f $@ ]; then make $*; fi

# include target/debug/${PROJNAME}.d target/release/${PROJNAME}.d
