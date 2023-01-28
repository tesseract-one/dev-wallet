//
//  SUI.swift
//  Developer Wallet
//
//  Created by Daniel Leping on 27/01/2023.
//

import Foundation

import TesseractUtils

import CWallet

extension SUI: CSwiftDropPtr {
    public typealias SObject = UI
}

extension SUI {
    public init(ui: UI) {
        self = SUI(value: ui)
        self.request_user_confirmation = fn_request_user_confirmation
    }
}

private func fn_request_user_confirmation(this: UnsafePointer<SUI>!, rx: CRequest) -> CFutureBool {
    var rx = rx
    let request = rx.owned()
    return CFutureBool {
        try await this.unowned().requestUserConfirmation(request: request)
    }
}

