
import json
import os
import sys
ENTERPRISE_DID = '2hoqvcwupRTUNkXn6ArYzs'
config = {
    "agency_pairwise_did": "7o2xT9Qtp83cJUJMUBTF3M",
    "agency_pairwise_verkey": "4hmBc54YanNhQHTD66u6XDp1NSgQm1BacPFbE7b5gtat",
    "agent_endpoint": "https://enym-eagency.pdev.evernym.com",
    "agent_enterprise_verkey": "By1CvKuLFRRdqMyGsmu8naVQQQfSH4MYna4K7d4KDvfy",
    "agent_pairwise_did": "NUHiPAuSi8XoPRPGnECPUo",
    "agent_pairwise_verkey": "Chj1oQYdmbTXKG96Fpo8C2sd6fRrt9UyCrbmuo4vzroK",
    "enterprise_did": "2hoqvcwupRTUNkXn6ArYzs",
    "enterprise_did_agent": "M7uZU89SUdsav7i4hVZtXp",
    "enterprise_name": "Planet Express",
    "enterprise_verkey": "vrWGArMA3toVoZrYGSAMjR2i9KjBS66bZWyWuYJJYPf",
    "genesis_path": "/var/lib/indy/verity-dev/pool_transactions_genesis",
    "logo_url": "https://robohash.org/default_config",
    "wallet_name": "my_real_wallet"
}
FILENAME = 'utils/vcxconfig.json'


def create_config(user_config):
    for i in user_config:
        config[i] = user_config[i]
    with open(FILENAME, 'w') as out_file:
        json.dump(config, out_file, indent=4, sort_keys=True)

