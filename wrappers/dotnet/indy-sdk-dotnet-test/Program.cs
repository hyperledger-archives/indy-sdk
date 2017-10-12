using Hyperledger.Indy.Test.PoolTests;
using System;
using System.Collections.Generic;
using System.Text;

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
