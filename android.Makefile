targets = armv7-linux-androideabi \
	i686-linux-android \
	aarch64-linux-android \
	x86_64-linux-android

src_dir = ./src/jarust.udl
out_dir = ./android_bindings/jarust/src/main/java

setup:
	@rustup target add ${targets}

clean:
	@cd ./android_bindings && ./gradlew clean

bindgen:
	cargo run -- generate ${src_dir} --language kotlin --out-dir ${out_dir}

build:
	@cd ./android_bindings && ./gradlew :jarust:assembleRelease
