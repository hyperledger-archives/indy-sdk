using Hyperledger.Indy.Utils;
using System;
using System.Runtime.InteropServices;
using System.Text;
using static Hyperledger.Indy.WalletApi.NativeMethods;

namespace Hyperledger.Indy.WalletApi
{
    /// <summary>
    /// Base type for implementing custom wallet types.
    /// </summary>
    public abstract class WalletType
    {
        /// <summary>
        /// Initializes a new WalletType.
        /// </summary>
        protected WalletType()
        {
            CreateCallback = CreateHandler;
            OpenCallback = OpenHandler;
            SetCallback = SetHandler;
            GetCallback = GetHandler;
            GetNotExpiredCallback = GetNotExpiredHandler;
            ListCallback = ListHandler;
            CloseCallback = CloseHandler;
            DeleteCallback = DeleteHandler;
            FreeCallback = FreeHandler;
        }

        /// <summary>
        /// The delegate to call when a wallet is being created.
        /// </summary>
        internal WalletTypeCreateDelegate CreateCallback { get; }

        /// <summary>
        /// The delegate to call when a wallet is being opened.
        /// </summary>
        internal WalletTypeOpenDelegate OpenCallback { get; }

        /// <summary>
        /// The delegate to call when a value is set  on a wallet.
        /// </summary>
        internal WalletTypeSetDelegate SetCallback { get; }

        /// <summary>
        /// The delegate to call when a value is requested from a wallet.
        /// </summary>
        internal WalletTypeGetDelegate GetCallback { get; }

        /// <summary>
        /// The delegate to call when an unexpired value is requested from a wallet.
        /// </summary>
        internal WalletTypeGetNotExpiredDelegate GetNotExpiredCallback { get; }

        /// <summary>
        /// The delegate to call when a list of values is requested from a wallet.
        /// </summary>
        internal WalletTypeListDelegate ListCallback { get; }

        /// <summary>
        /// The delegate to call when a wallet is being closed.
        /// </summary>
        internal WalletTypeCloseDelegate CloseCallback { get; }

        /// <summary>
        /// The delegate to call when a wallet is being deleted.
        /// </summary>
        internal WalletTypeDeleteDelegate DeleteCallback { get; }

        /// <summary>
        /// The delegate to call when a value returned by a wallet is to be freed.
        /// </summary>
        internal WalletTypeFreeDelegate FreeCallback { get; }

        /// <summary>
        /// Gets a wallet by its handle.
        /// </summary>
        /// <param name="walletHandle">The handle of the wallet.</param>
        /// <returns>The wallet instance associated with the handle.</returns>
        protected abstract ICustomWallet GetWalletByHandle(int walletHandle);

        /// <summary>
        /// Handler for wallet creation.
        /// </summary>
        /// <param name="name">The name of the wallet to create.</param>
        /// <param name="config">The configuration for the wallet.</param>
        /// <param name="credentials">The credentials of the wallet.</param>
        /// <returns>An ErrorCode indicating the outcome of the operation.</returns>
        private ErrorCode CreateHandler(string name, string config, string credentials)
        {
            ParamGuard.NotNullOrWhiteSpace(name, "name");

            try
            {
                return Create(name, config, credentials);

            }
            catch (Exception)
            {
                return ErrorCode.CommonInvalidState;
            }            
        }

        /// <summary>
        /// Handler for opening a wallet.
        /// </summary>
        /// <param name="name">The name of the wallet to open.</param>
        /// <param name="config">The wallet configuration that was registered on creation.</param>
        /// <param name="runtimeConfig">The runtime configuration to use for the wallet.</param>
        /// <param name="credentials">The wallet credentials.</param>
        /// <param name="handle">A handle returned for the opened wallet instance.</param>
        /// <returns>An ErrorCode indicating the outcome of the operation.</returns>
        private ErrorCode OpenHandler(string name, string config, string runtimeConfig, string credentials, ref int handle)
        {
            ParamGuard.NotNullOrWhiteSpace(name, "name");

            try
            {
                return Open(name, config, runtimeConfig, credentials, out handle);
            }
            catch (Exception)
            {
                return ErrorCode.CommonInvalidState;
            }
        }

