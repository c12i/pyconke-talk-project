# PyconKE 2022: How to speed up your python code with rust

This repo contains the demo project for the talk. You will find the two subfolders `rust` and `python` which contain the demo programs.

### Python
In `/python` you will find 3 programs that you can execute individually via `python` from the command line.

### Rust
The rust code is organized into a workspace of 3 projects:
- `palingrams_rs`: contains the palingrams functions 
- `palingrams_cpython`: contains cpython bindings; imports functions from `palingrams_rs`
- `palingrams_pyo3`: contains pyo3 bindings; imports functions from `palingrams_rs`

At the root of `/rust` you will find a `Makefile` which contains a set of targets to build the crates and run resulting code

Run rust examples:

```
make single_threaded

make multi_threaded
```

`palingrams_cpython` and `palingrams_pyo3` workspace projects contain an `examples.py` directory which contains the python code that consumes
the respective compiled python modules as well as a respective shared object `.so` file which is the compiled module itself.
To build the respective modules, run the build make targets:

```
make build_cpython

make build_pyo3
```

To run the python code consuming the respective compiled modules:

```
make run_cpython

make run_pyo3
```

Author: [Collins Muriuki](collinsmuriuki.xyz)