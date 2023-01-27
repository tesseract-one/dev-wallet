//
//  UI.swift
//  Signer
//
//  Created by Daniel Leping on 27/01/2023.
//

import Foundation

import CWallet

public class UI {
    func requestUserConfirmation(tx: String) async throws -> Bool {
        fatalError("Not implemented yet")
    }
    
    func asRust() -> SUI {
        SUI(ui: self)
    }
}
