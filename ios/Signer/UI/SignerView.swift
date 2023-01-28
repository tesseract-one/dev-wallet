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
            ZStack(alignment: .leading) {
                Color(red: 0xFF/0xFF,
                      green: 0x7D/0xFF,
                      blue: 0x00/0xFF)
                .edgesIgnoringSafeArea(.top)
                VStack(alignment: .leading) {
                    Text("Tesseract")
                        .font(.system(size: 48))
                        .padding(.bottom, 1)
                    Text("Developer Wallet")
                        .font(.system(size: 32))
                }.padding(.leading)
            }.aspectRatio(contentMode: .fit)
        
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
