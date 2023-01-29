//
//  LabeledTextField.swift
//  Developer Wallet
//
//  Created by Daniel Leping on 29/01/2023.
//

import SwiftUI

struct LabeledTextField: View {
    let alignment: HorizontalAlignment
    
    @Binding var label: String
    @Binding var text: String
    
    var body: some View {
        VStack(alignment: alignment) {
            Text(label + ":").padding(.top)
            TextField(label, text: $text).textFieldStyle(.roundedBorder)
        }
    }
}

struct LabeledTextField_Previews: PreviewProvider {
    static var previews: some View {
        LabeledTextField(alignment: .leading, label: .constant("Label"), text: .constant("some text"))
    }
}
