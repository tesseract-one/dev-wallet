//
//  UI.swift
//  Developer Wallet
//
//  Created by Daniel Leping on 24/01/2023.
//

import Foundation
import TesseractTransportsService
import CWallet

public class UI {
    func requestUserConfirmation(request: Request) async -> Result<Bool, WalletError> {
        fatalError("No UI is implemented in the Wallet. Use extension.")
    }
    
    func toCore() -> SUI {
        SUI(ui: self)
    }
}
