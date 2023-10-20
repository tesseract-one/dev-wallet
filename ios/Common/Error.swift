//
//  Error.swift
//  Developer Wallet
//
//  Created by Yehor Popovych on 20/10/2023.
//

import Foundation
import TesseractTransportsShared
import CWallet

enum WalletError: Error {
    case tesseract(TesseractError)
    case poison(String)
    case io(String)
    case config(String)
    case unknown(String)
}

extension WalletError {
    static func nested<E: Error>(_ error: E) -> Self {
        .tesseract(.swift(error: error as NSError))
    }
}

extension WalletError: TesseractErrorInitializable {
    init(tesseract: TesseractError) {
        guard case .custom(code: let code, reason: let reason) = tesseract else {
            self = .tesseract(tesseract)
            return
        }
        switch WalletErrorCode(code) {
        case WalletErrorCode_Poison: self = .poison(reason)
        case WalletErrorCode_IO: self = .io(reason)
        case WalletErrorCode_Config: self = .io(reason)
        case WalletErrorCode_Unknown: self = .unknown(reason)
        default: fatalError("Uknnown error: \(code), \(reason)")
        }
    }
}

extension WalletError: TesseractErrorConvertible {
    var tesseract: TesseractError {
        switch self {
        case .tesseract(let e): return e
        case .poison(let r):
            return .custom(code: WalletErrorCode_Poison.rawValue, reason: r)
        case .io(let r):
            return .custom(code: WalletErrorCode_IO.rawValue, reason: r)
        case .config(let r):
            return .custom(code: WalletErrorCode_Config.rawValue, reason: r)
        case .unknown(let r):
            return .custom(code: WalletErrorCode_Unknown.rawValue, reason: r)
        }
    }
}
