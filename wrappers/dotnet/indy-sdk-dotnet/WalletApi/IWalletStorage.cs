using System;
using System.Threading.Tasks;

namespace Hyperledger.Indy.WalletApi
{
    /// <summary>
    /// Represents custom wallet storage
    /// </summary>
    public interface IWalletStorage
    {

        /// <summary>
        /// Create the wallet storage (For example, database creation)
        /// </summary>
        /// <param name="name">wallet storage name (the same as wallet name).</param>
        /// <param name="config">wallet storage config (For example, database config).</param>
        /// <param name="credentialsJson">wallet storage credentials (For example, database credentials).</param>
        /// <param name="metadata">wallet metadata (For example encrypted keys).</param>
        /// <returns></returns>
        Task CreateAsync(string name, string config, string credentialsJson, string metadata);


        /// <summary>
        /// Open the wallet storage (For example, opening database connection)
        /// </summary>
        /// <param name="name">wallet storage name (the same as wallet name).</param>
        /// <param name="config">wallet storage config (For example, database config).</param>
        /// <param name="credentialsJson">wallet storage credentials (For example, database credentials).</param>
        /// <returns>Storage handle</returns>
        Task<int> OpenAsync(string name, string config, string credentialsJson);

        /// <summary>
        /// Close the opened walled storage (For example, closing database connection)
        /// </summary>
        /// <param name="storageHandle">opened storage handle (See open handler).</param>
        /// <returns></returns>
        Task CloseAsync(int storageHandle);

        /// <summary>
        /// Delete the wallet storage (For example, database deletion)
        /// </summary>
        /// <param name="name">wallet storage name (the same as wallet name).</param>
        /// <param name="config">wallet storage config (For example, database config).</param>
        /// <param name="credentialsJson">wallet storage credentials (For example, database credentials).</param>
        /// <returns></returns>
        Task DeleteAsync(string name, string config, string credentialsJson);

        /// <summary>
        /// Create a new record in the wallet storage
        /// </summary>
        /// <param name="storageHandle">opened storage handle (See open handler).</param>
        /// <param name="type">allows to separate different record types collections.</param>
        /// <param name="id">the id of record.</param>
        /// <param name="value">the value of the record.</param>
        /// <param name="tagsJson">the record tags used for search and storing meta information as json:
        ///   {
        ///     "tagName1": "tag value 1", // string value
        ///     "tagName2": 123, // numeric value
        ///   }
        ///   Note that null means no tags</param>
        /// <returns></returns>
        Task AddRecordAsync(int storageHandle, string type, string id, byte[] value, string tagsJson);

        /// <summary>
        /// Update a record value
        /// </summary>
        /// <param name="storageHandle">opened storage handle (See open handler).</param>
        /// <param name="type">allows to separate different record types collections.</param>
        /// <param name="id">the id of record.</param>
        /// <param name="value">the value of the record.</param>
        /// <returns></returns>
        Task UpdateRecordValueAsync(int storageHandle, string type, string id, byte[] value);

        /// <summary>
        /// Update a record tags
        /// </summary>
        /// <param name="storageHandle">opened storage handle (See open handler).</param>
        /// <param name="type">allows to separate different record types collections.</param>
        /// <param name="id">the id of record.</param>
        /// <param name="tagsJson">the new record tags used for search and storing meta information as json:
        ///   {
        ///     "tagName1": "tag value 1", // string value
        ///     "tagName2": 123, // numeric value
        ///   }
        ///   Note that null means no tags</param>
        /// <returns></returns>
        Task UpdateRecordTagsAsync(int storageHandle, string type, string id, string tagsJson);

        /// <summary>
        /// Add new tags to the record
        /// </summary>
        /// <param name="storageHandle">opened storage handle (See open handler).</param>
        /// <param name="type">allows to separate different record types collections.</param>
        /// <param name="id">the id of record.</param>
        /// <param name="tagsJson">the additional record tags as json:
        ///   {
        ///     "tagName1": "tag value 1", // string value
        ///     "tagName2": 123, // numeric value
        ///   }
        ///   Note that null means no tags</param>
        /// <returns></returns>
        Task AddRecordTagsAsync(int storageHandle, string type, string id, string tagsJson);

