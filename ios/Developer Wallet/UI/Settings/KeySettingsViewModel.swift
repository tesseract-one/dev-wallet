//
//  KeySettingsViewModel.swift
//  Developer Wallet
//
//  Created by Daniel Leping on 15/02/2023.
//

import Foundation

class KeySettingsViewModel: ObservableObject {
    let settingsProvider: KeySettingsProvider
    
    @Published var settings: KeySettings
    @Published var cache: KeySettings
    
    init(settingsProvider: KeySettingsProvider) throws {
        self.settingsProvider = settingsProvider
        
        let settings = try self.settingsProvider.load()
        
        self.settings = settings
        self.cache = settings
    }
    
    func revert() {
        let settings = try! self.settingsProvider.load()
        
        self.settings = settings
        self.cache = settings
    }
    
    func save() {
        try! self.settingsProvider.save(settings: self.settings)
        self.cache = self.settings
    }
}
