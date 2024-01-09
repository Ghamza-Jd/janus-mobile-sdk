targets = aarch64-apple-ios \
	x86_64-apple-ios \
	aarch64-apple-ios-sim

setup:
	@rustup target add ${targets}
