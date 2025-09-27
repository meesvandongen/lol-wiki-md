# Aurora

## Table of Contents
- [Basic Information](#basic-information)
- [Statistics](#statistics)
- [Abilities](#abilities)
- [Patch History](#patch-history)

## Basic Information

| Attribute | Value |
|-----------|------:|
| **Champion** | Aurora |
| **Title** | the Witch Between Worlds |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2024-07-17 |
| **Release Patch** | V14.14 |
| **Latest Changes** | V25.18 |
| **Roles** | Burst, Assassin |
| **Riot Positions** | Middle, Top |
| **External Positions** | Middle, Top |
| **Blue Essence** | 3150 |
| **Riot Points** | 975 |
| **Difficulty** | 2 |
| **Hero Type** | Mage |
| **Alt Type** | Assassin |
| **Adaptive Type** | Magic |
| **Damage** | 3 |
| **Toughness** | 1 |
| **Control** | 2 |
| **Mobility** | 3 |
| **Utility** | 1 |
| **Style** | 100 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $607.0$ | $+110.0$ |
| **Mana** | $475.0$ | $+30.0$ |
| **Health Regen** | $6.0$ | $+0.55$ |
| **Mana Regen** | $8.0$ | $+0.8$ |
| **Armor** | $23.0$ | $+4.5$ |
| **Magic Resist** | $32.0$ | $+1.3$ |
| **Attack Damage** | $53.0$ | $+3.0$ |
| **Attack Speed** | $0.668$ | |
| **Movement Speed** | $335.0$ | $+0.0$ |
| **Attack Range** | $550.0$ | $+0.0$ |
| **Base Attack Speed** | $0.668$ | |
| **Attack Speed Ratio** | $0.668$ | |
| **Bonus AS per Level** | $2.0\%$ | |
| **Missile Speed** | $1750$ units/second | |
| **Acquisition Radius** | $525$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $100$ units | |
| **Selection Height** | $135$ units | |
| **Critical Damage** | $175.0\%$ | |

## Abilities

### Passive: Spirit Abjuration

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Enemies, Self |
| **Damage Type** | magic |
| **Spell Shield** | False |
| **Spell Effects** | proc |
| **Parry** | True |

**INNATE:** **Aurora**’s damaging basic attacks and abilities apply a stack of *Spirit Abjuration* to enemies for 4 seconds, refreshing on subsequent applications and stacking up to 3 times. The third stack against a target consumes them all to deal **bonus** magic damage equal to 1% (+ $2.7$% per 100 AP) of the target's **maximum** health, capped at 100 / 270 against monsters.

Upon consuming the stacks from a champion, **Aurora** additionally exorcises them, freeing a Spirit from the target that follows her for the same duration. For each active Spirit, **Aurora** is healed for 3 to 20 (+ 2% AP) every second. Subsequent exorcisms free further Spirits and refresh all active Spirits. **Aurora** may have up to 4 Spirits at a time, for a maximum heal per tick of 3×4 to 20×4 (+ 8% AP).

**Notes:**

- The spirits grant vision of a small area around themselves.

---

### Q: Twofold Hex

| Attribute | Value |
|-----------|------:|
| **Range** | 900 (First cast missile range) units |
| **Cast Time** | $0.25$ (Active) / None (Recast) |
| **Effect Radius** | Global (Recast range for marked targets) |
| **Width** | 210 (First cast missile width) / er 90 (Recast missile width) units |
| **Speed** | 1600 (First cast missile speed) / 2000 (Recast missile speed) units/second |
| **Cost** | 60 mana |
| **Cooldown** | 9 / 8.5 / 8 / 7.5 / 7 seconds |
| **Cooldown Start** | on-cast |
| **Queue Time** | $0.05$ (First cast) / $0.05$ (Recast) seconds |
| **Targeting** | Direction / Auto |
| **Affects** | Enemies, Self |
| **Damage Type** | magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | True |

**ACTIVE:** **Aurora** fires a bolt of energy in the target direction that deals magic damage to enemies hit and marks them with a curse for $3.5$ seconds.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 45 / 70 / 95 / 120 / 145 (+ 40% AP) |

*Twofold Hex* can be recast after $0.1$ seconds while at least one target is marked, and does so automatically at the end of the mark's duration.

**RECAST:** **Aurora** expunges the curse from all marked targets, drawing the spiritual energy back to her as bolts that each deal magic damage to all enemies they pass through, reduced to 40% against minions and 50% against monsters and increased by 0 to 100*(1.5-1) for 11. Subsequent bolts against an enemy deal 20% damage.

| Attribute | Value |
|-----------|------:|
| **Minimum Magic Damage** | 45 / 70 / 95 / 120 / 145 (+ 40% AP) |
| **Maximum Magic Damage** | 67.5 / 105 / 142.5 / 180 / 217.5 (+ 60% AP) |

| Attribute | Value |
|-----------|------:|
| **Subsequent Bolt Minimum Magic Damage** | 9 / 14 / 19 / 24 / 29 (+ 8% AP) |
| **Subsequent Bolt Maximum Magic Damage** | 13.5 / 21 / 28.5 / 36 / 43.5 (+ 12% AP) |

**Notes:**

- *Twofold Hex* can also be recast for $0.66$ seconds after the $0.1$-second delay. If the recast is used during this time window, *Twofold Hex* will be buffered to recast automatically as soon as the bolt missile from the first cast has fizzled from reaching maximum range, even if no targets are marked. This automatic recast can also be buffered during *Twofold Hex*’s cast time.
- Spell shield can block the effects of the active and the recast.
- The marked targets do not need to be visible in order for **Aurora** to recast *Twofold Hex*.
  - They do however need to be targetable; if all marked targets are untargetable, *Twofold Hex*’s recast is disabled.
- Once **Aurora** has successfully recast *Twofold Hex*, she will not be able to recast it again for the duration regardless of any number of marked enemies still remaining.
  - If all enemies marked by *Twofold Hex* are untargetable when **Aurora** manually recasts *Twofold Hex*, the ability will be consumed but evidently without effect. It will however automatically recast again at the end of the marks' duration to expunge any remaining marks.

- This ability will cast from wherever the caster is at the end of the cast time.

---

### W: Across the Veil

| Attribute | Value |
|-----------|------:|
| **Range** | 300 (Regular dash distance) / 450 (Increased dash distance across terrain) units |
| **Cast Time** | none |
| **Speed** | 350 units/second |
| **Cost** | 80 mana |
| **Cooldown** | 22 / 21 / 20 / 19 / 18 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | direction |
| **Affects** | self |
| **Terrain Grace** | True |
| **Grounded** | True |
| **Knockdown** | True |

**ACTIVE:** **Aurora** hops in the target direction inside a portal to the Spirit Realm. Upon completing (See notes) the dash, she becomes invisible for a duration and gains *Realm Hopper* for 4 seconds.

| Attribute | Value |
|-----------|------:|
| **Invisibility Duration** | 1 / 1.15 / 1.3 / 1.45 / 1.6 seconds |

**REALM HOPPER:** **Aurora** becomes ghosted and gains **bonus** movement speed for the duration.

| Attribute | Value |
|-----------|------:|
| **Bonus Movement Speed** | 20 / 25 / 30 / 35 / 40% |

Scoring a takedown against an enemy champion within 3 seconds of damaging them will (ah) reset *Across the Veil*’s cooldown.

**Notes:**

- **Aurora** will **not** gain the stealth if her dash is interrupted.
  - She will however gain the stealth despite the dash being interrupted from colliding with *Between Worlds’* border.
    - Her dash is interrupted in this case from being overridden by the dash.
- **Aurora** is locked out of using all abilities, summoner spells, and item actives during the dash.
  - She cannot buffer these spells to cast after the lockout. This is in place to prevent her from breaking the stealth from certain spell casts.
    - This is the case even for spells that do not break stealth when cast.
- Using a basic attack breaks the stealth at the end of the attack windup.
- At rank 0, *Realm Hopper* will grant **Aurora** 15% **bonus** movement speed. This is only relevant for using *Between Worlds*.
- If the **total** duration of *Realm Hopper* gained from a new cast of *Across the Veil* or *Between Worlds* is longer than the **remaining** duration of the one gained from a previous cast, then re-gaining *Realm Hopper* will override the previous buff's duration.
  - If the **remaining** duration from a previous cast is shorter, then the same buff remains in place without refreshing.
    - In both cases, the buff cannot stack with itself.

---

### E: The Weirding

| Attribute | Value |
|-----------|------:|
| **Range** | 825 units |
| **Cast Time** | $0.35$ seconds |
| **Width** | 175 units |
| **Speed** | 150 + units/second |
| **Cost** | 80 mana |
| **Cooldown** | 15 / 14 / 13 / 12 / 11 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | False |
| **Grounded** | Special |
| **Knockdown** | True |

**ACTIVE:** **Aurora** blasts energy in a line in the target direction and recoils 250 units in the opposite direction. The blast deals magic damage to enemies hit and slows them by 80% for 1 second decaying after the first $0.15$ seconds.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 70 / 110 / 150 / 190 / 230 (+ 70% AP) |

**Notes:**

- **Aurora** is locked out of using all abilities, summoner spells, and item actives during the dash.
- While grounded or rooted, *The Weirding* can be cast but **Aurora** will not dash.
  - Similarly, if **Aurora** is affected by ground or root during the cast time, she will not dash afterwards.
    - This only applies to ground and root. Immobilizing effects will not prevent the dash afterwards.

- This ability will cast from wherever the caster is at the end of the cast time.

---

### R: Between Worlds

| Attribute | Value |
|-----------|------:|
| **Range** | 25 (Minimum initial dash distance) / 250 (Maximum initial dash distance) / 450 (Maximum extended initial dash distance through terrain) units |
| **Cast Time** | none |
| **Collision Radius** | 25 (Radius for border collision) units |
| **Effect Radius** | 700 (Rift radius) units |
| **Cost** | 100 mana |
| **Cooldown** | 140 / 130 / 120 / 110 / 100 seconds |
| **Cooldown Start** | post-effect |
| **Targeting** | Direction / Auto |
| **Affects** | Enemies, self |
| **Damage Type** | magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | False |
| **Terrain Grace** | True |
| **Grounded** | Special |
| **Knockdown** | False |

**ACTIVE:** **Aurora** leaps in the target direction with displacement immunity over $0.4$ seconds. She then gains *Realm Hopper* for a duration and unleashes a shockwave from 425 units (Estimated) in front of her, expanding over $0.75$ seconds to deal magic damage to enemies hit and slow them by 30% for 2 seconds.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 175 / 225 / 275 / 325 / 375 (+ 70% AP) |

| Attribute | Value |
|-----------|------:|
| ***Realm Hopper* Duration** | 3.5 / 3.875 / 4.25 / 4.625 / 5 seconds |

The shockwave leaves behind a circular to the Spirit Realm for a duration, forming borders that interact with **Aurora** and enemies. If **Aurora** collides (See notes) with the border, she dashes to the diametrically opposite of the over $0.4$ seconds, becoming untargetable and unable to act during the travel. If an enemy collides (See notes) with the border, they are slowed by 50% for a duration.

| Attribute | Value |
|-----------|------:|
| **Rift Duration** | 1.75 / 2.125 / 2.5 / 2.875 / 3.25 seconds |

| Attribute | Value |
|-----------|------:|
| **Slow Duration** | 1.5 / 1.625 / 1.75 / 1.875 / 2 seconds |

*Between Worlds* can be recast after 1 second of unleashing the shockwave while the is active.

**RECAST:** **Aurora** closes the prematurely.

*The closes immediately if **Aurora** dies, enters resurrection, leaves the area, or is not within the area upon creation.*

**Notes:**

- Collision with the border is essentially moving within range of it while being targetable, including via normal movement, dash, or displacement.
  - **Aurora** and enemies can collide with the border any number of times, as much as the duration permits.
    - Enemies can be applied the slow by colliding with the border only once after moving within collision radius of it. The slow may be applied again from collision only until after they have left the collision radius; it cannot be continuously applied while enemies remain within collision radius of the border.
  - Untargetable units cannot collide with the border, including **Aurora** herself.
- The in-game tooltip incorrectly lists the duration as $0.75$ seconds longer than it actually is.
  - It is seemingly including the time required for the shockwave to fully expand. However, the and its borders will only form after the fact, so the actual duration of the is shorter.
- *Between Worlds' * untargetability from the dash does not destroy in-flight projectiles.
- *Between Worlds' * first cast is disabled while grounded or rooted.
- **Aurora** can still dash from colliding with the border even if she is immobilized or grounded.
- Spell shield can block both the shockwave and border collision.
- A pair of portals signifying **Aurora**’s nearest possible entry and its corresponding exit is visible to her at all times.
- The following table refers for interactions while **Aurora** is dashing:
  - This lockout applies to both the initial dash and the dashes in the formed rift.
    - The lockout during the initial cast's dash persists for another $0.5$ seconds after the dash ends.
    - The lockout during the dash persists for another $0.15$ seconds after the dash ends.

| Aspect | State / Notes |
|--------|---------------|
| **Attacking** | Disabled |
| **Movement** | Disabled |
| **Abilities** | Disabled |
| **Items** | Disabled: All items |
| **Summoner Spells** | Usable: Barrier, Clarity, Cleanse, Exhaust, Ghost, Heal, Ignite, Smite; Disabled: Flash, Teleport, Recall, Hexflash |
| **Consumables** | Disabled |
| **Interrupted by** | Death |

---

## Patch History

### V25.18
- Spirit Abjuration
  - Mark target health ratio reduced to 1% of target's **maximum** health from $2.5$%.
  - Mark AP ratio increased to $2.7$% per 100 AP from 2% per 100 AP.
- Between Worlds
  - **Bug Fixes:** Tooltip now notes the proper *Realm Hopper* buff duration gained and no longer incorrectly notes that she gains it for the rift's duration only.

### V25.17
- Between Worlds
  - **Bug Fixes:** Tooltip now notes the correct duration.

### V25.15
- Spirit Abjuration
  - **UNDOCUMENTED / BUG FIX:** Tooltip now notes the mark consumption's monster damage cap.
    - The correct *current* numbers are noted, but the tooltip is not set up to automatically update if they change in the future.

### V25.14
- Between Worlds
  - **Bug Fixes:** Dash is no longer accidentally disabled if she casts *Between Worlds* close in time to Mark’s timer expiring.

### V25.12
- Stats
  - Base magic resistance increased to 32 from 30.
- Between Worlds
  - AP ratio increased to 70% AP from 60% AP.

### V25.10
- Twofold Hex
  - **Bug Fixes:** No longer fails to trigger Horizon Focus Hypershot when under valid conditions.
- The Weirding
  - **Bug Fixes:** No longer fails to trigger Horizon Focus Hypershot when under valid conditions.

### V25.09
- General
  - **Bug Fixes:** Realm Hopper's buff tooltip now has a proper name and description.

### V25.07
- Across the Veil
  - **Bug Fixes:** Corrected the capitalization of "move speed" to "Move Speed" in the tooltip.

### V25.05
- Spirit Abjuration
  - **Bug Fixes:** Stack application from basic attacks is now properly prevented by block.
- The Weirding
  - AP ratio reduced to 70% AP from 80% AP.
- Between Worlds
  - Exit slow strength reduced to 50% from 75%.

### V25.04
- General
  - **Bug Fixes:** Resolved several missing or incorrect SFX.

---
*This page was automatically generated from League of Legends Wiki data.*