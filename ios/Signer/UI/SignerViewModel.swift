//
//  SignerViewModel.swift
//  Signer
//
//  Created by Daniel Leping on 27/01/2023.
//

import Foundation
import TesseractTransportsService

class SignerViewModel: ObservableObject {
    private var continuation: UnsafeContinuation<Result<Bool, WalletError>, Never>?
    
    @Published var request: Request?
    
    @MainActor
    func confirm(request: Request) async -> Result<Bool, WalletError> {
        if (self.continuation != nil) {
            return .failure(.nested(SignerError.invalidState))
        }
        
        return await withUnsafeContinuation { cont in
            self.continuation = cont
            self.request = request
        }
    }
    
    func sign() {
        self.continuation?.resume(returning: .success(true))
        self.continuation = nil
    }
    
    func cancel() {
        self.continuation?.resume(returning: .success(false))
        self.continuation = nil
    }
}
