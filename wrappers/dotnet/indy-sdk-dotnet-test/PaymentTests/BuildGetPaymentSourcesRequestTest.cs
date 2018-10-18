using Hyperledger.Indy.PaymentsApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.PairwiseTests
{
    [TestClass]
    public class BuildGetPaymentSourcesRequestTest : PaymentIntegrationTestBase
    {
        [TestMethod]
        public async Task TestBuildGetPaymentSourcesRequestWorksForUnknownPaymentMethod()
        {
            var ex = await Assert.ThrowsExceptionAsync<UnknownPaymentMethodException>(() =>
                 Payments.BuildGetPaymentSourcesAsync(wallet, DID_TRUSTEE, paymentAddress)
             );
        }

        [TestMethod]
        public async Task TestBuildGetPaymentSourcesRequestWorksForInvalidPaymentAddress()
        {
            var ex = await Assert.ThrowsExceptionAsync<IncompatiblePaymentMethodsException>(() =>
                Payments.BuildGetPaymentSourcesAsync(wallet, DID_TRUSTEE, "pay:null1")
            );
        }
    }
}
