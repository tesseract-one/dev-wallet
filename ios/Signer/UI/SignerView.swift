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
        VStack(alignment: .leading) {
            HeaderView()
        
            Spacer()
            VStack {
                switch model.request {
                case .testSign(let request):
                    TestSignView(request: .constant(request))
                case .testError(let request):
                    TestErrorView(request: .constant(request))
                case .substrateAccount (let request):
                    SubstrateAccountView(request: .constant(request))
                case .substrateSign (let request):
                    SubstrateSignView(request: .constant(request))
                case .none:
                    HStack {
                        Spacer()
                        Text("Invalid request. Probably a bug in the wallet")
                        Spacer()
                    }
                }
            }.padding()
            Spacer()
            HStack {
                Spacer()
                Button(action: model.cancel) {
                    Image(systemName: "xmark.seal.fill")
                    Text("Reject")
                }
                Spacer()
                Button(action: model.sign) {
                    Image(systemName: "checkmark.seal.fill")
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
