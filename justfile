serve:
  cd frontend; RUSTFLAGS=--cfg=web_sys_unstable_apis trunk serve --dist ../dist

build:
  cd frontend; RUSTFLAGS=--cfg=web_sys_unstable_apis trunk build --release --dist ../dist

en:
  cd tools; cargo run --release --bin viz-rs-tools -- -i en -o ../dist/.stage/assets

zh:
  cd tools; cargo run --release --bin viz-rs-tools -- -i zh -o ../dist/.stage/assets

release-en: build en

release-zh: build zh
