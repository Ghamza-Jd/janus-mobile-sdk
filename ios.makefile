targets = aarch64-apple-ios \
	x86_64-apple-ios \
	aarch64-apple-ios-sim

src_dir = ./src/jarust.udl
out_dir = ./target/jarust_custom/ios/bindings/headers
module_name = JarustNative
libname = jarust
archive_name = libjarust.a
internal_dir = ./ios_bindings/jarust/internal
staticlib_out_dir = ./target/jarust_custom/ios/static-lib/ios
simstaticlib_out_dir = ./target/jarust_custom/ios/static-lib/ios-sim
x86_64_tar_dir = ./target/x86_64-apple-ios/release
arm64_tar_dir = ./target/aarch64-apple-ios/release
simarm64_tar_dir = ./target/aarch64-apple-ios-sim/release

setup:
	@rustup target add ${targets}

bindgen:
	cargo run -- generate ${src_dir} --language swift --out-dir ${out_dir}
	@mv ${out_dir}/${module_name}.modulemap ${out_dir}/module.modulemap

build:
	@mkdir -p ${staticlib_out_dir}
	@mkdir -p ${simstaticlib_out_dir}

	@for tar in ${targets} ; do \
		cargo build --release --target $$tar ; \
	done

	@lipo -create \
		${x86_64_tar_dir}/${archive_name} \
		${simarm64_tar_dir}/${archive_name} \
		-output ${simstaticlib_out_dir}/${archive_name}

	@cp ${arm64_tar_dir}/${archive_name} ${staticlib_out_dir}/${archive_name}

bundle:
	@xcodebuild -create-xcframework \
		-library ${staticlib_out_dir}/libjarust.a \
		-headers ${out_dir} \
		-library ${simstaticlib_out_dir}/libjarust.a \
		-headers ${out_dir} \
		-output ./target/jarust_custom/ios/JarustNative.xcframework
	@zip -r JarustNative.zip ./target/jarust_custom/ios/JarustNative.xcframework
	@openssl dgst -sha256 JarustNative.zip
	@mv JarustNative.zip ./target/jarust_custom/ios/JarustNative.zip

clean:
	@rm -rf ./target/jarust_custom/ios/JarustNative.xcframework
	@rm -rf ./target/jarust_custom/ios/JarustNative.zip
