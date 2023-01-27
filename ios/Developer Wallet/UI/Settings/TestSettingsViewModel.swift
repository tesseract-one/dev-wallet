//
//  TestSettingsViewModel.swift
//  Developer Wallet
//
//  Created by Daniel Leping on 23/01/2023.
//

import Foundation

class TestSettingsViewModel: ObservableObject {
    let settingsProvider: SettingsProvider
    
    @Published var signature: String
    @Published var invalidator: String
    
    init(settingsProvider: SettingsProvider) throws {
        self.settingsProvider = settingsProvider
        
        let settings = try self.settingsProvider.load();
        
        self.signature = settings.signature
        self.invalidator = settings.invalidator
    }
    
    func revert() {
        
    }
    
    func save() {
        
    }
}
