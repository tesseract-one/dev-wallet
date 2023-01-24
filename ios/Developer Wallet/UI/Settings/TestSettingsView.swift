//
//  TestSettingsView.swift
//  Developer Wallet
//
//  Created by Daniel Leping on 23/01/2023.
//

import SwiftUI

struct TestSettingsView: View {
    @StateObject var model = TestSettingsViewModel()
    
    var body: some View {
        VStack {
            HStack {
                Text("Signature:")
                Spacer()
            }.padding(.top, 20.0)
            TextField("Signature", text: $model.signature).padding(.bottom)
            
            HStack {
                Text("Invalidator:")
                Spacer()
            }
            TextField("Invalidator", text: $model.invalidator)
            
            Spacer()
            
            HStack {
                Spacer()
                Button(action: model.revert) {
                    Image(systemName: "arrow.counterclockwise")
                    Text("Revert")
                }
                Spacer()
                Button(action: model.save) {
                    Image(systemName: "square.and.arrow.down")
                    Text("Save")
                }
                Spacer()
            }
            
            Spacer()
        }.padding()
    }
}

struct TestSettingsView_Previews: PreviewProvider {
    static var previews: some View {
        TestSettingsView()
    }
}
