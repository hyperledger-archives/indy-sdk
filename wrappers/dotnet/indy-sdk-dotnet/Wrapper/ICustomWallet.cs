using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;

namespace Indy.Sdk.Dotnet.Wrapper
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
        ErrorCode Set(string key, string value);

        /// <summary>
        /// Allows an implementer to get a value from the wallet.
        /// </summary>
        /// <param name="key">The key of the value to get.</param>
        /// <param name="value">The value obtained from the wallet.</param>
        ErrorCode Get(string key, out string value);

        /// <summary>
        /// Allows an implementer to get a value from the wallet if it has not expired.
        /// </summary>
        /// <param name="key">The key of the value to get.</param>
        /// <param name="value">The value obtained from the wallet.</param>
        ErrorCode GetNotExpired(string key, out string value);

        /// <summary>
        /// Allows an implementer to get a list of values from the wallet that match a key prefix.
        /// </summary>
        /// <param name="keyPrefix">The key prefix for the values requested.</param>
        /// <param name="valuesJson">The JSON string containing the values associated with the key prefix.</param>
        ErrorCode List(string keyPrefix, out string valuesJson);         
    }
}
