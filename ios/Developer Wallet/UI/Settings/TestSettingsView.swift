//
//  TestSettingsView.swift
//  Developer Wallet
//
//  Created by Daniel Leping on 23/01/2023.
//

import SwiftUI

struct TestSettingsView: View {
    @StateObject private var model: TestSettingsViewModel
    
    init(settingsProvider: SettingsProvider) throws {
        let model = try TestSettingsViewModel(settingsProvider: settingsProvider)
        _model = StateObject(wrappedValue: model)
    }
    
    var body: some View {
        VStack {
            HStack {
                Text("Signature:")
                Spacer()
            }.padding(.top, 20.0)
            TextField("Signature", text: $model.settings.signature).padding(.bottom)
            
            HStack {
                Text("Invalidator:")
                Spacer()
            }
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

struct TestSettingsView_Previews: PreviewProvider {
    static var previews: some View {
        try! TestSettingsView(settingsProvider: SettingsProvider.dummy())
    }
}
