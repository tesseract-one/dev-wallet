//
//  Developer_WalletApp.swift
//  Developer Wallet
//
//  Created by Daniel Leping on 23/01/2023.
//

import SwiftUI

@main
struct Developer_WalletApp: App {
    let core = try! Core(ui: UI(), dataDir: settingsFolder()!, transport: .none)
    
    init() {
        //let aaa = FileManager.default.temporaryDirectory.path()
//        let url = FileManager.default.containerURL(forSecurityApplicationGroupIdentifier: "one.tesseract.Developer-Wallet.settings")
        
//        print(url ?? "!!!none!!!")
        
//        print("lalala")
//        
////        let url = FileManager.default.containerURL(forSecurityApplicationGroupIdentifier: "one.tessteract.Developer-Wallet.settings");
//
//        if let stringData = "test data".data(using: .utf8) {
//            try? stringData.write(to: url!)
//        }
        
        //Date(from: "Test")
    }
    
    var body: some Scene {
        WindowGroup {
            ContentView(core: core)
        }
    }
}
