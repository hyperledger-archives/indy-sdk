//
//  DictionaryExtension.swift
//  libindy
//

import Foundation


public extension Dictionary {
 
    /**
     Merge with provided dictionary.
     */
    mutating func merge(with dictionary: Dictionary<Key, Value>)
    {
        dictionary.forEach({ (key, value) in
            self.updateValue(value, forKey: key)
        })
    }
}
