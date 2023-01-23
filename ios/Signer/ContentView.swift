//
//  ContentView.swift
//  Signer
//
//  Created by Daniel Leping on 23/01/2023.
//

import SwiftUI

struct ContentView: View {
    @StateObject var model: ViewModel
    
    var body: some View {
        VStack {
            Text("Hello, !!!World!")
            AsyncImage(url: model.image)
        }
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        let image = URL(string: "https://cdn.britannica.com/30/94430-050-D0FC51CD/Niagara-Falls.jpg")!
        
        let model = ViewModel(image: image)
        
        ContentView(model: model)
    }
}
