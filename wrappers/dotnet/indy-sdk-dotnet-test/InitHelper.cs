using Hyperledger.Indy.Test.WalletTests;
using Hyperledger.Indy.WalletApi;
using System.IO;
using System.Reflection;
using System.Runtime.InteropServices;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test
{
    class InitHelper
    {
        private static bool _isInitialized = false;

        public static async Task InitAsync()
        {
            if (_isInitialized)
                return;

            await RegisterWalletTypeAsync();

            _isInitialized = true;
        }

        private static async Task RegisterWalletTypeAsync()
        {
            await Wallet.RegisterWalletTypeAsync("inmem", new InMemWalletType());
        }
    }
}
