//
//  Developer_WalletApp.swift
//  Developer Wallet
//
//  Created by Daniel Leping on 23/01/2023.
//

import SwiftUI

@main
struct Developer_WalletApp: App {
    private let core: Core
    
    init() {
        self.core = try! Core(ui: UI(), dataDir: settingsFolder()!, transport: .none)
    }
    
    var body: some Scene {
        WindowGroup {
            ContentView(core: self.core)
        }
    }
}
