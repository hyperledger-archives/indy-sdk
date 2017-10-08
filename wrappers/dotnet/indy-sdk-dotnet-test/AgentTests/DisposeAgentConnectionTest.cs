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
        private CreateAndStoreMyDidResult _myDid;
        private AgentListener _activeListener;

        public async Task PrepareForConnection(string endpoint)
        {
            _myDid = await Signus.CreateAndStoreMyDidAsync(_wallet, "{}");

            var identityJson = string.Format("{{\"did\":\"{0}\", \"pk\":\"{1}\", \"verkey\":\"{2}\", \"endpoint\":\"{3}\"}}",
                    _myDid.Did, _myDid.Pk, _myDid.VerKey, endpoint);

            await Signus.StoreTheirDidAsync(_wallet, identityJson);

            _activeListener = await AgentListener.ListenAsync(endpoint);

            await _activeListener.AddIdentityAsync(_pool, _wallet, _myDid.Did);
        }

        [TestCleanup]
        public async Task Cleanup()
        {
            if (_activeListener != null)
                await _activeListener.CloseAsync();
        }

        [TestMethod]
        public async Task CanDisposeClosedConnection()
        {
            await PrepareForConnection("127.0.0.1:9610");

            using (var connection = await AgentConnection.ConnectAsync(_pool, _wallet, _myDid.Did, _myDid.Did))
            {
                await connection.CloseAsync();
            }
        }

        [TestMethod]
        public async Task DisposeCanBeCalledRepeatedly()
        {
            await PrepareForConnection("127.0.0.1:9611");

            var connection = await AgentConnection.ConnectAsync(_pool, _wallet, _myDid.Did, _myDid.Did);
            connection.Dispose();
            connection.Dispose();
        }

        [TestMethod]
        [Ignore] //Appears endpoint cannot be re-connected to.  Requires further testing.
        public async Task EndpointCanBeReUsedAfterDispose()
        {
            await PrepareForConnection("127.0.0.1:9612");

            var connection = await AgentConnection.ConnectAsync(_pool, _wallet, _myDid.Did, _myDid.Did);
            await connection.CloseAsync();
            connection.Dispose();

            using (var newConnection = await AgentConnection.ConnectAsync(_pool, _wallet, _myDid.Did, _myDid.Did))
            {
            }
        }

        [TestMethod]
        public async Task CanCloseAfterDispose()
        {
            await PrepareForConnection("127.0.0.1:9618");

            var connection = await AgentConnection.ConnectAsync(_pool, _wallet, _myDid.Did, _myDid.Did);
            connection.Dispose();
            await connection.CloseAsync();
        }
    }
}
