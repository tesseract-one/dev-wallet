//
//  SettingsProvider.swift
//  Developer Wallet
//
//  Created by Daniel Leping on 26/01/2023.
//

import Foundation
import CWallet

class SettingsProvider {
    public var rust: CSettingsProvider;
    
    public init(rust: CSettingsProvider) {
        self.rust = rust
    }
    
    public static func dummy() -> SettingsProvider {
        SettingsProvider(rust: CSettingsProvider())
    }
}
