//
//  TestErrorView.swift
//  Signer
//
//  Created by Daniel Leping on 28/01/2023.
//

import SwiftUI

struct TestErrorView: View {
    @Binding var request: TestError
    
    var body: some View {
        VStack {
            Text("Transaction: ")
            Text(request.transaction)
            Text("Error: ")
            Text(request.error)
        }
    }
}

struct TestErrorView_Previews: PreviewProvider {
    static var previews: some View {
        TestErrorView(
            request: .constant(TestError(
                transaction: "preview_transaction",
                error: "preview_error"
            ))
        )
    }
}
