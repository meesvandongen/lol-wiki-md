# Aurora

## Overview

- **Title:** Aurora
- **Resource:** Mana

## Stats

| Stat | Base | Growth |
|------|------|--------|
| Armor | 23 | 4.5 |
| Attack Damage | 53 | 3 |
| Attack Speed | 0.668 | 2 |
| HP | 607 | 110 |
| HP Regen | 6 | 0.55 |
| MP | 475 | 30 |
| MP Regen | 8 | 0.8 |
| Magic Resist | 32 | 1.3 |
| Move Speed | 335 | 0 |
| Range | 550 | 0 |

## Advanced Stats

| Metric | Value |
|--------|-------|
| Acquisition Radius | 525 |
| Attack Delay Offset (s) | -0.0995 |
| Attack Speed Ratio | 0.668 |
| Missile Speed | 1750 |
| Pathing Radius | 35 |
| Selection Height | 135 |
| Selection Radius | 100 |

## Abilities

### Passive – Aurora Spirit Abjuration

**INNATE:** **Aurora's** damaging basic attacks and abilities apply a stack of _Spirit Abjuration_ to enemies for 4 seconds, refreshing on subsequent applications and stacking up to 3 times. The third stack against a target consumes them all to deal **bonus** magic damage equal to 1% (+ 2.70% per 100 AP) of the target's **maximum** health, capped at 90+10*x against monsters.

Upon consuming the stacks from a champion, **Aurora** additionally exorcises them, freeing a Spirit from the target that follows her for the same duration. For each active Spirit, **Aurora** is healed for 3 to 20 (+ 2% AP) every second. Subsequent exorcisms free further Spirits and refresh all active Spirits. **Aurora** may have up to 4 Spirits at a time, for a maximum heal per tick of 3*4 to 20*4 (+ (+2*4% AP)% AP).

| Detail | Value |
|--------|------:|
| **Targeting** | Passive |
| **Affects** | Enemies, Self |
| **Damage Type** | magic |
| **Spell Effects** | proc |
| **Spell Shield** | false |
| **Parry** | true |

**Notes:**

- The spirits grant vision of a small area around themselves. <!-- Blurb -->

### Q – Aurora Twofold Hex

| Attribute | Value |
|-----------|------:|
| **Range** | 900 (First cast missile range) |
| **Cast Time** | 0.25 (Active) / None (Recast) |
| **Effect Radius** | Global (Recast range for marked targets) |
| **Width** | 210 (First cast missile width) /  90 (Recast missile width) |
| **Speed** | 1600 (First cast missile speed) / 2000 (Recast missile speed) |
| **Cost** | 60 |
| **Cost Type** | mana |
| **Cooldown** | (+9 to 7% AP) |
| **Cooldown Start** | on-cast |
| **Queue Time** | 0.05 (First cast) / 0.05 (Recast) |

**ACTIVE:** **Aurora** fires a bolt of energy in the target direction that deals magic damage to enemies hit and marks them with a curse for 3.50 seconds.

_Twofold Hex_ can be recast after 0.10 seconds while at least one target is marked, and does so automatically at the end of the mark's duration.

**RECAST:** **Aurora** expunges the curse from all marked targets, drawing the spiritual energy back to her as bolts that each deal magic damage to all enemies they pass through, reduced to 40% against minions and 50% against monsters and increased by key= / type=target's **missing** health / 0 to 100*(1.5-1) for 11 / 0 to 100 / key1= / formula=0.5% per 1% of target's **missing** health / color=health. Subsequent bolts against an enemy deal (+0.2*100% AP)% damage.