        /// <summary>
        /// Delete tags from the record
        /// </summary>
        /// <param name="storageHandle">opened storage handle (See open handler).</param>
        /// <param name="type">allows to separate different record types collections.</param>
        /// <param name="id">the id of record.</param>
        /// <param name="tagNamesJson">the list of tag names to remove from the record as json array:
        ///   ["tagName1", "tagName2", ...]
        ///   Note that null means no tag names</param>
        /// <returns></returns>
        Task DeleteRecordTagsAsync(int storageHandle, string type, string id, string tagNamesJson);

        /// <summary>
        /// Delete an existing record in the wallet storage
        /// </summary>
        /// <param name="storageHandle">opened storage handle (See open handler).</param>
        /// <param name="type">allows to separate different record types collections.</param>
        /// <param name="id">the id of record.</param>
        /// <returns></returns>
        Task DeleteRecordAsync(int storageHandle, string type, string id);

        /// <summary>
        /// Get a wallet storage record by id
        /// </summary>
        /// <remarks>This method must thow if record is not found</remarks>
        /// <param name="storageHandle">opened storage handle (See open handler).</param>
        /// <param name="type">allows to separate different record types collections.</param>
        /// <param name="id">the id of record.</param>
        /// <param name="optionsJson">options json
        ///  {
        ///    retrieveType: (optional, false by default) Retrieve record type,
        ///    retrieveValue: (optional, true by default) Retrieve record value,
        ///    retrieveTags: (optional, true by default) Retrieve record tags
        ///  }</param>
        /// <returns>The record handle</returns>
        /// <remarks>Note if no record is found this method must throw any <see cref="Exception"/></remarks>
        Task<int> GetRecordAsync(int storageHandle, string type, string id, string optionsJson);

        /// <summary>
        /// Get an id for retrieved wallet storage record
        /// </summary>
        /// <param name="storageHandle">opened storage handle (See open handler).</param>
        /// <param name="recordHandle">retrieved record handle (See <see cref="GetRecordAsync"/>).</param>
        /// <returns>The id of the record</returns>
        Task<string> GetRecordIdAsync(int storageHandle, int recordHandle);

        /// <summary>
        /// Get a type for retrieved wallet storage record
        /// </summary>
        /// <param name="storageHandle">opened storage handle (See open handler).</param>
        /// <param name="recordHandle">retrieved record handle (See <see cref="GetRecordAsync"/>).</param>
        /// <returns>The type of record</returns>
        Task<string> GetRecordTypeAsync(int storageHandle, int recordHandle);

        /// <summary>
        /// Get a value for retrieved wallet storage record
        /// </summary>
        /// <param name="storageHandle">opened storage handle (See open handler).</param>
        /// <param name="recordHandle">retrieved record handle (See <see cref="GetRecordAsync"/>).</param>
        /// <returns>The value of the record</returns>
        Task<byte[]> GetRecordValueAsync(int storageHandle, int recordHandle);

        /// <summary>
        /// Get an tags for retrieved wallet record
        /// </summary>
        /// <param name="storageHandle">opened storage handle (See open handler).</param>
        /// <param name="recordHandle">retrieved record handle (See <see cref="GetRecordAsync"/>).</param>
        /// <returns>The tags of the record as json</returns>
        Task<string> GetRecordTagsAsync(int storageHandle, int recordHandle);

        /// <summary>
        /// Free retrieved wallet record (make retrieved record handle invalid)
        /// </summary>
        /// <param name="storageHandle">opened storage handle (See open handler).</param>
        /// <param name="recordHandle">retrieved record handle (See <see cref="GetRecordAsync"/>).</param>
        /// <returns></returns>
        Task FreeRecordAsync(int storageHandle, int recordHandle);

