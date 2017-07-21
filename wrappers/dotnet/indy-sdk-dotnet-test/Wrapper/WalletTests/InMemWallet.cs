using Indy.Sdk.Dotnet.Wrapper;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;
using System;
using System.Collections.Concurrent;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Test.Wrapper.WalletTests
{
    public class InMemWallet : WalletBase
    {
        private static IDictionary<string, string> _values = new Dictionary<string, string>();

        public bool IsOpen { get; set; }

        public override void Set(string key, string value)
        {
            _values.Add(key, value);
        }

        public override void Get(string key, out string value)
        {
            value = _values[key];
        }

        public override void GetNotExpired(string key, out string value)
        {
            value = _values[key]; //Nothing is ever expired in *this* wallet...
        }

        public override void List(string keyPrefix, out string valuesJson)
        {
            var matchingValues = _values.Where((x) => x.Key.StartsWith(keyPrefix));

            valuesJson = JsonConvert.SerializeObject(matchingValues.ToArray());            
        }
    }
}