        /// <summary>
        /// Handler for setting a value on an opened wallet instance.
        /// </summary>
        /// <param name="handle">The handle of the wallet instance.</param>
        /// <param name="key">The key of the value to set.</param>
        /// <param name="value">The value to set.</param>
        /// <returns>An ErrorCode indicating the outcome of the operation.</returns>
        private ErrorCode SetHandler(int handle, string key, string value)
        {
            ParamGuard.NotNullOrWhiteSpace(key, "key");

            try
            {
                var wallet = GetWalletByHandle(handle);
                return wallet.Set(key, value);                
            }
            catch (Exception)
            {
                return ErrorCode.CommonInvalidState;
            }
        }

        /// <summary>
        /// Handler for getting a value from an opened wallet instance.
        /// </summary>
        /// <param name="handle">The handle of the wallet instance.</param>
        /// <param name="key">The key of the value to get.</param>
        /// <param name="value_ptr">The returned pointer to the value.</param>
        /// <returns>An ErrorCode indicating the outcome of the operation.</returns>
        private ErrorCode GetHandler(int handle, string key, ref IntPtr value_ptr)
        {
            ParamGuard.NotNullOrWhiteSpace(key, "key");

            try
            {
                var wallet = GetWalletByHandle(handle);

                string value;
                var result = wallet.Get(key, out value);

                if (result != ErrorCode.Success)
                    return result;
                
                value_ptr = MarshalToUnmanaged(value);

                return ErrorCode.Success;
            }
            catch (Exception)
            {
                return ErrorCode.CommonInvalidState;
            }
        }

        /// <summary>
        /// Handler for getting an unexpired value from an open wallet instance.
        /// </summary>
        /// <param name="handle">The handle of the wallet instance.</param>
        /// <param name="key">The key of the value to get.</param>
        /// <param name="value_ptr">The returned pointer to the value.</param>
        /// <returns>An ErrorCode indicating the outcome of the operation.</returns>
        private ErrorCode GetNotExpiredHandler(int handle, string key, ref IntPtr value_ptr)
        {
            ParamGuard.NotNullOrWhiteSpace(key, "key");

            try
            {
                var wallet = GetWalletByHandle(handle);

                string value;
                var result = wallet.GetNotExpired(key, out value);

                if (result != ErrorCode.Success)
                    return result;

                value_ptr = MarshalToUnmanaged(value);

                return ErrorCode.Success;
            }
            catch (Exception)
            {
                return ErrorCode.CommonInvalidState;
            }
        }

        /// <summary>
        /// Handler for getting a list of values from an open wallet instance.
        /// </summary>
        /// <param name="handle">The handle of the wallet instance.</param>
        /// <param name="keyPrefix">The prefix to filter keys by.</param>
        /// <param name="values_json_ptr">The returned pointer to the value.</param>
        /// <returns>An ErrorCode indicating the outcome of the operation.</returns>
        private ErrorCode ListHandler(int handle, string keyPrefix, ref IntPtr values_json_ptr)
        {
            try
            {
                var wallet = GetWalletByHandle(handle);

                string value;
                var result = wallet.List(keyPrefix, out value);

                if (result != ErrorCode.Success)
                    return result;

                values_json_ptr = MarshalToUnmanaged(value);

                return ErrorCode.Success;
            }
            catch (Exception)
            {
                return ErrorCode.CommonInvalidState;
            }
        }

        /// <summary>
        /// Handler for closing an open wallet instance.
        /// </summary>
        /// <param name="handle">The handle of the wallet instance.</param>
        /// <returns>An ErrorCode indicating the outcome of the operation.</returns>
        private ErrorCode CloseHandler(int handle)
        {
            try
            {
                return Close(handle);                
            }
            catch (Exception)
            {
                return ErrorCode.CommonInvalidState;
            }
        }

        /// <summary>
        /// Handler for deleting a wallet.
        /// </summary>
        /// <param name="name">The name of the wallet to delete.</param>
        /// <param name="config">The configuration of the wallet.</param>
        /// <param name="credentials">The credentials of the wallet.</param>
        /// <returns>An ErrorCode indicating the outcome of the operation.</returns>
        private ErrorCode DeleteHandler(string name, string config, string credentials)
        {
            ParamGuard.NotNullOrWhiteSpace(name, "name");

            try
            {
                return Delete(name, config, credentials);
            }
            catch (Exception)
            {
                return ErrorCode.CommonInvalidState;
            }
        }

