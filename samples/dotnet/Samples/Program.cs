using System;
using System.Diagnostics;
using System.Threading.Tasks;
using Hyperledger.Indy.PoolApi;

namespace Hyperledger.Indy.Samples
{
    class Program
    {
        static void Main(string[] args)
        {
            ExecuteDemos().Wait();

            Console.WriteLine("Press any key to continue...");
            Console.ReadKey(true);
        }

        static async Task ExecuteDemos()
        {
            await Pool.SetProtocolVersionAsync(2);

            await WalletDemo.Execute();
            await NonSecretsDemo.Execute();
            await AnonCredsDemo.Execute();
            await AnonCredsRevocationDemo.Execute();
            await LedgerDemo.Execute();
            await CryptoDemo.Execute();
        }
    }
}
