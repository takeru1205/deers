clean:
	cargo clean
	rm -rf *~ dist *.egg-info build target

build:
	maturin build -i python3 --release
	
develop:
	maturin develop --release

run:
	maturin build -i python3 --release && \
	pip install .

pytest:
	maturin build -i python3 --release && \
	pip install . && \
	pytest

fmt:
	rustfmt src/*
	black test/*
