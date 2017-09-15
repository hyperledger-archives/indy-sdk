using Hyperledger.Indy.PoolApi;
using System;

namespace Hyperledger.Indy.Samples
{
    class Program
    {
        static void Main(string[] args)
        {
            Pool.DeletePoolLedgerConfigAsync("dummy").Wait();

            AgentDemo.Demo().Wait();
            AnonCredsDemo.Execute().Wait();
            LedgerDemo.Execute().Wait();
            SignusDemo.Execute().Wait();

            Console.WriteLine("Press any key to continue...");
            Console.ReadKey(true);
        }
    }
}
