serve:
  cd frontend; trunk serve --dist ../dist

build:
  cd frontend; trunk build --dist ../dist

en:
  cd tools; cargo run --release --bin viz-rs-tools -- -i en -o ../dist/.stage/assets

zh:
  cd tools; cargo run --release --bin viz-rs-tools -- -i zh -o ../dist/.stage/assets

release-en: build en

release-zh: build zh
