# Vex

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
| **Champion** | Vex |
| **Title** | the Gloomist |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2021-09-23 |
| **Release Patch** | V11.19 |
| **Latest Changes** | V25.09 |
| **Roles** | Burst |
| **Riot Positions** | Middle |
| **External Positions** | Middle |
| **Blue Essence** | 1575 |
| **Riot Points** | 790 |
| **Difficulty** | 2 |
| **Hero Type** | Mage |
| **Adaptive Type** | Magic |
| **Damage** | 3 |
| **Toughness** | 2 |
| **Control** | 2 |
| **Mobility** | 2 |
| **Utility** | 1 |
| **Style** | 90 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $590.0$ | $+104.0$ |
| **Mana** | $490.0$ | $+32.0$ |
| **Health Regen** | $6.5$ | $+0.6$ |
| **Mana Regen** | $6.0$ | $+0.8$ |
| **Armor** | $23.0$ | $+4.45$ |
| **Magic Resist** | $28.0$ | $+1.3$ |
| **Attack Damage** | $54.0$ | $+2.75$ |
| **Attack Speed** | $0.669$ | |
| **Movement Speed** | $335.0$ | $+0.0$ |
| **Attack Range** | $550.0$ | $+0.0$ |
| **Base Attack Speed** | $0.669$ | |
| **Attack Speed Ratio** | $0.625$ | |
| **Bonus AS per Level** | $1.0\%$ | |
| **Missile Speed** | $2000$ units/second | |
| **Acquisition Radius** | $750$ units | |
| **Gameplay Radius** | $55$ units | |
| **Pathing Radius** | $30$ units | |
| **Selection Radius** | $100$ units | |
| **Selection Height** | $100$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $95.0\%$ |
| **Damage Taken** | $100.0\%$ |

#### URF

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $90.0\%$ |
| **Damage Taken** | $100.0\%$ |

## Abilities

### Passive: Doom 'n Gloom

| Attribute | Value |
|-----------|------:|
| **Effect Radius** | 1600 (Dash and blink detection radius) units |
| **Static Cooldown** | 25@1; 22@6; 19@11; 16@16 |
| **Targeting** | Passive |
| **Affects** | Self, Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | Special |
| **Spell Effects** | proc |
| **Projectile** | False |
| **Parry** | True |

**PASSIVE - DOOM:** Periodically, **Vex** empowers her next basic ability to knock down and fear enemies hit for 0.75@1; 1@6; 1.25@9; 1.5@13 seconds, during which they are slowed by 60%–99%@250–800 (@=distance from **Vex**). If *Looming Darkness* triggers *Doom*, enemies hit will flee from the epicenter instead.
*Doom*’s cooldown resets upon respawning.

**INNATE - GLOOM:** Nearby enemy champions and monsters that dash or blink will be marked with *Gloom* for 6 seconds. **Vex**’s next basic attack, which becomes non-projectile, or basic ability hit against an enemy with *Gloom* will detonate the mark. *Looming Darkness* will also inflict *Gloom*, but cannot detonate it.

*Gloom*’s detonation deals 40 to 150 (+ 25% AP) **bonus** magic damage and refunds 25% of *Doom*’s cooldown. Against non-champions, this instead deals 40%@1; 45%@6; 50%@9; 55%@13; 60%@16 damage and refunds 10% of *Doom*’s cooldown.

**Notes:**

- *Gloom* will mark Rift Herald and Rift Scuttler when they use their dashes.
- *Gloom* can mark clones, but not other pets.
- *Gloom*’s mark has a very brief cooldown and refreshes on subsequent dashes or blinks nearby enemies use.
- *Gloom*’s mark duration will refresh to $0.5$ seconds when **Vex** starts an attack windup against a target that has a mark which is about to expire.
- *Gloom* will mark enemies even if they are untargetable.
- *Gloom* will mark enemies that are inside the detection radius when they blink, but will not mark those that blink inside from far away.
- *Doom*’s cooldown starts as soon as the basic ability is cast.
- *Doom*’s fear will not be removed when **Vex** dies, unlike any other fear in the game.
- Non-champions (e.g. Rift Scuttler) are not knocked down by the fear-empowered ability.
- Spell shield will block all of *Doom*’s effects but not *Gloom*’s empowered attack nor mark.
- Nearby enemies that become displaced do not count for being marked by *Gloom*.
- Soul Unbound’s recast does not cause the caster to be marked by *Gloom*.
- **Vex**’s basic abilities use a different icon when *Doom* is ready:
  - Mistral Bolt
  - Personal Space
  - Looming Darkness

