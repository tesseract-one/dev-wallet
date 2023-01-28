//
//  Bundle+UIExtension.swift
//  Signer
//
//  Created by Daniel Leping on 27/01/2023.
//

import Foundation

extension Bundle {
    public var extensionClassName: String? {
        var config: [String: Any]?
                
        if let infoPlistPath = self.url(forResource: "Info", withExtension: "plist") {
            do {
                let infoPlistData = try Data(contentsOf: infoPlistPath)
                
                if let dict = try PropertyListSerialization.propertyList(from: infoPlistData, options: [], format: nil) as? [String: Any] {
                    config = dict
                }
            } catch {
                print(error)
            }
        }
        
        return (config?["UIExtension"] as? NSDictionary)?["UIExtensionMain"].flatMap {
            $0 as? String
        }
    }
}
