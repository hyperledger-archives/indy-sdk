using Hyperledger.Indy.Sdk.Test.WalletTests;
using Hyperledger.Indy.Sdk.WalletApi;
using System.IO;
using System.Reflection;
using System.Runtime.InteropServices;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Sdk.Test
{
    class InitHelper
    {
        private static bool _isInitialized = false;

        [DllImport("kernel32.dll", SetLastError = true)]
        static extern bool SetDllDirectory(string lpPathName);

        public static async Task InitAsync()
        {
            if (_isInitialized)
                return;

            LoadIndyDll();
            await RegisterWalletTypeAsync();

            _isInitialized = true;
        }

        private static void LoadIndyDll()
        {
            var assemblyLocation = typeof(InitHelper).GetTypeInfo().Assembly.Location;
            var executingLocation = Path.GetDirectoryName(assemblyLocation);
            var libDir = Path.Combine(executingLocation, "../../../../lib");
            var dir = Path.GetFullPath(libDir);

            SetDllDirectory(dir);
        }

        private static async Task RegisterWalletTypeAsync()
        {
            await Wallet.RegisterWalletTypeAsync("inmem", new InMemWalletType());
        }
    }
}
