CARGO_BUILD = ETC_DIR="@sysconfdir@" CACHE_DIR="@localstatedir@/cache/hubauth" cargo build --release

man_MANS = target/man/hubauth.1 \
	target/man/hubauth-cached.1 \
	target/man/hubauth-sync.1 \
	target/man/hubauth-list.1 \
	target/man/hubauth-fetch.1 \
	target/man/hubauth.yml.5

EXTRA_DIST = Cargo.toml \
	build.rs \
	src \
	doc \
	vendor \
	.cargo/config

EXTRA_PROGRAMS = target/@CARGO_TARGET_DIR@/hubauth
target_@CARGO_TARGET_DIR@_hubauth_SOURCES =

DISTCLEANFILES = hubauth-.tar.xz

target/man/hubauth-sync.1 target/man/hubauth.1 target/man/hubauth-cached.1 target/man/hubauth-list.1 target/man/hubauth-fetch.1: build.rs doc/man/cli.rs doc/man/mod.rs doc/mod.rs all-local
target/man/hubauth.yml.5: build.rs doc/man/config.rs doc/man/mod.rs doc/mod.rs all-local

vendor .cargo/config:
	[ -d .cargo ] || mkdir .cargo
	cargo vendor > .cargo/config

all-local:
	$(CARGO_BUILD)

clean-local:
	cargo clean

install-data-local:
	$(MKDIR_P) @docdir@
	$(INSTALL) -c $(srcdir)/doc/hubauth.yml.example @docdir@/
	$(MKDIR_P) @localstatedir@/cache
	$(INSTALL) -d -m 600 @localstatedir@/cache/hubauth

uninstall-local:
	-rm -rf @docdir@
	-rm -rf @localstatedir@/cache/hubauth
