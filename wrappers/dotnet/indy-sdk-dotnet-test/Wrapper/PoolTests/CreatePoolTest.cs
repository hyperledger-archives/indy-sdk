﻿using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.IO;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Test.Wrapper.PoolTests
{
    [TestClass]
    public class CreatePoolTest : IndyIntegrationTestBase
    {
        [TestMethod]
        public async Task TestCreatePoolWorksForNullConfig()
        {
            var txnFile = "testCreatePoolWorks.txn";

            try
            {
                File.Create(txnFile).Dispose();
                await Pool.CreatePoolLedgerConfigAsync("testCreatePoolWorks", null);
            }
            finally
            {
                File.Delete(txnFile);
            }
        }

        [TestMethod]
        public async Task TestCreatePoolWorksForConfigJSON()
        {
            var genesisTxnFile = PoolUtils.CreateGenesisTxnFile("genesis.txn");
            var path = Path.GetFullPath(genesisTxnFile).Replace('\\', '/');

            var configJson = string.Format("{{\"genesis_txn\":\"{0}\"}}", path);

            await Pool.CreatePoolLedgerConfigAsync("testCreatePoolWorks", configJson);
        }

        [TestMethod]
        public async Task TestCreatePoolWorksForEmptyName()
        {

            var genesisTxnFile = PoolUtils.CreateGenesisTxnFile("genesis.txn");
            var path = Path.GetFullPath(genesisTxnFile).Replace('\\', '/');

            var configJson = string.Format("{{\"genesis_txn\":\"{0}\"}}", path);

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Pool.CreatePoolLedgerConfigAsync("", configJson)
            );

            Assert.AreEqual(ErrorCode.CommonInvalidParam2, ex.ErrorCode);
        }

        [TestMethod]
        public async Task TestCreatePoolWorksForTwice()
        {

            var genesisTxnFile = PoolUtils.CreateGenesisTxnFile("genesis.txn");

            var path = Path.GetFullPath(genesisTxnFile).Replace('\\', '/');

            var configJson = string.Format("{{\"genesis_txn\":\"{0}\"}}", path);

            await Pool.CreatePoolLedgerConfigAsync("pool1", configJson);

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Pool.CreatePoolLedgerConfigAsync("pool1", configJson)
            );

            Assert.AreEqual(ErrorCode.PoolLedgerConfigAlreadyExistsError, ex.ErrorCode);
        }
    }
}
