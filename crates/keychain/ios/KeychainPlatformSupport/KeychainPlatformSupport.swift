//
//  KeychainPlatformSupport.swift
//  KeychainPlatformSupport
//
//  Created by Shun Kakinoki on 5/18/23.
//

import Foundation

public final class PlatformSupport {
  public static let shared = KeychainPlatformSupport()

  private let keychain: Keychain

  private init() {
    self.keychain = Keychain()

    initPlatformSupport(keychain: self.keychain)
  }
}
