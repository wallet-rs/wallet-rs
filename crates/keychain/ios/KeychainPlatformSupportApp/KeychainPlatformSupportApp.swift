//
//  KeychainPlatformSupportAppApp.swift
//  KeychainPlatformSupportApp
//
//  Created by Shun Kakinoki on 5/18/23.
//

import SwiftUI
import KeychainPlatformSupport


@main
struct KeychainPlatformSupportApp: App {
    private let keychain = KeychainPlatformSupport.shared
  var body: some Scene {
    WindowGroup {
      ContentView()
    }
  }
}
