using Hyperledger.Indy.Sdk.Test.WalletTests;
using Hyperledger.Indy.Sdk.WalletApi;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Sdk.Test
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