| Detail | Value |
|--------|------:|
| **Targeting** | [Direction](./Targeting.md#Direction) / [Auto](./Targeting.md#Auto-targeted) |
| **Affects** | Enemies, Self |
| **Damage Type** | magic |
| **Spell Effects** | spellaoe |
| **Spell Shield** | true |
| **Projectile** | true |

**Notes:**

- _Twofold Hex_ can also be recast for 0.66 seconds after the 0.10-second delay. If the recast is used during this time window, _Twofold Hex_ will be buffered to recast automatically as soon as the bolt missile from the first cast has fizzled from reaching maximum range, even if no targets are marked. This automatic recast can also be buffered during _Twofold Hex's_ cast time.
- Spell shield can block the effects of the active and the recast.
- The marked targets do not need to be visible in order for **Aurora** to recast _Twofold Hex_.
  - They do however need to be targetable; if all marked targets are untargetable, _Twofold Hex's_ recast is disabled.
- Once **Aurora** has successfully recast _Twofold Hex_, she will not be able to recast it again for the duration regardless of any number of marked enemies still remaining.
  - If all enemies marked by _Twofold Hex_ are untargetable when **Aurora** manually recasts _Twofold Hex_, the ability will be consumed but evidently without effect. It will however automatically recast again at the end of the marks' duration to expunge any remaining marks. <!-- Blurb -->

### W – Aurora Across the Veil

| Attribute | Value |
|-----------|------:|
| **Range** | 300 (Regular dash distance) / 450 (Increased dash distance across terrain) |
| **Cast Time** | none |
| **Speed** | 350 |
| **Cost** | 80 |
| **Cost Type** | mana |
| **Cooldown** | (+22 to 18% AP) |
| **Cooldown Start** | on-cast |

**ACTIVE:** **Aurora** hops in the target direction inside a portal to the Spirit Realm. Upon completing (See notes) the dash, she becomes invisible for a duration and gains _Realm Hopper_ for 4 seconds.

**REALM HOPPER:** **Aurora** becomes ghosted and gains **bonus** movement speed for the duration.

Scoring a takedown against an enemy champion within 3 seconds of damaging them will reset _Across the Veil's_ cooldown.

| Detail | Value |
|--------|------:|
| **Targeting** | direction |
| **Affects** | self |
| **Terrain Grace** | true |
| **Grounded** | true |
| **Knockdown** | true |

**Notes:**

- **Aurora** will **not** gain the stealth if her dash is interrupted.
  - She will however gain the stealth despite the dash being interrupted from colliding with _Aurora_ border.
    - Her dash is interrupted in this case from being overridden by the rift dash.
- **Aurora** is locked out of using all abilities, summoner spells, and item actives during the dash.
  - She cannot buffer these spells to cast after the lockout. This is in place to prevent her from breaking the stealth from certain spell casts.
    - This is the case even for spells that do not break stealth when cast.
- Using a basic attack breaks the stealth at the end of the attack windup.
- At rank 0, _Realm Hopper_ will grant **Aurora** 15% **bonus** movement speed. This is only relevant for using _Aurora_.
- If the **total** duration of _Realm Hopper_ gained from a new cast of _Across the Veil_ or _Aurora_ is longer than the **remaining** duration of the one gained from a previous cast, then re-gaining _Realm Hopper_ will override the previous buff's duration.
  - If the **remaining** duration from a previous cast is shorter, then the same buff remains in place without refreshing.
    - In both cases, the buff cannot stack with itself. <!-- Blurb -->

### E – Aurora The Weirding

| Attribute | Value |
|-----------|------:|
| **Range** | 825 |
| **Cast Time** | 0.35 |
| **Width** | 175 |
| **Speed** | 150 + 200% movement speed (Dash speed) |
| **Cost** | 80 |
| **Cost Type** | mana |
| **Cooldown** | (+15 to 11% AP) |

**ACTIVE:** **Aurora** blasts energy in a line in the target direction and recoils 250 units in the opposite direction. The blast deals magic damage to enemies hit and slows them by 80% for 1 second decaying after the first 0.15 seconds.

| Detail | Value |
|--------|------:|
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | magic |
| **Spell Effects** | spellaoe |
| **Spell Shield** | true |
| **Projectile** | false |
| **Grounded** | special |
| **Knockdown** | true |

**Notes:**

- **Aurora** is locked out of using all abilities, summoner spells, and item actives during the dash.
- While grounded or rooted, _The Weirding_ can be cast but **Aurora** will not dash.
  - Similarly, if **Aurora** is affected by ground or root during the cast time, she will not dash afterwards.
    - This only applies to ground and root. Immobilizing effects will not prevent the dash afterwards. <!-- Blurb -->

### R – Aurora Between Worlds

| Attribute | Value |
|-----------|------:|
| **Range** | 25 (Minimum initial dash distance) / 250 (Maximum initial dash distance) / 450 (Maximum extended initial dash distance through terrain) |
| **Cast Time** | none |
| **Collision Radius** | 25 (Radius for border collision) |
| **Effect Radius** | 700 (Rift radius) |
| **Cost** | 100 |
| **Cost Type** | mana |
| **Cooldown** | (+140 to 100% AP) |
| **Cooldown Start** | post-effect |

**ACTIVE:** **Aurora** leaps in the target direction with displacement immunity over 0.40 seconds. She then gains _Aurora_ for a duration and unleashes a shockwave from 425 units (Estimated) in front of her, expanding over 0.75 seconds to deal magic damage to enemies hit and slow them by 30% for 2 seconds.

The shockwave leaves behind a circular rift to the Spirit Realm for a duration, forming borders that interact with **Aurora** and enemies. If **Aurora** collides (See notes) with the border, she dashes to the diametrically opposite side of the rift over 0.40 seconds, becoming untargetable and unable to act during the travel. If an enemy collides (See notes) with the border, they are slowed by 50% for a duration.

_Between Worlds_ can be recast after 1 second of unleashing the shockwave while the rift is active.

**RECAST:** **Aurora** closes the rift prematurely.

_The rift closes immediately if **Aurora** dies, enters resurrection, leaves the area, or is not within the area upon creation._

| Detail | Value |
|--------|------:|
| **Targeting** | [Direction](./Direction-targeted.md) / [Auto](./Auto-targeted.md) |
| **Affects** | Enemies, self |
| **Damage Type** | magic |
| **Terrain Grace** | true |
| **Spell Effects** | spellaoe |
| **Spell Shield** | true |
| **Projectile** | false |
| **Grounded** | special |
| **Knockdown** | false |

**Notes:**

- Collision with the border is essentially moving within range of it while being targetable, including via normal movement, dash, or displacement.
  - **Aurora** and enemies can collide with the border any number of times, as much as the duration permits.
    - Enemies can be applied the slow by colliding with the border only once after moving within collision radius of it. The slow may be applied again from collision only until after they have left the collision radius; it cannot be continuously applied while enemies remain within collision radius of the border.
  - Untargetable units cannot collide with the border, including **Aurora** herself.
- The in-game tooltip incorrectly lists the rift duration as 0.75 seconds longer than it actually is.
  - It is seemingly including the time required for the shockwave to fully expand. However, the rift and its borders will only form after the fact, so the actual duration of the rift is shorter.
- _Between Worlds' _ untargetability from the rift dash does not destroy in-flight projectiles.
- _Between Worlds' _ first cast is disabled while grounded or rooted.
- **Aurora** can still dash from colliding with the border even if she is immobilized or grounded.
- Spell shield can block both the shockwave and border collision.
- A pair of portals signifying **Aurora's** nearest possible entry and its corresponding exit is visible to her at all times.
- The following table refers for interactions while **Aurora** is dashing:
  - This lockout applies to both the initial dash and the dashes in the formed rift.
    - The lockout during the initial cast's dash persists for another 0.50 seconds after the dash ends.
    - The lockout during the rift dash persists for another 0.15 seconds after the dash ends. (cast=false)

## Trivia

<!-- Raw excerpt (first 20 lines) -->

<details><summary>Raw excerpt</summary>

```wikitext

== Abilities ==

==Champion skins==
:_This article section only contains champion skins. For all associated collection items, see [Aurora (Collection)](./Aurora_Cosmetics.md)._

==Trivia==
* 

```
</details>
