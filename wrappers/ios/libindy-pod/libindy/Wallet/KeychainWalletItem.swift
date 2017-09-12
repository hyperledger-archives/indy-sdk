//
//  KeychainWalletItem.swift
//  libindy
//
//  Created by Anastasia Tarasova on 04/09/2017.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

import Foundation

@objc public class KeychainWalletItem: NSObject
{
    static let serviceName = "IndyKeychainWallet"
    
    //fileprivate (set) var poolName: String?
    
    public fileprivate (set) var name: String
    
    public fileprivate (set) var config: String?
    public fileprivate (set) var credentials: String?
    
    public var freshnessTime: UInt  = 0
    
    fileprivate var keychain: KeychainWrapper
    
    fileprivate var dateFormat = "yyyy-MM-dd HH:mm:ss"
    
    public init(name: String, config: String? = nil, credentials: String? = nil)
    {
        self.name = name
        self.config = config
        self.credentials = credentials
        
        self.keychain = KeychainWrapper(service: KeychainWalletItem.serviceName, account: self.name)
    }
    
    /**
     Data read from keychain
     */
    fileprivate var resultData = [String: Any]()
    
    fileprivate var values: [String: WalletValue] = [:]
    
    /**
     Combined data ready to be written to keychain.
     Merged with resultData and values.
     */
    
    fileprivate var data: [String: Any]
    {
        var dictionary: [String: Any] = [:]
        
        // merge with data, fetched from keychain
        dictionary.merge(with: resultData)
        
        if self.config != nil
        {
            dictionary[WalletAttributes.config.rawValue] = self.config
        }
        
        // merge existing values dictionary with new one
        if var oldValues = dictionary[WalletAttributes.values.rawValue] as? [String: Any]
        {
            oldValues.merge(with: self.values)
            dictionary[WalletAttributes.values.rawValue] = oldValues
        }
        
        return dictionary
    }
}

// MARK: - Public action methods

extension KeychainWalletItem
{
    public func updateInKeychain() throws
    {
        try self.keychain.update(data: self.data)
        
    }
    
    public func readFromKeychain() throws
    {
        var resultData = [String: Any]()
        do
        {
            resultData = try self.keychain.read()
        }
        catch
        {
            throw error
        }
        
        self.resultData = resultData
    }
    
    public func deleteFromKeychain() throws
    {
        try self.keychain.delete()
    }
    
    static public func allStoredWalletNames() -> Array<String>
    {
        return KeychainWrapper.allKeys(forService: KeychainWalletItem.serviceName)
    }
  
    public func setWalletValue(_ value: String, forKey key: String) throws
    {
        try self.readFromKeychain()
        
        self.values[key] = WalletValue(value: value, timeCreated: self.currentTimeString)
        
        try self.keychain.update(data: self.data)
    }
    
    public func getValue(forKey key: String) -> String?
    {
        do
        {
            try self.readFromKeychain()
        }
        catch
        {
            return nil
        }
        
        guard let valuesDictionary = self.resultData[WalletAttributes.values.rawValue] as? [String: Any] else
        {
            return nil
        }
        
        guard let valueItem = valuesDictionary[key] as? WalletValue else
        {
            return nil
        }
        
        return valueItem.value
    }
    
    
    public func getNotExpiredValue(forKey key: String) -> String?
    {
        do
        {
            try self.readFromKeychain()
        }
        catch
        {
            return nil
        }
        
        guard let valuesDictionary = self.resultData[WalletAttributes.values.rawValue] as? [String: Any] else
        {
            return nil
        }
        
        guard let valueItem = valuesDictionary[key] as? WalletValue else
        {
            return nil
        }
        
        guard let itemDate = valueItem.timeCreated.toDate(withFormat: self.dateFormat) else
        {
            return nil
        }
        
        let currentTime = Date()
        
        
        if currentTime.timeIntervalSince(itemDate) > Double(self.freshnessTime)
        {
            return nil
        }
        
        return valueItem.value
    }
    
    public func listValuesJson(forKeyPrefix prefix: String) -> String
    {
        var valuesJson = [String: Any]()
        
        do
        {
            try self.readFromKeychain()
        }
        catch
        {
            return valuesJson.toString() ?? String.emptyJson
        }
        
        guard let valuesDictionary = self.resultData[WalletAttributes.values.rawValue] as? [String: Any] else
        {
            return valuesJson.toString() ?? String.emptyJson
        }
        
        for (key, value) in valuesDictionary
        {
            if key.hasPrefix(prefix)
            {
                valuesJson[key] = value
            }
        }
        
        return valuesJson.toString() ?? String.emptyJson
    }
}

// MARK: - Prepare & pass data

extension KeychainWalletItem
{
    // MARK: Utilities
    
    fileprivate var currentTimeString: String
    {
        let dateFormatter : DateFormatter = DateFormatter()
        dateFormatter.dateFormat = self.dateFormat
        let date = Date()
        let dateString = dateFormatter.string(from: date)
        return dateString
    }
}

// MARK: - Structs & Enums

extension KeychainWalletItem
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
       // case poolName = "poolName"
        case config = "config"
        case values = "values"
    }
}


