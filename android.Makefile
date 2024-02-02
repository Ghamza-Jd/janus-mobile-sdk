targets = armv7-linux-androideabi \
	i686-linux-android \
	aarch64-linux-android \
	x86_64-linux-android

src_dir = ./src/jarust.udl
out_dir = ./jarust-android-package/jarust/src/main/java
package_dir = ./jarust-android-package

.PHONY: all

all: clean build

setup:
	@rustup target add ${targets}

clean:
	@cd ${package_dir} && ./gradlew clean

bindgen:
	cargo run -- generate ${src_dir} --language kotlin --out-dir ${out_dir}

build:
	@cd ${package_dir} && ./gradlew :jarust:assembleRelease
