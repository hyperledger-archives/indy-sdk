using Hyperledger.Indy.Test.PoolTests;

namespace Hyperledger.Indy.Test
{
    class Program
    {
        static void Main(string[] args)
        {
            var test = new CreatePoolTest();
            test.TestCreatePoolWorksForNullConfig().Wait();
        }
    }

}
