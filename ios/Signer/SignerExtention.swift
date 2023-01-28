//
//  SignerExtention.swift
//  Signer
//
//  Created by Daniel Leping on 27/01/2023.
//

import SwiftUI

import TesseractService

class SignerExtention: UIExtention {
    private let core: Core
    private var model: SignerViewModel
    
    required init(controller: UIViewController) {
        let model = SignerViewModel()
        let ui = UI(model: model)
        let settings = settingsFolder()!
        let transport = IPCTransportIOS(controller)
        
        self.model = model
        self.core = try! Core(ui: ui, dataDir: settings, transport: transport)
    }
    
    var body: some View {
        SignerView(model: self.model)
    }
}
