serve-en:
  cd app; RUSTFLAGS=--cfg=web_sys_unstable_apis trunk serve --dist ../dist

build-en:
  cd app; RUSTFLAGS=--cfg=web_sys_unstable_apis trunk build --release --dist ../dist

en:
  cd tools; cargo run --bin tools -- -i en -o ../dist/.stage/assets

serve-zh-cn:
  cd app; RUSTFLAGS=--cfg=web_sys_unstable_apis trunk --config Trunk.zh-cn.toml serve --dist ../dist

build-zh-cn:
  cd app; RUSTFLAGS=--cfg=web_sys_unstable_apis trunk --config Trunk.zh-cn.toml build --release --dist ../dist 

zh-cn:
  cd tools; cargo run --bin tools --features "zh-cn" --no-default-features -- -i zh-cn -o ../dist/.stage/assets
