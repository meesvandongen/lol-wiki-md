# Aatrox

Aatrox is a champion in League of Legends.

## Overview

- **Title:** The Darkin Blade
- **Roles:** Top, Jungle
- **Resource:** Blood Well

## Stats

| Stat | Base | Growth |
|------|------|--------|
| Armor | 38 | 4.8 |
| Attack Damage | 60 | 5 |
| Attack Speed | 0.651 | 2.5 |
| HP | 650 | 114 |
| HP Regen | 3 | 0.5 |
| MP | 0 | 0 |
| MP Regen | 0 | 0 |
| Magic Resist | 32 | 2.05 |
| Move Speed | 345 | 0 |
| Range | 175 | 0 |

## Advanced Stats

| Metric | Value |
|--------|-------|
| Acquisition Radius | 475 |
| Attack Cast Time (s) | 0.3 |
| Attack Speed Ratio | 0.651 |
| Base Attack Time (s) | 1.52 |
| Pathing Radius | 35 |
| Selection Height | 180 |
| Selection Radius | 135 |
| Windup % | 19.74% |

## Special Statistics

### ARAM

| Metric | Value |
|--------|-------|
| Damage Dealt | 1.05 |
| Damage Taken | 1 |

### Ultra Rapid Fire

| Metric | Value |
|--------|-------|
| Damage Dealt | 1.15 |
| Damage Taken | 0.7 |

## Abilities

### Passive – Deathbringer Stance

| Attribute | Value |
|-----------|------:|
| **Static** | 22 – 10 (based on level) |

**INNATE:** Periodically, **Aatrox** empowers his next basic attack to gain 50 **bonus** range and deal **bonus** magic damage equal to 4% – 10% (based on level) of the target's **maximum** health, capped at 100 – 320 (based on level) against monsters. _Deathbringer Stance's_ damage applies life steal.

**Aatrox** heals for the post-mitigation **bonus** damage (Damage calculated after modifiers) dealt, reduced to 25% against minions.

Whenever **Aatrox** hits at least one enemy champion or large monster with a basic attack on-hit or an ability, _Deathbringer Stance's_ **current** cooldown is reduced by 2 seconds, doubled to 4 if he hits with the Sweetspot of _The Darkin Blade_.

| Detail | Value |
|--------|------:|
| **Targeting** | Passive |
| **Affects** | Self, Enemies |
| **Damage Type** | magic |
| **Spell Effects** | proc |
| **Parry** | Yes |

**Notes:**

- **Aatrox** will assume stance when an enemy champion is in range of Infernal Chains.
  - The [toggle](./Emoting.md) expression (default **CTRL + 5**) switches between stances manually.
- If _Deathbringer Stance_ becomes available during a standard attack's windup, it will not be consumed or trigger the **bonus** damage.
- Even if the ability hit is spell shielded _Deathbringer Stance's_ cooldown will still be reduced.
- This ability goes on cooldown on death and refreshes upon respawn.
- The empowered attack will not trigger against structures and wards.

### Q – The Darkin Blade

| Attribute | Value |
|-----------|------:|
| **Range** | Varied |
| **Cast Time** | 0.6 |
| **Effect Radius** | Varied |
| **Cooldown** | 14 / 12 / 10 / 8 / 6 |
| **Cooldown Start** | Cooldown starts on first cast and restarts on each successive recast |

**Aatrox** can activate _The Darkin Blade_ three times before the ability goes on cooldown, with a 1-second static cooldown (Unaffected by ability haste) between casts. If **Aatrox** does not recast the ability within 4 seconds of the previous cast, it goes on cooldown.

**ACTIVE:** **Aatrox** performs a strike with his greatsword for each of the three casts, dealing physical damage to enemies hit within an area. Enemies hit within a Sweetspot of the area take 70% **bonus** damage and are also knocked up for 0.25 seconds. Each subsequent cast gains 25% more damage.

**FIRST CAST:** **Aatrox's** first strike affects a 625×180-unit rectangular area in the target direction, with him centered on the back line and the Sweetspot at the farthest edge.

