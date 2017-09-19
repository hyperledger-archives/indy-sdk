//
//  IndyKeychainError.swift
//  libindy
//

import Foundation


public enum KeychainError: String, Error
{
    /// Failed to allocate memory
    case allocate = "Failed to allocate memory."
    
    /// Unable to decode the provided data.
    case decode = "Unable to decode the provided data."
    
    /// The item already exists.
    case duplicate = "The item already exists."
    
    /// Interaction with the Security Server is not allowed.
    case interactionNotAllowed = "Interaction with the Security Server is not allowed."
    
    /// No trust results are available.
    case notAvailable = "No trust results are available."
    
    /// The item cannot be found.
    case notFound = "The item cannot be found."
    
    /// One or more parameters passed to the function were not valid.
    case param = "One or more parameters passed to the function were not valid."
    
    /// The request was not set
    case requestNotSet = "The request was not set"
    
    /// The type was not found
    case typeNotFound = "The type was not found"
    
    /// Unable to clear the keychain
    case unableToClear = "Unable to clear the keychain"
    
    /// An undefined error occurred
    case undefined = "An undefined error occurred"
    
    /// Function or operation not implemented.
    case unimplemented = "Function or operation not implemented."
    
    init?(fromStatusCode code: Int)
    {
        switch code
        {
        case Int(errSecAllocate):
            self = .allocate
        case Int(errSecDecode):
            self = .decode
        case Int(errSecDuplicateItem):
            self = .duplicate
        case Int(errSecInteractionNotAllowed):
            self = .interactionNotAllowed
        case Int(errSecItemNotFound):
            self = .notFound
        case Int(errSecNotAvailable):
            self = .notAvailable
        case Int(errSecParam):
            self = .param
        case Int(errSecUnimplemented):
            self = .unimplemented
        default:
            return nil
        }
    }
}
