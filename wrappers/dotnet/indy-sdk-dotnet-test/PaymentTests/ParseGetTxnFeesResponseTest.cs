using Hyperledger.Indy.PaymentsApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.PairwiseTests
{
    [TestClass]
    public class ParseGetTxnFeesResponseTest : PaymentIntegrationTestBase
    {
        [TestMethod]
        public async Task TestParseGetTxnFeesResponseResponseTestWorksForUnknownPaymentMethod()
        {
            var ex = await Assert.ThrowsExceptionAsync<UnknownPaymentMethodException>(() =>
                 Payments.ParseGetTxnFeesResponseAsync(paymentMethod, emptyObject)
             );
        }
    }
}