---

### Q: Mistral Bolt

| Attribute | Value |
|-----------|------:|
| **Range** | 1200 units |
| **Cast Time** | $0.15$ seconds |
| **Width** | 360 (First part) / 160 (Second part) units |
| **Speed** | 600 (First part) / 3200 (Second part) units/second |
| **Cost** | 45 / 50 / 55 / 60 / 65 Mana |
| **Cooldown** | 8 / 7 / 6 / 5 / 4 seconds |
| **Targeting** | Direction |
| **Affects** | Ememies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | True |

**ACTIVE:** **Vex** launches a wave of mist in the target direction that deals magic damage to enemies hit. After travelling 500 units, the wave accelerates but also narrows itself.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 70 / 115 / 160 / 205 / 250 (+ 70% AP) |

**Notes:**

Effect at cast time end
  - *Mistral Bolt* will fire from wherever **Vex** is at the end of the cast time, towards the originally targeted location or 1200 units in the originally targeted direction if cast beyond that.

---

### W: Personal Space

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Effect Radius** | 475 / 550 (Against dashing enemies) units |
| **Cost** | 75 mana |
| **Cooldown** | 16 / 15 / 14 / 13 / 12 seconds |
| **Queue Time** | $0.1$ seconds |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |

**ACTIVE:** **Vex** emits a shockwave around her before the cast time, dealing magic damage to nearby enemies and granting herself a shield for $2.5$ seconds.

| Attribute | Value |
|-----------|------:|
| **Shield Strength** | 50 / 75 / 100 / 125 / 150 (+ 75% AP) |

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 80 / 120 / 160 / 200 / 240 (+ 30% AP) |

***Vex** can move during Personal Space's cast time.*

**Notes:**

- *Personal Space* can be buffered $0.1$ seconds before it comes off cooldown or becomes available otherwise.

---

### E: Looming Darkness

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 800 units |
| **Effect Radius** | 200–300@0–800 (@=cast distance) units |
| **Speed** | 1300 units/second |
| **Cost** | 70 / 80 / 90 / 100 / 110 mana |
| **Cooldown** | 13 seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | True |
| **Out of Range Behavior** | cast at max |

**ACTIVE:** **Vex** tosses her *Shadow* to explode at the target location, dealing magic damage to enemies hit and slowing them for 2 seconds. The explosion's radius increases based on cast distance.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 50 / 70 / 90 / 110 / 130 (+ 40 / 45 / 50 / 55 / 60% AP) |

| Attribute | Value |
|-----------|------:|
| **Slow** | 30 / 35 / 40 / 45 / 50% |

Killing an enemy with *Looming Darkness* refunds 10% of *Doom*’s cooldown, increased to 25% for champion kills.

**Notes:**

- No additional details.

---

### R: Shadow Surge

| Attribute | Value |
|-----------|------:|
| **Range** | 2000 / 2250 / 2500 / 2750 / 3000 (Shadow missile range) units |
| **Cast Time** | $0.25$ (Active) / None (Recast) |
| **Effect Radius** | sight650 (Missile sight radius) / Global (Recast radius) |
| **Width** | 260 (First cast missile width) units |
| **Speed** | 1600 (First cast missile speed) / 2200 (Dash speed, estimated) units/second |
| **Cost** | 100 mana |
| **Cooldown** | 140 / 130 / 120 / 110 / 100 (Starts post-effect of initial cast) seconds |
| **Cooldown Start** | post-effect |
| **Targeting** | Direction / Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spell |
| **Projectile** | True |
| **Grounded** | Special |
| **Knockdown** | False |

