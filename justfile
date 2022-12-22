@test tb='native':
    cargo test
    python -m pytest --tb={{tb}}

@lint:
    ruff algovault
    black --check algovault
    cargo fmt --all -- --check
    mypy algovault

@example:
    /usr/local/bin/jupyter-nbconvert --to notebook --execute example_notebook.ipynb --inplace

@run:
    cargo run

@fmt:
    cargo fmt
    black algovault
    
@build:
    cargo build
    maturin build
    pip install target/wheels/* '--force-reinstall'

@ci: lint build test run example
    echo CI PASS