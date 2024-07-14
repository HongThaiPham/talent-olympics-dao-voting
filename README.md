# DAO Voting Program

### SOLANA - The Talent Olympics

## Introduction

Develop a DAO voting program using Anchor. This program should allow users to vote on proposals and display results. Optionally, implement "privacy" voting using Zero-Knowledge (ZK) proofs or verifiable compute. Reward points should be given to users for participation.

- Create a DAO voting system using Anchor.

- Implement a voting system and display the results.

- Optionally, add privacy voting using ZK proofs or verifiable compute.

- Reward points to users for voting participation.

## Features

- Init a proposal with content and maximun votes
- Vote for or agains proposal, one per user in timeframe
- Result show how many vote for/agains proposal

## How to use

### Install the required dependencies:

- Rust
- Solana CLI
- Anchor

### Clone the repository:

```bash
git clone git@github.com:HongThaiPham/talent-olympics-dao-voting.git

cd talent-olympics-dao-voting
```

### Build the program:

```bash
anchor build
```

### Run the tests:

```bash
anchor test
```

Test case:

- [x] Should init a proposal successfully
- [x] Should vote on a proposal successfully
- [x] Should vote fail if maximum votes reached
- [x] Should creator close proposal fail when proposal not expired
- [x] Should creator close proposal successfully when proposal expired

### Deploy the program:

```bash
anchor deploy
```

## Video demo

[![Watch the video](https://cdn.loom.com/sessions/thumbnails/a37370bae08f47e2835b65d772152118-with-play.gif)](https://www.loom.com/share/a37370bae08f47e2835b65d772152118)
