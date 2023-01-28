//
//  Request.swift
//  Signer
//
//  Created by Daniel Leping on 28/01/2023.
//

import Foundation

public enum Request {
    case testSign(TestSign)
    case testError(TestError)
}
