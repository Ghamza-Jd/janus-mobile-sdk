targets = aarch64-apple-ios \
	x86_64-apple-ios \
	aarch64-apple-ios-sim

src_dir = ./src/jarust.udl
out_dir = ./target/jarust_custom/ios/bindings/headers
ios_target_dir = ./target/jarust_custom/ios
module_name = JarustNative
archive_name = libjarust.a
staticlib_out_dir = ./target/jarust_custom/ios/static-lib/ios
simstaticlib_out_dir = ./target/jarust_custom/ios/static-lib/ios-sim
x86_64_tar_dir = ./target/x86_64-apple-ios/release
arm64_tar_dir = ./target/aarch64-apple-ios/release
simarm64_tar_dir = ./target/aarch64-apple-ios-sim/release
pkg_dir = ./jarust-ios-package

.PHONY: all

all: clean build bindgen bundle

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
		-library ${staticlib_out_dir}/${archive_name} \
		-headers ${out_dir} \
		-library ${simstaticlib_out_dir}/${archive_name} \
		-headers ${out_dir} \
		-output ${ios_target_dir}/${module_name}.xcframework
	@zip -r ${module_name}.zip ${ios_target_dir}/${module_name}.xcframework
	@openssl dgst -sha256 ${module_name}.zip
	@mv ${module_name}.zip ${ios_target_dir}/${module_name}.zip

clean:
	@rm -rf ${ios_target_dir}/${module_name}.xcframework
	@rm -rf ${ios_target_dir}/${module_name}.zip

cp-pkg:
	@cp ${ios_target_dir}/${module_name}.zip ${pkg_dir}/${module_name}.zip
