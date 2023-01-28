//
//  CTestSign.swift
//  Signer
//
//  Created by Daniel Leping on 28/01/2023.
//

import Foundation

import TesseractUtils

import CWallet

extension CTestSign: CType, CPtr {
    public func copied() -> TestSign {
        TestSign(
            transaction: self.transaction.copied(),
            signature: self.signature.copied(),
            result: self.result.copied()
        )
    }
    
    public mutating func owned() -> TestSign {
        defer {
            self.free()
        }
        
        return self.copied()
    }
    
    public mutating func free() {
        wallet_test_sign_free(&self)
    }
    
    public typealias Val = TestSign
}
