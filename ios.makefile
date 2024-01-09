targets = aarch64-apple-ios \
	x86_64-apple-ios \
	aarch64-apple-ios-sim

src_dir = ./src/jarust.udl
out_dir = ./target/jarust_custom/ios/bindings/headers
module_name = JarustNative
libname = jarust
internal_dir = ./ios_bindings/jarust/internal

setup:
	@rustup target add ${targets}

bindgen:
	cargo run -- generate ${src_dir} --language swift --out-dir ${out_dir}
	@mv ${out_dir}/${module_name}.modulemap ${out_dir}/module.modulemap
	@cp ${out_dir}/${libname}.swift ${internal_dir}/${libname}.swift

build:
	@for tar in ${targets} ; do \
		cargo build --release --target $$tar ; \
	done
