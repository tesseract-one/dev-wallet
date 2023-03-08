//
//  CSubstrateAccount.swift
//  Developer Wallet
//
//  Created by Daniel Leping on 08/03/2023.
//

import Foundation

import TesseractUtils

import CWallet

extension CSubstrateAccount: CType, CPtr {
    public func copied() -> SubstrateAccount {
        SubstrateAccount(
            algorithm: self.algorithm.copied(),
            path: self.path.copied(),
            key: self.key.copied()
        )
    }
    
    public mutating func owned() -> SubstrateAccount {
        defer {
            self.free()
        }
        
        return self.copied()
    }
    
    public mutating func free() {
        wallet_substrate_account_free(&self)
    }
    
    public typealias Val = SubstrateAccount
}
