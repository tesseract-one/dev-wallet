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
        var tx: String = "AAAAAAA"
        
        switch request {
        case .testSign(let testSign): tx = testSign.transaction
        default: fatalError()
        }
        
        return try await model.confirm(tx: tx)
    }
    
    func asRust() -> SUI {
        SUI(ui: self)
    }
}
