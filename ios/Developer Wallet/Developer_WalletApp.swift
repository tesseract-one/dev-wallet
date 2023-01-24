//
//  Developer_WalletApp.swift
//  Developer Wallet
//
//  Created by Daniel Leping on 23/01/2023.
//

import SwiftUI

@main
struct Developer_WalletApp: App {
    let core = try! Core(ui: UI(), dataDir: "lalala")
    
    var body: some Scene {
        WindowGroup {
            ContentView()
        }
    }
}
