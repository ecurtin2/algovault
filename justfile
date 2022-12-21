@test tb='native':
    cargo test
    python -m pytest --tb={{tb}}

@lint:
    ruff algovault
    black --check algovault
    cargo fmt --all -- --check
    mypy algovault

@run:
    cargo run

@fmt:
    cargo fmt
    black algovault
    
@build:
    maturin build
    pip install target/wheels/* '--force-reinstall'
