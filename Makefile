android_targets = armv7-linux-androideabi \
	i686-linux-android \
	aarch64-linux-android \
	x86_64-linux-android

android-setup:
	@rustup target add ${android_targets}

android-clean:
	@cd ./android_bindings && ./gradlew clean
