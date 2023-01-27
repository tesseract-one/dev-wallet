//
//  PreviewSettingsProvider.swift
//  Developer Wallet
//
//  Created by Daniel Leping on 27/01/2023.
//

import Foundation

class PreviewSettingsProvider {
    fileprivate var saved: TestSettings
    
    init() {
        self.saved = TestSettings(signature: "signed_by_tesseract", invalidator: "err")
    }
}

extension PreviewSettingsProvider: TestSettingsProvider {
    func load() throws -> TestSettings {
        self.saved
    }
    
    func save(settings: TestSettings) throws {
        self.saved = settings
    }
}
