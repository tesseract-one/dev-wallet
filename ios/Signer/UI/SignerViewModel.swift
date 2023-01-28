//
//  SignerViewModel.swift
//  Signer
//
//  Created by Daniel Leping on 27/01/2023.
//

import Foundation

class SignerViewModel: ObservableObject {
    private var continuation: UnsafeContinuation<Bool, Error>?
    
    @Published var request: Request?
    
    func confirm(request: Request) async throws -> Bool {
        if (self.continuation != nil) {
            throw SignerError.invalidState
        }
        
        return try await withUnsafeThrowingContinuation { cont in
            self.continuation = cont
            DispatchQueue.main.async {
                self.request = request
            }
        }
    }
    
    func sign() {
        self.continuation?.resume(returning: true)
        self.continuation = nil
    }
    
    func cancel() {
        self.continuation?.resume(returning: false)
        self.continuation = nil
    }
}
