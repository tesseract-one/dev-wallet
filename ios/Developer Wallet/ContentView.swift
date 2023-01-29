//
//  ContentView.swift
//  Developer Wallet
//
//  Created by Daniel Leping on 23/01/2023.
//

import SwiftUI
import CWallet

struct ContentView: View {
    let core: CoreProtocol;
    
    init(core: CoreProtocol) {
        self.core = core
    }
    
    var body: some View {
        VStack {
            HeaderView()
            TabView {
                Text("Home")
                    .tabItem {
                        Image(systemName: "house")
                        Text("Home")
                    }
                Text("Screen3")
                    .tabItem {
                        Image(systemName: "mappin.circle.fill")
                        Text("Screen3")
                    }
                try! TestSettingsView(settingsProvider: try! core.testSettingsProvider)
                    .tabItem {
                        Image(systemName: "testtube.2")
                        Text("Test Protocol")
                    }
            }.padding()
        }
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView(core: PreviewCore())
    }
}
