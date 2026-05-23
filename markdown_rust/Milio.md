# Milio

## Overview

- **Title:** Milio
- **Resource:** Mana

## Stats

| Stat | Base | Growth |
|------|------|--------|
| Armor | 26 | 4.6 |
| Attack Damage | 48 | 3.2 |
| Attack Speed | 0.625 | 3 |
| HP | 560 | 88 |
| HP Regen | 5 | 0.5 |
| MP | 365 | 43 |
| MP Regen | 11.5 | 0.4 |
| Magic Resist | 30 | 1.3 |
| Move Speed | 330 | 0 |
| Range | 525 | 0 |

## Advanced Stats

| Metric | Value |
|--------|-------|
| Acquisition Radius | 550 |
| Attack Cast Time (s) | 0.27 |
| Attack Speed Ratio | 0.625 |
| Base Attack Time (s) | 1.575 |
| Gameplay Radius | 55 |
| Missile Speed | 1900 |
| Pathing Radius | 30 |
| Selection Height | 200 |
| Selection Radius | 100 |
| Windup % | 17.1% |

## Abilities

### Passive – Fired Up!

**INNATE:** **Milio's** [ability](./champion_ability.md) hits on himself and allied champions grant an enchantment for 4 seconds, which causes the next basic attack or ability hit against enemies to deal 7;11;15 / 1;6;9 / key= of enchanted target's AD  and apply a burn that dealsÃ£ÂÂ 10 to 50 (+ 20% of **Milio's** AP) magic damage over 1.50 seconds. Ã¢ÂÂ· 10/6 to 50/6 / round=2 (+ (+20/6/round=2% AP)% of **Milio's** AP) magic damage every 0.25 seconds over 1.50 seconds. Ã£ÂÂ

| Detail | Value |
|--------|------:|
| **Targeting** | Passive |
| **Affects** | Allies, Enemies |
| **Damage Type** | Magic |
| **Spell Effects** | periodic |
| **Spell Shield** | Special |
| **Parry** | Unknown |

**Notes:**

- Both the burst and the burn respect enchantment redirection.
- Subsequent applications of _Fired Up!_ only refresh the duration.
- Spell shield will not block the effect if applied by a basic attack. <!--Blurb-->

### Q – Ultra Mega Fire Kick

| Attribute | Value |
|-----------|------:|
| **Range** | 1200 |
| **Cast Time** | none |
| **Effect Radius** | 100 (Sight radius of missile) /  250 (Champion hit reveal and explosion radius) / 275 (Minion hit reveal and explosion radius) / 190 (Champion knock back distance) |
| **Width** | 120 |
| **Speed** | 1200 |
| **Cost** | (+50 to 70% AP) |
| **Cost Type** | Mana |
| **Cooldown** | 10 |

**ACTIVE:** After a 0.25-second delay, **Milio** kicks a fireball in the target direction that grants sight of its path and knocks back and stuns the first enemy it hits over 1 second.

Upon collision, the ball bounces once in the same (parallel to original cast direction) direction from the target's location, granting sight of the area before exploding in the same radius after a brief delay, dealing magic damage to enemies hit and slowing them for 1.50 seconds.

If the primary target is a non-champion, the ball knocks back further and creates a larger explosion. _Ultra Mega Fire Kick_ refunds 50% of its mana cost if it hits at least one champion with the fireball or explosion.

**_Milio** cannot cast other abilities during Ultra Mega Fire Kick's delay._

| Detail | Value |
|--------|------:|
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Effects** | spellaoe |
| **Spell Shield** | true |
| **Projectile** | true |

**Notes:**

- <!--Blurb-->

### W – Cozy Campfire

| Attribute | Value |
|-----------|------:|
| **Cast Time** | 0.25 |
| **Target Range** | 650 (Location range) / 3000 (Targeted unit recast range) |
| **Effect Radius** | 415 (Center to edge) |
| **Cost** | (+90 to 130% AP) |
| **Cost Type** | Mana |
| **Cooldown** | (+29 to 21% AP) |
| **Cooldown Start** | post-effect |
| **Queue Time** | 0.25 (First cast) / 0.25 (Recasts) |

**ACTIVE:** **Milio** summons a fuemigo at the target location or upon the target allied champion for 6 seconds that follows the nearest allied champion and grants sight of its surroundings. Allied champions near the fuemigo gain **bonus** attack range equal to a percentage of their **base** attack range and heal every  over the duration.

_Cozy Campfire_ can be recast after 0.50 seconds within the duration.

**RECAST:** **Milio** commands the fuemigo to follow the target allied champion to within  150-units, placing the recast on a 0.50-second static (Unaffected by ability haste) cooldown.

**_Milio** counts as an allied champion for this ability. Cozy Campfire may grant Milio upon being summoned and at most once every 3 seconds thereafter._

| Detail | Value |
|--------|------:|
| **Targeting** | [Location](./Location-targeted.md) / [Unit](./Unit-targeted.md) |
| **Affects** | Self, Allies |
| **Out of Range** | Target at maximum range (first cast clamped regardless of targeting location or ally) |

**Notes:**

- _Cozy Campfire's_ initial cast and recast have a forgiveness radius of 175 units for their unit-targeted version.
  - The summoned fuemigo will still follow the nearest allied champion in range, even if the initial cast was targeted on a different ally.
- The attack range increase lingers on allies for the entire duration of _Cozy Campfire_, even if they leave the zone. <!--Blurb-->

### E – Warm Hugs

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 650 |
| **Cost** | (+50 to 90% AP) |
| **Cost Type** | mana |
| **Static** | 0.50 |
| **Recharge** | (+17 to 13% AP) |

**ACTIVE:** **Milio** envelops himself or the target allied champion in protective flames, granting the target a shield and **bonus** movement speed for 2.50 seconds.

**Milio** periodically [stock](./stock.md)s a _Warm Hugs_ charge, up to a maximum of 2.

_Warm Hugs' effects can stack up to 2 times._

| Detail | Value |
|--------|------:|
| **Targeting** | Unit |
| **Affects** | Self, Allies |

**Notes:**

- _Warm Hugs_ has a forgiveness radius of 175 units. <!--Blurb-->

### R – Breath of Life

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 700 (Center to edge, pending for test) |
| **Cost** | 100 |
| **Cost Type** | Mana |
| **Cooldown** | (+160 to 130% AP) |

**ACTIVE:** **Milio** explodes in soothing flames, healing and cleansing himself and nearby allied champions of non-airborne crowd control, and granting them 65% tenacity for 3 seconds.

**_Milio** cannot cast his other abilities for 0.75 seconds after Breath of Life's activation. Breath of Life cannot be used while affected by cast-inhibiting crowd control._

| Detail | Value |
|--------|------:|
| **Targeting** | Auto |
| **Affects** | Self, Allies |

**Notes:**

- _Breath of Life_ affects untargetable units. <!--Blurb-->

<!-- Raw excerpt (first 20 lines) -->

<details><summary>Raw excerpt</summary>

```wikitext

== Abilities ==

==Champion skins==
:_This article section only contains champion skins. For all associated collection items, see [Milio (Collection)](./Milio_Cosmetics.md)._

==Patch history==

==See also==
```
</details>
