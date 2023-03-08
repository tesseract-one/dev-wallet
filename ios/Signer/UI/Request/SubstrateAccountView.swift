//
//  SubstrateAccountView.swift
//  Signer
//
//  Created by Daniel Leping on 08/03/2023.
//

import SwiftUI

struct SubstrateAccountView: View {
    @Binding var request: SubstrateAccount
    
    var body: some View {
        VStack(alignment: .leading) {
            Text("Algorithm: ")
            Text(request.algorithm).padding(.bottom)
            Text("Path: ")
            Text(request.path).padding(.bottom)
            Text("Public key: ")
            Text(request.key).padding(.bottom)
        }
        .padding()
    }
}

struct SubstrateAccountView_Previews: PreviewProvider {
    static var previews: some View {
        SubstrateAccountView(
            request: .constant(SubstrateAccount(
                algorithm: "algo",
                path: "path",
                key: "placeholderpublickey"
            ))
        )
    }
}
