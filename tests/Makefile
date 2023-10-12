ROOT_DIR:=$(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))
VERSION=$(shell cat Cargo.toml|grep version | cut -f2 -d\")
VERSION_MAJOR=$(shell echo $(VERSION) | cut -f1 -d.)
VERSION_MINOR=$(shell echo $(VERSION) | cut -f2 -d.)
VERSION_MICRO=$(shell echo $(VERSION) | cut -f3 -d.)

ifeq ($(shell uname), Linux)
    CLIB_SO_DEV = libbink.so
else ifeq ($(shell uname), Darwin)
    CLIB_SO_DEV = libbink.dylib
else
    $(error Unsupported OS)
endif

CLIB_SO_FULL=$(CLIB_SO_DEV).$(VERSION)
CLIB_HEADER=bink.h
CLIB_SO_DEV_RELEASE=target/release/$(CLIB_SO_DEV)
CLIB_SO_DEV_DEBUG=target/debug/$(CLIB_SO_DEV)
PREFIX ?= /usr/local

#outdir is used by COPR as well: https://docs.pagure.org/copr.copr/user_documentation.html
outdir ?= $(ROOT_DIR)

CPU_BITS = $(shell getconf LONG_BIT)
ifeq ($(CPU_BITS), 32)
    LIBDIR ?= $(PREFIX)/lib
else
    LIBDIR ?= $(PREFIX)/lib$(CPU_BITS)
endif

INCLUDE_DIR ?= $(PREFIX)/include
PKG_CONFIG_LIBDIR ?= $(LIBDIR)/pkgconfig

SKIP_VENDOR_CREATION ?=0
RELEASE ?=0

.PHONY: debug
debug: $(CLIB_SO_DEV_DEBUG)
	cp include/$(CLIB_HEADER) target/debug/
	mv $(CLIB_SO_DEV_DEBUG) target/debug/$(CLIB_SO_FULL)
	ln -sfv $(CLIB_SO_FULL) target/debug/$(CLIB_SO_DEV)

$(CLIB_SO_DEV_RELEASE):
	cargo build --all --release

$(CLIB_SO_DEV_DEBUG):
	cargo build --all

clib: $(CLIB_HEADER) $(CLIB_SO_DEV_RELEASE) 
	cp include/$(CLIB_HEADER) target/release/
	mv $(CLIB_SO_DEV_RELEASE) target/release/$(CLIB_SO_FULL)
	ln -sfv $(CLIB_SO_FULL) target/release/$(CLIB_SO_DEV)

.PHONY: clib_check
clib_check: $(CLIB_SO_DEV_DEBUG)
	$(eval TMPDIR := $(shell mktemp -d))
	cp $(CLIB_SO_DEV_DEBUG) $(TMPDIR)/$(CLIB_SO_FULL)
	ln -sfv $(CLIB_SO_FULL) $(TMPDIR)/$(CLIB_SO_DEV)
	mv $(CLIB_HEADER) $(TMPDIR)/$(shell basename $(CLIB_HEADER))
	cc -g -Wall -Wextra -L$(TMPDIR) -I$(TMPDIR) \
		-o $(TMPDIR)/bink_test clib/tests/bink_test.c -lbink
	LD_LIBRARY_PATH=$(TMPDIR) \
		valgrind --trace-children=yes --leak-check=full \
		--error-exitcode=1 \
		$(TMPDIR)/bink_test 1>/dev/null
	rm -rf $(TMPDIR)

rust_check:
	cargo test -- --show-output;

check: rust_check clib_check

test: clean debug
	cc -g -Wall -Wextra -Ltarget/debug -Itarget/debug \
		-o target/debug/bink_test ./tests/bink_test.c -lbink
	LD_LIBRARY_PATH=target/debug \
		target/debug/bink_test
	cc -g -Wall -Wextra -Ltarget/debug -Itarget/debug \
		-o target/debug/bink_tags_test ./tests/bink_tags_test.c -lbink
	LD_LIBRARY_PATH=target/debug \
		target/debug/bink_tags_test

clean:
	- cargo clean

install: $(CLI_EXEC_RELEASE) clib
	install -p -v -D -m755 $(CLI_EXEC_RELEASE) \
		$(DESTDIR)$(PREFIX)/bin/$(CLI_EXEC)
	install -p -D -m755 $(CLIB_SO_DEV_RELEASE) \
		$(DESTDIR)$(LIBDIR)/$(CLIB_SO_FULL)
	ln -sfv $(CLIB_SO_FULL) $(DESTDIR)$(LIBDIR)/$(CLIB_SO_DEV)
	install -p -v -D -m644 $(CLIB_HEADER) \
		$(DESTDIR)$(INCLUDE_DIR)/$(shell basename $(CLIB_HEADER))

uninstall:
	- rm -fv $(DESTDIR)$(PREFIX)/bin/$(CLI_EXEC)
	- rm -fv $(DESTDIR)$(LIBDIR)/$(CLIB_SO_DEV)
	- rm -fv $(DESTDIR)$(LIBDIR)/$(CLIB_SO_FULL)
	- rm -fv $(DESTDIR)$(INCLUDE_DIR)/$(shell basename $(CLIB_HEADER))

