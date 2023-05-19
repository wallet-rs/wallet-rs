import Foundation

final class Keychain {
  func getSigningKey(identifier: String) throws -> String {
    do {
      return try! "from native swift"
    } catch {
        throw KeychainError.Fatal(error: "Unable to initalize native swift keychain module")
    }
  }
}
