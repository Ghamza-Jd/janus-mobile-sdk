android_make = android.Makefile
ios_make = ios.Makefile

.PHONY: help

help:
	@echo "Usage:"
	@echo "  make <command>"
	@echo ""
	@echo "Commands:"
	@echo " Android:"
	@echo " ========"
	@echo "  android-setup      Install android build targets"
	@echo "  android-clean      Clean android build directory"
	@echo "  android-bindgen    Generate kotlin bindings"
	@echo "  android-build      Generate android archive"
	@echo ""
	@echo " iOS:"
	@echo " ===="
	@echo "  ios-setup          Install ios build targets"

android-setup:
	@make -f ${android_make} setup

android-clean:
	@make -f ${android_make} clean

android-bindgen:
	@make -f ${android_make} bindgen

android-build:
	@make -f ${android_make} build

ios-setup:
	@make -f ${ios_make} setup
