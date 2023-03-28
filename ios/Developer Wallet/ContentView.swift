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
                HomeView()
                    .tabItem {
                        Image(systemName: "house")
                        Text("Home")
                    }
                try! KeySettingsView(settingsProvider: try! core.keySettingsProvider)
                    .tabItem {
                        Image(systemName: "person.badge.key")
                        Text("Private Key")
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
