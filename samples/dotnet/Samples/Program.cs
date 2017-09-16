using System;
using System.Threading.Tasks;

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
            await AgentDemo.Execute();
            await AnonCredsDemo.Execute();
            await LedgerDemo.Execute();
            await SignusDemo.Execute();
        }
    }
}
