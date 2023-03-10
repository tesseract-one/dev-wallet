//
//  TestSignView.swift
//  Signer
//
//  Created by Daniel Leping on 28/01/2023.
//

import SwiftUI

struct TestSignView: View {
    @Binding var request: TestSign
    
    var body: some View {
        VStack(alignment: .leading) {
            
            Text("Transaction: ")
            Text(request.transaction).padding(.bottom)
            Text("Signature: ")
            Text(request.signature).padding(.bottom)
            Text("Result: ")
            Text(request.result).padding(.bottom)
        }
        .padding()
    }
}

struct TestSignView_Previews: PreviewProvider {
    static var previews: some View {
        TestSignView(
            request: .constant(TestSign(
                transaction: "preview_transaction",
                signature: "preview_signature",
                result: "preview_result"
            ))
        )
    }
}
