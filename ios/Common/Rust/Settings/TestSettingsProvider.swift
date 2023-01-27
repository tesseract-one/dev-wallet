//
//  TestSettingsProvider.swift
//  Developer Wallet
//
//  Created by Daniel Leping on 26/01/2023.
//

import Foundation
import TesseractUtils
import CWallet

protocol TestSettingsProvider {
    func load() throws -> TestSettings;
    func save(settings: TestSettings) throws;
}

extension SettingsProvider: TestSettingsProvider {
    func load() throws -> TestSettings {
        try CResult<CTestSettings>.wrap { value, error in
            wallet_test_settings_provider_load(self.rust, value, error)
        }.get().copied()
    }
    
    func save(settings: TestSettings) throws {
        try CResult<Void>.wrap { err  in
            wallet_test_settings_provider_save(self.rust, settings.copiedPtr(), err)
        }.get()
    }
}
