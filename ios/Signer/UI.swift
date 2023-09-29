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
    
    func requestUserConfirmation(request: Request) async throws -> Bool {
        return try await model.confirm(request: request)
    }
    
    func asCore() -> SUI {
        SUI(ui: self)
    }
}
