man_MANS = target/man/hubauth*.1 \
	target/man/hubauth*.5

EXTRA_DIST = Cargo.toml \
	build.rs \
	src/*.rs \
	doc/*

EXTRA_PROGRAMS = target/@CARGO_TARGET_DIR@/hubauth
target_@CARGO_TARGET_DIR@_hubauth_SOURCES =

all-local:
	cargo build --release

clean-local:
	cargo clean

install-exec-local:
	$(MKDIR_P) $(DESTDIR)$(bindir)
	$(INSTALL) -c -m 755 target/@CARGO_TARGET_DIR@/hubauth $(DESTDIR)$(bindir)
	$(MKDIR_P) $(DESTDIR)$(docdir)/
	$(INSTALL) -c doc/hubauth.yml.example $(DESTDIR)$(docdir)/
	$(INSTALL) -c -m 600 doc/hubauth.yml.example /etc/hubauth.yml
	$(INSTALL) -d -m 600 /var/cache/hubauth
