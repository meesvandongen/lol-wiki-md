# Quinn

## Table of Contents
- [Basic Information](#basic-information)
- [Statistics](#statistics)
  - [Map-specific Stats](#map-specific-stats)
- [Pets](#pets)
- [Abilities](#abilities)
- [Patch History](#patch-history)
- [Trivia](#trivia)

## Basic Information

| Attribute | Value |
|-----------|------:|
| **Champion** | Quinn |
| **Title** | Demacia's Wings |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2013-03-01 |
| **Release Patch** | V3.03 |
| **Latest Changes** | V25.15 |
| **Roles** | Specialist |
| **Riot Positions** | Top |
| **External Positions** | Top |
| **Blue Essence** | 2400 |
| **Riot Points** | 880 |
| **Difficulty** | 2 |
| **Hero Type** | Marksman |
| **Alt Type** | Assassin |
| **Adaptive Type** | Physical |
| **Damage** | 3 |
| **Toughness** | 1 |
| **Control** | 2 |
| **Mobility** | 3 |
| **Utility** | 1 |
| **Style** | 20 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $565.0$ | $+107.0$ |
| **Mana** | $269.0$ | $+35.0$ |
| **Health Regen** | $5.5$ | $+0.55$ |
| **Mana Regen** | $7.0$ | $+0.4$ |
| **Armor** | $28.0$ | $+4.7$ |
| **Magic Resist** | $30.0$ | $+1.3$ |
| **Attack Damage** | $59.0$ | $+2.7$ |
| **Attack Speed** | $0.668$ | |
| **Movement Speed** | $330.0$ | $+0.0$ |
| **Attack Range** | $525.0$ | $+0.0$ |
| **Base Attack Speed** | $0.668$ | |
| **Attack Speed Ratio** | $0.668$ | |
| **Bonus AS per Level** | $3.1\%$ | |
| **Missile Speed** | $2000$ units/second | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $120$ units | |
| **Selection Height** | $155$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $110.0\%$ |
| **Damage Taken** | $90.0\%$ |

#### URF

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $100.0\%$ |
| **Damage Taken** | $95.0\%$ |

## Pets

### Valor

| Attribute | Value |
|-----------|------:|
| **Damage Type** | Physical |
| **Control** | N/A |
| **Targeting** | N/A |

---

## Abilities

### Passive: Harrier

| Attribute | Value |
|-----------|------:|
| **Effect Radius** | 525 units |
| **Cooldown Start** | post-effect |
| **Static Cooldown** | 8 / 1.45 (Starts after the mark expired or was consumed or overwritten by another mark.) |
| **Targeting** | Passive |
| **Affects** | Self |
| **Damage Type** | Physical |
| **Spell Shield** | False |
| **Spell Effects** | Proc |
| **Parry** | True |

**INNATE:** **Quinn**’s *Blinding Assault* against the primary target, *Vault*, and *Skystrike* mark enemies hit with *Vulnerable* for 4 seconds, during which they are revealed. **Valor** will periodically mark a nearby visible enemy if no *Vulnerable* targets exist for 1 second.

**Quinn**’s basic attacks against *Vulnerable* targets are empowered to consume the mark to deal 10 to 95 (+ 16 to 50 AD) **bonus** physical damage.

While *Behind Enemy Lines* is active, *Harrier* is disabled and all *Vulnerable* marks are removed.

**Notes:**

- If a mark applied by an ability is overwritten, consumed, or timed out, *Harrier* goes on a 1-second cooldown if it's not already on cooldown.
- *Harrier*’s targeting priority:
  1. Last unit hit (if it was a small minion it appears to select a different target).
  1. Lowest-health enemy champion.
  1. Lowest-health enemy minion.
- *Harrier* is consumed even if it is parried, the attack is negated in any case.
- *Harrier* will not mark enemy minions while **Quinn** is not visible, but will mark enemy champions.

---

### Q: Blinding Assault

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 1050 units |
| **Effect Radius** | 210 (Damage radius) units |
| **Width** | 120 units |
| **Speed** | 1550 units/second |
| **Cost** | 50 / 55 / 60 / 65 / 70 Mana |
| **Cooldown** | 11 / 10.5 / 10 / 9.5 / 9 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | True |
| **Sight Reduction** | 350 (Reduces sight radius to this number) |

**ACTIVE:** **Quinn** sends **Valor** in the target direction who stops upon hitting an enemy, marking them as *Vulnerable* and dealing physical damage to all nearby enemies.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 65 / 100 / 135 / 170 / 205 (+ 80 / 90 / 100 / 110 / 120% **bonus** AD) (+ 50% AP) |

The primary target is nearsighted for $1.75$ seconds if they are a champion, and disarmed for the same duration otherwise.

**Notes:**

- *Blinding Assault*’s nearsight will persist through death.
- This ability will cast from wherever the caster is at the start of the cast time.

---

### W: Heightened Senses

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 2100 units |
| **Cooldown** | 50 / 45 / 40 / 35 / 30 seconds |
| **Targeting** | Auto |
| **Affects** | Self |
| **Spell Shield** | False |

**PASSIVE:** Whenever **Quinn** uses a basic attack on-attack against a *Vulnerable* target or consumes their mark, she gains **bonus** attack speed and (ms) **bonus** movement speed for 2 seconds.

| Attribute | Value |
|-----------|------:|
| **Bonus Attack Speed** | 28 / 41 / 54 / 67 / 80% |

| Attribute | Value |
|-----------|------:|
| **Bonus Movement Speed** | 20 / 25 / 30 / 35 / 40% |

**ACTIVE:** Valor grants sight of the surrounding area for 2 seconds and reveals enemy champions within for the same duration.

**Notes:**

- **Quinn** marks enemy champions who were previously unseen but were revealed *Heightened Senses* in order to gain assist credit, lasting for the standard credit timer.

---

### E: Vault

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 600 units |
| **Speed** | 2500 (Initial dash speed, estimated) / 850 (Second dash speed, estimated) units/second |
| **Cost** | 50 Mana |
| **Cooldown** | 12 / 11 / 10 / 9 / 8 seconds |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | Spell |
| **Call For Help** | True |
| **Grounded** | True |
| **Knockdown** | True |

**ACTIVE:** **Quinn** dashes to the target enemy, marking them as *Vulnerable*, dealing physical damage, knocking them back a very short distance over $0.5$ seconds, and slowing them by 50% decaying over $1.5$ seconds. She then leaps back 525 units away from them.

| Attribute | Value |
|-----------|------:|
| **Physical damage** | 40 / 65 / 90 / 115 / 140 (+ 20% **bonus** AD) |

*Vault resets **Quinn**’s basic attack timer shortly after (See notes) the initial dash. Heightened Senses can be cast during either of the dashes.*

**Notes:**

- **Quinn** will track the target if they change locations.
  - She will dash to the target's previous location without applying *Vault*’s effects if the target is too far away or moves beyond 1200 (Estimated) units.
- *Vault* resets **Quinn**’s remaining attack cooldown a short delay after the first dash ends (even if interrupted).
  - This delay is a fuzzy $0.25$ to $0.5$ seconds and this matches up with the moment the target of *Vault* is marked as Vulnerable.
  - The basic attack reset does not count as one for Hail of Blades.
- **Quinn** will automatically attempt to attack the target once she completes *Vault*.
- The target will turn to face the opposite direction after being knocked back.
- Unlike its constant cast range *Vault*’s bounce's can vary to allow **Quinn** to close or to make a gap between herself and the target (depending on casting position) or even to jump through a wall (if there is enough landing room on the other side).
- *Vault* also interrupts basic attacks with uncancellable windups.

---

### R: Behind Enemy Lines

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Cost** | 100 / 75 / 50 / 25 / 0 Mana |
| **Cooldown** | 3 seconds |
| **Cooldown Start** | post-effect |
| **Targeting** | Auto |
| **Affects** | Self |
| **Grounded** | True |
| **Silence** | True |

**ACTIVE:** **Quinn** channels for 2 seconds, signaling **Valor** to pair up. Upon completion, he picks her up and they unite, increasing her (ms) **total** movement speed (Increases base movement as well as bonus movement speed), granting her ghosting, and allowing her to cast *Skystrike*. *Behind Enemy Lines* can be recast after $0.5$ seconds during the channel.

| Attribute | Value |
|-----------|------:|
| **Total Movement Speed Increase** | 70 / 85 / 100 / 115 / 130% |

Taking damage from non-minions while *Behind Enemy Lines* is active or while **Quinn** is channeling the ability causes her to lose the **bonus total** movement speed for 3 seconds. Becoming immobilized, grounded, or silenced ends *Behind Enemy Lines* immediately and puts it on cooldown without performing *Skystrike*.

Once *Behind Enemy Lines* has been learned, respawning or entering any (see notes) summoning platform will spawn **Valor** instantly.

**RECAST:** **Quinn** cancels the channel, placing *Behind Enemy Lines* on cooldown.

**Notes:**

- Taking damage from non-minions removes the bonus movement speed buff temporarily by suppressing it with the status effect of a slow.
- *Behind Enemy Lines* does not refund its mana cost if the channel was canceled via recast.
  - The only exception is if **Quinn**’s mana falls below the mana cost, in which case she is granted enough mana back (the equal amount needed) to be able to cast *Behind Enemy Lines* again.
- Respawning or returning to the summoning platform will not spawn **Valor** if *Behind Enemy Lines* is on cooldown.
- If **Valor** is spawned by respawning or returning to the summoning platform, Skystrike will be disabled from being manually cast for $2.5$ seconds upon uniting. However, it will still automatically activate upon declaring a basic attack or casting Blinding Assault or Vault.
- *Behind Enemy Lines* ends immediately without Skystrike being cast if she enters or is inside the enemy fountain. The ability will go on full cooldown.
  - **Quinn** will also spawn **Valor** when entering the enemy fountain.
- Self-immobilizations such as Zhonya's Hourglass also count for ending *Behind Enemy Lines*.
- The following table refers for interactions while **Quinn** is channeling:
  - *Behind Enemy Lines* is not a movement channel, and so will not be interrupted by root and ground.

| Aspect | State / Notes |
|--------|---------------|
| **Type** | Charge channel |
| **Attacking** | Interrupts |
| **Movement** | Disabled |
| **Abilities** | Interrupts |
| **Items** | Interrupts / Allowed / Disabled |
| **Summoner Spells** | Allowed / Interrupts |

---

### R: Skystrike

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 700 units |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |

**ACTIVE:** **Quinn** detaches from **Valor**, ending *Behind Enemy Lines’* effects and raining arrows down around her, dealing 70% AD physical damage to nearby enemies and marking them as *Vulnerable*.

Declaring a basic attack or casting *Blinding Assault* or *Vault* during *Behind Enemy Lines* automatically activates *Skystrike*.

**Notes:**

- If **Quinn** uses Vault while Behind Enemy Lines is active, *Skystrike* will activate at her target's location after she bounces back.
- Upon respawning or recalling, **Quinn**’s first *Skytrike* will deal double the normal amount of damage (140% AD) to enemies hit if activated by an ability.

---

## Patch History

### V25.15
- Blinding Assault
  - Base damage increased to 65 / 100 / 135 / 170 / 205 from 20 / 40 / 60 / 80 / 100.
  - AD ratio reduced to 80 / 90 / 100 / 110 / 120% **bonus** AD from 80 / 90 / 100 / 110 / 120% **total** AD.

### V25.S1.3
- Heightened Senses
  - Bonus attack speed increased to 28 / 41 / 54 / 67 / 80% from 28 / 36 / 44 / 52 / 60%.

### V14.19
- General
  - Updated visual effects.
  - The following skins are affected: Quinn, Quinn, Quinn, Quinn, Quinn.
  - Quinn
    - Updated Blinding Assault VFX.

### V14.4
- Behind Enemy Lines
  - **Bug Fixes:** Linger VFX no longer follow her while in her ultimate form.

### V13.20
- Stats
  - Attack damage growth increased to $2.7$ from $2.4$.
  - Base health reduced to 565 from 603.
  - Base movement speed reduced to 330 from 335.

### V13.17
- Stats
  - Health growth increased to 107 from 99.
- Blinding Assault
  - Base damage reduced to 20 / 40 / 60 / 80 / 100 from 20 / 45 / 70 / 95 / 120.
- Vault
  - Base damage reduced to 40 / 65 / 90 / 115 / 140 from 40 / 70 / 100 / 130 / 160.

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
- Heightened Senses
  - Bonus attack speed increased to 28 / 36 / 44 / 52 / 60% from 20 / 30 / 40 / 50 / 60%.
- Skystrike
  - AD ratio increased to 70% AD from 40% AD.

## Trivia

- Quinn is the first champion whose name starts with **Q** (back then the only remaining unused letter in the alphabet regarding champion names).
- Quinn's dance references Are You That Somebody by Aaliyah.
  - A side-by-side comparison can be seen here.
- Only Skystrike fires Quinn's crossbow despite it being her weapon of choice.
  - Quinn's bolts are Fletching with blue feathers which turn to a much lighter shade when hitting a target marked by Harrier.

---
*This page was automatically generated from League of Legends Wiki data.*