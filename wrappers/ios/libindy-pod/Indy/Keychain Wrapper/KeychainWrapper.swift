

import Foundation

class KeychainWrapper
{
    static let defaultServiceName = Bundle.main.infoDictionary![String(kCFBundleIdentifierKey)] as? String ?? "com.indykeychainwrapper.defaultService"

    private (set) public var service: String
    
    private (set) public var account: String?

    private (set) public var accessGroup: String?
    
    static let standard = KeychainWrapper()
    
    var accessible: KeychainAccessibility?
    
    var generic: NSData?
    
    // MARK: - Init
   
    init(service: String, account: String? = nil, accessGroup: String? = nil) {
        self.service = service
        self.account = account
        self.accessGroup = accessGroup
    }
    
    private convenience init() {
        self.init(service: KeychainWrapper.defaultServiceName)
    }
    
    /**
     Get all account for service
     */
    static func allKeys(forService service: String, withAccessGroup accessGroup: String? = nil) -> Array<String>
    {
        var keychainQueryDictionary: [String:Any] = [
            String(kSecClass): kSecClassGenericPassword,
            String(kSecAttrService): service,
            String(kSecReturnAttributes): kCFBooleanTrue,
            String(kSecMatchLimit): kSecMatchLimitAll,
            ]
        
        if let accessGroup = accessGroup
        {
            keychainQueryDictionary[String(kSecAttrAccessGroup)] = accessGroup
        }
        
        var result: AnyObject?
        let status = SecItemCopyMatching(keychainQueryDictionary as CFDictionary, &result)
        
        guard status == errSecSuccess else { return [] }
        
        var keys = Array<String>()
        guard let results = result as? [[AnyHashable: Any]] else
        {
            return keys
        }
        
        for attributes in results
        {
            if let account = attributes[String(kSecAttrAccount)] as? String
            {
                    keys.append(account)
            }
        }
        
        return keys
    }
    
    // MARK: - Query

    
    fileprivate var genericPasswordBaseQuery: [String: Any]
    {
        var dictionary = [String: Any]()
        
        dictionary[String(kSecAttrService)] = service
        if let account = self.account
        {
            dictionary[String(kSecAttrAccount)] = account
        }
        
        if let generic = self.generic
        {
            dictionary[String(kSecAttrGeneric)] = generic
        }
        
        dictionary[String(kSecClass)] = kSecClassGenericPassword
        
        dictionary[String(kSecAttrAccessGroup)] = accessGroup
        dictionary[String(kSecAttrAccessible)] = accessible?.rawValue

        return dictionary
    }
    
    func setupKeychainWriteQuery(withData data:[String: Any]) -> [String: Any]
    {
        var dictionary = [String: Any]()
        dictionary.merge(with: genericPasswordBaseQuery)
        
        dictionary[String(kSecValueData)] = NSKeyedArchiver.archivedData(withRootObject: data)
        
        return dictionary
    }
    
    var asReadableQuery: [String: Any] {
        var old = genericPasswordBaseQuery
        old[String(kSecReturnData)] = kCFBooleanTrue
        old[String(kSecMatchLimit)] = kSecMatchLimitOne
        old[String(kSecReturnAttributes)] = kCFBooleanTrue
        
        return old
    }
    
    var asDeleteableQuery: [String: Any] {
        return genericPasswordBaseQuery
    }

    // MARK: - Operations with provided dictionary

    // MARK: Create
    
    fileprivate func createInKeychain(query:[String: Any]) throws
    {
        try performKeychainStorageAction(closure: performCreateRequestClosure, keychainQuery: query)
    }

    // MARK: Update
    func update(data: [String: Any]) throws
    {
        let query = self.setupKeychainWriteQuery(withData: data)
        
        try self.updateInKeychain(query: query)
    }

    fileprivate func updateInKeychain(query: [String: Any]) throws
    {
        var attributesToUpdate = query
        attributesToUpdate[String(kSecClass)] = nil
        
        let status = SecItemUpdate(query as CFDictionary, attributesToUpdate as CFDictionary)
        
        if let error = KeychainError(fromStatusCode: Int(status))
        {
            if error == .notFound || error == .notAvailable
            {
                try self.createInKeychain(query: query)
            } else
            {
                throw error
            }
        }
        else
        {
            if status != errSecSuccess
            {
                throw KeychainError.undefined
            }
        }
    }
    
    // MARK: Read
    
    func read() throws -> [String: Any]
    {
        do {
            guard let result = try performKeychainStorageAction(closure: performReadRequestClosure, keychainQuery: asReadableQuery) else
            {
                throw KeychainError.notAvailable
            }
            
            guard let data = self.unarchiveData(fromQuery: result) else
            {
                throw KeychainError.notAvailable
            }
            
            return data
        }
        catch
        {
            throw error
        }
    }
    
    // MARK: Delete
    
    func delete() throws
    {
        try performKeychainStorageAction(closure: performDeleteRequestClosure, keychainQuery: asDeleteableQuery)
    }
    
    func unarchiveData(fromQuery query: [String: Any]) -> [String: Any]?
    {
        guard let archivedData = query[String(kSecValueData)] as? Data else
        {
            return nil
        }
        
        guard let dictionary = NSKeyedUnarchiver.unarchiveObject(with: archivedData) as? [String : Any] else
        {
            return nil
        }
        
        return dictionary
    }
}

// MARK: - Storage operation closures

extension KeychainWrapper
{
    typealias PerformKeychainRequestClosureType = (_ requestReference: CFDictionary, _ result: inout AnyObject?) -> (OSStatus)
    
    @discardableResult
    fileprivate func performKeychainStorageAction(closure: PerformKeychainRequestClosureType,
                                                  keychainQuery: [String: Any]) throws -> [String: Any]?
    {
        var result: AnyObject?
        let request = keychainQuery
        let requestReference = request as CFDictionary
        
        let status = closure(requestReference, &result)
        
        let statusCode = Int(status)
        
        if let error = KeychainError(fromStatusCode: statusCode)
        {
            throw error
        }
        
        guard status == errSecSuccess else
        {
            return nil
        }
        
        guard let dictionary = result as? NSDictionary else
        {
            return nil
        }
        
        if dictionary[String(kSecValueData)] as? NSData == nil {
            return nil
        }
        
        return result as? [String: Any]
    }
    
    var performReadRequestClosure: PerformKeychainRequestClosureType
    {
        return { (requestReference: CFDictionary, result: inout AnyObject?) in
            return withUnsafeMutablePointer(to: &result) { SecItemCopyMatching(requestReference, UnsafeMutablePointer($0)) }
        }
    }
    
    var performCreateRequestClosure: PerformKeychainRequestClosureType {
        return { (requestReference: CFDictionary, result: inout AnyObject?) in
            return withUnsafeMutablePointer(to: &result) { SecItemAdd(requestReference, UnsafeMutablePointer($0)) }
        }
    }
    
    var performDeleteRequestClosure: PerformKeychainRequestClosureType {
        return { (requestReference, _) in
            return SecItemDelete(requestReference)
        }
    }
}
