STATIC_LIB_NAME := libwallet_core.a

.PHONY: ios

ios:
	@make build-targets
	@make bindgen-swift
	@make assemble-frameworks
	@make xcframework
	@make cp-xcframework-source

build-targets:
	cargo build --release --target x86_64-apple-ios --package wallet-rs
	cargo build --release --target aarch64-apple-ios-sim --package wallet-rs
	cargo build --release --target aarch64-apple-ios --package wallet-rs

build-targets-debug:
	cargo build --target x86_64-apple-ios --package wallet-rs
	cargo build --target aarch64-apple-ios-sim --package wallet-rs
	cargo build --target aarch64-apple-ios --package wallet-rs

bindgen-swift:
	cargo uniffi-bindgen generate crates/core/src/WalletCore.udl --language swift
	sed -i '' 's/module\ WalletCoreFFI/framework\ module\ WalletCoreFFI/' crates/core/src/WalletCoreFFI.modulemap

assemble-frameworks:
	find . -type d -name WalletCoreFFI.framework -exec rm -rf {} \; || echo "rm failed"
	cd target/x86_64-apple-ios/release && mkdir -p WalletCoreFFI.framework && cd WalletCoreFFI.framework && mkdir Headers Modules Resources && cp ../../../../crates/core/src/WalletCoreFFI.modulemap ./Modules/module.modulemap && cp ../../../../crates/core/src/WalletCoreFFI.h ./Headers/WalletCoreFFI.h && cp ../$(STATIC_LIB_NAME) ./WalletCoreFFI && cp ../../../../crates/core/misc/apple/Info.plist ./Resources
	cd target/aarch64-apple-ios-sim/release && mkdir -p WalletCoreFFI.framework && cd WalletCoreFFI.framework && mkdir Headers Modules Resources && cp ../../../../crates/core/src/WalletCoreFFI.modulemap ./Modules/module.modulemap && cp ../../../../crates/core/src/WalletCoreFFI.h ./Headers/WalletCoreFFI.h && cp ../$(STATIC_LIB_NAME) ./WalletCoreFFI && cp ../../../../crates/core/misc/apple/Info.plist ./Resources
	cd target/aarch64-apple-ios/release && mkdir -p WalletCoreFFI.framework && cd WalletCoreFFI.framework && mkdir Headers Modules Resources && cp ../../../../crates/core/src/WalletCoreFFI.modulemap ./Modules/module.modulemap && cp ../../../../crates/core/src/WalletCoreFFI.h ./Headers/WalletCoreFFI.h && cp ../$(STATIC_LIB_NAME) ./WalletCoreFFI && cp ../../../../crates/core/misc/apple/Info.plist ./Resources

xcframework:
	lipo -create target/x86_64-apple-ios/release/WalletCoreFFI.framework/WalletCoreFFI target/aarch64-apple-ios-sim/release/WalletCoreFFI.framework/WalletCoreFFI -output target/aarch64-apple-ios-sim/release/WalletCoreFFI.framework/WalletCoreFFI
	rm -rf target/WalletCoreFFI.xcframework || echo "skip removing"
	xcodebuild -create-xcframework -framework target/aarch64-apple-ios/release/WalletCoreFFI.framework -framework target/aarch64-apple-ios-sim/release/WalletCoreFFI.framework -output target/WalletCoreFFI.xcframework

cp-xcframework-source:
	cp -r target/WalletCoreFFI.xcframework ios
	cp crates/core/src/WalletCore.swift ios/WalletCoreSource/Sources/Generated
