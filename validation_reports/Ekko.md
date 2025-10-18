# Ekko

## Overview

- **Title:** Ekko
- **Resource:** Unknown

## Stats

| Stat | Base | Growth |
|------|------|--------|

## Abilities

### Passive – Z-Drive Resonance

| Attribute | Value |
|-----------|------:|
| **On-Target CD Static** | 5 (Begins after the stacks are consumed) |

**INNATE - RESONANCE:** **Ekko's** basic attacks on-hit and damaging abilities apply a stack of _Resonance_ to enemies hit for 4 seconds, refreshing on subsequent hits and stacking up to 3 times. The third stack consumes them all to deal 30 to 80 by 10; 80 to 140 / color=magic damage (+ 90% AP) **bonus** magic damage. _Z-Drive Resonance_ deals 300% damage against monsters.

_Resonance_ cannot affect the same target more than once every few seconds.

**INNATE - STOLEN TIME:** Triggering _Resonance_ against a champion grants **Ekko** key= / 50 to 80 for 4 / 1 to 16 / color=movement speed **bonus** movement speed for 2 to 3 for 3 / 1 to 11 seconds.

| Detail | Value |
|--------|------:|
| **Targeting** | Passive |
| **Affects** | Enemies / Self |
| **Damage Type** | Magic |
| **Spell Effects** | Single target |
| **Spell Shield** | True |
| **Parry** | true |

**Notes:**

- **Ekko's** basic attacks have different animations based on how many stacks his target has. His attack pattern is as follows:
  - Downwards (first stack) Ã¢ÂÂ Upwards (second stack) Ã¢ÂÂ Sideways (third stack to consume all stacks)
    - If the target has already been affected by _Resonance_, **Ekko's** basic attack animations will kick or attack the target sideways.
- _Resonance_ stacks will not be applied nor consumed if the basic attack is dodged or blocked or if the attack misses. <!-- Blurb -->

### Q – Timewinder

| Attribute | Value |
|-----------|------:|
| **Cast Time** | 0.25 |
| **Target Range** | 1100 |
| **Effect Radius** | 160 Ã¢ÂÂ 210 (Temporal Sickness expansion) |
| **Width** | 120 (Outward) / 200 (Returning) |
| **Speed** | 1650 (Outward) / 200 (Temporal Sickness (Estimated)) / 2300 (Returning) |
| **Cost** | (+50 to 90% AP) |
| **Cost Type** | Mana |
| **Cooldown** | (+9 to 7% AP) |

**ACTIVE:** **Ekko** throws a temporal grenade in the target direction that deals magic damage to enemies hit.

At 700 units or upon hitting an enemy champion, the grenade slows down for 1.75 seconds to gradually expand into a _Temporal Sickness_ field that slows nearby enemies, travelling for another 190 units.

Afterwards, the grenade contracts and homes back to **Ekko** at an increased speed, dealing magic damage to enemies hit.

_Enemies can be hit only once per pass._

| Detail | Value |
|--------|------:|
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Effects** | spellaoe |
| **Spell Shield** | Special |
| **Projectile** | True |

**Notes:**

- Spell shield will block only a single instance of damage. <!-- Blurb -->

### W – false

| Attribute | Value |
|-----------|------:|
| **Cast Time** | 0.25 |
| **Target Range** | 1600 |
| **Effect Radius** | 375 |
| **Cost** | (+30 to 50% AP) |
| **Cost Type** | Mana |
| **Cooldown** | (+22 to 14% AP) |
| **Cooldown Start** | on-cast |

**PASSIVE:** **Ekko's** basic attacks deal **bonus** magic damage equal to 3% (+ 3% per 100 AP) of the target's **missing** health against enemies below 30% of their **maximum** health. The damage has a minimum threshold of 15 and is capped at 150 against minions and monsters.

**ACTIVE:** **Ekko** creates an afterimage of himself that, after 2 seconds, bats a device to the target location and grants sight of the area for 2.50 seconds. After travelling over 1.25 seconds, the device expands into a chronosphere that lasts for 1.50 seconds and which slows enemies within by 40%.

If **Ekko** enters the sphere before it expires, it detonates to grant him a shield for 2 seconds and stun enemies within for 2.25 seconds.

_Enemies can see the indicator for Parallel Convergence 2 seconds after casting._

| Detail | Value |
|--------|------:|
| **Targeting** | Location |
| **Damage Type** | magic |
| **Out of Range** | walk to location |
| **Spell Effects** | proc |
| **Spell Shield** | True |
| **Parry** | true |
| **Projectile** | False |

