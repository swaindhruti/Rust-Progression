# 🌀 Turbine: Solana’s Original Block Propagation Protocol

## 📦 What Is Turbine?

**Turbine** is Solana’s protocol for distributing blocks (transaction data) efficiently from the block leader to all validator nodes. It’s essential for maintaining Solana’s high throughput without overwhelming the network.

---

### ⚙️ How Turbine Works

- **Block Fragmentation:**  
  The leader splits each block into smaller packets/fragments.
- **Tree-Based Relay:**  
  Packets are sent in a tree-like fashion:  
  - Leader → a few nodes  
  - Those nodes → more nodes  
  - Continues until all validators have the full block
- **Bandwidth Efficiency:**  
  Each node relays data to only a few peers, avoiding bottlenecks and keeping the network scalable.

---

### 🧑‍💻 Technical Highlights

- Multi-layer tree structure for data relay.
- Fast block propagation and high throughput.
- Reduced bandwidth demand keeps Solana decentralized and speedy.

---

## 🔗 Turbine & Alpenglow: The Connection

**Alpenglow** is Solana’s upcoming upgrade, introducing **Rotor** as Turbine’s successor.

- **Rotor** builds on Turbine’s architecture but streamlines block propagation:
  - **Single-hop:** Leader sends data to all validators in one round.
  - **Stake-weighted relays:** Nodes with more stake relay more data.
  - **Erasure coding:** Blocks are split into pieces that can be reassembled even if some are missing, boosting reliability.

- **Result:**  
  - Faster, more robust block delivery  
  - Latency drops from ~12.8s (Turbine) to ~150ms (Rotor/Alpenglow)  
  - Eliminates multi-hop tree overhead for “instant” confirmations

---

## 📝 Comparison Table

| Protocol   | Used In          | Delivery Method                   | Latency      |
|------------|------------------|-----------------------------------|--------------|
| **Turbine**| Solana (Now)     | Tree, multiple hops               | ~12.8s       |
| **Rotor**  | Alpenglow (Soon) | Single-hop, stake-weighted, codes | ~150ms       |

---

## 🏁 Summary

- **Turbine** enabled Solana’s high-speed block propagation.
- **Rotor** (in Alpenglow) refines these ideas for even faster, more reliable delivery—making instant block confirmations possible for next-gen Web3 apps.

---