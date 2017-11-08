//
//  DictionaryExtension.swift
//  libindy
//

import Foundation


public extension Dictionary
{
    /**
     Merge with provided dictionary.
     */
    mutating func merge(with dictionary: Dictionary<Key, Value>)
    {
        dictionary.forEach({ (key, value) in
            self.updateValue(value, forKey: key)
        })
    }
    
    func toString() -> String?
    {
        guard let jsonData = try? JSONSerialization.data(withJSONObject: self) else
        {
            return nil
        }
        
        return String(data: jsonData, encoding: .utf8)
    }
}
