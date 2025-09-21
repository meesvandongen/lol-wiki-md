# Annie

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
| **Champion** | Annie |
| **Title** | the Dark Child |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2009-02-21 |
| **Release Patch** | Alpha Week 2 |
| **Roles** | Burst |
| **Riot Positions** | Middle |
| **External Positions** | Middle |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $560.0$ | $+96.0$ | $2192.0$ |
| **Mana** | $418.0$ | $+25.0$ | $843.0$ |
| **Health Regen** | $5.5$ | $+0.55$ | $14.9$ |
| **Mana Regen** | $8.0$ | $+0.8$ | $21.6$ |
| **Armor** | $23.0$ | $+4.0$ | $91.0$ |
| **Magic Resist** | $30.0$ | $+1.3$ | $52.1$ |
| **Attack Damage** | $50.0$ | $+2.65$ | $95.0$ |
| **Attack Speed** | $0.610$ | $+1.4\%$ | $0.751$ |
| **Movement Speed** | $335.0$ | $+0.0$ | $335.0$ |
| **Attack Range** | $625.0$ | $+0.0$ | $625.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.61$ |
| **Attack Speed Ratio** | $0.625$ |
| **Bonus AS per Level** | $1.4\%$ |
| **Missile Speed** | $1500 units/second$ |
| **Acquisition Radius** | $625 units$ |
| **Gameplay Radius** | $55 units$ |
| **Pathing Radius** | $30 units$ |
| **Selection Radius** | $100 units$ |
| **Selection Height** | $85 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Pyromania

**Innate:** **Annie**’s spell-casts generate stacks. At maximum stacks, her next offensive spell will consume them to stun the target.

**Innate - Pyromania:** **Annie** generates a stack of *Pyromania* whenever she hits an enemy with **Disintegrate** or casts her other abilities, stacking up to 4 times, at which she gains *Energized*. **Energized:** **Annie** empowers her next cast of **Disintegrate**, **Incinerate**, or **Summon: Tibbers** to consume all *Pyromania* stacks to stun enemies hit for 1.25@1; 1.5@6; 1.75@11 seconds. **Annie** gains maximum stacks of *Pyromania* when the game starts and upon respawning. She will lose *Energized* and all *Pyromania* stacks upon death.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Self, Enemies |
| **Spell Shield** | True |

**Notes:**

- **Annie** does not lose any stacks upon entering or exiting resurrection.
- Stacks are gained even if the ability is blocked by spell shield.
- 'Pyromania's* current stacks are represented by a counter under *'Annie's' health bar, visible to all players. It lights up when the empowered effect is available.

---

### Q: Disintegrate

**Active:** **Annie** hurls a fireball at the target enemy that deals magic damage.

*If this kills the target, the *cooldown* is reduced and the is refunded.*

**Active:** **Annie** hurls a fireball at the target enemy that deals magic damage. If this kills the target, 'Disintegrate's *cooldown* is reduced by 50% and its is refunded.

| Attribute | Value |
|-----------|-------|
| **Range** | 625 units |
| **Cooldown** | 4 seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $60-80$ Mana |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Speed** | 1400 units/second |
| **Spell Shield** | True |
| **Spell Effects** | spell |
| **Projectile** | True |

**Scaling:**
- **Magic Damage:** $80-240$ (+ 80% AP)

**Notes:**

- *Disintegrate* will also grant the cooldown reduction and mana cost refund if the target is dead upon the missile's arrival.

---

### W: Incinerate

**Active:** **Annie** casts a blazing cone of fire, dealing magic damage to enemies hit.

**Active:** **Annie** releases fire in a cone in the target direction, dealing magic damage to enemies hit.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | 7 seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $70-90$ Mana |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | 600 units |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | False |

**Scaling:**
- **Magic Damage:** $70-230$ (+ 80% AP)

**Notes:**

- *Incinerate* can hit targets behind **Annie**, provided their radius intersects with the cone hitbox. Effect at cast time end

---

### E: Molten Shield

**Active:** **Annie** grants herself or an allied champion—and Tibbers—a shield with a burst of ms.

*While the shield holds, enemies who basic attack it take magic damage.*

**Active:** **Annie** grants herself or the target allied champion and Tibbers a shield for 3 seconds and 20 to 50 **bonus movement speed** that decays over $1.5$ seconds. While *Molten Shield* is active, enemies that deal damage to it take magic damage. This may only occur once per enemy per cast for each active *Molten Shield*.

| Attribute | Value |
|-----------|-------|
| **Range** | 800 units |
| **Cooldown** | $12-10$ seconds |
| **Cast Time** | none |
| **Cost** | $60-80$ Mana |
| **Targeting** | Unit / Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | Magic |
| **Spell Effects** | aoe |

**Scaling:**
- **Shield Strength:** $60-200$ (+ 40% AP)
- **Magic Damage:** $25-65$ (+ 40% AP)

**Notes:**

- *Molten Shield* casts on ally if targeted or if very close to targeting them (within a range of 225); otherwise self casts.
- *Molten Shield* does not deal damage to turret when attacked by them.
- *Molten Shield* has a forgiveness radius of 175 units.
- Attacks that are dodged or blind against the shielded target will not cause the shield to deal damage, while block attacks still deal damage to the attacker.
- The reaction damage does not trigger from Noxious Trap and Jack in the Box.

---

