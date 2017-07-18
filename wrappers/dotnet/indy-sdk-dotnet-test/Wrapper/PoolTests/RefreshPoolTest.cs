using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Test.Wrapper.PoolTests
{
    [TestClass]
    public class RefreshPoolTest : IndyIntegrationTest
    {
        [TestMethod]
        public void TestRefreshPoolWorks()
        {
            var pool = PoolUtils.CreateAndOpenPoolLedger();

            Assert.IsNotNull(pool);
            _openedPools.Add(pool);

            pool.RefreshAsync().Wait();
        }       
    }
}
