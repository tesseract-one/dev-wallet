//
//  TestSettingsViewModel.swift
//  Developer Wallet
//
//  Created by Daniel Leping on 23/01/2023.
//

import Foundation

class TestSettingsViewModel: ObservableObject {
    let settingsProvider: TestSettingsProvider
    
    @Published var settings: TestSettings
    @Published var cache: TestSettings
    
    init(settingsProvider: TestSettingsProvider) throws {
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