### R: Summon: Tibbers

**Active:** **Annie** summons her bear Tibbers, dealing magic damage to enemies in the area.

*Tibbers deals magic damage with his attacks and also burns nearby enemies. Re-cast to direct him.*

**Passive:** **Annie** gains *magic penetration*. **Active:** **Annie** summons Tibbers to the target location in a burst of flame, dealing magic damage to enemies near him. *Summon: Tibbers* can be recast at any time while Tibbers is alive. Tibbers then remains on the field as a controllable pet for up to 45 seconds. **Recast:** **Annie** directs Tibbers to the target location. 'Summon: Tibber's recast can be used while affected by cast-inhibiting crowd control. See [Pets](#Pets) for more details about Tibbers.'

| Attribute | Value |
|-----------|-------|
| **Range** | 600 units |
| **Cooldown** | $130-100$ seconds |
| **Cast Time** | $0.25$ / None |
| **Cost** | 100 Mana |
| **Targeting** | Location |
| **Affects** | Enemies / Tibbers |
| **Damage Type** | magic |
| **Effect Radius** | 250 / cr 350 units |
| **Spell Shield** | True |
| **Spell Effects** | Special |

**Scaling:**
- **Magic Penetration:** $10-20 3$%
- **Initial Magic Damage:** $150-400$ (+ 75% AP)

**Notes:**

- Burst of flame deals area damage and 'Tibbers'basic attacks deal pet damage.
- Tibbers will blink back next to **Annie** if he gets too far away.
- Recasting *Summon: Tibbers* does not interrupt channel.

---

## Patch History

### V25.18
- *Incinerate*
  - Base damage reduced to $70-230$ from $70-250$.
- *Summon: Tibbers*
  - Magic penetration reduced to $10-20 3$% from $15-20 3$%.

### V25.11
- Stats
  - Base armor increased to 23 from 19.
  - Armor growth reduced to 4 from $4.7$.
- *Incinerate*
  - Base damage reduced to $70-250$ from $70-270$.
  - AP ratio reduced to 80% AP from 85% AP.

### V25.09
- *Disintegrate*
  - Base damage increased to $80-240$ from $70-210$.
- *Incinerate*
  - Mana cost reduced to $70-90$ from $90-110$.
- *Summon: Tibbers*
  - Passive magic penetration increased to $15-20 3$% from $10-15 3$%.

### V25.08
- Stats
  - Health growth reduced to 96 from 102.
- *Disintegrate*
  - AP ratio increased to 80% AP from 75% AP.
- *Incinerate*
  - Base damage increased to $70-270$ from $70-250$.
  - Cooldown reduced to 7 seconds from 8.
- *Summon: Tibbers*
  - **New Effect:** Now passively grants $10-15 3$% magic penetration.
  - Tibbers attack damage reduced to $30-60 3$ from $50-100 3$.
  - Tibbers attack damage AP ratio reduced to 10% AP from 15% AP.
  - Tibbers aura damage per second reduced to $8-16 3$ from $20-40 3$.
  - Tibbers aura AP ratio per second reduced to 4% AP from 12% AP.

### V25.04
- *Summon: Tibbers*
  - **Bug Fixes:** Tibbers attacks against champions no longer trigger minion call-for-help against the summoner (if in valid range).
- *Disintegrate*
  - **Bug Fixes:** Now properly gains the refund effects if her other damaging spells kill the target before the fireball has reached them.
- *Disintegrate*
  - **New Effect:** Now refunds the mana cost and 50% of the cooldown if the target dies while the projectile is in flight.

### V14.20
- *Molten Shield*
  - Targeting forgiveness is now handled at the engine-level rather than the script-level.
  - Forgiveness radius reduced to 175 units from 225.

### V14.4
- *Molten Shield*
  - **Bug Fixes:** No longer reflects the damage to an allied **Nami** that casted *Tidecaller's Blessing* on her right before *Tibbers* despawned.

### V13.24
- Annie
  - **Bug Fixes:** Fire VFX in her idle animation no longer disappears momentarily.

### V13.22
- Stats
  - Base attack speed increased to $0.61$ from $0.579$.
  - Attack speed ratio increased to $0.625$ from $0.579$.
  - Basic attack missile speed increased to 1500 from 1200.

### V13.15
- *Summon: Tibbers*
  - Tibbers base resistances changed to 30–90@6–18 from $30-90 3$.
  - **Removed:*** Tibbers' resistances no longer scale with 5% AP.
  - Tibbers base health changed to 1150–3500@6–18 from $1300/2200/3100$.
  - Tibbers health AP ratio reduced to 50% AP from 75% AP.
  - **Bug Fixes:** Pet can now consistently be commanded to attack inhibitors and the Nexus via their controller spells or automatic attack priorities.
    - 'This was fixed but not documented in

## Trivia

- Annie was one of the first champions designed (the others being **Lee Sin**, **Singed**, **Sion**, **Sivir**, and **Twisted Fate**).
- Annie was the first champion to have 12 skins including Classic.
- Annie is the second champion to have her ability icons remade with a Visual Update (the first being **Katarina**).
- Annie is the first fire-themed champion (the second being **Brand**).
- In the now-removed official League of Legends forums, the original icon of *Molten Shield* was used to represent the "PVP.net Discussion" section.

---
*This page was automatically generated from League of Legends Wiki data.*