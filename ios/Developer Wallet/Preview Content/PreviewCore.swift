//
//  PreviewCore.swift
//  Developer Wallet
//
//  Created by Daniel Leping on 27/01/2023.
//

import Foundation

final class PreviewCore {
}

extension PreviewCore: CoreProtocol {
    convenience init(ui: UI, dataDir: String) throws {
        self.init()
    }
    
    var testSettingsProvider: TestSettingsProvider {
        get throws {
            PreviewSettingsProvider()
        }
    }
}
