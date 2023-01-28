//
//  CTestError.swift
//  Signer
//
//  Created by Daniel Leping on 28/01/2023.
//

import Foundation

import TesseractUtils

import CWallet

extension CTestError: CType, CPtr {
    public func copied() -> TestError {
        TestError(
            transaction: self.transaction.copied(),
            error: self.error.copied()
        )
    }
    
    public mutating func owned() -> TestError {
        defer {
            self.free()
        }
        
        return self.copied()
    }
    
    public mutating func free() {
        wallet_test_error_free(&self)
    }
    
    public typealias Val = TestError
}
