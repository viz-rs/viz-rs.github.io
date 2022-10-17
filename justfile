serve-en:
  cd frontend; RUSTFLAGS=--cfg=web_sys_unstable_apis trunk serve --dist ../dist

build-en:
  cd frontend; RUSTFLAGS=--cfg=web_sys_unstable_apis trunk build --release --dist ../dist

en:
  cd tools; cargo run --release --bin viz-rs-tools -- -i en -o ../dist/.stage/assets

serve-zh:
  cd frontend; RUSTFLAGS=--cfg=web_sys_unstable_apis trunk --config Trunk.zh.toml serve --dist ../dist

build-zh:
  cd frontend; RUSTFLAGS=--cfg=web_sys_unstable_apis trunk --config Trunk.zh.toml build --release --dist ../dist

zh:
  cd tools; cargo run --release --bin viz-rs-tools -- -i zh -o ../dist/.stage/assets
