//
//  HeaderView.swift
//  Developer Wallet
//
//  Created by Daniel Leping on 29/01/2023.
//

import SwiftUI

struct HeaderView: View {
    var body: some View {
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
    }
}

struct HeaderView_Previews: PreviewProvider {
    static var previews: some View {
        HeaderView()
    }
}
