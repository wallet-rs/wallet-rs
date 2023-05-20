import KeychainPlatformSupport
import UIKit

class AppDelegate: UIResponder, UIApplicationDelegate {
  private var keychainPlatformSupport: KeychainPlatformSupport?

  func application(
    _ application: UIApplication,
    didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey: Any]?
  ) -> Bool {
    self.keychainPlatformSupport = KeychainPlatformSupport.shared

    // Perform any app-level setup here
    return true
  }

  func application(
    _ application: UIApplication, didReceiveRemoteNotification userInfo: [AnyHashable: Any]
  ) {
    // Handle push notifications here
  }

  // Add any other app-level methods you need here
}