        /// <summary>
        /// Get storage metadata
        /// </summary>
        /// <param name="storageHandle">opened storage handle (See open handler).</param>
        /// <returns>A tuple of
        ///     - metadata as base64 value
        ///     - metadata handle
        /// </returns>
        Task<Tuple<string, int>> GetStorageMetadataAsync(int storageHandle);

        /// <summary>
        /// Set storage metadata
        /// </summary>
        /// <param name="storageHandle">opened storage handle (See open handler).</param>
        /// <param name="metadata">base64 value of metadata.</param>
        /// <remarks>Note if storage already have metadata record it will be overwritten.</remarks>
        /// <returns></returns>
        Task SetStorageMetadataAsync(int storageHandle, string metadata);

        /// <summary>
        /// Free retrieved storage metadata record (make retrieved storage metadata handle invalid)
        /// </summary>
        /// <param name="stroageHandle">opened storage handle (See open_wallet_storage).</param>
        /// <param name="metadataHandle">retrieved record handle (See <see cref="GetStorageMetadataAsync"/>).</param>
        /// <returns></returns>
        Task FreeStorageMetadataAsync(int stroageHandle, int metadataHandle);

        /// <summary>
        /// Search for wallet storage records
        /// </summary>
        /// <param name="storageHandle">opened storage handle (See open handler).</param>
        /// <param name="type">allows to separate different record types collections.</param>
        /// <param name="queryJson">MongoDB style query to wallet record tags:
        ///  {
        ///    "tagName": "tagValue",
        ///    $or: {
        ///      "tagName2": { $regex: 'pattern' },
        ///      "tagName3": { $gte: 123 },
        ///    },
        ///  }.</param>
        /// <param name="optionsJson">The options json.
        /// {
        ///    retrieveRecords: (optional, true by default) If false only "counts" will be calculated,
        ///    retrieveTotalCount: (optional, false by default) Calculate total count,
        ///    retrieveType: (optional, false by default) Retrieve record type,
        ///    retrieveValue: (optional, true by default) Retrieve record value,
        ///    retrieveTags: (optional, true by default) Retrieve record tags,
        ///  }</param>
        /// <returns>Search handle</returns>
        Task<int> SearchRecordsAsync(int storageHandle, string type, string queryJson, string optionsJson);

        /// <summary>
        /// Search for all wallet storage records.
        /// </summary>
        /// <param name="storageHandle">opened storage handle (See open handler).</param>
        /// <returns>Search handle</returns>
        Task<int> SearchAllRecordsAsync(int storageHandle);

        /// <summary>
        /// Get total count of records that corresponds to wallet storage search query
        /// </summary>
        /// <param name="storageHandle">opened storage handle (See open handler).</param>
        /// <param name="searchHandle">wallet search handle (See <see cref="SearchRecordsAsync"/>).</param>
        /// <returns></returns>
        Task<int> GetSearchTotalCountAsync(int storageHandle, int searchHandle);

        /// <summary>
        /// Get the next wallet storage record handle retrieved by this wallet search.
        /// </summary>
        /// <param name="storageHandle">opened storage handle (See open handler).</param>
        /// <param name="searchHandle">wallet search handle (See <see cref="SearchRecordsAsync"/>).</param>
        /// <returns>record handle (the same as for <see cref="GetRecordAsync"/> handler)</returns>
        /// <remarks>Note if no more records this method must throw any <see cref="Exception"/></remarks>
        Task<int> FetchSearchNextRecordAsync(int storageHandle, int searchHandle);

        /// <summary>
        /// Free wallet search (make search handle invalid)
        /// </summary>
        /// <param name="storageHandle">opened storage handle (See open handler).</param>
        /// <param name="searchHandle">wallet search handle (See <see cref="SearchRecordsAsync"/>).</param>
        /// <returns></returns>
        Task FreeSearchAsync(int storageHandle, int searchHandle);
    }
}