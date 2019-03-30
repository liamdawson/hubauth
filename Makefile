all: bin doc

bin:
    cargo build --release --out-dir dist/bin

doc: man examples

man:
    ronn -r doc/*.ronn -o dist/man

examples:
    cp doc/hubauth.yml.example dist/examples/
