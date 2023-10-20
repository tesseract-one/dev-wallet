//
//  Core.swift
//  Developer Wallet
//
//  Created by Daniel Leping on 23/01/2023.
//

import Foundation
import TesseractTransportsService

import CWallet

final class Core {
    fileprivate var `internal`: CCore
    
    public init(ui: UI, dataDir: String, transport: IPCTransportIOS?) throws {
        self.internal = try Result<CCore, WalletError>.wrap { value, error in
            if let transport = transport {
                return wallet_ccore_create_extension(ui.toCore(), dataDir,
                                                     transport.toCore(),
                                                     value, error)
            } else {
                return wallet_ccore_create_app(ui.toCore(), dataDir,
                                               value, error)
            }
        }.get()
    }
    
    fileprivate var settingsProvider: SettingsProvider {
        get throws {
            let rust = try Result<CSettingsProvider, WalletError>.wrap { value, error in
                wallet_ccore_test_settings_provider(self.internal, value, error)
            }.get()
            
            return SettingsProvider(rust: rust)
        }
    }
    
    deinit {
        self.internal.free()
    }
}

extension Core: CoreProtocol {
    var testSettingsProvider: TestSettingsProvider {
        get throws {
            try self.settingsProvider
        }
    }
    
    var keySettingsProvider: KeySettingsProvider {
        get throws {
            try self.settingsProvider
        }
    }
}
