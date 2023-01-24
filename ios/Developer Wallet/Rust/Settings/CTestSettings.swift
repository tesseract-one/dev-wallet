//
//  TestSettings.swift
//  Developer Wallet
//
//  Created by Daniel Leping on 24/01/2023.
//

import Foundation

import TesseractUtils

import CWallet

extension TestSettings: AsCPtrCopy {
    public func copiedPtr() -> CTestSettings {
        CTestSettings(signature: self.signature.copiedPtr(), invalidator: self.signature.copiedPtr())
    }
    
    public typealias CopyPtr = CTestSettings
    
}

extension CTestSettings: CType, CPtr {
    public func copied() -> TestSettings {
        TestSettings(signature: self.signature.copied(), invalidator: self.invalidator.copied())
    }
    
    public mutating func owned() -> TestSettings {
        defer {
            self.free()
        }
        
        return self.copied()
    }
    
    public mutating func free() {
        wallet_test_settings_free(&self)
    }
    
    public typealias Val = TestSettings
}
