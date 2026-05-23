# Hecarim

## Overview

- **Title:** Hecarim
- **Resource:** Unknown

## Stats

| Stat | Base | Growth |
|------|------|--------|

## Abilities

### Passive – Warpath

**INNATE:** **Hecarim** gains **bonus** attack damage equal to key= / 12 to 24 for 7 / 1;3 to 18 / formula=12%+2% every 3 levels starting from level 3 of his **bonus** movement speed.

| Detail | Value |
|--------|------:|
| **Targeting** | Passive |
| **Affects** | Self |

**Notes:**

- _Warpath's_ **bonus** attack damage will adjust itself to any temporary movement speed buffs and debuffs **Hecarim** is affected by.
  - Since this only affects **bonus** values **Hecarim** will not dip below his regular attack damage if he is slowed below his **base** movement speed.
- The amount of movement speed scaling to **bonus** attack damage is refreshed every 0.25 seconds.

### Q – Rampage

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 375 |
| **Cost** | (+28 to 20% AP) |
| **Cost Type** | Mana |
| **Cooldown** | 4-0.75*(x-1) / 0 to 3 by 1 / type=_Rampage_ stacks / formula=4 - 0.75 * [_Rampage_ stacks], capped at 3 stacks. / label=cooldown |

**ACTIVE:** **Hecarim** cleaves his glaive around himself, dealing physical damage to nearby enemies, reduced to 60% against minions.

If this damages an enemy, **Hecarim** gains a stack of _Rampage_ for 8 seconds,  refreshing on subsequent damage and stacking up to 3 times. Each stack increases _Rampage's_ damage by 3% (+ 4% per 100 **bonus** AD) and reduces its **base** cooldown by 0.75 seconds, up to a 9% (+ 12% per 100 **bonus** AD) damage increase and a 2.25-second reduction of the **base** cooldown at maximum stacks. Stacks expire by one every 1 second when the duration ends.

| Detail | Value |
|--------|------:|
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical |
| **Spell Effects** | spellaoe |
| **Spell Shield** | True |

### W – Spirit of Dread

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 525 |
| **Cost** | (+50 to 70% AP) |
| **Cost Type** | Mana |
| **Cooldown** | 14 |
| **Cooldown Start** | on-cast |

**ACTIVE:** **Hecarim** surrounds himself with the Spirit of Dread for 4 seconds, dealing magic damage every second to nearby enemies.

While active, **Hecarim** gains **bonus** armor and **bonus** magic resistance and is healed for 25% of the post-mitigation damage (Damage calculated after modifiers) dealt to enemies within the area from all sources, halved to (+25/2% AP)% for damage dealt by allies. The healing is capped against minions and monsters.

| Detail | Value |
|--------|------:|
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | Magic |
| **Spell Effects** | AoeDoT |
| **Spell Shield** | False |

**Notes:**

- The healing cap against minions accounts for every single one regardless of how many are present and/or affected by _Spirit of Dread_ at the time of cast.

### E – Devastating Charge

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Cost** | 60 |
| **Cost Type** | Mana |
| **Cooldown** | (+20 to 16% AP) |
| **Cooldown Start** | on-cast |

**ACTIVE:** **Hecarim** becomes ghosted and gains type=time active / key= / 25 to 65 for 13 / 0 to 3 / formula=3.3% **total bonus** movement speed every 0.25 seconds. _This is capped at 3 seconds._ **bonus total** movement speed for 4 seconds.

During this time, **Hecarim** empowers his next basic_attack to gain type=distance traveled / 50 to 250 for 9 / 0 to 1200 **bonus** range and cause him to dash in the target's direction, revealing them for 1 second. If the target remains nearby during the dash, the ghosting and **bonus** movement speed ends prematurely and **Hecarim** knocks them back type=distance traveled / 150 to 350 for 9 / 0 to 1200 units, though not through terrain, stuns them for 0.25 seconds, and deals them **modified** physical damage, increased by type=distance traveled / key= / 0 to 100 for 9 / 0 to 1200.

_Devastating Charge_ can critically strike for total critical damage AD **bonus** physical damage.

_Devastating Charge resets **Hecarim's** basic attack timer. **Hecarim** can cast any of his abilities during the dash. Devastating Charge's duration is paused during Hecarim._

| Detail | Value |
|--------|------:|
| **Targeting** | Auto |
| **Affects** | Self |
| **Damage Type** | Physical |
| **Spell Effects** | spell |
| **Spell Shield** | True |
| **Parry** | Special |
| **Grounded** | special |
| **Knockdown** | special |

**Notes:**

