# CS488 - A4: Ray Tracer

-   Name: Devin Leamy
-   UW ID: 20872933
-   UW User ID: dleamy

## WALL-E

> A ray-tracer named after the most noble robot out there :)

<p align="center">
  <img src="./images/sample.png" width="300" height="300">
</p>

### Structure

-   `/wall-e`: The Rust ray-tracing library.
-   `/wall-e-py`: Rust Python3 bindings, for scripting.
-   `/wall-e-py-macros`: Rust macros used in `/wall-e-py`.
-   `/scripts`: Scripts (scenes) you want to render.
-   `/images`: Images from the scenes requested in the assignment guidelines.
-   `SCRIPTING.md`: Documentation for the scripting API.
-   `PERFORMANCE.md`: Documentation of the impact of introducing bounding volumes on meshes.

### Setup

> _Note: You will need to have Rust and Python3 installed on your system._

Create a virtual environment and install `maturin`.

```bash
cd wall-e-py
python -m venv .env
source .env/bin/activate
pip install maturin
```

### Usage

Scripts, which are python3 files, are put in `/scripts`.

To run a script `./scripts/custom-script.py`, from the root folder, call

```bash
# No debug output.
./run.sh custom-script
# With debug output.
./debug.sh custom-script
```

### Dependencies

-   [pyo3](https://github.com/PyO3/pyo3): Interacting with Rust code using Python.
-   `nalgebra`: Linear algebra cratea.
-   `image`: Image processing crate.
-   `lazy_static`: Utility crate for creating global mutable variables with non-const initialization.
-   `obj`: Utility crate for loading `obj` files.
-   [maturin](https://github.com/PyO3/maturin): Building python3 module generated using `pyo3`
