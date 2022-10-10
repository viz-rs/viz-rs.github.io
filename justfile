serve:
  cd frontend; trunk serve --dist ../dist

docs:
  cd docs; cargo run --release --bin viz-rs-tools -- --nocapture

build:
  cd frontend; trunk build --dist ../dist
  just docs
