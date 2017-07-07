using Indy.Sdk.Dotnet.Wrapper;
using Indy.Sdk.Dotnet.Wrapper.Wallet;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Api
{
    public class Wallet
    {
        internal IntPtr Handle { get; }

        private Wallet(IntPtr handle)
        {
            Handle = handle;
        }

        public static Task CreateAsync(string poolName, string name, string type, string config, string credentials)
        {
            return WalletWrapper.CreateWalletAsync(poolName, name, type, config, credentials);
        }

        public static Task DeleteAsync(string name, string credentials)
        {
            return WalletWrapper.DeleteWalletAsync(name, credentials);
        }

        public static async Task<Wallet> OpenAsync(string name, string runtimeConfig, string credentials)
        {
            var walletHandle = await WalletWrapper.OpenWalletAsync(name, runtimeConfig, credentials);
            return new Wallet(walletHandle);
        }

        public Task CloseAsync()
        {
            return WalletWrapper.CloseWalletAsync(Handle);
        }

        public Task SetSeqNoForValueAsync(string walletKey)
        {
            return WalletWrapper.WalletSetSeqNoForValueAsync(Handle, walletKey);
        }

        public Task<CreateAndStoreMyDidResult> CreateAndStoreMyDidAsync(string didJson)
        {
            return SignusWrapper.CreateAndStoreMyDidAsync(Handle, didJson);
        }

        public Task<string> SignAsync(string did, string msg)
        {
            return SignusWrapper.SignAsync(Handle, did, msg);
        }

    }
}
