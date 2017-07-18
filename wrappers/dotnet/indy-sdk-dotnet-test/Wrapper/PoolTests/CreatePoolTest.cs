using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Test.Wrapper.PoolTests
{
    [TestClass]
    public class CreatePoolTest : IndyIntegrationTest
    {
        [TestMethod]
        public void TestCreatePoolWorksForNullConfig()
        {
            Pool.CreatePoolLedgerConfigAsync("testCreatePoolWorks", null).Wait();
        }

        [TestMethod]
        public void TestCreatePoolWorksForConfigJSON()
        {
            var genesisTxnFile = PoolUtils.CreateGenesisTxnFile("genesis.txn");

            var configJson = string.Format("{\"genesis_txn\":{0}}", Path.GetFullPath(genesisTxnFile));

            Pool.CreatePoolLedgerConfigAsync("testCreatePoolWorks", configJson).Wait();
        }

        [TestMethod]
        public async Task TestCreatePoolWorksForEmptyName()
        {

            var genesisTxnFile = PoolUtils.CreateGenesisTxnFile("genesis.txn");

            var configJson = string.Format("{\"genesis_txn\":{0}}", Path.GetFullPath(genesisTxnFile));

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Pool.CreatePoolLedgerConfigAsync("", configJson)
            );

            Assert.AreEqual(ErrorCode.CommonInvalidParam2, (ErrorCode)ex.ErrorCode);
        }
    }
}
