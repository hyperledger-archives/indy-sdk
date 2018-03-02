import copy
import json
import sys
ENTERPRISE_DID = '2hoqvcwupRTUNkXn6ArYzs'
config_dev = {
  "agent_enterprise_verkey": "ECFLTKPNisizb8AyrG4xEMyng1WoRdjspucWNNEPvnfm",
  "enterprise_did": "2hoqvcwupRTUNkXn6ArYzs",
  "enterprise_name": "<CHANGE_ME>",
  "agent_pairwise_verkey": "nroWbyqRFs3SB6EAKCH62dgqWauoySLGiJgtvFRSjgX",
  "agency_pairwise_verkey": "J8Yct6FwmarXjrE2khZesUXRVVSVczSoa9sFaGe6AD2v",
  "wallet_name": "zoidberg",
  "logo_url": "<CHANGE_ME>",
  "wallet_key": "zoidberg",
  "agent_pairwise_did": "2T94qQsPLUnXmfucC8W1AQ",
  "agent_endpoint": "https://enym-eagency.pdev.evernym.com",
  "enterprise_did_agent": "RD34zYXMtAkXpbm8GSmSei",
  "agency_pairwise_did": "YRuVCckY6vfZfX9kcQZe3u",
  "genesis_path": "<CHANGE_ME>",
  "enterprise_verkey": "vrWGArMA3toVoZrYGSAMjR2i9KjBS66bZWyWuYJJYPf"
}

config = {
  "enterprise_did": "XqLGQ6NCi538najLyKBc5y",
  "enterprise_did_agent": "QDD24KzHvAscC5o2YNfUi7",
  "agent_endpoint": "https://eas01.pps.evernym.com",
  "wallet_key": "jesse",
  "agent_enterprise_verkey": "DejGJXVGMi2ikwg2TTQ5DgwvMkGbT2t6Z9yKEqkSWKTM",
  "agent_pairwise_verkey": "HzRKLMoP19XRvwLek2GuJvReYPKWx56dDUZiJS1tBzDz",
  "agency_pairwise_verkey": "871Kg9brdb6u9tGczaf6eeYobeEnWau5YtdxaZZHmURD",
  "enterprise_verkey": "HohhuEmSwsXTWoUeW4quNgHGoZQ1KwLePGp6NkorGhE4",
  "agency_pairwise_did": "E2vjDzFQP2b1AqtY66n2st",
  "agent_pairwise_did": "YAzmMwSFCubxQjjZXTS2vS",
  "logo_url": "<CHANGE_ME>",
  "enterprise_name": "<CHANGE_ME>",
  "genesis_path": "<CHANGE_ME>",
  "wallet_name": "jesse"
}


def update_json_values(new_values, old_values):
    return_json = copy.deepcopy(old_values)
    for i in new_values:
        return_json[i] = new_values[i]
    return return_json

