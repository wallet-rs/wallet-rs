//
//  KeychainPlatformSupportAppApp.swift
//  KeychainPlatformSupportApp
//
//  Created by Shun Kakinoki on 5/18/23.
//

import SwiftUI

@main
struct KeychainPlatformSupportApp: App {
  @UIApplicationDelegateAdaptor(AppDelegate.self) var appDelegate

  var body: some Scene {
    WindowGroup {
      ContentView()
    }
  }
}
