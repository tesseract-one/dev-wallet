//
//  Core.swift
//  Developer Wallet
//
//  Created by Daniel Leping on 23/01/2023.
//

import Foundation
import CWallet
import TesseractUtils

class Core {
    private var `internal`: CCore
    
    public init(ui: UI, dataDir: String) throws {
        self.internal = try dataDir.withRef { dataDir in
            try CResult<CCore>.wrap { value, error in
                wallet_ccore_create(UI().asRust(), dataDir, value, error)
            }.get()
        }
    }
    
    deinit {
        self.internal.free()
    }
}

func test111() {
    
}