- **Hecarim** can cast any of his abilities while dashing.
  - Hecarim will interrupt the dash.
- If the target does not remain nearby during the dash, _Devastating Charge's_ effect will not be consumed.
- The **bonus** movement speed stacks multiplicatively with other sources of movement speed boosts.
- The bonus attack range stacks additively with Rapid Firecannon.
- _Devastating Charge_ will still apply its effects to the target even if the dash is interrupted, but not if he goes into resurrection.
- _Devastating Charge's_ damage will apply link=true and will affect structures.
- _"Distance traveled"_ can be simple movement as well as dashing and blinking.
- _Devastating Charge's_ damage will not bypass block or dodge but the knock back will still be applied.
  - _Devastating Charge_ will still deal damage while blinded.
- While grounded or rooted, **Hecarim** loses the **bonus** attack range from _Devastating Charge_.
  - **Hecarim** will not use the empowered attack while rooted.
- The dash does not follow targets. The target's position at the time of the enhanced attack is the direction **Hecarim** will leap to.
- Displacement immunity will not resist the application of the stun.

### R – Onslaught of Shadows

| Attribute | Value |
|-----------|------:|
| **Range** | 1510 (Spectral riders missile range, estimated) |
| **Cast Time** | none |
| **Target Range** | 300 (Minimum range) / 1000 (Maximum range) |
| **Effect Radius** | 315 (Fear radius around dash end) |
| **Width** | 80 (Individual spectral riders missile width) / 480 (Width of full effect given by outermost riders) |
| **Speed** | 1100 (Both dash and spectral riders missile speed) |
| **Cost** | 100 |
| **Cost Type** | Mana |
| **Cooldown** | (+140/120/100% AP) |

**ACTIVE:** **Hecarim** dashes with displacement immunity to the target location and summons 5 spectral riders in an arrow formation (see notes) that charge alongside him in the target direction, dealing magic damage to all enemies in their path and revealing them for 2.50 seconds.

Upon arrival, he fears nearby enemies for type=distance traveled / 0.75 to 1.5 for 11 / 0 to 1000 seconds and slows them by type=distance from **Hecarim** / 0;0 to 99 / 0;150 to 300 by 25 / formula=Minimum of 0% slow at &le;150 centered range from **Hecarim**, maximum of 99% at 300 centered range from **Hecarim** (linearly interpolated). / key=.

_The wave of spectral riders travels independently of **Hecarim** and will always charge at the same distance._

| Detail | Value |
|--------|------:|
| **Targeting** | Location |
| **Damage Type** | Magic |
| **Out of Range** | cast at max |
| **Spell Effects** | spellaoe |
| **Spell Shield** | True |
| **Projectile** | True |
| **Grounded** | True |
| **Knockdown** | False |

**Notes:**

- The spectral rider missiles originate at **Hecarim's** location, 145 units behind and 100 units to either side of him, and 290 units behind and 200 units to either side. They travel 1510 units forward parallel to Hecarim from their origin each.
- Only the spectral riders deal damage to enemies they pierce, **Hecarim** only dashes and fears enemies at the end of it.
- The slow improperly counts as an additive percent movement speed modifier rather than a multiplicative one.
  - The strength is unaffected by slow resist as a result of this behavior.
  - This means it can unintentionally stack with the effects of other slows.

## Trivia

- Hecarim might have been inspired by the Four Horsemen of the Apocalypse (specifically War) and/or [http://www.khwiki.com/Assault_Rider Assault Rider] from Kingdom Hearts.
- His dance references [https://www.youtube.com/watch?v=fljKx9nvrL4 'Dope Zebra'] by [https://www.youtube.com/channel/UCbochVIwBCzJb9I2lLGXGjQ 'rhettandlink'].
  - A side-by-side comparison can be seen [http://www.youtube.com/watch?v=ST3WBD3SNbY here.]
- Hecarim is one of the two abilities in the game with damage scaling with movement speed. The other being  Janna.
- Hecarim might be referencing the fight with [https://zeldawiki.wiki/wiki/Ganondorf Ganondorf] in The Legend of Zelda: Twilight Princess.
  - The spectral riders used to be members of the Iron Order Hecarim was Knight Commander of and, together with him, became undead when the Ruination of the Blessed Isles was unleashed.
- Hecarim was deemed overpowered in Ultra_Rapid_Fire (2014 edition) and was ultimately disabled in non-custom games.

<!-- Raw excerpt (first 20 lines) -->

<details><summary>Raw excerpt</summary>

```wikitext

== Abilities ==

==Champion skins==
:_This article section only contains champion skins. For all associated collection items, see Hecarim_(Collection)._

==Patch history==

```
</details>
