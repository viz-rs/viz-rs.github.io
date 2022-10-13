serve:
  cd frontend; trunk serve --dist ../dist

build:
  cd frontend; trunk build --dist ../dist

en:
  cd tools; cargo run --release --bin viz-rs-tools -- -i en

zh:
  cd tools; cargo run --release --bin viz-rs-tools -- -i zh

release-en: build en

release-zh: build zh
