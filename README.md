<p align="center">
  <img src="Logo.png" alt="BittyWorld Logo" width="180"/>
</p>

# BittyWorld

**The Next Generation of Decentralized Web3 Social Gaming on Solana**

[Website](https://bittyworld.fun) | [Twitter/X](https://x.com/BittyWorldSOL)

---

## Vision

BittyWorld is pioneering a decentralized global gaming ecosystem, seamlessly merging 2D chibi-style multiplayer real-time shooting battles with vibrant social interaction and blockchain innovation. Our mission is to empower every player to compete, connect, and thrive in a player-driven digital economy, where every achievement, chat, and contribution is rewarded.

## Project Overview

BittyWorld is a cutting-edge Web3 social gaming platform built on the Solana blockchain. Players can create decentralized rooms—no central server required—supporting up to eight players per room, with multiple rooms running concurrently. Engage in dynamic 2D battles, wield diverse weapons, and traverse multi-terrain, switchable maps. The platform features:

- **Decentralized Multiplayer Battles**: Peer-to-peer room creation, real-time action, and customizable game settings.
- **Dynamic $BITTY Token Rewards**: Earn through victories, social engagement, and referrals, with a transparent buyback mechanism.
- **Vibrant Social Layer**: Real-time all-channel chat, leaderboards, and cross-platform sharing.
- **Fair Launch**: 100% Pump.fun fair launch, no presale or team allocation—community-driven from day one.
- **Solana-Powered**: High throughput, low fees, and on-chain transparency for all gameplay and rewards.

## Gameplay Logic

1. **Room Creation & Matchmaking**
   - Players connect via Solana wallets (Phantom, Solflare, etc.) and create or join decentralized rooms.
   - Each room supports up to 8 players, with customizable settings (duration, health, armor, map).

2. **Real-Time Battles**
   - Players use a variety of weapons (hammers, grenades, submachine guns) and collect health/armor packs.
   - Multi-terrain maps (urban, desert, forest) offer unique tactical scenarios.
   - All actions and scores are recorded on-chain for transparency.

3. **Social Interaction**
   - Real-time, all-channel chat enables cross-room communication and community building.
   - Players can like, comment, and share highlights, with top moments earning additional $BITTY rewards.

4. **Token Economy & Rewards**
   - $BITTY tokens are earned for kills, leaderboard rankings, event participation, and social contributions.
   - Buyback wallet sustains the reward pool, with periodic repurchases and community-driven governance.
   - $BITTY is used for premium skins, power-ups, custom rooms, and on-chain voting.

5. **Governance & Community**
   - $BITTY holders vote on new features, maps, and events, shaping the future of BittyWorld.
   - Transparent reporting and on-chain analytics ensure trust and long-term engagement.

## Technical Architecture

- **Frontend**: React/Next.js, Phaser.js for game rendering, Solana Web3.js for wallet integration.
- **Backend**: Node.js/Express, WebSocket for real-time sync, PostgreSQL & Redis for data.
- **Smart Contracts**: Rust & Anchor on Solana, managing $BITTY distribution, buyback, and governance.
- **Decentralized Storage**: Arweave for battle snapshots and chat logs.
- **Infrastructure**: Docker, Kubernetes, QuickNode RPC for high performance.

## Quick Start

### Prerequisites
- Node.js (v18+)
- Rust (latest stable)
- Solana CLI & Anchor

### Installation
```bash
# Clone the repository
$ git clone https://github.com/Bittyworld/BittyWorld.git
$ cd bittyworld

# Install dependencies
$ cd client && npm install
$ cd ../server && npm install
$ cd ../contracts && cargo build
```

### Development
```bash
# Start frontend
yarn dev # or npm run dev in client/
# Start backend
npm run dev in server/
# Deploy smart contracts
anchor deploy in contracts/
```

## Tokenomics & Fair Launch
- 100% Pump.fun fair launch: No presale, no team allocation, all tokens distributed to the community.
- Dynamic buyback and burn mechanisms to sustain rewards and deflate supply.
- On-chain governance for all major decisions.

## Community & Social
- **Website**: [bittyworld.fun](https://bittyworld.fun)
- **Twitter/X**: [@BittyWorldSOL](https://x.com/BittyWorldSOL)
- **Discord**: Coming soon

## License
MIT License. See [LICENSE](LICENSE).

---

<p align="center">
  <b>BittyWorld — Redefining Web3 Gaming and Social Interaction</b>
</p> 
