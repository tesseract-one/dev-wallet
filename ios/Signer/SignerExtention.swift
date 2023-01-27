//
//  SignerExtention.swift
//  Signer
//
//  Created by Daniel Leping on 27/01/2023.
//

import SwiftUI

import TesseractService

class SignerExtention: UIExtention {
    let core: Core
    
    required init(controller: UIViewController) {
        let ui = UI()
        let settings = settingsFolder()!
        let transport = IPCTransportIOS(controller)
        
        self.core = try! Core(ui: ui, dataDir: settings, transport: transport)
    }
    
    var body: some View {
        Text("!!!It's alive!!!5")
    }
}
