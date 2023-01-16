app:
  cd app; npm i

serve-en:
  cd app; RUSTFLAGS=--cfg=web_sys_unstable_apis trunk --config Trunk.toml serve

build-en:
  cd app; RUSTFLAGS=--cfg=web_sys_unstable_apis trunk --config Trunk.toml build --release

en:
  cd tools; cargo run --bin tools -- -i en -o ../dist/en/.stage/assets

serve-zh-cn:
  cd app; RUSTFLAGS=--cfg=web_sys_unstable_apis trunk --config Trunk.zh-cn.toml serve

build-zh-cn:
  cd app; RUSTFLAGS=--cfg=web_sys_unstable_apis trunk --config Trunk.zh-cn.toml build --release

zh-cn:
  cd tools; cargo run --bin tools --features "zh-cn" --no-default-features -- -i zh-cn -o ../dist/zh-cn/.stage/assets
