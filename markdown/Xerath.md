# Xerath

## Table of Contents
- [Basic Information](#basic-information)
- [Statistics](#statistics)
  - [Map-specific Stats](#map-specific-stats)
- [Abilities](#abilities)
- [Patch History](#patch-history)
- [Trivia](#trivia)

## Basic Information

| Attribute | Value |
|-----------|------:|
| **Champion** | Xerath |
| **Title** | the Magus Ascendant |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2011-10-05 |
| **Release Patch** | V1.0.0.126 |
| **Latest Changes** | V25.08 |
| **Roles** | Artillery |
| **Riot Positions** | Middle, Support |
| **External Positions** | Middle, Support |
| **Blue Essence** | 675 |
| **Riot Points** | 585 |
| **Difficulty** | 1 |
| **Hero Type** | Mage |
| **Alt Type** | Support |
| **Adaptive Type** | Magic |
| **Damage** | 3 |
| **Toughness** | 1 |
| **Control** | 2 |
| **Mobility** | 1 |
| **Utility** | 1 |
| **Style** | 90 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $596.0$ | $+106.0$ |
| **Mana** | $400.0$ | $+22.0$ |
| **Health Regen** | $5.5$ | $+0.55$ |
| **Mana Regen** | $6.85$ | $+0.8$ |
| **Armor** | $22.0$ | $+4.7$ |
| **Magic Resist** | $30.0$ | $+1.3$ |
| **Attack Damage** | $55.0$ | $+3.0$ |
| **Attack Speed** | $0.658$ | |
| **Movement Speed** | $340.0$ | $+0.0$ |
| **Attack Range** | $525.0$ | $+0.0$ |
| **Base Attack Speed** | $0.658$ | |
| **Attack Speed Ratio** | $0.625$ | |
| **Bonus AS per Level** | $1.4\%$ | |
| **Missile Speed** | $2000$ units/second | |
| **Acquisition Radius** | $600$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $120$ units | |
| **Selection Height** | $302.778$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $93.0\%$ |
| **Damage Taken** | $105.0\%$ |

## Abilities

### Passive: Mana Surge

| Attribute | Value |
|-----------|------:|
| **Static Cooldown** | 16 |
| **Targeting** | Passive |
| **Affects** | Self |
| **Parry** | unknown |

**INNATE:** Periodically, **Xerath** empowers his next basic attack to restore (mana) 30 / 33 / 36 / 42 / 48 / 54 / 63 / 72 / 81 / 90 / 102 / 114 / 126 / 138 / 150 / 165 / 180 / 195 mana, doubled to 60 / 66 / 72 / 84 / 96 / 108 / 126 / 144 / 162 / 180 / 204 / 228 / 252 / 276 / 300 / 330 / 360 / 390 against enemy champions.

*Mana Surge*’s cooldown is reduced by $3.5$ seconds whenever **Xerath** kills an enemy.

*Mana Surge does not trigger if **Xerath** would restore above his **maximum** mana.*

**Notes:**

- *Mana Surge* will trigger even if the attack is blocked.
- The empowered attack will trigger against structures.
- : *Mana Surge's * interactions with dodging, and blinding effects.

---

### Q: Arcanopulse

| Attribute | Value |
|-----------|------:|
| **Range** | 700 / 807.14 / 914.29 / 1021.43 / 1128.57 / 1235.71 / 1342.86 / 1450 107.14 per 0.25 seconds (after an intial delay of 0 to 0.25). *This is capped at 1450 range.* units |
| **Cast Time** | none |
| **Width** | 145 units |
| **Cost** | 80 / 90 / 100 / 110 / 120 mana |
| **Cooldown** | 9 / 8 / 7 / 6 / 5 seconds |
| **Cooldown Start** | post-effect |
| **Targeting** | Auto / Direction |
| **Affects** | Self, Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | False |
| **Silence** | True |

**ACTIVE:** **Xerath** charges while being slowed by 0% / 10% / 15% / 20% / 25% / 30% / 35% / 40%5% per 0.25 seconds channeled, capped at 40% for up to 3 seconds to increase *Arcanopulse*’s range over the first $1.5$ to $1.75$ seconds of the channel.

*Arcanopulse* can be recast within the duration. If the charge is interrupted or completes without reactivation, *Arcanopulse* is cancelled and refunds (mana) half the mana cost.

| Attribute | Value |
|-----------|------:|
| **Mana Refunded** | 40 / 45 / 50 / 55 / 60 |

**RECAST:** **Xerath** becomes unable to act for and afterwards fires a beam of energy in a line in the target direction that deals magic damage to enemies hit.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 75 / 115 / 155 / 195 / 235 (+ 90% AP) |

**Notes:**

- The onset of *Arcanopulse*’s self-slow and first range increase is at a random interval of 0 to $0.25$ seconds. The real time to maximum self-slow and range is thus $1.5$ to $1.75$ seconds.
- The rectangle hitbox's minimum range is er 0, right through **Xerath**’s center.
- *Arcanopulse* will not fire and deal damage if **Xerath** dies during the recast delay.
- The following table refers for interactions while **Xerath** is channeling:

#### Channel Behavior (charge)

| Aspect | State / Notes |
|--------|---------------|
| **Attacking** | Disabled |
| **Movement** | Allowed |
| **Abilities** | Disabled |
| **Items** | Disabled |
| **Summoner Spells** | Allowed / Disabled / Recasts |
| **Consumables** | Disabled |
| **Notes** | but can still use trinkets. |
- The following table refers for interactions while **Xerath** is locked out during the recast delay:

---

### W: Eye of Destruction

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 1000 units |
| **Effect Radius** | 275 (Effect radius) / 125 (Inner radius) / 250 (Sight radius) units |
| **Cost** | 80 / 90 / 100 / 110 / 120 mana |
| **Cooldown** | 14 / 13 / 12 / 11 / 10 seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |
| **Out of Range Behavior** | walk to location |

**ACTIVE:** **Xerath** casts down a blast of arcane energy that strikes the target location after , briefly granting sight of the area and dealing magic damage to enemies hit and slowing them by 25% for $2.5$ seconds.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 50 / 85 / 120 / 155 / 190 (+ 65% AP) |

Enemies in the epicenter take $66.7$% increased damage and are slowed by a greater amount, decaying to 25% over the duration.

| Attribute | Value |
|-----------|------:|
| **Increased Damage** | 83.35 / 141.695 / 200.04 / 258.385 / 316.73 (+ 108.355% AP) |

| Attribute | Value |
|-----------|------:|
| **Increased Slow** | 60 / 65 / 70 / 75 / 80% |

**Notes:**

- **Xerath**’s location gets revealed to the enemy (400 radius for 4.5 seconds) if *Eye of Destruction*’s cast location is within 100 units of a valid enemy at the start of the cast time due to something related to targeting types.
- *Eye of Destruction*’s targeting indicator appears on the player's minimap, despite the ability not having the longer range of others with this quality.
- *Eye of Destruction* will not fire and deal damage if **Xerath** dies during the 0.25s cast time.

---

### E: Shocking Orb

| Attribute | Value |
|-----------|------:|
| **Range** | 1125 units |
| **Cast Time** | $0.25$ seconds |
| **Width** | 120 units |
| **Speed** | 1400 units/second |
| **Cost** | 60 / 65 / 70 / 75 / 80 mana |
| **Cooldown** | 13 / 12.5 / 12 / 11.5 / 11 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Single target |
| **Projectile** | True |

**ACTIVE:** **Xerath** fires an orb of energy in the target direction that deals magic damage to the first enemy hit and stuns them for 0.75–2.25@0–1050 (@=orb travel distance) seconds.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 70 / 100 / 130 / 160 / 190 (+ 45% AP) |

**Notes:**

Effect at cast time end

---

### R: Rite of the Arcane

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 5000 units |
| **Effect Radius** | 200 units |
| **Cost** | 100 mana |
| **Cooldown** | 130 / 122.5 / 115 / 107.5 / 100 seconds |
| **Cooldown Start** | post-effect |
| **Targeting** | Auto / Location |
| **Affects** | Self, Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | False |
| **Out of Range Behavior** | Target at maximum range (recast clamped) |
| **Silence** | True |

**ACTIVE:** **Xerath** channels for up to 10 seconds, gaining the ability to recast *Rite of the Arcane* multiple times after $0.5$ seconds within the duration. If *Rite of the Arcane* ends without any of the recasts being used, half of its cooldown is refunded.

| Attribute | Value |
|-----------|------:|
| **Number of Recasts** | 4 / 4.5 / 5 / 5.5 / 6 |

**RECAST - ARCANE BARRAGE:** **Xerath** catapults an arcane missile that strikes the target location after a delay, briefly granting sight of the area and dealing magic damage to enemies hit. Each cast has a static cooldown (Unaffected by ability haste) of $0.5$ seconds.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 170 / 195 / 220 / 245 / 270 (+ 45% AP) |
| **Total Magic Damage** | 680 / 1100 / 1620 (+ 180 / 202.5 / 225 / 247.5 / 270% AP) |

Hitting at least one enemy champion grants a stack of *Arcane Perfection* during the channel, stacking up to a maximum amount. Each stack increases *Arcane Barrage*’s damage.

| Attribute | Value |
|-----------|------:|
| **Maximum Stacks** | 3 / 3.5 / 4 / 4.5 / 5 |

| Attribute | Value |
|-----------|------:|
| **Increased Damage per Stack** | 20 / 25 / 30 (+ 5% AP) |

**Xerath** will reveal himself if a missile strikes near an enemy.

| Attribute | Value |
|-----------|------:|
| **Impact Distance to Reveal** | 175 / 187.5 / 200 / 212.5 / 225 |

**Notes:**

- *Rite of the Arcane* cannot be canceled via movement commands during the first second of its channel.
- Each cast counts as an ability activation for the purposes of on-cast effects such as Spellblade and triggering Force Pulse’s passive.
- *Arcane Barrage* will not apply its increased damage per stack to the cast that granted the stack.
  - Only successive casts of the ability will benefit from the damage increase.
- A circular indicator will be displayed at the target location to signify that the area will be blasted.
- The circle indicating the ability's range expands rapidly from near **Xerath** up to the maximum range over 1 second during the channel.
  - The maximum-range indicator is visible to both allies and enemies and is color coded for friendly (blue) or hostile (red).
- *Rite of the Arcane* uses a barrage system so that it can be recast multiple times within a given period at no additional cost.
- **Xerath** will turn to face toward the direction of the target location after using a recast.
- **Xerath** gains a wider field of view during *Rite of the Arcane*.
- **Xerath** can use Dark Passage while channeling.
- The following table refers for interactions while **Xerath** is channeling:
  - Teleport and Recall are disabled for the first $1.25$ seconds and otherwise interrupt the channel if they are used.

#### Channel Behavior (channel)

| Aspect | State / Notes |
|--------|---------------|
| **Attacking** | Interrupts |
| **Movement** | Interrupts |
| **Abilities** | Disabled |
| **Items** | Interrupts / Allowed / Disabled |
| **Summoner Spells** | Allowed / Disabled / Interrupts |
| **Notes** | Disabled for the first $1.25$ seconds otherwise |

---

## Patch History

### V25.08
- Mana Surge
  - Cooldown reduction per kill increased to $3.5$ seconds from $2.5$.
- Arcanopulse
  - Base damage increased to 75 / 115 / 155 / 195 / 235 from 70 / 110 / 150 / 190 / 230.

### V25.07
- Arcanopulse
  - AP ratio increased to 90% AP from 85% AP.
- Eye of Destruction
  - Base damage reduced to 50 / 85 / 120 / 155 / 190 from 60 / 95 / 130 / 165 / 200.
    - Center base damage reduced to 83.35 / 141.695 / 200.04 / 258.385 / 316.73 from 100.02 / 158.365 / 216.71 / 275.055 / 333.4.
  - AP ratio increased to 65% AP from 60% AP.
    - Center AP ratio increased to 108.355% AP from 100.02% AP.
- Shocking Orb
  - Base damage reduced to 70 / 100 / 130 / 160 / 190 from 80 / 110 / 140 / 170 / 200.
- Rite of the Arcane
  - Base damage reduced to 170 / 220 / 270 from 180 / 230 / 280.
  - AP ratio increased to 45% AP from 40% AP.

### V14.19
- Xerath
  - **Bug Fixes:** Resolved VO audio being too low in certain languages due to the audio filter.

### V14.9
- Stats
  - Selection radius increased to 120 units from $88.8889007568359$.
- Arcanopulse, Eye of Destruction, Shocking Orb
  - **Bug Fixes:** No longer reference to his long-deprecated ammo system.
- Eye of Destruction
  - **Bug Fixes:** Vision bubble is no longer inconsistent inside terrain, causing some targets to remain undetected in its radius.

### V14.7
- General
  - Adjusted splash artwork for Xerath.

### V13.22
- General
  - The two basic attack animations now have a 50% chance to play instead of 75% for the first and 25% for the second.
- Stats
  - Base attack speed increased to $0.658$ from $0.625$.
  - Attack windup reduced to 20% from $25.074$%.

### V13.17
- Stats
  - Base mana regeneration reduced to $6.85$ from 8.
- Mana Surge
  - Cooldown refund increased to $2.5$ seconds from 2.
- Eye of Destruction
  - Mana cost increased to 80 / 90 / 100 / 110 / 120 from 70 / 80 / 90 / 100 / 110.

### V13.15
- Rite of the Arcane
  - **Bug Fixes:** Stacking damage buff is now properly applied to the last recast.

### V13.14
- Stats
  - Base mana reduced to 400 from 459.
- Mana Surge
  - Cooldown increased to 16 seconds from 12.
  - **New Effect:** Cooldown is now reduced by 2 seconds per unit killed.
  - **New Effect:** Now triggers against structures.
- Rite of the Arcane
  - Number of recasts increased to 4 / 5 / 6 from 3 / 4 / 5.
  - Base damage reduced to 180 / 230 / 280 from 200 / 250 / 300.
  - AP ratio reduced to 40% AP from 45% AP.
  - **New Effect:** Hitting at least one enemy champion grants a stack of Arcane Perfection during the channel, stacking up to 4 / 5 / 6 times. Each stack increases Arcane Barrage's damage by 20 / 25 / 30 (+ 5% AP).

### V13.4
- General
  - **Bug Fixes:** Emote SFX no longer continue to play even after the animation was interrupted halfway.

## Trivia

- When Xerath dies, his body quickly becomes unstable and overly shiny, just to blow up a moment later, much like a nuclear-powered machine.
- The icon for Xerath's ability, Arcanopulse is slightly similar to that of the Season 3 mastery Blast.
- Xerath's dance is a reference "industrial dancing", which is evident in the song Pong by Eisenfunk.
  - A side-by-side comparison can be seen here.
- Xerath himself can be seen when he performs his taunt. As he disarms the parts of his sarcophagus, a person made of pure energy or electricity can be briefly seen.
  - When he dies this "person" disappears, what remains on the ground are the shattered remains of his sarcophagus.
  - It's likely that that form was his actual look before he got locked away.
- Some speculate that Xerath is ambidextrous as he uses both left and right hands for different attacks and abilities, unlike most champions who almost always use their dominant hand (usually their right one).
- Xerath, Blitzcrank, Caitlyn, Lissandra, Rumble, Sion, Varus, Vi, Viego, and Ziggs are the only champions who can apply crowd control on themselves.
  - Xerath's ethereal form imprisoned in a human-shapen sarcophagus also resembles Battle Realms' , consisting of an ethereal warlock inside another warlock's skeletal remains.
- Xerath features as the Ancient Ascendant in the Ascension featured game mode.

---
*This page was automatically generated from League of Legends Wiki data.*