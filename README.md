
# Unchain Protocol
  

**Private, censorship-resistant monetization for the new creator economy.**

Powered by Solana and Arcium.


[![Website](https://img.shields.io/badge/Website-unchainprotocol.adityapatwa.tech-blue)](https://unchainprotocol.adityapatwa.tech/)
[![Frontend Repo](https://img.shields.io/badge/Frontend-GitHub-181717?logo=github)](https://github.com/Aditya-Patwa/TheUnchainProtocol/)
  

## Overview

Unchain Protocol enables creators to receive support without fear of financial censorship, while allowing supporters to contribute privately without leaving a public trace. Built on Solana for speed and low fees, with Arcium's confidential computing for privacy-preserving transactions.

### Key Features

- **Anonymous Tipping** - Support creators without revealing your identity
- **Encrypted Balances** - All financial data stored in encrypted format using Arcium MCP
- **Censorship-Resistant** - Decentralized payments that can't be blocked or reversed
- **Instant Settlement** - Near-instant transactions powered by Solana
- **Multi-Platform** - Chrome extension for tipping across social media (X, YouTube, Twitch, Discord)
- **Escrow System** - Tips held securely for unregistered creators until they claim them

  

### Roadmap

-  **Phase 1** (Current): Anonymous tipping platform

-  **Phase 2**: Subscription-based creator memberships

-  **Phase 3**: Exclusive content distribution with privacy-preserving access control

  

## Technical Architecture


### Core Components

#### 1. Unchain Vault

The protocol's central treasury that stores all deposited funds. Acts as an anonymization layer by breaking the on-chain link between supporters and creators.

- Holds pooled funds from all users
- Manages tip distribution through encrypted instructions
- Implements withdrawal mechanisms for creators

#### 2. Unchain Profile

A one-time profile created for each user (both creators and supporters) that tracks their encrypted balance and activity.

- **Encrypted Storage**: All balance and transaction data stored using Arcium's confidential computing
- **Universal Identity**: Single profile works across all platforms (website + extension)
- **Privacy-First**: No public association between profile and on-chain wallet

#### 3. Escrow System

Smart contract-based escrow for tips sent to unregistered creators.

- Tips locked in escrow until creator claims them
- Social proof mechanism (public tagging) to notify creators
- Secure claim process with identity verification
- Automatic release after verification




## How It Works

### Initial Setup (All Users)

1. **Create Unchain Profile**

- One-time setup connecting your Solana wallet
- Generates encrypted profile for future transactions
- Enables access to tipping and (future) exclusive content

2. **Fund Your Profile**

- Deposit funds into the Unchain Vault
- Your balance is recorded in your Unchain Profile (encrypted)
- Breaks the on-chain link between your wallet and future tips

---  
### For Creators

#### Registration & Setup

```
1. Connect Solana wallet
2. Create Unchain Profile
3. Link social accounts (X, YouTube, Discord, Twitch, etc.)
4. Start receiving tips instantly
```

#### Receiving Tips

- **From Website**: Direct tips to your dedicated creator page
- **From Extension**: Tips sent directly on your social media profiles
- **All earnings encrypted**: Total balance and transaction history stored privately using Arcium MCP

#### Future: Subscription Content (Phase 2)

- Create tiered membership levels
- Distribute exclusive content to subscribers
- Privacy-preserving access control (subscribers remain anonymous)

---

  

### For Supporters

#### Tipping via Website

**Flow for Registered Creators:**

```
1. Visit creator's dedicated page on Unchain Protocol
2. Enter tip amount
3. Submit transaction
4. MCP processes encrypted payment instruction
5. Tip sent from Vault to creator (your identity never revealed)
```

  

#### Tipping via Chrome Extension

  

**Flow for Registered Creators:**

```
1. Visit creator's profile on any supported platform (X, YouTube, etc.)
2. Click Unchain extension button
3. Enter tip amount in modal
4. Submit tip
5. MCP processes encrypted payment from Vault
6. Creator receives tip instantly
```

**Flow for Unregistered Creators:**

```
1. Visit creator's profile on any supported platform
2. Click Unchain extension button and send tip
3. Funds locked in Escrow contract tied to creator's username
4. Automated notification post: "@username Someone tipped you anonymously on @UnchainProtocol! Create a profile and verify your handle to claim."
5. Creator joins platform and verifies identity
6. Escrow automatically releases all pending tips to creator
``` 

## Privacy & Security

### Privacy Guarantees

- **Supporter Anonymity**: Tips sent from Vault, not your personal wallet
- **Encrypted Balances**: All financial data encrypted using Arcium's MCP
- **No Public Trail**: No on-chain link between your wallet and the tips you send
- **Creator Privacy**: Optional - creators can keep earnings private

  
### Security Model

- **Non-Custodial**: Users maintain control of funds via Solana wallet
- **Smart Contract Escrow**: Tips for unregistered creators held in auditable contracts
- **Decentralized Infrastructure**: No single point of failure
- **Transparent Fees**: 2.5% protocol fee on successful tips
  

## Technology Stack

- **Blockchain**: Solana (high throughput, low fees)
- **Privacy Layer**: Arcium MCP (confidential computing for encrypted data)
- **Smart Contracts**: Anchor framework
- **Frontend**: Next.js, TypeScript, TailwindCSS
- **Extension**: Chrome Extension Manifest V3
- **Wallet Integration**: Phantom, Solflare, and other Solana wallets


## Links

  

- **Website**: [unchainprotocol.adityapatwa.tech](https://unchainprotocol.adityapatwa.tech/)
- **Frontend Repository**: [GitHub](https://github.com/Aditya-Patwa/TheUnchainProtocol/)
- **Twitter**: [@UnchainProtocol](https://x.com/UnchainProtocol)
- **Documentation**: Coming soon

  

## Reach Out to Us

For questions or queries, reach out to us at theunchainprotocol@gmail.com.

---

  

*Unchain the Creator Economy. Support Privately. Create Freely.*