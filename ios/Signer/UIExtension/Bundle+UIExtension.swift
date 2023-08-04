//
//  Bundle+UIExtension.swift
//  Signer
//
//  Created by Daniel Leping on 27/01/2023.
//

import Foundation

extension Bundle {
    public var extensionClassName: String? {
        infoDictionary?["UIExtension"]
            .flatMap { $0 as? [String: Any] }
            .flatMap { $0["UIExtensionMain"] }
            .flatMap { $0 as? String }
    }
}
