//
//  UIExtension.swift
//  Signer
//
//  Created by Daniel Leping on 27/01/2023.
//

import Foundation
import SwiftUI

@MainActor
public protocol UIExtension {
    associatedtype Body: View
    
    init(controller: UIViewController)
    
    @ViewBuilder var body: Body {
        get
    }
}
