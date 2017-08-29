import asyncio

from tests import anoncreds, signus, ledger, agent


def run_samples():
    loop = asyncio.get_event_loop()
    loop.run_until_complete(anoncreds.anoncreds_demo())
    loop.run_until_complete(signus.signus_demo())
    loop.run_until_complete(ledger.ledger_demo())
    loop.run_until_complete(agent.agent_demo())


if __name__ == '__main__':
    run_samples()
