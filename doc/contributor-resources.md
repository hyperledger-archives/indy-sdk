# Contributing to Indy SDK


Before contributing to Indy SDK, there are a couple steps that will help your process go much more smoothly.

First, please take a look at our contributing guidelines: [how to contribute to Hyperledger Indy](http://bit.ly/2ugd0bq).

If you are looking for how to sign current or previous commits, go here: [signing your commits](signing-commits.md)

## Connect with the Community

Hyperledger Indy has a vibrant and active community of developers willing to help you answer questions, learn more about self-sovereign identity, and get involved.

You will find the best and most update resources on chat board here: [Hyperledger Rocket Chat](https://chat.hyperledger.org/home)

\#indy-sdk, \#indy-node, and \#indy are the some of the best channels to get started. Please introduce yourself and let us know what you want to accomplish!

## How to Start Working with the Code

1. Fork the indy-sdk repository on Github to your personal account.

1. Add the hyperledger/indy-sdk as the remote upstream:  
   `git remote add upstream https://github.com/hyperledger/indy-sdk.git`

1. Set up Developer Certificate of Origin and learn how to [sign your commits](signing-commits.md)  

1. Take a look at our [release workflow](release-workflow.md)

## How to send a PR

- Do not create big PRs; send a PR for one feature or bug fix only.
 If a feature is too big, consider splitting a big PR to a number of small ones.
- Consider sending a design doc into `design` folder (as markdown or PlantUML diagram) for a new feature  before implementing it
- Make sure that a new feature or fix is covered by tests (try following TDD)
- Make sure that documentation is updated according to your changes
- Provide a full description of changes in the PR including Jira ticket number if any  
- Make sure that all your commits have a DCO sign-off from the author. (add the `-s` flag to all commits)
- Put the link to the PR into `#indy-pr-review` channel in Rocket.Chat
- A reviewer needs to start your tests first (add `test this please` comment to the PR)
- You need to make sure that all the tests pass
- A reviewer needs to review the code and approve the PR. If there are review comments, they will be put into the PR itself.
- You must process them (feel free to reply in the PR threads, or have a discussion in Rocket.Chat if needed)
- A reviewer or maintainer will merge the PR
