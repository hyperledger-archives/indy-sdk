//
//  KeychainWalletItem.swift
//  libindy
//
//  Created by Anastasia Tarasova on 04/09/2017.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

import Foundation

@objc protocol IndyWalletKeychainStorable: NSObjectProtocol
{

}


@objc class IndyKeychainWalletItem: NSObject, IndyWalletKeychainStorable
{
    var serviceName = "IndyKeychainWallet"
    
    fileprivate (set) var poolName: String
    
    fileprivate (set) var name: String
    
    fileprivate (set) var config: String?
    
    fileprivate var values: [String: WalletValue] = [:]
    
    fileprivate var keychain: KeychainWrapper
    
    
    init(name: String, poolName: String, config: String, credentials: String)
    {
        self.name = name
        self.poolName = poolName
        self.config = config
        
        self.keychain = KeychainWrapper(service: self.serviceName, account: self.name)
    }

    
    fileprivate var data: [String: Any] {
        var dictionary: [String: Any] = [:]
        dictionary[WalletAttributes.poolName.rawValue] = self.poolName
        dictionary[WalletAttributes.config.rawValue] = self.config
        dictionary[WalletAttributes.values.rawValue] = self.values
        
        return dictionary
    }
}

extension IndyKeychainWalletItem
{
      func deleteFromKeychain() throws
    {
        try self.keychain.delete()
    }
    
    func readFromKeychain()
    {
        guard let resultData = try? self.keychain.read() else
        {
            return
        }
        
        
    }
    
    
    func set(value: String, forKey key: String) throws
    {
        var storedData = [String: Any]()
        do
        {
            storedData = try self.keychain.read()
        }
        catch
        {
            throw error
        }
        
        
        
        
        self.values[key] = WalletValue(value: value, timeCreated: self.currentTime)
        
        try self.keychain.update(data: self.data)
    }
    
    func getValue(forKey key: String) throws
    {
        
    }
}

// MARK: - Prepare & pass data

extension IndyKeychainWalletItem
{
    
    func parceFromKeychain(data: [String: Any])
    {
        
    }
    
    // MARK: Utilities
    
    fileprivate var currentTime: String
    {
        let dateFormatter : DateFormatter = DateFormatter()
        dateFormatter.dateFormat = "yyyy-MM-dd HH:mm:ss"
        let date = Date()
        let dateString = dateFormatter.string(from: date)
        return dateString
    }
}

// MARK: - Structs & Enums

extension IndyKeychainWalletItem
{
    struct WalletValue
    {
        var value: String
        var timeCreated: String
        
        init(value: String, timeCreated: String)
        {
            self.value = value
            self.timeCreated = timeCreated
        }
    }
    
    fileprivate enum WalletAttributes: String
    {
        case poolName = "poolName"
        case config = "config"
        case values = "values"
    }
}


