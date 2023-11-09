# CS488 - A4: Ray Tracer

-   Name: Devin Leamy
-   UW ID: 20872933
-   UW User ID: dleamy

## WALL-E

> A ray-tracer named after the most noble robot out there :)

### Structure

-   `/wall-e`: The Rust ray-tracing library.
-   `/wall-e-py`: Rust Python3 bindings, for scripting.
-   `/wall-e-py-macros`: Rust macros used in `/wall-e-py`.
-   `/scripts`: Scripts (scenes) you want to render.
-   `SCRIPTING.md`: Documentation for the scripting API.

### Usage

Scripts, which are python3 files, are put in `/scripts`.

To run a script `custom-script.py`, call

```bash
./run.sh custom-script
```
