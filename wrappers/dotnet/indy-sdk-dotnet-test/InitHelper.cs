using Indy.Sdk.Dotnet.Test.Wrapper.WalletTests;
using Indy.Sdk.Dotnet.Wrapper;
using System.IO;
using System.Reflection;
using System.Runtime.InteropServices;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Test
{
    class InitHelper
    {
        private static bool _isInitialized = false;

        [DllImport("kernel32.dll", SetLastError = true)]
        static extern bool SetDllDirectory(string lpPathName);

        public static void Init()
        {
            if (_isInitialized)
                return;

            LoadIndyDll();
            RegisterWalletType();

            _isInitialized = true;
        }

        private static void LoadIndyDll()
        {
            var executingLocation = Assembly.GetExecutingAssembly().Location;
            var libDir = Path.Combine(executingLocation, "../../../../lib");
            var dir = Path.GetFullPath(libDir);

            SetDllDirectory(dir);
        }

        private static void RegisterWalletType()
        {
            Wallet.RegisterWalletTypeAsync("inmem", new InMemWalletType()).Wait();
        }
    }
}
