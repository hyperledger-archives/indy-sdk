using System;
using System.Threading.Tasks;
using Hyperledger.Indy.PoolApi;

namespace Hyperledger.Indy.Samples
{
    public class Program
    {
        public static async Task Main(string[] args)
        {
            await Pool.SetProtocolVersionAsync(2);

            await WalletDemo.Execute();
            await NonSecretsDemo.Execute();
            await AnonCredsDemo.Execute();
            await AnonCredsRevocationDemo.Execute();
            await CryptoDemo.Execute();
            await LedgerDemo.Execute();

            Console.WriteLine("Press any key to continue...");
            Console.ReadKey(true);
        }
    }
}