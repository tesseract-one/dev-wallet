//
//  CKeySettings.swift
//  Developer Wallet
//
//  Created by Daniel Leping on 15/02/2023.
//

import Foundation

import TesseractUtils

import CWallet

extension KeySettings: AsCPtrCopy {
    public func copiedPtr() -> CKeySettings {
        CKeySettings(mnemonic: self.mnemonic.copiedPtr())
    }
    
    public typealias CopyPtr = CKeySettings
    
}

extension CKeySettings: CType, CPtr {
    public func copied() -> KeySettings {
        KeySettings(mnemonic: self.mnemonic.copied())
    }
    
    public mutating func owned() -> KeySettings {
        defer {
            self.free()
        }
        
        return self.copied()
    }
    
    public mutating func free() {
        wallet_key_settings_free(&self)
    }
    
    public typealias Val = KeySettings
}
