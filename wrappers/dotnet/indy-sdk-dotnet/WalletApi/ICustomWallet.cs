namespace Hyperledger.Indy.WalletApi
{
    /// <summary>
    /// Features all custom wallets must implement.
    /// </summary>
    public interface ICustomWallet
    {
        /// <summary>
        ///  Allows an implementer to set a value in the wallet.
        /// </summary>
        /// <param name="key">The key of the value to set.</param>
        /// <param name="value">The value to set.</param>
        /// <returns>An <see cref="ErrorCode"/> value indicating the outcome of the operation.</returns>
        ErrorCode Set(string key, string value);

        /// <summary>
        /// Allows an implementer to get a value from the wallet.
        /// </summary>
        /// <remarks>
        /// If the key does not exist the method should return <see cref="ErrorCode.WalletNotFoundError"/>.
        /// </remarks>
        /// <param name="key">The key of the value to get.</param>
        /// <param name="value">The value obtained from the wallet.</param>
        /// <returns>An <see cref="ErrorCode"/> value indicating the outcome of the operation.</returns>
        ErrorCode Get(string key, out string value);

        /// <summary>
        /// Allows an implementer to get a value from the wallet if it has not expired.
        /// </summary>
        /// <remarks>
        /// If the key does not exist or the record associated with the key has
        /// expired then the method should return <see cref="ErrorCode.WalletNotFoundError"/>.
        /// </remarks>
        /// <param name="key">The key of the value to get.</param>
        /// <param name="value">The value obtained from the wallet.</param>
        /// <returns>An <see cref="ErrorCode"/> value indicating the outcome of the operation.</returns>
        ErrorCode GetNotExpired(string key, out string value);

        /// <summary>
        /// Allows an implementer to get a list of values from the wallet that match a key prefix.
        /// </summary>
        /// <remarks>
        /// The method should return a JSON string that conforms to the following format:
        /// <code>
        /// {
        ///     "values":[
        ///         {"key":"key_1", "value":"value_1"}, 
        ///         ...
        ///     ]
        /// }
        /// </code>
        /// If no values matching the <paramref name="keyPrefix"/> parameter are found the <c>values</c> 
        /// array in the JSON should be empty.
        /// </remarks>
        /// <param name="keyPrefix">The key prefix for the values requested.</param>
        /// <param name="valuesJson">The JSON string containing the values associated with the key prefix.</param>
        /// <returns>An <see cref="ErrorCode"/> value indicating the outcome of the operation.</returns>
        ErrorCode List(string keyPrefix, out string valuesJson);         
    }
}