        /// <summary>
        /// Handler for freeing a value returned by an open wallet instance.
        /// </summary>
        /// <param name="handle">The handle of the wallet instance.</param>
        /// <param name="value">The pointer to the value to free.</param>
        /// <returns>An ErrorCode indicating the outcome of the operation.</returns>
        private ErrorCode FreeHandler(int handle, IntPtr value)
        {
            try
            {
                var wallet = GetWalletByHandle(handle);

                Marshal.FreeHGlobal(value);                
                return ErrorCode.Success;
            }
            catch (Exception)
            {
                return ErrorCode.CommonInvalidState;
            }

        }

        /// <summary>
        /// Marshals a string to unmanaged memory.
        /// </summary>
        /// <param name="value">The string value to marshal.</param>
        /// <returns>A pointer to the unmanaged memory.</returns>
        private IntPtr MarshalToUnmanaged(string value)
        {
            byte[] buffer = Encoding.UTF8.GetBytes(value); // not null terminated
            Array.Resize(ref buffer, buffer.Length + 1);
            buffer[buffer.Length - 1] = 0; // terminating 0
            IntPtr unmanagedMemoryPtr = Marshal.AllocHGlobal(buffer.Length);
            Marshal.Copy(buffer, 0, unmanagedMemoryPtr, buffer.Length);
            return unmanagedMemoryPtr;
        }

        /// <summary>
        /// Allows an implementer to create a new wallet.
        /// </summary>
        /// <remarks>
        /// <para>When implementing a custom wallet this method is responsible for creating the new wallet
        /// and storing its configuration and credentials.
        /// </para>
        /// </remarks>
        /// <param name="name">The name of the wallet to create.</param>
        /// <param name="config">The configuration for the wallet.</param>
        /// <param name="credentials">The credentials for the wallet.</param>
        /// <returns>An <see cref="ErrorCode"/> value indicating the outcome of the operation.</returns>
        public abstract ErrorCode Create(string name, string config, string credentials);

        /// <summary>
        ///  Allows an implementer to open a wallet.
        /// </summary>
        /// <remarks>
        /// When implementing a custom wallet this method is responsible for opening the wallet and returning
        /// a handle for the opened wallet.  The value of the <paramref name="runtimeConfig"/> parameter
        /// should override any corresponding values provided in the <paramref name="config"/> parameter
        /// and value of the <paramref name="credentials"/> parameter should be used to control access 
        /// to the wallet.
        /// </remarks>
        /// <param name="name">The name of the wallet to open.</param>
        /// <param name="config">The configuration for the wallet.</param>
        /// <param name="runtimeConfig">The runtime configuration for the wallet.</param>
        /// <param name="credentials">The credentials of the wallet.</param>
        /// <param name="walletHandle">The handle assigned to the wallet.</param> 
        /// <returns>An <see cref="ErrorCode"/> value indicating the outcome of the operation.</returns>
        public abstract ErrorCode Open(string name, string config, string runtimeConfig, string credentials, out int walletHandle);

        /// <summary>
        ///  Allows an implementer to close a wallet.
        /// </summary>
        /// <remarks>
        /// When implementing a custom wallet this method is responsible for closing the wallet with 
        /// the handle allocated earlier in the <see cref="Open(string, string, string, string, out int)"/>
        /// method.
        /// </remarks>
        /// <param name="walletHandle">The handle of the wallet to close.</param>
        /// <returns>An <see cref="ErrorCode"/> value indicating the outcome of the operation.</returns>
        public abstract ErrorCode Close(int walletHandle);

        /// <summary>
        ///  Allows an implementer to delete a wallet.
        /// </summary>
        /// <remarks>
        /// When implementing a custom wallet this method is responsible for deleting a wallet created
        /// earlier via the <see cref="Create(string, string, string)"/> method.  The value of the 
        /// <paramref name="credentials"/> parameter should be used to control access whether or not
        /// the wallet can be deleted.</remarks>
        /// <param name="name">The name of the wallet being deleted</param>
        /// <param name="config">The configuration of the wallet.</param>
        /// <param name="credentials">The credentials of the wallet.</param>
        /// <returns>An <see cref="ErrorCode"/> value indicating the outcome of the operation.</returns>
        public abstract ErrorCode Delete(string name, string config, string credentials);
    }
}