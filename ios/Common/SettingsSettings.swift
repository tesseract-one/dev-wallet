//
//  SettingsSettings.swift
//  Signer
//
//  Created by Daniel Leping on 27/01/2023.
//

import Foundation

func settingsFolder() -> String? {
    FileManager.default.containerURL(forSecurityApplicationGroupIdentifier: "one.tesseract.Developer-Wallet.settings")?.path()
}
