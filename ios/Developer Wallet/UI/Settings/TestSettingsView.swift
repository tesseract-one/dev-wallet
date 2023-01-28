//
//  TestSettingsView.swift
//  Developer Wallet
//
//  Created by Daniel Leping on 23/01/2023.
//

import SwiftUI

struct TestSettingsView: View {
    @StateObject private var model: TestSettingsViewModel
    
    init(settingsProvider: TestSettingsProvider) throws {
        let model = try TestSettingsViewModel(settingsProvider: settingsProvider)
        _model = StateObject(wrappedValue: model)
    }
    
    var body: some View {
        VStack(alignment: .leading) {
            Text("Test Protocol settings:")
                .font(.system(size: 32))
            
            VStack(alignment: .leading) {
                Text("Signature:").padding(.top)
                TextField("Signature", text: $model.settings.signature)
                
                Text("Invalidator:").padding(.top)
                TextField("Invalidator", text: $model.settings.invalidator)
                
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

struct TestSettingsView_Previews: PreviewProvider {
    
    static var previews: some View {
        try! TestSettingsView(settingsProvider: PreviewSettingsProvider())
    }
}
