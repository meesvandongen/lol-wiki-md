# Quinn

## Table of Contents
- [Basic Information](#basic-information)
- [Statistics](#statistics)
  - [Advanced Stats](#advanced-stats)
- [Abilities](#abilities)
- [Patch History](#patch-history)
- [Trivia](#trivia)

## Basic Information

| Attribute | Value |
|-----------|-------|
| **Champion** | Quinn |
| **Title** | Demacia's Wings |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2013-03-01 |
| **Release Patch** | V3.03 |
| **Roles** | Specialist |
| **Riot Positions** | Top |
| **External Positions** | Top |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $565.0$ | $+107.0$ | $2384.0$ |
| **Mana** | $269.0$ | $+35.0$ | $864.0$ |
| **Health Regen** | $5.5$ | $+0.55$ | $14.9$ |
| **Mana Regen** | $7.0$ | $+0.4$ | $13.8$ |
| **Armor** | $28.0$ | $+4.7$ | $107.9$ |
| **Magic Resist** | $30.0$ | $+1.3$ | $52.1$ |
| **Attack Damage** | $59.0$ | $+1.0$ | $76.0$ |
| **Attack Speed** | $0.668$ | $+3.1\%$ | $1.020$ |
| **Movement Speed** | $330.0$ | $+0.0$ | $330.0$ |
| **Attack Range** | $525.0$ | $+0.0$ | $525.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.668$ |
| **Attack Speed Ratio** | $0.668$ |
| **Bonus AS per Level** | $3.1\%$ |
| **Missile Speed** | $2000 units/second$ |
| **Pathing Radius** | $35 units$ |
| **Selection Radius** | $120 units$ |
| **Selection Height** | $155 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Harrier

**Innate:** **Quinn**’s damaging abilities mark enemies with *Vulnerable* and standard sight them for a few seconds. Periodically, Valor marks a nearby enemy with *Vulnerable*.

*''Quinn's* next basic attack against a *Vulnerable' target consumes the mark to deal **bonus** physical damage.*

**Innate:** ''Quinn's* **Blinding Assault** against the primary target, **Vault**, and **Skystrike** mark enemies hit with *Vulnerable* for 4 seconds, during which they are standard sight. **Valor**** will periodically mark a nearby sight enemy if no *Vulnerable' targets exist for 1 second. ''Quinn's* basic attacks against *Vulnerable' targets are empowered to consume the mark to deal 10 to 95 (+ key=% AD) **bonus** physical damage. While **Behind Enemy Lines** is active, *Harrier* is disabled and all *Vulnerable* marks are removed.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Self |
| **Damage Type** | Physical |
| **Effect Radius** | 525 units |
| **Spell Shield** | False |
| **Spell Effects** | Proc |

**Notes:**

- If a mark applied by an ability is overwritten, consumed, or timed out, *Harrier* goes on a 1-second cooldown if it's not already on cooldown.
- 'Harrier's targeting priority: *# Last unit hit (if it was a small minion it appears to select a different target). *# Lowest-health enemy champion. *# Lowest-health enemy minion.
- *Harrier* is consumed even if it is parried, the attack is negated in any case.
- *Harrier* will not mark enemy minions while **Quinn** is not sight, but will mark enemy champions.

---

### Q: Blinding Assault

**Active:** **Quinn** sends **Valor** in the target direction who stops upon hitting an enemy, dealing physical damage all nearby enemies.

*The primary target is nearsight briefly if they are a champion, and disarm otherwise.*

**Active:** **Quinn** sends **Valor*** in the target direction who stops upon hitting an enemy, marking them as Vulnerable*** and dealing physical damage to all nearby enemies. The primary target is nearsight for $1.75$ seconds if they are a champion, and disarm for the same duration otherwise.

| Attribute | Value |
|-----------|-------|
| **Range** | 1050 units |
| **Cooldown** | $11-9$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $50-70$ Mana |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Speed** | 1550 units/second |
| **Effect Radius** | 210 units |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | True |

**Scaling:**
- **Physical Damage:** $65-205$
- *bonus AD) (+ 50% AP)

**Notes:**

- 'Blinding Assault's nearsight will persist through death. Effect at cast time start

---

### W: Heightened Senses

**Passive:** Attacking a *Vulnerable* target or consuming a mark grants **Quinn** **bonus attack speed** and *ms **bonus** movement speed* for a short time.

**Active:** **Valor** gains sight of the surrounding area for a short time, standard sight enemies detected.

**Passive:** Whenever **Quinn** uses a basic attack on-attack against a **Vulnerable** target or consumes their mark, she gains **bonus attack speed** and *ms **bonus** movement speed* for 2 seconds. **Active:** **Valor** grants sight of the surrounding area for 2 seconds and standard sight enemy champions within for the same duration.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $50-30$ seconds |
| **Cast Time** | none |
| **Targeting** | Auto |
| **Affects** | Self |
| **Effect Radius** | 2100 units |
| **Spell Shield** | False |

**Scaling:**
- **Bonus Attack Speed:** $28-80$%
- **Bonus Movement Speed:** $20-40$%

**Notes:**

- **Quinn** marks enemy champions who were previously unseen but were revealed *Heightened Senses* in order to gain assist credit, lasting for the standard credit timer.

---

### E: Vault

**Active:** **Quinn** dashes to the target enemy, dealing physical damage, airborne, marking them as *Vulnerable*, and briefly slow them. She then dash backward.

**Active:** **Quinn** dashes to the target enemy, marking them as **Vulnerable**, dealing physical damage, airborne a very short distance over $0.5$ seconds, and slow them by 50% decaying over $1.5$ seconds. She then dash 525 units away from them. *Vault basic attack reset *'Quinn's* basic attack timer shortly after the initial dash. *Heightened Senses* can be cast during either of the dashes.*

| Attribute | Value |
|-----------|-------|
| **Range** | 600 units |
| **Cooldown** | $12-8$ seconds |
| **Cast Time** | none |
| **Cost** | 50 Mana |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Speed** | 2500 / 850 units/second |
| **Spell Shield** | True |
| **Spell Effects** | Spell |

**Scaling:**
- **Physical damage:** $40-140$ (+ 20% bonus AD)

**Notes:**

- **Quinn** will track the target if they change locations.
  - She will dash to the target's previous location without applying 'Vault's effects if the target is too far away or moves beyond 1200 units.
- *Vault* basic attack reset ''Quinn's' remaining attack cooldown a short delay after the first dash ends (even if interrupted).
  - This delay is a fuzzy $0.25$ to $0.5$ seconds and this matches up with the moment the target of *Vault* is marked as *Vulnerable*.
  - The basic attack reset does not count as one for *Hail of Blades*.
- **Quinn** will automatically attempt to attack the target once she completes *Vault*.
- The target will turn to face the opposite direction after being airborne.
- Unlike its constant cast range 'Vault's bounce's can vary to allow **Quinn** to close or to make a gap between herself and the target (depending on casting position) or even to jump through a wall (if there is enough landing room on the other side).
- *Vault* also interrupts basic attacks with uncancellable windup.

---

### R: Behind Enemy Lines

**Active:** **Quinn** channels for a short time to pair up with **Valor**. They then become united, increasing her and becoming ghosted.

*The duo lose the movement speed boost upon taking non-minion damage, and disband upon being immobilize, ground, or silence.*

**Active:** **Quinn** channels for 2 seconds, signaling **Valor**** to pair up. Upon completion, he picks her up and they unite, increasing her *ms **total** movement speed*, granting her ghosted, and allowing her to cast **Skystrike**. *Behind Enemy Lines* can be recast after $0.5$ seconds during the channel. Taking damage from non-minions while *Behind Enemy Lines* is active or while **Quinn** is channel the ability causes her to lose the **bonus total** movement speed for 3 seconds. Becoming immobilize, ground, or silence ends *Behind Enemy Lines* immediately and puts it on *cooldown* without performing **Skystrike**. Once *Behind Enemy Lines* has been learned, respawning or entering any summoning platform will spawn **Valor**** instantly. **Recast:** **Quinn** cancels the channel, placing *Behind Enemy Lines* on cooldown.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | 3 seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $100-0$ Mana |
| **Targeting** | Auto |
| **Affects** | Self |

**Scaling:**
- **Total Movement Speed Increase:** $70-130$%

**Notes:**

- Taking damage from non-minions removes the bonus movement speed buff temporarily by suppressing it with the status effect of a slow.
- *Behind Enemy Lines* does not refund its *mana cost* if the channel was canceled via recast.
  - The only exception is if ''Quinn's* mana falls below the mana cost, in which case she is granted enough mana back (the equal amount needed) to be able to cast *Behind Enemy Lines' again.
- Respawning or returning to the summoning platform will not spawn **Valor**** if *Behind Enemy Lines* is on cooldown.
- If **Valor**** is spawned by respawning or returning to the summoning platform, *Skystrike* will be disabled from being manually cast for $2.5$ seconds upon uniting. However, it will still automatically activate upon declaring a basic attack or casting *Blinding Assault* or *Vault*.
- *Behind Enemy Lines* ends immediately without *Skystrike* being cast if she enters or is inside the enemy fountain. The ability will go on full *cooldown*.
  - **Quinn** will also spawn **Valor**** when entering the enemy fountain.
- Self-immobilize such as *Zhonya's Hourglass* also count for ending *Behind Enemy Lines*.
- The following table refers for interactions while **Quinn** is channel:
  - *Behind Enemy Lines* is not a movement channel, and so will not be interrupt by root and ground.

---

### R: Skystrike

**Active:** **Quinn** disbands from **Valor** and rains arrows down around her, dealing physical damage to nearby enemies.

*''Quinn's* attacks and damaging abilities automatically activate *Skystrike'.*

**Active:** **Quinn** detaches from **Valor**, ending **Behind Enemy Lines*’s* effects and raining arrows down around her, dealing 70% AD physical damage to nearby enemies and marking them as **Vulnerable**. Declaring a basic attack or casting **Blinding Assault** or **Vault** during **Behind Enemy Lines** automatically activates *Skystrike*.

| Attribute | Value |
|-----------|-------|
| **Cast Time** | none |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Effect Radius** | 700 units |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |

**Notes:**

- If **Quinn** uses *Vault* while *Behind Enemy Lines* is active, *Skystrike* will activate at her target's location after she bounces back.
- Upon respawning or recalling, ''Quinn's* first *Skytrike' will deal double the normal amount of damage (140% AD) to enemies hit if activated by an ability.

---

## Patch History

### V25.15
- *Blinding Assault*
  - Base damage increased to $65-205$ from $20-100$.
  - AD ratio reduced to $80-120$% *bonus AD from $80-120$% **total** AD.
- *Heightened Senses*
  - Bonus attack speed increased to $28-80$% from $28-60$%.

### V14.19
- General
  - Updated visual effects.
  - The following skins are affected: Quinn, Quinn, Quinn, Quinn, Quinn.
  - Quinn
    - Updated *Blinding Assault* VFX.

### V14.4
- *Behind Enemy Lines*
  - **Bug Fixes:** Linger VFX no longer follow her while in her ultimate form.

### V13.20
- Stats
  - Attack damage growth increased to $2.7$ from $2.4$.
  - Base health reduced to 565 from 603.
  - Base movement speed reduced to 330 from 335.

### V13.17
- Stats
  - Health growth increased to 107 from 99.
- *Blinding Assault*
  - Base damage reduced to $20-100$ from $20-120$.
- *Vault*
  - Base damage reduced to $40-140$ from $40-160$.

### V12.22
- Stats
  - Base mana regeneration increased to 7 from $6.97$.

### V12.14
- General
  - **Bug Fixes:** Knocked up/stunned animation is now correctly played.

### V12.10
- Stats
  - Base health increased to 603 from 533.
  - Health growth increased to 99 from 85.
  - Armor growth increased to $4.7$ from $3.5$.
  - Magic resistance growth increased to $1.3$ from $0.5$.

### V12.3
- *Heightened Senses*
  - Bonus attack speed increased to $28-60$% from $20-60$%.
- *Skystrike*
  - AD ratio increased to 70% AD from 40% AD.

### V11.21
- *Heightened Senses*
  - Bonus attack speed reduced to $20-60$% from $20-80$%.

## Trivia

- Quinn is the first champion whose name starts with **Q** (back then the only remaining unused letter in the alphabet regarding champion names).
- Quinn's dance references Are You That Somebody by Aaliyah.
  - A side-by-side comparison can be seen here.
- Only *Skystrike* fires Quinn's crossbow despite it being her weapon of choice.
  - Quinn's bolts are Fletching with blue feathers which turn to a much lighter shade when hitting a target marked by *Harrier*.

---
*This page was automatically generated from League of Legends Wiki data.*