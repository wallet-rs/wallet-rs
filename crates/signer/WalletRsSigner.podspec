Pod::Spec.new do |spec|
  spec.name         = 'WalletRsSigner'
  spec.version      = '0.1.0'
  spec.summary      = 'A signer library for wallet-rs'
  spec.homepage     = "https://github.com/wallet-rs/wallet-rs"
  spec.license      = "MPL-2.0"
  spec.author       = { "author" => "shunkakinoki@gmail.com" }

  spec.platforms    = { :ios => "15.0" }
  spec.source       = { :git => "https://github.com/wallet-rs/wallet-rs.git" }
  
  spec.source_files = "ios/*.h"
  spec.public_header_files = "ios/*.h"
  spec.preserve_paths = "ios/*.h"
  spec.vendored_libraries = "ios/libwallet_signer.a"
  spec.xcconfig = { 'HEADER_SEARCH_PATHS' => "${PODS_ROOT}/#{spec.name}/ios/**" }
end