# Rell

## Overview

- **Title:** Rell
- **Resource:** Unknown

## Stats

| Stat | Base | Growth |
|------|------|--------|

## Abilities

### Passive – Break the Mold

**INNATE:** **Rell's** basic attacks deal **bonus** magic damage on-hit equal to the sum of 5% of her **total** armor and 5% of her **total** magic resistance.

**INNATE:** **Rell's** basic attacks and abilities against non-minions apply a stack of _Break The Mold_ for 5 seconds, refreshing on subsequent hits and stacking up to 5 times. Each stack reduces the target's armor and magic resistance by 3% for a maximum of 15% reduction.

**Rell** gains **bonus** armor and **bonus** magic resistance equal to the sum resistances reduced from all afflicted enemies.

_Break the Mold_ will reduce the target's armor and magic resistance by a minimum of 1.5 to 3.

| Detail | Value |
|--------|------:|
| **Targeting** | Passive |
| **Affects** | Self, Enemies |
| **Damage Type** | magic |
| **Spell Effects** | proc |
| **Spell Shield** | false |
| **Parry** | true |

**Notes:**

- The attacks do not affect structures.
- The first stack of _Break The Mold_ that is applied to a target will snapshot the targets current armor and magic resistance. All subsequent stacks will reduce the targets resistances based on the snapshotted amount, even if the target's total resistances change. This snapshot lasts until the debuff expires.
  - For example, applying a stack of _Break The Mold_ to a champion with 100 armor and magic resistance will reduce their resistance by 3 and grant **Rell** the lost stats. If their resistances are then boosted to 500 before another stack is applied, the subsequent stack will only reduce their resistance by a further 3 despite 3% of 500 being 15.

### Q – Shattering Strike

| Attribute | Value |
|-----------|------:|
| **Range** | 520 (Forward Range) / -220 (Backward Range) |
| **Cast Time** | 0.40 |
| **Width** | 150 (Rectangle width) |
| **Cost** | 50 |
| **Cost Type** | mana |
| **Cooldown** | (+11 to 9% AP) |
| **Queue Time** | 0.25 |

**ACTIVE:** **Rell** thrusts her lance in the target direction, lunging forward 100 units and destroying the damage-mitigating shields of all enemies hit (excluding the shields of monsters) before dealing them magic damage and stunning them for 0.65 seconds.

| Detail | Value |
|--------|------:|
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | magic |
| **Spell Effects** | spellaoe |
| **Spell Shield** | true |
| **Projectile** | false |

**Notes:**

- **Rell** is locked out of moving, attacking, and casting any other ability for 0.35 seconds after _Shattering Strike_ is cast.

### E – Full Tilt

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 1200 |
| **Effect Radius** | 300 (Explosion radius) / 2200 (Empowered Ally detection range, estimated) / 1600 (Enemy Champion detection range, estimated) |
| **Cost** | 40 |
| **Cost Type** | Mana |
| **Cooldown** | (+14 to 10% AP) |
| **Queue Time** | 0.50 |

**ACTIVE:** **Rell** powers up herself and the target allied champion for 3 seconds, both gaining 10% **bonus** movement speed, increased to 25% while facing the empowered ally or a visible enemy champion.

Additionally, **Rell's** next basic attack or _Rell_ within 5 seconds creates an explosion around the target (See Notes) that deals **bonus** magic damage. The damage based on the target's health is capped at 150 to 300 against monsters and structures.

If cast without a valid target, or self-cast, _Full Tilt_ will automatically target the closest allied champion in range.

| Detail | Value |
|--------|------:|
| **Targeting** | Unit / Auto |
| **Affects** | Self, Allies, Enemies |
| **Damage Type** | Magic |
| **Out of Range** | If targeting an ally, walk in range of the target unit to cast |
| **Spell Effects** | spellaoe |
| **Spell Shield** | true |

**Notes:**

- If _Rell_ hits more than one enemy, the _Full Tilt_ explosion will be centered around the unit with the lowest Spawn ID (among enemy champions, this would also be the enemy first pick in Blind Pick mode).
  - "Spawn ID" is an unofficial abbreviation to describe the spawn order for all units at the beginning of games. <!--Blurb-->

### R – Magnet Storm

| Attribute | Value |
|-----------|------:|
| **Cast Time** | 0.25 |
| **Effect Radius** | 450 (Initial knockup radius) /  375 (Continous effect radius, center to edge) |
| **Inner Radius** | 225 (Pulls target into at least this radius, continuous kinetics does not affect targets already in this radius) |
| **Speed** | 300 (Kinematic attraction speed, in units per second) |
| **Cost** | 100 |
| **Cost Type** | Mana |
| **Cooldown** | (+120 to 80% AP) |
| **Cooldown Start** | on-cast |

**ACTIVE:** **Rell** erupts with magnetic fury, pulling nearby enemies inward (to a ring 225 units around Rell, but minimum 100 pull distance) and creating a gravitational field around her for the next 2 seconds that deals magic damage every 0.25 seconds to nearby enemies and drags them towards her.

| Detail | Value |
|--------|------:|
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Effects** | aoedot |
| **Spell Shield** | special |

**Notes:**

- _Magnet Storm_ will **not** drag units that are:
  - Dashing.
  - Being displaced (e.g. by airborne effects)
  - Attached.
  - Displacement immune.
  - Immune to crowd control.
- Spell shield will block the initial pull but not the dragging effect.
- _Magnet Storm_ cannot be cast again while it is active.

## Trivia

- Rell's lance is featured in the promotional_art for Preseason 2021 Mythic Forge as a reveal teaser.

<!-- Raw excerpt (first 20 lines) -->

<details><summary>Raw excerpt</summary>

```wikitext

== Abilities ==

==Champion skins==
:_This article section only contains champion skins. For all associated collection items, see Rell_(Collection)._

==Patch history==

==Trivia==
```
</details>
