//
//  UI.swift
//  Developer Wallet
//
//  Created by Daniel Leping on 24/01/2023.
//

import Foundation

import CWallet

public class UI {
    func requestUserConfirmation(tx: String) async throws -> Bool {
        fatalError("No UI is implemented in the Wallet. Use extension.")
    }
    
    func asRust() -> SUI {
        SUI(ui: self)
    }
}
