using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Wrapper
{
    /// <summary>
    /// Base class that custom wallets must be implemented for.
    /// </summary>
    public abstract class CustomWalletBase
    {
        private List<IntPtr> _valuePointers = new List<IntPtr>();

        internal List<IntPtr> ValuePointers
        {
            get { return _valuePointers; }
        }

        /// <summary>
        /// Sets a value in the wallet.
        /// </summary>
        /// <param name="key">The key of the value to set.</param>
        /// <param name="value">The value to set.</param>
        public abstract ErrorCode Set(string key, string value);

        /// <summary>
        /// Gets a value from the wallet.
        /// </summary>
        /// <param name="key">The key of the value to get.</param>
        /// <param name="value">The value obtained from the wallet.</param>
        public abstract ErrorCode Get(string key, out string value);

        /// <summary>
        /// Gets a value from the wallet if it has not expired.
        /// </summary>
        /// <param name="key">The key of the value to get.</param>
        /// <param name="value">The value obtained from the wallet.</param>
        public abstract ErrorCode GetNotExpired(string key, out string value);

        /// <summary>
        /// Gets a list of values from the wallet that match a key prefix.
        /// </summary>
        /// <param name="keyPrefix">The key prefix for the values requested.</param>
        /// <param name="valuesJson">The JSON string containing the values associated with the key prefix.</param>
        public abstract ErrorCode List(string keyPrefix, out string valuesJson);

        

        /// <summary>
        /// Disposes a wallet instance.
        /// </summary>
        public void Dispose()
        {
            //Free any outstanding handles.
            for (int i = _valuePointers.Count - 1; i >= 0; i--)
            {
                Marshal.FreeHGlobal(_valuePointers[i]);
                _valuePointers.RemoveAt(i);
            }
        }
    }
}
