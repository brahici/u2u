BIN=u2u
TARGET?=/usr/local/bin
INSTALL=${TARGET}/${BIN}

all: ${BIN}

${BIN}:
	cargo build --release
	@strip -s -v target/release/${BIN}

install: ${BIN}
	@install -v target/release/${BIN} ${INSTALL}

clean:
	@cargo clean

coverage:
	@rustup component add llvm-tools-preview
	@cargo install cargo-llvm-cov
	@cargo llvm-cov run
	@cargo llvm-cov --text
