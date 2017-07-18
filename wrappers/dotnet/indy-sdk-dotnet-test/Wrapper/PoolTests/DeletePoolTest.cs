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
    public class DeletePoolTest : IndyIntegrationTest
    {
        [TestMethod]
        public void TestOpenPoolWorksForNullConfig()
        {
            var poolName = PoolUtils.CreatePoolLedgerConfig();
            var pool = Pool.OpenPoolLedgerAsync(poolName, null).Result;

            Assert.IsNotNull(pool);

            _openedPools.Add(pool);
        }

        [TestMethod]
        public void TestOpenPoolWorksForConfig()
        {
            var poolName = PoolUtils.CreatePoolLedgerConfig();

            var config = "{\"refreshOnOpen\":true,\"autoRefreshTime\":false,\"networkTimeout\":false}";
            var pool = Pool.OpenPoolLedgerAsync(poolName, config).Result;


            Assert.IsNotNull(pool);
            _openedPools.Add(pool);
        }

        [TestMethod]
        public async Task TestOpenPoolWorksForTwice()
        {
            var poolName = PoolUtils.CreatePoolLedgerConfig();
            var pool = Pool.OpenPoolLedgerAsync(poolName, null).Result;

            Assert.IsNotNull(pool);
            _openedPools.Add(pool);

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
               Pool.OpenPoolLedgerAsync(poolName, null)
            );

            Assert.AreEqual(ErrorCode.PoolLedgerInvalidPoolHandle, (ErrorCode)ex.ErrorCode);

        }

        [TestMethod]
        public void TestOpenPoolWorksForTwoNodes()
        {
            var poolName = PoolUtils.CreatePoolLedgerConfig(2);

            var pool = Pool.OpenPoolLedgerAsync(poolName, null).Result;

            Assert.IsNotNull(pool);
            _openedPools.Add(pool);
        }

        [TestMethod]
        public void TestOpenPoolWorksForThreeNodes()
        {
            var poolName = PoolUtils.CreatePoolLedgerConfig(3);

            var pool = Pool.OpenPoolLedgerAsync(poolName, null).Result;

            Assert.IsNotNull(pool);
            _openedPools.Add(pool);
        }
    }
}
