# Lissandra

## Overview

- **Title:** Lissandra
- **Resource:** Unknown

## Stats

| Stat | Base | Growth |
|------|------|--------|

## Abilities

### Passive – Iceborn Subjugation

| Attribute | Value |
|-----------|------:|
| **Effect Radius** | 1350 (Aura radius around Lissandra, center-to-edge) /  450 (Thrall slow field and explosion radius) |

**INNATE:** Whenever a nearby enemy champion dies, **Lissandra** spawns a _Frozen Thrall_ from their corpse.

_Frozen Thralls_ are ice spirits that have 325*.103;325*.242;325*.361;325*.461;325*.547;325*.619;325*.681;325*.733;325*.778;325*.812;325*.848;325*.876;325*.899;325*.919;325*.935;325*.95;325 / 0 to 4 / type=seconds alive movement speed and slow nearby enemies by 25%. They will chase nearby visible enemies for 4 seconds, prioritizing champions, after which they shatter to deal 120 to 340;370;400;430;460;490;520 (+ 50% AP) magic damage to nearby enemies.

| Detail | Value |
|--------|------:|
| **Targeting** | Passive |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Effects** | spellaoe |
| **Spell Shield** | true |

**Notes:**

- _Thralls_ are units that are considered champions (they are clones) and they take the form of the champion they arise from.
  - Additionally, near the end of their life span, they will cue that champion's basic_attack animation.
  - They cannot be teleported by an allied Ryze.
- _Thralls_ are untargetable and invulnerable.
- _Thralls_ are revealed to enemies through the fog of war.
- _Thralls_ will continue to chase their target even if they enter a brush.
- _Iceborn Subjugation_ will not summon a _Thrall_ against enemy champions that enter a zombie state, and will instead summon one after they've left the state.

### Q – Ice Shard

| Attribute | Value |
|-----------|------:|
| **Range** | 825 (Standard range including area check after missile end) / 950 (Enhanced range from cast origin) |
| **Cast Time** | 0.25 |
| **Width** | 150 (standard width) / 180 (Width after shattering) |
| **Speed** | 2200 (Both before and after shattering) |
| **Cost** | (+55 to 75% AP) |
| **Cost Type** | Mana |
| **Cooldown** | (+8 to 4% AP) |

**ACTIVE:** **Lissandra** launches a shard of ice in the target direction that deals magic damage and slows enemies hit for 1.50 seconds.

If _Ice Shard_ hits an enemy, it will shatter, increasing its width and maximum range.

| Detail | Value |
|--------|------:|
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | magic |
| **Spell Effects** | aoe |
| **Spell Shield** | Special |
| **Projectile** | true |

**Notes:**

- _Ice Shard_ picks a location 950 units away in the direction of the cast for the shattered missile to end up at.
- The initial _Ice Shard_ missile has a range of 700 units which it arrives at after . If it hasn't collided with an enemy in this path when it does, it checks for enemies in a  100 radius around the point 25 units in front of it.
  - Colliding or hitting an enemy in either fashion creates a new "shattered" missile with the same speed but greater width that continues to travel along the same line to the designated point 950 units from the cast's original position, originating at the location at which the initial missile collided at.<!-- or collision point? what about when spawned from the lollipop hit? to be determined later-->
- Spell shield will block the damage and the slow but will not stop the projectile from shattering.

### W – Ring of Frost

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 275 |
| **Cost** | 40 |
| **Cost Type** | Mana |
| **Cooldown** | (+10 to 8% AP) |

**ACTIVE:** **Lissandra** freezes nearby enemies, dealing magic damage and rooting them for a duration.

| Detail | Value |
|--------|------:|
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | magic |
| **Spell Effects** | spellaoe |
| **Spell Shield** | True |

**Notes:**

- No additional information.

### E – Glacial Path

| Attribute | Value |
|-----------|------:|
| **Range** | 1025 |
| **Cast Time** | 0.25 |
| **Width** | 250 |
| **Speed** | 1200 (starting speed) / 640 (minimum speed) |
| **Cost** | (+80 to 100% AP) |
| **Cost Type** | Mana |
| **Cooldown** | (+24 to 12% AP) (Starts on cast) |

**ACTIVE:** **Lissandra** sends a claw of ice in the target direction that deals magic damage to enemies it passes through, decelerating over 1.25 seconds. _Glacial Path_ can be recast after 0.50 seconds while the claw is active.

**RECAST:** **Lissandra** consumes the claw and blinks to its current location.

