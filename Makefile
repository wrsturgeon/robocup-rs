.PHONY: check clean format playtest pr run run-release test format update update-ext
.PRECIOUS: deps-installed tests-passing
.SUFFIXES:

# https://doc.rust-lang.org/rustc/command-line-arguments.html
CARGOFLAGS:=-Zunstable-options# --keep-going
RUSTFLAGS:=--verbose -Clto -Cembed-bitcode # not sure why, but `embed-bitcode` is required by LTO yet not enabled by it
RUSTDBGFLAGS:=-Copt-level=0 -Zsanitizer=address
RUSTOPTFLAGS:=-Copt-level=3 -Ctarget-cpu=native

USERNAME:=$(shell cd ~ && pwd | rev | cut -d '/' -f 1 | rev)
TARGET:=$(shell rustc -vV | grep host | cut -d ' ' -f 2-)
cargo=cargo +nightly ${1} --target=${TARGET}
PROJNAME:=$(shell pwd | rev | cut -d '/' -f 1 | rev)
RELEASE_COND=$(shell echo "$@" | grep -q '.*release.*' && echo '-r' || :)
FMTFLAGS:=--config-path $(shell pwd)/rustfmt.toml --unstable-features --error-on-unformatted
format=cargo fmt $1 -- ${FMTFLAGS}
bindeps=Cargo.toml rust-toolchain.toml .cargo/config rustfmt.toml deps-installed $(shell find target -path '*/${1}/*' -name robocup-rs.d -type f -print -quit | xargs cat | cut -d ':' -f 2- | tr ' ' '\n' | grep -v 'spl/' | tr '\n' ' ') | spl-headers.h

release: target/${TARGET}/release/${PROJNAME} tests-passing
debug: target/${TARGET}/debug/${PROJNAME} tests-passing
run-debug: debug; target/${TARGET}/debug/${PROJNAME}
run-release: release; target/${TARGET}/release/${PROJNAME}
run-debug-bg: debug; target/${TARGET}/debug/${PROJNAME} &
run-release-bg: release; target/${TARGET}/release/${PROJNAME} &
disassemble-debug: debug; TODO
disassemble-release: release; TODO

target/%/debug/${PROJNAME}: $(call bindeps,debug)
	make update # sneak this in here so dependencies aren't constantly out of date
	RUSTFLAGS="$(strip ${RUSTFLAGS} ${RUSTDBGFLAGS})" $(call cargo,rustc) ${CARGOFLAGS}
	
target/%/release/${PROJNAME}: $(call bindeps,release)
	make update # sneak this in here so dependencies aren't constantly out of date
	RUSTFLAGS="$(strip ${RUSTFLAGS} ${RUSTOPTFLAGS})" $(call cargo,rustc) ${CARGOFLAGS} -r

spl-headers.h: update-ext
	echo '#ifndef SPL_HEADERS_H /* NOLINT(llvm-header-guard) */' > $@
	echo '#define SPL_HEADERS_H' >> $@
	for file in $$(find ext -type f); do echo '#include "'$${file}'"' >> $@; done
	echo '#endif /* SPL_HEADERS_H */' >> $@

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

update: | update-ext
	rustup $@
	cargo $@

ext:
	mkdir -p $@

update-ext: | ext
	git submodule update --init --remote --recursive

deps-installed: install-dependencies.sh
	rm -f $@
	sh $<
	touch $@

ext/GameController/bin/%: $(shell find ext/GameController -type f ! -path '*/bin/*') | update-ext
	cd ext/GameController && ant

open-gc-%: ext/GameController/bin/%.jar
	ps aux | grep -v grep | grep -q '.*$*.*' || { cd ext/GameController/bin && java -jar $*.jar & }

kill:
	killall ${PROJNAME} || :
	kill $$(ps | grep '.*TeamCommunicationMonitor.*' | grep -v grep | cut -d ' ' -f 1) || :

playtest: open-gc-GameController kill run-debug-bg open-gc-TeamCommunicationMonitor
	@echo ''
	@echo ''
	@echo '%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%'
	@echo "Press the enter key when you'd like to stop."
	@echo '%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%'
	@echo ''
	@echo ''
	@read
	make kill

GIT_NOT_PERFECT_COPY:=[ ! -z "$$(git status --porcelain)" ]
commit_and=

add:
	git branch --show-current | grep -q main && git checkout -b ${USERNAME}-dev || :
	git add -A

commit: add
	@if ${GIT_NOT_PERFECT_COPY}; then \
	  echo "Please write a very brief (~5-word) description of your changes:" \
	  && read -r line_read \
	  && git commit -m "$${line_read}"; \
	  fi

pull: commit
	# git pull
	git pull origin main

push: pull
	git push -u origin $$(git branch --show-current)

pr: push # check
	gh pr create -t "$$(git log -1 --pretty=%B | head -n 1)" -b '${USERNAME} used `make pr`' || :
	gh pr merge --auto --merge
	make clean
	make check
	git checkout main
	git pull
	git branch -d ${USERNAME}-dev || echo 'No `${USERNAME}-dev` branch; this is fine, but if you have a development branch by another name, you should manually delete or update it'
	git remote prune origin
