//
//  CSubstrateSign.swift
//  Developer Wallet
//
//  Created by Daniel Leping on 08/03/2023.
//

import Foundation

import TesseractUtils

import CWallet

extension CSubstrateSign: CType, CPtr {
    public func copied() -> SubstrateSign {
        SubstrateSign(
            algorithm: self.algorithm.copied(),
            path: self.path.copied(),
            key: self.key.copied(),
            data: self.data.copied()
        )
    }
    
    public mutating func owned() -> SubstrateSign {
        defer {
            self.free()
        }
        
        return self.copied()
    }
    
    public mutating func free() {
        wallet_substrate_sign_free(&self)
    }
    
    public typealias Val = SubstrateSign
}
