all: bin doc

bin:
	cargo build --release
	mkdir -p dist
	cp target/release/hubauth dist/

doc: man examples html

man:
	mkdir -p dist/man
	ronn -r doc/*.ronn
	bash -c "cp doc/*.{1,5} dist/man/"

html:
	mkdir -p dist/html
	ronn -5 doc/*.ronn
	cp doc/*.html dist/html

examples:
	mkdir -p dist/
	cp doc/hubauth.yml.example dist/