**SECOND CAST:** **Aatrox's** second strike affects a trapezoidal area in the target direction, with the Sweetspot at the farthest edge. The hitbox begins 100-units behind **Aatrox** and extends 475-units in front of him, measuring between 300 and 500-units wide from behind to in front.

**THIRD CAST:** **Aatrox's** third strike affects a 300-radius circular area centered on a target location that is 200 units in front of him, with a 180-radius Sweetspot within.

_The Darkin Blade_ deals 55% – 70% (based on level; formula: 55% base + 5% every 5 levels thereafter) damage against minions. Against monsters, the first cast deals 25 **bonus** physical damage, which is affected by the subsequent cast and Sweetspot damage modifiers; and the knock up duration from hitting the Sweetspot against them is doubled to 0.5 seconds.

- **First Cast Damage:** 10 / 25 / 40 / 55 / 70 (+ 60 / 67.5 / 75 / 82.5 / 90% AD)
- **First Sweetspot Damage:** 17 / 42.5 / 68 / 93.5 / 119 (+ 102 / 114.75 / 127.5 / 140.25 / 153% AD)

- **Second Cast Damage:** 12.5 / 31.25 / 50 / 68.75 / 87.5 (+ 75 / 84.38 / 93.75 / 103.13 / 112.5% AD)
- **Second Sweetspot Damage:** 21.25 / 53.13 / 85 / 116.88 / 148.75 (+ 127.5 / 143.44 / 159.38 / 175.31 / 191.25% AD)

- **Third Cast Damage:** 15 / 37.5 / 60 / 82.5 / 105 (+ 90 / 101.25 / 112.5 / 123.75 / 135% AD)
- **Third Sweetspot Damage:** 25.5 / 63.75 / 102 / 140.25 / 178.5 (+ 153 / 172.13 / 191.25 / 210.38 / 229.5% AD)

- **Maximum Non-Minion Non-Sweetspot Damage:** 37.5 / 93.75 / 150 / 206.25 / 262.5 (+ 225 / 253.13 / 281.25 / 309.38 / 337.5% AD)
- **Maximum Non-Minion Sweetspot Damage:** 63.75 / 159.38 / 255 / 350.63 / 446.25 (+ 382.5 / 430.31 / 478.13 / 525.94 / 573.75% AD)

| Detail | Value |
|--------|------:|
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Effects** | spellaoe |
| **Spell Shield** | Yes |

**Notes:**

- The highlighted areas denote the sweetspot - enemies are affected where the two areas overlap (See edge range and center range). The first and second casts function like any point-blank AOE (i.e. an effect centered on the caster). An enemy is considered to be hit by the ability based on edge range - i.e. if any part of your gameplay radius is within the hitbox, you are affected.
- The third cast, as well as the Sweetspot for the first and second cast, function like [ground-targeted](./ground-targeted.md) abilities. An enemy is considered to be hit by the ability based on center range - i.e. an enemy's center has to be within the hitbox to be affected.
- As implied by the previous point, the Sweetspot for the first and second cast is implemented as separate [areas of effect](./area_of_effect.md) to the main component of the ability. Enemies must be within both areas to trigger the bonus damage, knock up and Deathbringer Stance's cooldown reduction.
  - The 'target gets hit' SFX plays whenever an enemy is within the Sweetspot - meaning it is possible to trigger the sound effect without affecting an enemy.
- In the game, Sweetspot damage is incorrectly displayed as a real critical strike.[Bug]
- Each cast counts as an ability activation for the purposes of on-cast effects such as Spellblade and triggering Force Pulse's passive.
- The hitbox and **Aatrox's** model are fixed to the initial target direction.
  - **Aatrox's** facing-direction, for effects such as Petrifying Gaze, is the direction he is moving, and not the direction the model is facing.
- All damage modifiers stack multiplicatively.
- There's a small period of time in which Aatrox can't declare basic attacks after casting _The Darkin Blade_.[Bug]
- This ability's damage is calculated based on the caster's current stats and changes dynamically. (effect determined at cast time end)
- _The Darkin Blade_ maintains its casting angle direction in relation to **Aatrox** if he moves during the cast time. (cast)