**ACTIVE:** **Vex** sends her *Shadow* in the target direction that grants sight around its trajectory (Does not see into bush or across terrain) and deals magic damage to enemies hit.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 75 / 100 / 125 / 150 / 175 (+ 20% AP) |

*Shadow* stops upon hitting an enemy champion to mark them for 4 seconds, during which they are revealed. *Shadow Surge* can be recast while the target is marked.

**RECAST:** **Vex** dashes towards the marked target with displacement immunity. Upon arrival, she consumes their mark and deals magic damage.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 150 / 200 / 250 / 300 / 350 (+ 50% AP) |
| **Total Magic Damage** | 225 / 300 / 375 / 450 / 525 (+ 70% AP) |

If **Vex** scores a takedown against *Shadow Surge*’s marked target within 6 seconds of its application, **Vex** can cast *Shadow Surge* again within 12 seconds at no cost after $0.5$ seconds.

**Notes:**

Effect at cast time end
- **Vex**’s dash will track the target if they change locations.
  - She will not stop tracking the target until she reaches them, as there is no maximum tracking distance.
- *Shadow Surge* grants sight of the area along its path for $0.4 (Estimated$) seconds each.
- *Shadow Surge* cannot be recast while grounded or rooted, or if the target is untargetable.
- *Shadow Surge* may also be recast if the target died to the ability.
- The following table refers for interactions while **Vex** is dashing:
  - All movement spells are disabled during the first $0.25$ seconds of the dash.

---

## Patch History

### V25.09
- General
  - **Bug Fixes:** Corrected Homeguard animation.

### V14.9
- Stats
  - Gameplay radius reduced to 55 units from 65.
  - Pathing radius reduced to 30 units from 35.

### V14.5
- Doom 'n Gloom
  - Base damage increased to 40 to 150 from 30 to 140.
  - AP ratio increased to 25% AP from 20% AP.
  - **New Effect:** Doom's cooldown is now also partially refunded upon killing units with Looming Darkness.
    - Champion kills refund the cooldown by 25% and non-champion kills do so by 10%.

### V13.21
- Shadow Surge
  - **Bug Fixes:** Now properly damages minions and monsters hit by the missile.
    - *Note: This fix was already in effect in the previous patch.*

### V13.17
- Mistral Bolt
  - Base damage increased to 70 / 115 / 160 / 205 / 250 from 60 / 105 / 150 / 195 / 240.
- Personal Space
  - Cooldown reduced to 16 / 15 / 14 / 13 / 12 seconds from 20 / 18 / 16 / 14 / 12.
- Shadow Surge
  - **New Effect:** Pinging the ability now displays visible targets in chat.

### V13.13
- Doom 'n Gloom
  - **Bug Fixes:** Now properly applies *Gloom* to a nearby enemy Galio casting Justice Punch.

### V13.6
- Doom 'n Gloom
  - **Bug Fixes:** Can no longer mark an enemy Aurelion Sol dashing from Astral Flight with *Gloom* more than once.

### V13.1
- Doom 'n Gloom
  - **Bug Fixes:** Ability icon is now properly displayed for its buff icon.

### V12.13
- Mistral Bolt
  - AP ratio increased to 70% AP from 60% AP.
  - Cooldown reduced to 8 / 7 / 6 / 5 / 4 seconds from 9 / 8 / 7 / 6 / 5.

### V12.10
- Stats
  - Base health increased to 590 from 520.
  - Health growth increased to 104 from 90.
  - Armor growth increased to $4.45$ from $3.25$.
  - Magic resistance growth increased to $1.3$ from $0.5$.
- Personal Space
  - Base shield reduced to 50 / 75 / 100 / 125 / 150 from 50 / 80 / 110 / 140 / 170.
  - Shield AP ratio reduced to 75% AP from 80% AP.

## Trivia

- Vex was the third of three champions released in 2021 tied to Viego’s return and a continuation of the Ruined King's story.
- Vex's Shadow cannot be disabled in game.
- Vex shares a few similarities with from *Guilty Gear* series.
  - Both fight with a shadow companion. Her shadow baring a slight resemblance to design in the game.
- Vex's laugh animation references the Laughing Spongebob meme.

---
*This page was automatically generated from League of Legends Wiki data.*