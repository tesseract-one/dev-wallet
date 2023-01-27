//
//  UIExtension.swift
//  Signer
//
//  Created by Daniel Leping on 27/01/2023.
//

import Foundation
import SwiftUI

public protocol UIExtention {
    associatedtype Body: View
    
    init(controller: UIViewController)
    
    @ViewBuilder @MainActor var body: Body {
        get
    }
}
