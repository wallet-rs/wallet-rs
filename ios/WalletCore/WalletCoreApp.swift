//
//  WalletCoreApp.swift
//  WalletCore
//
//  Created by Shun Kakinoki on 5/4/23.
//
import Generated
import SwiftUI

@main
struct WalletCoreApp: App {
  var body: some Scene {
    WindowGroup {
      ContentView()
      Button("Print") {
        print(rustGreeting(name: "Bob"))
        print(setKeychain(key: "Bob"))
      }
    }
  }
}
