//
//  UIExtensionController.swift
//  Signer
//
//  Created by Daniel Leping on 27/01/2023.
//

import Foundation
import SwiftUI

public class UIExtentionController: UIHostingController<AnyView> {
    @objc
    public init(nibName nibNameOrNil: String?, bundle nibBundleOrNil: Bundle?) {
        super.init(rootView: AnyView(EmptyView()))
    }
    
    @MainActor required dynamic init?(coder aDecoder: NSCoder) {
        super.init(coder: aDecoder, rootView: AnyView(EmptyView()))
    }
    
    public override func viewDidLoad() {
        super.viewDidLoad()
        
        guard let className = Bundle.main.extentionClassName else {
            fatalError("Please set UIExtension/UIExtentionMain in your Info.plist to your extension class")
        }
        
        guard let clazz = Bundle.main.classNamed(className) else {
            fatalError("Your extension class `" + className + "` was not found. Maybe it needs a module prefix: `$(PRODUCT_MODULE_NAME)." + className + "`?")
        }
        
        guard let extClazz = clazz as? any UIExtention.Type else {
            fatalError("Your extension class `" + className + "` does not implement protocol `Extension`")
        }
        
        let ext = extClazz.init(controller: self)
        let view = ext.body
        self.rootView = AnyView(view)
    }
}
