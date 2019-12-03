generated_files=main.out

all: main.out

main.out: main.rs
	rustc main.rs

clean:
	rm -f ${generated_files}
