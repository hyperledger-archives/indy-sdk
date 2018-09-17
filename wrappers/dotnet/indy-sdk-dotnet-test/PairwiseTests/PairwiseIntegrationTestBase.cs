using Hyperledger.Indy.DidApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.PairwiseTests
{
    [TestClass]
    public class PairwiseIntegrationTestBase : IndyIntegrationTestWithSingleWallet
    {
        protected string myDid;
        protected string theirDid;
        protected string theirVerkey;
        protected const string PAIR_TEMPLATE = "{{\"my_did\":\"{0}\",\"their_did\":\"{1}\"}}";

        [TestInitialize]
        public async Task CreateDids()
        {
            var result = await Did.CreateAndStoreMyDidAsync(wallet, "{}");
            myDid = result.Did;

            result = await Did.CreateAndStoreMyDidAsync(wallet, "{}");
            theirDid = result.Did;
            theirVerkey = result.VerKey;

            await Did.StoreTheirDidAsync(wallet, string.Format("{{\"did\":\"{0}\"}}", result.VerKey));
        }
    }
}
