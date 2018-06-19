#ifndef __indy__non_secrets__included__
#define __indy__non_secrets__included__

#ifdef __cplusplus
extern "C" {
#endif

    /// Create a new non-secret record in the wallet
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context
    /// wallet_handle: wallet handle (created by open_wallet)
    /// type_: allows to separate different record types collections
    /// id: the id of record
    /// value: the value of record
    /// tags_json: the record tags used for search and storing meta information as json:
    ///   {
    ///     "tagName1": <str>, // string tag (will be stored encrypted)
    ///     "tagName2": <str>, // string tag (will be stored encrypted)
    ///     "~tagName3": <str>, // string tag (will be stored un-encrypted)
    ///     "~tagName4": <str>, // string tag (will be stored un-encrypted)
    ///   }
    ///   Note that null means no tags
    ///   If tag name starts with "~" the tag will be stored un-encrypted that will allow
    ///   usage of this tag in complex search queries (comparison, predicates)
    ///   Encrypted tags can be searched only for exact matching

    extern indy_error_t indy_add_wallet_record(indy_handle_t  command_handle,
                                               indy_handle_t  wallet_handle,
                                               const char*    type_,
                                               const char*    id,
                                               const char*    value,
                                               const char*    tags_json,
                                               void           (*fn)(indy_handle_t xcommand_handle,
                                                                    indy_error_t err)
                                              );

    /// Update a non-secret wallet record value
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context
    /// wallet_handle: wallet handle (created by open_wallet)
    /// type_: allows to separate different record types collections
    /// id: the id of record
    /// value: the new value of record

    extern indy_error_t indy_update_wallet_record_value(indy_handle_t  command_handle,
                                                        indy_handle_t  wallet_handle,
                                                        const char*    type_,
                                                        const char*    id,
                                                        const char*    value,
                                                        void           (*fn)(indy_handle_t xcommand_handle,
                                                                             indy_error_t err)
                                                       );

/// Update a non-secret wallet record tags
///
/// #Params
/// command_handle: command handle to map callback to caller context
/// wallet_handle: wallet handle (created by open_wallet)
/// type_: allows to separate different record types collections
/// id: the id of record
/// tags_json: the record tags used for search and storing meta information as json:
///   {
///     "tagName1": <str>, // string tag (will be stored encrypted)
///     "tagName2": <str>, // string tag (will be stored encrypted)
///     "~tagName3": <str>, // string tag (will be stored un-encrypted)
///     "~tagName4": <str>, // string tag (will be stored un-encrypted)
///   }
///   If tag name starts with "~" the tag will be stored un-encrypted that will allow
///   usage of this tag in complex search queries (comparison, predicates)
///   Encrypted tags can be searched only for exact matching

    extern indy_error_t indy_update_wallet_record_tags(indy_handle_t  command_handle,
                                                       indy_handle_t  wallet_handle,
                                                       const char*    type_,
                                                       const char*    id,
                                                       const char*    tags_json,
                                                       void           (*fn)(indy_handle_t xcommand_handle,
                                                                            indy_error_t err)
                                                      );

    /// Add new tags to the wallet record
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context
    /// wallet_handle: wallet handle (created by open_wallet)
    /// type_: allows to separate different record types collections
    /// id: the id of record
    /// tags_json: the record tags used for search and storing meta information as json:
    ///   {
    ///     "tagName1": <str>, // string tag (will be stored encrypted)
    ///     "tagName2": <str>, // string tag (will be stored encrypted)
    ///     "~tagName3": <str>, // string tag (will be stored un-encrypted)
    ///     "~tagName4": <str>, // string tag (will be stored un-encrypted)
    ///   }
    ///   If tag name starts with "~" the tag will be stored un-encrypted that will allow
    ///   usage of this tag in complex search queries (comparison, predicates)
    ///   Encrypted tags can be searched only for exact matching
    ///   Note if some from provided tags already assigned to the record than
    ///     corresponding tags values will be replaced

    extern indy_error_t indy_add_wallet_record_tags(indy_handle_t  command_handle,
                                                    indy_handle_t  wallet_handle,
                                                    const char*    type_,
                                                    const char*    id,
                                                    const char*    tags_json,
                                                    void           (*fn)(indy_handle_t xcommand_handle,
                                                                         indy_error_t err)
                                                   );

    /// Delete tags from the wallet record
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context
    /// wallet_handle: wallet handle (created by open_wallet)
    /// type_: allows to separate different record types collections
    /// id: the id of record
    /// tag_names_json: the list of tag names to remove from the record as json array:
    ///   ["tagName1", "tagName2", ...]

    extern indy_error_t indy_delete_wallet_record_tags(indy_handle_t  command_handle,
                                                       indy_handle_t  wallet_handle,
                                                       const char*    type_,
                                                       const char*    id,
                                                       const char*    tag_names_json,
                                                       void           (*fn)(indy_handle_t xcommand_handle,
                                                                            indy_error_t err)
                                                      );

    /// Delete an existing wallet record in the wallet
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context
    /// wallet_handle: wallet handle (created by open_wallet)
    /// type_: record type
    /// id: the id of record

    extern indy_error_t indy_delete_wallet_record(indy_handle_t  command_handle,
                                                  indy_handle_t  wallet_handle,
                                                  const char*    type_,
                                                  const char*    id,
                                                  void           (*fn)(indy_handle_t xcommand_handle,
                                                                       indy_error_t err)
                                                 );

    /// Get an wallet record by id
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context
    /// wallet_handle: wallet handle (created by open_wallet)
    /// type_: allows to separate different record types collections
    /// id: the id of record
    /// options_json: //TODO: FIXME: Think about replacing by bitmask
    ///  {
    ///    retrieveType: (optional, false by default) Retrieve record type,
    ///    retrieveValue: (optional, true by default) Retrieve record value,
    ///    retrieveTags: (optional, true by default) Retrieve record tags
    ///  }
    /// #Returns
    /// wallet record json:
    /// {
    ///   id: "Some id",
    ///   type: "Some type", // present only if retrieveType set to true
    ///   value: "Some value", // present only if retrieveValue set to true
    ///   tags: <tags json>, // present only if retrieveTags set to true
    /// }

    extern indy_error_t indy_get_wallet_record(indy_handle_t  command_handle,
                                               indy_handle_t  wallet_handle,
                                               const char*    type_,
                                               const char*    id,
                                               const char*    options_json,
                                               void           (*fn)(indy_handle_t xcommand_handle,
                                                                    indy_error_t  err,
                                                                    const char*   record_json)
                                              );

    /// Search for wallet records.
    ///
    /// Note instead of immediately returning of fetched records
    /// this call returns wallet_search_handle that can be used later
    /// to fetch records by small batches (with indy_fetch_wallet_search_next_records).
    ///
    /// #Params
    /// wallet_handle: wallet handle (created by open_wallet)
    /// type_: allows to separate different record types collections
    /// query_json: MongoDB style query to wallet record tags:
    ///  {
    ///    "tagName": "tagValue",
    ///    $or: {
    ///      "tagName2": { $regex: 'pattern' },
    ///      "tagName3": { $gte: '123' },
    ///    },
    ///  }
    /// options_json: //TODO: FIXME: Think about replacing by bitmask
    ///  {
    ///    retrieveRecords: (optional, true by default) If false only "counts" will be calculated,
    ///    retrieveTotalCount: (optional, false by default) Calculate total count,
    ///    retrieveType: (optional, false by default) Retrieve record type,
    ///    retrieveValue: (optional, true by default) Retrieve record value,
    ///    retrieveTags: (optional, true by default) Retrieve record tags,
    ///  }
    /// #Returns
    /// search_handle: Wallet search handle that can be used later
    ///   to fetch records by small batches (with indy_fetch_wallet_search_next_records)

    extern indy_error_t indy_open_wallet_search(indy_handle_t  command_handle,
                                                indy_handle_t  wallet_handle,
                                                const char*    type_,
                                                const char*    query_json,
                                                const char*    options_json,
                                                void           (*fn)(indy_handle_t xcommand_handle,
                                                                     indy_error_t err,
                                                                     indy_handle_t search_handle)
                                               );

    /// Get an wallet record by id
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context
    /// wallet_handle: wallet handle (created by open_wallet)
    /// type_: allows to separate different record types collections
    /// id: the id of record
    /// options_json: //TODO: FIXME: Think about replacing by bitmask
    ///  {
    ///    retrieveType: (optional, false by default) Retrieve record type,
    ///    retrieveValue: (optional, true by default) Retrieve record value,
    ///    retrieveTags: (optional, true by default) Retrieve record tags
    ///  }
    /// #Returns
    /// wallet record json:
    /// {
    ///   id: "Some id",
    ///   type: "Some type", // present only if retrieveType set to true
    ///   value: "Some value", // present only if retrieveValue set to true
    ///   tags: <tags json>, // present only if retrieveTags set to true
    /// }

    extern indy_error_t indy_fetch_wallet_search_next_records(indy_handle_t  command_handle,
                                                              indy_handle_t  wallet_handle,
                                                              indy_handle_t  wallet_search_handle,
                                                              indy_u32_t   count,
                                                              void           (*fn)(indy_handle_t xcommand_handle,
                                                                                   indy_error_t  err,
                                                                                   const char*   records_json)
                                                             );

    /// Close wallet search (make search handle invalid)
    ///
    /// #Params
    /// wallet_search_handle: wallet search handle

    extern indy_error_t indy_close_wallet_search(indy_handle_t  command_handle,
                                                 indy_handle_t  wallet_search_handle,
                                                 void           (*fn)(indy_handle_t xcommand_handle,
                                                                      indy_error_t  err)
                                                );

#ifdef __cplusplus
}
#endif

#endif

