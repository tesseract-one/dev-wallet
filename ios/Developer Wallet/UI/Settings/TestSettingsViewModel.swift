//
//  TestSettingsViewModel.swift
//  Developer Wallet
//
//  Created by Daniel Leping on 23/01/2023.
//

import Foundation
import SwiftUI

class TestSettingsViewModel: ObservableObject {
    let settingsProvider: SettingsProvider
    
    @Published var settings: TestSettings
    @Published var cache: TestSettings
    
    init(settingsProvider: SettingsProvider) throws {
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
