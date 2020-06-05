all:
	cargo clean
	-scp -r ../basic_classic_theorycrafting grizzly:
	ssh -t grizzly ". ~/.profile && cd basic_classic_theorycrafting && cargo run --release"

