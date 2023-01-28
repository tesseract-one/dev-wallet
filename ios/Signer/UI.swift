//
//  UI.swift
//  Signer
//
//  Created by Daniel Leping on 27/01/2023.
//

import Foundation

import CWallet

public class UI {
    private let model: SignerViewModel
    
    init(model: SignerViewModel) {
        self.model = model
    }
    
    func requestUserConfirmation(tx: String) async throws -> Bool {
        try await model.confirm(tx: tx)
    }
    
    func asRust() -> SUI {
        SUI(ui: self)
    }
}
