//
//  Core.swift
//  Developer Wallet
//
//  Created by Daniel Leping on 23/01/2023.
//

import Foundation
import CWallet
import TesseractUtils

class Core {
    private var `internal`: CCore
    
    public init(ui: UI, dataDir: String) throws {
        self.internal = try dataDir.withRef { dataDir in
            try CResult<CCore>.wrap { value, error in
                wallet_ccore_create(ui.asRust(), dataDir, value, error)
            }.get()
        }
    }
    
    public static func dummy() throws -> Core {
        try Core(ui: UI(), dataDir: "lalala")
    }
    
    var settingsProvider: SettingsProvider {
        get throws {
            let rust = try CResult<CSettingsProvider>.wrap { value, error in
                wallet_ccore_test_settings_provider(self.internal, value, error)
            }.get()
            
            return SettingsProvider(rust: rust)
        }
    }
    
    deinit {
        self.internal.free()
    }
}

func test111() {
    
}
