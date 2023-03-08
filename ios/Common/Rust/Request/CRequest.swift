//
//  CRequest.swift
//  Signer
//
//  Created by Daniel Leping on 28/01/2023.
//

import Foundation

import TesseractUtils

import CWallet

extension CRequest: CType, CPtr {
    public func copied() -> Request {
        switch self.tag {
            case CRequest_TestSign:
                return Request.testSign(self.test_sign.copied())
            case CRequest_TestError:
                return Request.testError(self.test_error.copied())
            case CRequest_SubstrateAccount:
                return Request.substrateAccount(self.substrate_account.copied())
            case CRequest_SubstrateSign:
                return Request.substrateSign(self.substrate_sign.copied())
        default:
            fatalError("shity data")
        }
    }
    
    public mutating func owned() -> Request {
        defer {
            self.free()
        }
        
        return self.copied()
    }
    
    public mutating func free() {
        wallet_crequest_free(&self)
    }
    
    public typealias Val = Request
}