**Notes:**

- The passive will proc with the attack that triggers the final stack of Ekko if it reduces a target's health below the 30% threshold.
- **Ekko** can detonate the expansion even while untargetable (i.e. Ekko dash), but not if he is resurrecting.
- _Parallel Convergence_ will continue to slow enemies even if its expansion is detonated. <!-- Blurb -->

### E – Phase Dive

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 325 (Dash distance) / 550 (Maximum increased dash distance across terrain) |
| **Cost** | (+40 to 60% AP) |
| **Cost Type** | Mana |
| **Cooldown** | (+9 to 7% AP) |
| **Cooldown Start** | post-effect |

**ACTIVE:** **Ekko** dashes in the target direction, then empowers his next basic attack within 3 seconds to have a 0.25-second cast time, gain 300 **bonus** range, cause him to blink within 125 (Estimated) range of the target, and deal **bonus** magic damage.

_Phase Dive resets **Ekko's** basic attack timer. **Ekko** can cast any of his abilities during the dash._

| Detail | Value |
|--------|------:|
| **Targeting** | Direction |
| **Affects** | Self |
| **Damage Type** | Magic |
| **Terrain Grace** | true |
| **Spell Effects** | Spell |
| **Spell Shield** | true |
| **Parry** | True |
| **Grounded** | True |
| **Knockdown** | True |

**Notes:**

- The dash distance can be extended to up-to 550 units when targeting across terrain. It will only extend for the distance needed to cross the wall.
- While grounded or rooted, **Ekko** loses the **bonus** attack range from _Phase Dive_.
- _Phase Drive's_ attack can miss and be blocked and dodged, consuming the basic attack without dealing any damage.
- If the target becomes untargetable, dies, or is too far away during the empowered attack's cast time, it is cancelled but not consumed. <!-- Blurb -->

### R – false

| Attribute | Value |
|-----------|------:|
| **Cast Time** | 0.50 |
| **Effect Radius** | 375 (Damage radius) / Global (Activation radius) |
| **Cost** | 100 |
| **Cost Type** | Mana |
| **Cooldown** | (+110 to 50% AP) |

**PASSIVE:** Upon learning _Chronobreak_ or if its **current** cooldown is lower than 4 seconds, **Ekko** reveals a time-delayed _afterimage_ of himself that constantly tracks where he was 4 seconds ago.

**ACTIVE:** **Ekko** enters stasis at the start of the cast time, and afterwards heals himself and dashes to his _afterimage's_ location at the time of cast over 0.50 seconds. Upon arrival, the stasis ends and he creates an explosion that deals magic damage to nearby enemies.

**_Ekko** is immune to all displacements during Chronobreak_.

| Detail | Value |
|--------|------:|
| **Targeting** | Auto |
| **Affects** | Enemies / Self |
| **Damage Type** | Magic |
| **Spell Effects** | spellaoe |
| **Spell Shield** | True |
| **Grounded** | True |
| **Knockdown** | False |

**Notes:**

- **Ekko** will attempt to basic attack the closest target after appearing at the cast location, but can do so quicker if he manually attack commands after the dash, except if Ekko is primed.
- The _afterimage's_ location will explode even if **Ekko's** dash is interrupted.
- A _link_ can also be seen between the _afterimage_ and **Ekko** that traces along his path.
  - The _link_ follows the same visibility rules as the _afterimage_.
  - Upon activation, **Ekko** travels full distance through the _link_, and thus will detonate Ekko that the _link_ passes through.
- icononly=true Quicksilver incurs a 1-second cooldown upon casting _Chronobreak_.

## Trivia

- The name _Ekko_ is a play on the English word _echo_.
  - Besides the proverbial-and-literal 'Ekkos of the past' his name, having two Ks (KK) references the Rewind symbol Ã¢ÂÂª.
  - His name could also be a reference to John 'Mikky Ekko' Stephen Sudduth and his first album 'Time' although this has never been confirmed.
- _Chronobreak_ gives its namesake to the proprietary [https://technology.riotgames.com/news/determinism-league-legends-introduction time manipulation tool] developed by Riot_Games for use in professional matches.

<!-- Raw excerpt (first 20 lines) -->

<details><summary>Raw excerpt</summary>

```wikitext

== Abilities ==

==Champion skins==
:_This article section only contains champion skins. For all associated collection items, see Ekko_(Collection)._

==Patch history==

==Trivia==

```
</details>
