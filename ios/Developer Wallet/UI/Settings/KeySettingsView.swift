//
//  KeySettingsView.swift
//  Developer Wallet
//
//  Created by Daniel Leping on 15/02/2023.
//

import SwiftUI

struct KeySettingsView: View {
    @StateObject private var model: KeySettingsViewModel
    
    init(settingsProvider: KeySettingsProvider) throws {
        let model = try KeySettingsViewModel(settingsProvider: settingsProvider)
        _model = StateObject(wrappedValue: model)
    }
    
    var body: some View {
        VStack(alignment: .leading) {
            Text("Private Key settings:")
                .font(.system(size: 32))
            
            VStack(alignment: .leading) {
                LabeledTextField(alignment: .leading, label: .constant("12 words secret key phrase"), text: $model.settings.mnemonic)
                
                Spacer()
                
                HStack {
                    Spacer()
                    Button(action: model.revert) {
                        Image(systemName: "arrow.counterclockwise")
                        Text("Revert")
                    }.disabled(model.settings == model.cache)
                    Spacer()
                    Button(action: model.save) {
                        Image(systemName: "square.and.arrow.down")
                        Text("Save")
                    }.disabled(model.settings == model.cache)
                    Spacer()
                }
                
                Spacer()
            }.padding()
        }
    }
}

struct KeySettingsView_Previews: PreviewProvider {
    
    static var previews: some View {
        try! KeySettingsView(settingsProvider: PreviewSettingsProvider())
    }
}
