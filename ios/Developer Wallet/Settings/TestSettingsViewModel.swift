//
//  TestSettingsViewModel.swift
//  Developer Wallet
//
//  Created by Daniel Leping on 23/01/2023.
//

import Foundation

class TestSettingsViewModel: ObservableObject {
    @Published var signature: String
    @Published var invalidator: String
    
    init() {
        self.signature = "sig_cardcoded"
        self.invalidator = "errorhardcoded"
    }
    
    func revert() {
        
    }
    
    func save() {
        
    }
}
