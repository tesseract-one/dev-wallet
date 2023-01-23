//
//  ActionViewController.swift
//  Signer
//
//  Created by Daniel Leping on 23/01/2023.
//

import UIKit
import MobileCoreServices
import UniformTypeIdentifiers
import CWallet
import SwiftUI

class ViewModel: ObservableObject {
    @Published var image: URL
    
    init(image: URL) {
        self.image = image
    }
}

class ActionViewController: UIHostingController<ContentView> {
    var model: ViewModel

    @objc
    public init(
        nibName nibNameOrNil: String?,
        bundle nibBundleOrNil: Bundle?
    ) {
        let image = URL(string: "https://cdn.britannica.com/30/94430-050-D0FC51CD/Niagara-Falls.jpg")!
        
        self.model = ViewModel(image: image);
        
        super.init(rootView: ContentView(model: self.model))
    }

    @MainActor required dynamic init?(coder aDecoder: NSCoder) {
        let image = URL(string: "https://cdn.britannica.com/30/94430-050-D0FC51CD/Niagara-Falls.jpg")!
        
        self.model = ViewModel(image: image);
        
        super.init(coder: aDecoder, rootView: ContentView(model: self.model))
    }
    
    override func viewDidLoad() {
        super.viewDidLoad()
    
        // Get the item[s] we're handling from the extension context.
        
        // For example, look for an image and place it into an image view.
        // Replace this with something appropriate for the type[s] your extension supports.
        var imageFound = false
        for item in self.extensionContext!.inputItems as! [NSExtensionItem] {
            for provider in item.attachments! {
                if provider.hasItemConformingToTypeIdentifier(UTType.image.identifier) {
                    // This is an image. We'll load it, then place it in our image view.
                    weak var weakModel = self.model
                    provider.loadItem(forTypeIdentifier: UTType.image.identifier, options: nil, completionHandler: { (imageURL, error) in
                        if let strongModel = weakModel {
                            if let imageURL = imageURL as? URL {
                                strongModel.image = imageURL
                            }
                        }
                    })
                    
                    imageFound = true
                    break
                }
            }
            
            if (imageFound) {
                // We only handle one image, so stop looking for more.
                break
            }
        }
    }
}