### W – Infernal Chains

| Attribute | Value |
|-----------|------:|
| **Range** | 825 • 765 |
| **Cast Time** | 0.25 |
| **Width** | 160 |
| **Speed** | 1800 |
| **Cooldown** | 20 / 18 / 16 / 14 / 12 |

**ACTIVE:** **Aatrox** sends a chain in the target direction that deals physical damage to the first enemy hit, doubled against minions, and slowing them for 1.5 seconds.

If this hits an enemy champion or large monster, a tether is formed between the target and the ground beneath them for 1.5 seconds, during which they are revealed.

If the tether is not broken by the end of its duration, the target is dealt the same physical damage again and pulled to the center of the area.

- **Physical Damage:** 30 / 40 / 50 / 60 / 70 (+ 40% AD)
- **Minion Damage:** 60 / 80 / 100 / 120 / 140 (+ 80% AD)

- **Slow:** 25 / 27.5 / 30 / 32.5 / 35%

- **Total Damage:** 60 / 80 / 100 / 120 / 140 (+ 80% AD)

| Detail | Value |
|--------|------:|
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Effects** | spell |
| **Spell Shield** | Special |
| **Projectile** | Yes |

**Notes:**

- The impact area is oriented relative to **Aatrox's** position when the projectile hits, not where the projectile originated from.
- The location that the target is dragged to is not at the target's original location, but slightly closer towards **Aatrox's** position when the zone expires.
- Spell shield will block the chain's application and initial damage but not the aftereffects of one already applied.
- This ability's damage is calculated based on the caster's current stats and changes dynamically. (effect determined at cast time end)
- _Infernal Chains_ maintains its casting angle direction in relation to **Aatrox** if he moves during the cast time.

### E – Umbral Dash

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 75 - 300 (Minimum and maximum dash distance) • 500 (Maximum extended dash distance across terrain) |
| **Speed** | 800 (Dash speed, slightly higher when measured) • 1340 (Maximum increased dash speed, when dashing through wide terrain) |
| **Cooldown** | 9 / 8 / 7 / 6 / 5 |

**PASSIVE:** **Aatrox** heals for 16% (+ 1.1% per 100 **bonus** health) of non-persistent post-mitigation damage (Damage calculated after modifiers) he deals against enemy champions.

**ACTIVE:** **Aatrox** dashes in the target direction.

_Umbral Dash resets **Aatrox's** basic attack timer and can be cast during his other abilities without cancelling them and vice versa._

| Detail | Value |
|--------|------:|
| **Targeting** | Location |
| **Affects** | Self |
| **Out of Range** | cast at max |
| **Terrain Grace** | Yes |
| **Grounded** | Yes |
| **Knockdown** | Yes |

**Notes:**

- The dash distance can be extended to up-to 500 units when targeting across terrain. It will only extend for the distance needed to cross the wall.
  - This may upscale his dash speed so that the dash takes the same total time as the standard maximum-range dash.
- If _Umbral Dash_ is cast towards a movement command's end point, _Umbral Dash_ will re-issue a movement command to that point when the dash ends.
  - He will be unable to buffer any commands during the dash if it is casted this way. [Bug]
- **Aatrox's** model darkens for 1.5 seconds upon casting _Umbral Dash_, which is a remnant of an attack damage buff he received before it got removed in [V9.9](./V9.9.md).

### R – World Ender

| Attribute | Value |
|-----------|------:|
| **Cast Time** | 0.25 |
| **Effect Radius** | 600 (Flee radius) |
| **Cooldown** | 120 / 110 / 100 / 90 / 80 |
| **Cooldown Start** | on-cast |

**ACTIVE:** **Aatrox** unleashes his true form for 10 seconds, fearing nearby enemy minions and monsters for 3 seconds, during which they are gradually slowed by up to 99% over the duration. He also gains **bonus** movement speed that decays by 10% of the **current bonus** every 0.25 seconds, lasting until _World Ender_ has ended.

Whenever **Aatrox** scores a champion takedown, he extends the duration by 5 seconds, up to its original length, and becomes unleashed again.

