ep-ser:
	env RUSTFLAGS="-C target-cpu=native" cargo build --release --bin ep-ser --features $(CLASS)
ep-parallel:
	env RUSTFLAGS="-C target-cpu=native" cargo build --release --bin ep-parallel --features $(CLASS)
is-ser:
	env RUSTFLAGS="-C target-cpu=native" cargo build --release --bin is-ser --features $(CLASS)