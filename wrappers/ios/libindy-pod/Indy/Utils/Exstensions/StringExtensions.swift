//
//  StringExtensions.swift
//  libindy
//

import Foundation


extension String
{
    static let emptyJson: String = "{}"
    
    func toDate(withFormat dateFormat: String) -> Date?
    {
        let dateFormatter : DateFormatter = DateFormatter()
        dateFormatter.dateFormat = dateFormat
        return dateFormatter.date(from: self)
    }
    
    static func isValid(_ string: String?) -> Bool
    {
        guard let str = string else
        {
            return false
        }
        
        return !str.isEmpty
    }

}
