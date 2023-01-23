//
//  ContentView.swift
//  Developer Wallet
//
//  Created by Daniel Leping on 23/01/2023.
//

import SwiftUI
import CWallet

struct ContentView: View {
    var body: some View {
        TabView {
            TestSettingsView()
                .tabItem {
                    Image(systemName: "testtube.2")
                    Text("Test")
                }
            Text("Screen2")
                .tabItem {
                    Image(systemName: "globe")
                    Text("Screen2")
                }
            Text("Screen3")
                .tabItem {
                    Image(systemName: "mappin.circle.fill")
                    Text("Screen3")
                }
        }
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
