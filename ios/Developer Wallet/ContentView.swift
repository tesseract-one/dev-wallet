//
//  ContentView.swift
//  Developer Wallet
//
//  Created by Daniel Leping on 23/01/2023.
//

import SwiftUI
import CWallet

struct ContentView: View {
    let core: Core;
    
    init(core: Core) {
        self.core = core
    }
    
    var body: some View {
        TabView {
            TestSettingsView(settingsProvider: try! core.settingsProvider)
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
        ContentView(core: try! Core.dummy())
    }
}
