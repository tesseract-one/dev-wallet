//
//  SignerView.swift
//  Signer
//
//  Created by Daniel Leping on 28/01/2023.
//

import SwiftUI

struct SignerView: View {
    @ObservedObject private var model: SignerViewModel
    
    init(model: SignerViewModel) {
        self.model = model
    }
    
    var body: some View {
        VStack {
            Text(model.transaction ?? "")
            HStack {
                Spacer()
                Button(action: model.cancel) {
                    Image(systemName: "xmark.seal")
                    Text("Reject")
                }
                Spacer()
                Button(action: model.sign) {
                    Image(systemName: "checkmark.seal")
                    Text("Sign")
                }
                Spacer()
            }
        }
    }
}

struct SignerView_Previews: PreviewProvider {
    static var previews: some View {
        SignerView(model: SignerViewModel())
    }
}
