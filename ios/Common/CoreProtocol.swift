//
//  CoreProtocol.swift
//  Developer Wallet
//
//  Created by Daniel Leping on 27/01/2023.
//

import Foundation

protocol CoreProtocol {
    var testSettingsProvider: TestSettingsProvider {
        get throws
    }
    
    var keySettingsProvider: KeySettingsProvider {
        get throws
    }
}
