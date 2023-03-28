//
//  HomeView.swift
//  Developer Wallet
//
//  Created by Daniel Leping on 28/03/2023.
//

import SwiftUI

struct HomeView: View {
    var body: some View {
        ScrollView() {
            Text("""
            Thanks for installing the Tesseract developer wallet!\
            
            It's designed to help you test your dApp integration with Wallets through Tesseract protocol.
            
            Here, in the app itself, you won't find the balance or tokens like in the wallets designed to hold crypto. For the sake of simplicity, the app contains only settings to provide your testing private key (don't use the key where you keep your crypto!) and set up the "tesseract test" protocol if needed.
            
            Currently supported protocols:\
            
            - Substrate/Polkadot
            - Tesseract Test protocol (to test tesseract connectivity)
            """).multilineTextAlignment(.leading)
        }
    }
}

struct HomeView_Previews: PreviewProvider {
    static var previews: some View {
        HomeView()
    }
}
