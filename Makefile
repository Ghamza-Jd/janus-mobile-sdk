android_targets = armv7-linux-androideabi \
	i686-linux-android \
	aarch64-linux-android \
	x86_64-linux-android

android-setup:
	@rustup target add ${android_targets}

android-clean:
	@cd ./android_bindings && ./gradlew clean

android-bindgen:
	@cargo run -- generate ./src/jarust.udl --language kotlin --out-dir ./android_bindings/jarust/src/main/java

android-build:
	@cd ./android_bindings && ./gradlew jarust:assembleRelease
