//
//  KeySettingsProvider.swift
//  Developer Wallet
//
//  Created by Daniel Leping on 15/02/2023.
//

import Foundation
import TesseractUtils
import CWallet

protocol KeySettingsProvider {
    func load() throws -> KeySettings;
    func save(settings: KeySettings) throws;
}

extension SettingsProvider: KeySettingsProvider {
    func load() throws -> KeySettings {
        try CResult<CKeySettings>.wrap { value, error in
            wallet_key_settings_provider_load(self.rust, value, error)
        }.get().copied()
    }
    
    func save(settings: KeySettings) throws {
        try CResult<Void>.wrap { err  in
            wallet_key_settings_provider_save(self.rust, settings.copiedPtr(), err)
        }.get()
    }
}
