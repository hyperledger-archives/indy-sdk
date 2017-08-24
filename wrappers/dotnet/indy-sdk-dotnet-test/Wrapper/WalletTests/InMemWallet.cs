﻿using Indy.Sdk.Dotnet.Wrapper;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;
using System;
using System.Collections.Generic;
using System.Linq;

namespace Indy.Sdk.Dotnet.Test.Wrapper.WalletTests
{
    public class InMemWallet : CustomWalletBase
    {
        private static IDictionary<string, WalletRecord> _records = new Dictionary<string, WalletRecord>();
        private TimeSpan _freshnessDuration;

        public InMemWallet(TimeSpan freshnessDuration)
        {
            _freshnessDuration = freshnessDuration;
        }

        public bool IsOpen { get; set; }
        
        public override ErrorCode Set(string key, string value)
        {
            var record = new WalletRecord() { Value = value, TimeCreated = DateTime.Now };

            _records[key] = record;
            return ErrorCode.Success;
        }

        public override ErrorCode Get(string key, out string value)
        {
            value = null;

            if (!_records.ContainsKey(key))
                return ErrorCode.WalletNotFoundError;

            var record = _records[key];

            value = record.Value;

            return ErrorCode.Success;
        }

        public override ErrorCode GetNotExpired(string key, out string value)
        {
            value = null;

            if (!_records.ContainsKey(key))
                return ErrorCode.WalletNotFoundError;

            var record = _records[key];
            var recordAge = DateTime.Now - record.TimeCreated;

            if (recordAge > _freshnessDuration)
                return ErrorCode.WalletNotFoundError;

            value = record.Value;

            return ErrorCode.Success;
        }

        public override ErrorCode List(string keyPrefix, out string valuesJson)
        {
            var matchingItems = _records.Where(kvp => kvp.Key.StartsWith(keyPrefix)).ToList();

            var array = new JArray();
            foreach(var item in matchingItems)
            {
                var record = item.Value;
                array.Add(record.Value);
            }

            valuesJson = array.ToString(Formatting.None);

            return ErrorCode.Success;
        }

        private class WalletRecord
        {
            public string Value { get; set; }
            public DateTime TimeCreated { get; set; }
        }
    }
}
