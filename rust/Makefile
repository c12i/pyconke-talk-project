single_threaded:
	cargo run --example single_threaded --release

multi_threaded:
	cargo run --example multi_threaded --release

build_cpython: build
	cp ./target/release/libpalingrams_cpython.dylib ./palingrams_cpython/examples.py/palingrams_cpython.so

build_pyo3: build
	cp ./target/release/libpalingrams_pyo3.dylib ./palingrams_pyo3/examples.py/palingrams_pyo3.so

run_cpython: build_cpython
	python ./palingrams_cpython/examples.py/test.py

run_pyo3: build_pyo3
	python ./palingrams_pyo3/examples.py/test.py

build:
	cargo build --workspace --release