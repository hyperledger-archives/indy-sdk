using Hyperledger.Indy.LedgerApi;
using Hyperledger.Indy.PoolApi;
using Hyperledger.Indy.SignusApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.PairwiseTests
{
    [TestClass]
    public class PairwiseIntegrationTestBase : IndyIntegrationTestWithSingleWallet
    {
        protected string _myDid;
        protected string _theirDid;
        protected const string METADATA = "some metadata";
        protected const string PAIRWISE_TEMPLATE = "{{\"my_did\":\"{0}\",\"metadata\":\"{1}\"}}";
        protected const string PAIR_TEMPLATE = "{{\"my_did\":\"{0}\",\"their_did\":\"{1}\"}}";

        [TestInitialize]
        public async Task CreateDids()
        {
            var result = await Signus.CreateAndStoreMyDidAsync(_wallet, "{}");
            _myDid = result.Did;

            result = await Signus.CreateAndStoreMyDidAsync(_wallet, "{}");
            _theirDid = result.Did;

            await Signus.StoreTheirDidAsync(_wallet, string.Format("{{\"did\":\"{0}\"}}", _theirDid));
        }
    }
}
