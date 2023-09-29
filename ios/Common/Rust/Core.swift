//
//  Core.swift
//  Developer Wallet
//
//  Created by Daniel Leping on 23/01/2023.
//

import Foundation
import TesseractUtils
import TesseractService

import CWallet

final class Core {
    fileprivate var `internal`: CCore
    
    public init(ui: UI, dataDir: String, transport: IPCTransportIOS?) throws {
        self.internal = try CResult<CCore>.wrap { value, error in
            if let transport = transport {
                return wallet_ccore_create_extension(ui.asCore(), dataDir,
                                                     transport.asNative(),
                                                     value, error)
            } else {
                return wallet_ccore_create_app(ui.asCore(), dataDir,
                                               value, error)
            }
        }.get()
    }
    
    fileprivate var settingsProvider: SettingsProvider {
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
