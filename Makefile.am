CARGO_BUILD = ETC_DIR="@sysconfdir@" CACHE_DIR="@localstatedir@/cache/hubauth" cargo build --release

man_MANS = target/man/hubauth.1 \
	target/man/hubauth-cached.1 \
	target/man/hubauth-sync.1 \
	target/man/hubauth-list.1 \
	target/man/hubauth-fetch.1 \
	target/man/hubauth.yml.5

EXTRA_DIST = Cargo.toml \
	build.rs \
	src/cli.rs \
	src/configuration.rs \
	src/fetch.rs \
	src/fs_cache.rs \
	src/lib.rs \
	src/main.rs \
	src/models.rs \
	src/commands/fetch.rs \
	src/commands/list.rs \
	src/commands/sync.rs \
	src/commands/cached.rs \
	src/commands/mod.rs \
	doc/man/mod.rs \
	doc/man/cli.rs \
	doc/man/config.rs \
	doc/mod.rs \
	doc/hubauth.yml.example

EXTRA_PROGRAMS = target/@CARGO_TARGET_DIR@/hubauth
target_@CARGO_TARGET_DIR@_hubauth_SOURCES =

target/man/hubauth-sync.1 target/man/hubauth.1 target/man/hubauth-cached.1 target/man/hubauth-list.1 target/man/hubauth-fetch.1: build.rs doc/man/cli.rs doc/man/mod.rs doc/mod.rs
	$(CARGO_BUILD)
target/man/hubauth.yml.5: build.rs doc/man/config.rs doc/man/mod.rs doc/mod.rs
	$(CARGO_BUILD)

all-local:
	$(CARGO_BUILD)

clean-local:
	cargo clean

install-exec-local:
	$(MKDIR_P) @bindir@
	$(INSTALL) -c -m 755 $(srcdir)/target/@CARGO_TARGET_DIR@/hubauth @bindir@
	$(MKDIR_P) @docdir@
	$(INSTALL) -c $(srcdir)/doc/hubauth.yml.example @docdir@/
	$(MKDIR_P) @sysconfdir@
	$(INSTALL) -c -m 600 $(srcdir)/doc/hubauth.yml.example @sysconfdir@/hubauth.yml
	$(MKDIR_P) @localstatedir@/cache
	$(INSTALL) -d -m 600 @localstatedir@/cache/hubauth
