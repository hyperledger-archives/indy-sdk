using Hyperledger.Indy.AgentApi;
using Hyperledger.Indy.LedgerApi;
using Hyperledger.Indy.PoolApi;
using Hyperledger.Indy.SignusApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System;
using System.Collections.Generic;
using System.Text;
using System.Threading;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.AgentTests
{
    [TestClass]
    public class DisposeAgentConnectionTest : AgentIntegrationTestBase
    {
        private CreateAndStoreMyDidResult myDid;
        private AgentListener activeListener;

        [TestInitialize]
        public async Task PrepareForConnection()
        {
           var endpoint = "127.0.0.1:9603";

            myDid = await Signus.CreateAndStoreMyDidAsync(_wallet, "{}");

            var identityJson = string.Format("{{\"did\":\"{0}\", \"pk\":\"{1}\", \"verkey\":\"{2}\", \"endpoint\":\"{3}\"}}",
                    myDid.Did, myDid.Pk, myDid.VerKey, endpoint);

            await Signus.StoreTheirDidAsync(_wallet, identityJson);

            activeListener = await AgentListener.ListenAsync(endpoint);

            await activeListener.AddIdentityAsync(_pool, _wallet, myDid.Did);
        }

        [TestCleanup]
        public async Task Cleanup()
        {
            await activeListener.CloseAsync();
        }

        [TestMethod]
        public async Task CanDisposeClosedConnection()
        {
            using (var connection = await AgentConnection.ConnectAsync(_pool, _wallet, myDid.Did, myDid.Did))
            {
                await connection.CloseAsync();
            }
        }

        [TestMethod]
        public async Task DisposeCanBeCalledRepeatedly()
        {
            var connection = await AgentConnection.ConnectAsync(_pool, _wallet, myDid.Did, myDid.Did);
            connection.Dispose();
            connection.Dispose();
        }

        [TestMethod]
        [Ignore] //Appears endpoint cannot be re-connected to.  Requires further testing.
        public async Task EndpointCanBeReUsedAfterDispose()
        {
            var connection = await AgentConnection.ConnectAsync(_pool, _wallet, myDid.Did, myDid.Did);
            await connection.CloseAsync();
            connection.Dispose();

            using (var newConnection = await AgentConnection.ConnectAsync(_pool, _wallet, myDid.Did, myDid.Did))
            {
            }
        }

        [TestMethod]
        public async Task CanCloseAfterDispose()
        {
            var connection = await AgentConnection.ConnectAsync(_pool, _wallet, myDid.Did, myDid.Did);
            connection.Dispose();

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                connection.CloseAsync()
            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }
    }
}
