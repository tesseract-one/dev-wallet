//
//  PreviewSettingsProvider.swift
//  Developer Wallet
//
//  Created by Daniel Leping on 27/01/2023.
//

import Foundation

class PreviewSettingsProvider {
    fileprivate var test: TestSettings
    fileprivate var key: KeySettings
    
    init() {
        self.test = TestSettings(signature: "signed_by_tesseract", invalidator: "err")
        self.key = KeySettings(mnemonic: "my precious too risky too risky thieves they stole it from us")
    }
}

extension PreviewSettingsProvider: TestSettingsProvider {
    func load() throws -> TestSettings {
        self.test
    }
    
    func save(settings: TestSettings) throws {
        self.test = settings
    }
}

extension PreviewSettingsProvider: KeySettingsProvider {
    func load() throws -> KeySettings {
        self.key
    }
    
    func save(settings: KeySettings) throws {
        self.key = settings
    }
}
