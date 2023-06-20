require "json"

package = JSON.parse(File.read(File.join(__dir__, "package.json")))

Pod::Spec.new do |s|
  s.name         = "WalletRsSigner"
  s.version      = package["version"]
  s.summary      = package["description"]
  s.homepage     = "https://github.com/wallet-rs/wallet-rs"
  s.license      = "MIT"
  s.author       = { "author" => "shunkakinoki@gmail.com" }
  s.platforms    = { :ios => "15.0" }
  s.source       = { :git => "https://github.com/wallet-rs/wallet-rs.git", :tag => "wallet-signer-v#{s.version}" }

  s.source_files = "ios/*.h"
  s.public_header_files = "ios/*.h"
  s.preserve_paths = "ios/*.h"
  s.vendored_libraries = "ios/libwallet_signer.a"
  s.xcconfig = { 'HEADER_SEARCH_PATHS' => "${PODS_ROOT}/#{s.name}/ios/**" }
end
