Pod::Spec.new do |spec|
  spec.name         = 'WalletRsSigner'
  spec.version      = '0.1.0'
  spec.summary      = 'A signer library for wallet-rs'
  spec.homepage     = "https://github.com/wallet-rs/wallet-rs"
  spec.license      = "MPL-2.0"
  spec.author       = { "author" => "shunkakinoki@gmail.com" }

  spec.platforms    = { :ios => "15.0" }
  spec.source       = { :git => "https://github.com/wallet-rs/wallet-rs.git", :tag => "feat/ini-binaries" }
  
  spec.source_files = "lib/*.h"
  spec.public_header_files = "lib/*.h"
  spec.preserve_paths = "lib/*.h"
  spec.vendored_libraries = "lib/libwallet_signer.a"
  spec.xcconfig = { 'HEADER_SEARCH_PATHS' => "${PODS_ROOT}/#{spec.name}/lib/**" }
end