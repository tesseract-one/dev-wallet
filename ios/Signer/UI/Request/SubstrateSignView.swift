//
//  SubstrateSignView.swift
//  Signer
//
//  Created by Daniel Leping on 08/03/2023.
//

import SwiftUI

struct SubstrateSignView: View {
    @Binding var request: SubstrateSign
    
    var body: some View {
        ScrollView {
            VStack(alignment: .leading) {
                Text("Algorithm: ")
                Text(request.algorithm).padding(.bottom)
                Text("Path: ")
                Text(request.path).padding(.bottom)
                Text("Public key: ")
                Text(request.key).padding(.bottom)
                Text("Data: ")
                Text(request.data).padding(.bottom)
            }
        }.padding()
    }
}

struct SubstrateSignView_Previews: PreviewProvider {
    static var previews: some View {
        SubstrateSignView(
            request: .constant(SubstrateSign(
                algorithm: "algo",
                path: "path",
                key: "placeholderpublickey",
                data: "{\"jsonroot\": \"jsonhereshouldbe\"}"
            ))
        )
    }
}
