# thanks https://www.krakend.io/blog/automating-linux-packaging/ !

VERSION := $(shell cargo read-manifest | jq '.version' -r)
PKGNAME := hubauth
LICENSE := Apache 2.0 OR MIT
URL     := https://github.com/liamdawson/hubauth
RELEASE := 0
USER    := hubauth
ARCH    := amd64
DESC    := Use remote sources for SSH authorized_keys, caching for network unavailability
MAINTAINER := Liam Dawson <liam@ldaws.com>

FPM_OPTS= -s dir -v $(VERSION) -n $(PKGNAME) \
	--force \
	--license "$(LICENSE)" \
	--vendor "$(VENDOR)" \
	--maintainer "$(MAINTAINER)" \
	--architecture $(ARCH) \
	--url "$(URL)" \
	--description  "$(DESC)" \
	--config-files etc/ \
	--verbose

DEEP_FPM_OPTS= $(FPM_OPTS) -C dist/deep \
	--before-install scripts/preinst \
    --directories var/cache/hubauth

TAR_NAME= $(PKGNAME)-$(VERSION)-$(ARCH)
DEB_OPTS= $(DEEP_FPM_OPTS) -t deb --deb-use-file-permissions --after-install scripts/postinst
RPM_OPTS= $(DEEP_FPM_OPTS) -t rpm --rpm-use-file-permissions --rpm-attr 2700,hubauth,hubauth:/var/cache/hubauth
TXZ_OPTS= $(FPM_OPTS) -t tar -p "packages/$(TAR_NAME).tar"

all: predist deb rpm txz

txz: binaries docs
	mkdir -p "packages"
	fpm $(TXZ_OPTS) -C dist/shallow/
	bzip2 -fz "packages/$(TAR_NAME).tar"

deb: binaries docs config
	mkdir -p "packages"
	fpm $(DEB_OPTS) -C dist/deep/ -p packages/

rpm: binaries docs config
	mkdir -p "packages"
	fpm $(RPM_OPTS) -C dist/deep/ -p packages/

predist:
	[ ! -d dist ] || rm -rf dist
	mkdir -p dist/shallow dist/docs dist/deep/var/cache/hubauth
	chmod 700 dist/deep/var/cache/hubauth

docs:
	ronn --manual="$(PKGNAME)" -o dist/docs -r5 doc/*.ronn
	cp README.md dist/docs/README
	cp CHANGELOG.md dist/docs/CHANGELOG
	cp doc/hubauth.yml.example dist/docs/
	mkdir -p dist/deep/usr/share/man/man1 dist/deep/usr/share/man/man5
	mkdir -p dist/deep/usr/share/doc/hubauth
	mv dist/docs/*.1 dist/deep/usr/share/man/man1/
	mv dist/docs/*.5 dist/deep/usr/share/man/man5/
	cp dist/docs/* dist/deep/usr/share/doc/hubauth/
	cp -r dist/docs/ dist/shallow/
	rm -rf dist/docs/

binaries:
	cargo build --release
	cp target/release/hubauth dist/shallow
	mkdir -p dist/deep/usr/bin
	cp target/release/hubauth dist/deep/usr/bin/
	chmod 755 dist/shallow/hubauth dist/deep/usr/bin/hubauth

config:
	mkdir -p dist/deep/etc
	cp doc/hubauth.yml.example dist/deep/etc/hubauth.yml
	chmod 600 dist/deep/etc/hubauth.yml