During _World Ender_, **Aatrox** gains **bonus** attack damage, has 5% increased [size](./size.md), is ghosted, and receives increased self-healing from all sources.

***Aatrox** can move during World Ender's cast time.*

- **Bonus Movement Speed:** 60 / 70 / 80 / 90 / 100%

- **Bonus Attack Damage:** 20 / 25 / 30 / 35 / 40% AD

- **Increased Healing:** 50 / 62.5 / 75 / 87.5 / 100%

| Detail | Value |
|--------|------:|
| **Targeting** | Auto |
| **Affects** | Self, Enemies |

**Notes:**

- His resource bar indicates the remaining duration of _World Ender_.
- While _World Ender_ is active, the screen will have a red tint. Near the end of the duration, the tint is intensified.
- _World Ender_ persists through resurrection effects.
  - _World Ender's_ healing amplification applies to the health gained by resurrecting.
- _World Ender_ will cast from wherever **Aatrox** is at the end of the cast time.
- Casting _World Ender_ while it is still active removes the timer on the resource bar. [Bug]
  - If this happens while using Primordian, the intensified tint effect of _World Ender_ will stay on the screen for the rest of the game. [Bug]

## Trivia

- Aatrox is the first champion, since Lulu's [patch](./V1.0.0.136.md) introduced champion specific [login screens](./Login_Screen.md), to receive two login screens and themes released for his Classic skin.
- In-game, Aatrox can toggle his banner-style wings and his sword stance [default: Ctrl + 5].
- **Aatrox** is one of the champions that do not have a single ability power ratio on any of their abilities.
- The icon for World Ender is reused for the [Teamfight Tactics](./Teamfight_Tactics.md) item .
- Aatrox's passive, Deathbringer Stance, is a reference to Final Fantasy IV in both name and function. "Deathbringer" is the name of the last sword you get in the game as a Dark Knight and the icon and ability looks exactly how Cecil holds his sword using his "Darkness" ability.
- Aatrox, Elise, Jax, Sejuani, Vayne, and Volibear are the only abilities in _League of Legends_ that have the same name as their champion's title.
- Aatrox's Series 1 [Eternals](./Eternals.md) make the following references:
  - _Chains Yanked_ references the idiom [yank one's chain](https://idioms.thefreedictionary.com/yank+chain) as well as the pull of Infernal Chains.
  - _QQ's_ references the [QQ emoticon](https://www.dictionary.com/e/slang/qq/) as well the multi-cast nature of The Darkin Blade.
- Out-of-universe, _Aatrox_ is a word play on Latin [_atrox_](http://en.wiktionary.org/wiki/atrox) "fierce, savage, cruel" in turn from _ater_ "dull black, dark", from root PIE _*h₂eh₁ter-_ "fire";[Source: DeVaan, M. _Etymoligical Dictionary of Latin and other Italic languages_, p. 60]
  - All of these reference the [Darkin](./Darkin.md) weapon's physical manifestation and demonic nature (punning on _dark kin_), various historical fire-based weapons, the scorched earth strategy, and his 'war as an artform' philosophy. Latin _atrox_ gives rise to Anglo-French _atrocity._
- Enemies hit by Massacre will leave a blood trail between themselves and Aatrox that he will rapidly absorb (this is a remaining visual effect from PBE testing, where Aatrox would gain attack speed based on how many enemies had been hit by Massacre, that was recycled in [V5.6](./V5.6.md)).
- Aatrox can still use emotes during Blood Well's revive.
- Sterak's Gage is speculated to have been **Aatrox's** missing left arm glove. Its passive was also coincidentally similar to Aatrox's.
- Prior to his rework, on the statement "This champion needs an update", players ranked Aatrox 5th in NA; 4th in BR; 3rd in KR and 14th in CN. He ranked 4th overall.[Source: [Riot Blaustoise Aatrox data tweet](https://twitter.com/RiotBlaustoise/status/1031656660006133760)]

## Validation

- Patch box transclusion detected for `Aatrox`, but `Aatrox/Patch history` was not found.
- SkinData transclusion detected, but structured skins rendering is not yet implemented.

