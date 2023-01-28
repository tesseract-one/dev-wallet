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
            Spacer()
            switch model.request {
            case .testSign(let request):
                TestSignView(request: .constant(request))
            case .testError(let request):
                TestErrorView(request: .constant(request))
            case .none:
                Text("Invalid request. Probably a bug in the wallet")
            }
            Spacer()
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
            Spacer()
        }
    }
}

struct SignerView_Previews: PreviewProvider {
    static var previews: some View {
        SignerView(model: SignerViewModel())
    }
}
