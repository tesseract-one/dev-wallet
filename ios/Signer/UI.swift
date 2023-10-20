//
//  UI.swift
//  Signer
//
//  Created by Daniel Leping on 27/01/2023.
//

import Foundation
import TesseractTransportsService
import CWallet

public class UI {
    private let model: SignerViewModel
    
    init(model: SignerViewModel) {
        self.model = model
    }
    
    func requestUserConfirmation(request: Request) async -> Result<Bool, WalletError> {
        await model.confirm(request: request)
    }
    
    func toCore() -> SUI {
        SUI(ui: self)
    }
}