| Detail | Value |
|--------|------:|
| **Targeting** | Direction / Auto |
| **Affects** | Enemies, Self |
| **Damage Type** | magic |
| **Spell Effects** | aoe |
| **Spell Shield** | True |
| **Projectile** | true |
| **Grounded** | special |

**Notes:**

- Both casts count as ability activations for the purposes of on-cast effects such as Spellblade and triggering Kassadin passive.
- **Lissandra** can reactivate _Glacial Path_ for the duration (plus another 0.20-0.30 seconds after the claw 'sinks' into the ground).
  - The Claw can also hit enemies near the end point at this time, shortly after the missile has reached its maximum range.
- _Glacial Path_ allows **Lissandra** to surpass through every single wall in all maps, so long as the claw is at least halfway through them.
- _Glacial Path_ cannot be recast while grounded or rooted.
- _Glacial Path's_ endpoint shows through terrain, fog_of_war and brush to enemies within 600 range of it.

### R – Frozen Tomb

| Attribute | Value |
|-----------|------:|
| **Cast Time** | 0.38 (Enemy cast) / None (Self cast) |
| **Target Range** | 550 |
| **Effect Radius** | 550 |
| **Cost** | 100 |
| **Cost Type** | Mana |
| **Cooldown** | (+120 to 80% AP) |

**ACTIVE:** **Lissandra** can cast _Frozen Tomb_ on herself or an enemy champion.

**ENEMY CAST:** **Lissandra** freezes the target enemy champion, knocking them down and stunning them for 1.50 seconds.

**SELF CAST:** **Lissandra** instantly entombs herself in ice, entering stasis for 2.50 seconds and healing herself every 0.25 seconds over the duration. The healing is increased by type=**missing** health at the time of cast / key= / 0 to 100 for 11 / key1= / 0 to 100 / formula=1% per 1% of **missing** health.

_Frozen Tomb_ creates a field of ice that spreads out from the target over 1.50 seconds and covers the surrounding area for 3 seconds, dealing magic damage to enemies and slowing them for 0.50 seconds, refreshing every 0.25 seconds while they remain.

| Detail | Value |
|--------|------:|
| **Targeting** | Unit |
| **Affects** | Enemies, Self |
| **Damage Type** | magic |
| **Spell Effects** | spellaoe |
| **Spell Shield** | special |
| **Call For Help** | true |

**Notes:**

- Enemies can only be damaged by the field of ice once every 4 seconds.
  - Under normal circumstances, this means once per cast of _Frozen Tomb_.
- Enemies who come in contact with _Frozen Tomb's_ slow field will be damaged and a small visual and sound effect will play.
- Spell shield will block the single-targeted portion and the radiating ice damage, but will not stop it from spreading.
- If the target becomes untargetable, dies, or is too far away during the cast time, this ability will cancel but does not go on cooldown nor pay its cost.
  - This only applies to the enemy cast.

## Trivia

- Lissandra first appeared in the Journal_of_Justice, Issue_2, several years before her announcement as a champion. Like now, she was portrayed as the leader of one of the three tribes of the Freljord, but her portrayal was originally vastly different, as a mortal princess allied with Ashe's tribe.
  - This was later revealed to be her alter ego, and most people are not aware of her being the Ice Witch herself.
- If Ashe, Lissandra, and/or Sejuani are played on opposing teams, Battle_for_Freljord will trigger. This in-game quest consists in one killing the other to earn the title of 'Queen of Freljord' (complete with a floating ice crown floating above their heads) referencing the civil war taking place there, between the three tribes each lead.
- Lissandra is the only champion to possess a single-targeted ability (Lissandra) that can be used on herself but not on allies.
- Lissandra, Elise, and Lucian are the only champions to feature a monologue on their login screens.
- _Lissandra_ may be the feminine form of Greek name ÃÂÃÂÃÂÃÂ±ÃÂ½ÃÂ´ÃÂÃÂ¿ÃÂ (Lysander or "liberator").
- Her champion theme is the same as the pick music of ARAM and is titled "Freljord".
- Lissandra, Blitzcrank, Caitlyn, Rumble, Sion, Varus, Vi, Viego, Xerath, and Ziggs are the only champions who can apply crowd_control on themselves.
- Lissandra's Series 1 Eternals make the following references:
  - _En-Thrall-ing_ is a name puns between "Enchanted" with "Frozen Thralls" from the Lissandra passive.

<!-- Raw excerpt (first 20 lines) -->

<details><summary>Raw excerpt</summary>

```wikitext

== Abilities ==

==Champion skins==
:_This article section only contains champion skins. For all associated collection items, see Lissandra_(Collection)._

==Patch history==

==Trivia==
```
</details>
