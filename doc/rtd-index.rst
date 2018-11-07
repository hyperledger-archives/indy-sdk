Hyperledger Indy SDK
====================

.. image:: https://raw.githubusercontent.com/hyperledger/indy-node/master/collateral/logos/indy-logo.png
   :width: 50%

Distributed ledger purpose-built for decentralized identity.

Hyperledger Indy provides tools, libraries, and reusable components for providing digital identities 
rooted on blockchains or other distributed ledgers so that they are interoperable across administrative 
domains, applications, and any other silo. 

Introduction
------------
This is the official documentation for the `Hyperledger Indy SDK <https://www.hyperledger.org/projects>`_,
which provides a distributed-ledger-based foundation for `self-sovereign identity <https://sovrin.org>`_. 
Indy provides a software ecosystem for private, secure, and powerful identity, and the Indy SDK enables clients for it.
The major artifact of the SDK is a c-callable library; there are also convenience wrappers 
for various programming languages and Indy CLI tool.

All bugs, stories, and backlog for this project are managed through 
`Hyperledger's Jira <https://jira.hyperledger.org/secure/RapidBoard.jspa>`_ in project IS 
(note that regular Indy tickets are in the INDY project instead...). Also, make sure to join
us on `Hyperledger's Rocket.Chat <https://chat.hyperledger.org/>`_ at 
`#indy-sdk <https://chat.hyperledger.org/channel/indy-sdk>`_ to discuss. You will need a Linux 
Foundation login to get access to these channels.

Understanding Hyperledger Indy
------------------------------

If you have just started learning about self-sovereign identity, 
here are some resources to increase your understanding:

  * Hyperledger Indy Working Group calls happen every Thursday at 8amPT, 9amMT, 11amET, 4pmBST.
    Add to your calendar and join from any device: https://zoom.us/j/232861185 
  
  * A recent webinar explaining self-sovereign identity using Hyperledger Indy and Sovrin: 
    `SSI Meetup Webinar <https://youtu.be/RllH91rcFdE?t=4m30s>`_

  * Visit the main resource for all things "Indy" to get acquainted with the code base, 
    helpful resources, and up-to-date information: 
    `Hyperledger Wiki-Indy <https://wiki.hyperledger.org/projects/indy>`_

  * The next page contains an extended tutorial introduces Indy, explains how the whole ecosystem works, and how the
    functions in the SDK can be used to construct rich clients.

.. toctree::
   :maxdepth: 1

   getting-started/getting-started.md 
   rtd-tutorials
   rtd-design
   rtd-building
   rtd-migration-guides
   release-workflow.md
   signing-commits.md
   rtd-node-index

