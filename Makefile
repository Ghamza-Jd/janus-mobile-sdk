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
	@echo "  ios                Clean, build, bindgen and bundle"
	@echo "  ios-setup          Install ios build targets"
	@echo "  ios-clean          Clean iOS xcframework and zip"
	@echo "  ios-bindgen        Generate swift bindings"
	@echo "  ios-bundle         Create xcframework from archives (.a files)"

android-setup:
	@make -f ${android_make} setup

android-clean:
	@make -f ${android_make} clean

android-bindgen:
	@make -f ${android_make} bindgen

android-build:
	@make -f ${android_make} build

ios:
	@make -f ${ios_make}

ios-setup:
	@make -f ${ios_make} setup

ios-bindgen:
	@make -f ${ios_make} bindgen

ios-build:
	@make -f ${ios_make} build

ios-bundle:
	@make -f ${ios_make} bundle

ios-clean:
	@make -f ${ios_make} clean
