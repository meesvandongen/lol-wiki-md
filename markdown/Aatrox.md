# Aatrox

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
| **Champion** | Aatrox |
| **Title** | the Darkin Blade |
| **Resource** | Blood Well |
| **Range Type** | Melee |
| **Release Date** | 2013-06-13 |
| **Release Patch** | V3.8 |
| **Latest Changes** | V25.12 |
| **Roles** | Juggernaut |
| **Riot Positions** | Top |
| **External Positions** | Top |
| **Blue Essence** | 2400 |
| **Riot Points** | 880 |
| **Difficulty** | 2 |
| **Hero Type** | Fighter |
| **Alt Type** | Tank |
| **Adaptive Type** | Physical |
| **Damage** | 3 |
| **Toughness** | 3 |
| **Control** | 2 |
| **Mobility** | 2 |
| **Utility** | 2 |
| **Style** | 20 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $650.0$ | $+114.0$ |
| **Mana** | $0.0$ | $+0.0$ |
| **Health Regen** | $3.0$ | $+0.5$ |
| **Armor** | $38.0$ | $+4.8$ |
| **Magic Resist** | $32.0$ | $+2.05$ |
| **Attack Damage** | $60.0$ | $+5.0$ |
| **Attack Speed** | $0.651$ | |
| **Movement Speed** | $345.0$ | $+0.0$ |
| **Attack Range** | $175.0$ | $+0.0$ |
| **Base Attack Speed** | $0.651$ | |
| **Attack Speed Ratio** | $0.651$ | |
| **Bonus AS per Level** | $2.5\%$ | |
| **Attack Windup** | $19.7\%$ | |
| **Acquisition Radius** | $475$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $135$ units | |
| **Selection Height** | $180$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $105.0\%$ |
| **Damage Taken** | $100.0\%$ |

#### URF

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $115.0\%$ |
| **Damage Taken** | $70.0\%$ |

## Abilities

### Passive: Deathbringer Stance

| Attribute | Value |
|-----------|------:|
| **Static Cooldown** | 22 to 10 |
| **Targeting** | Passive |
| **Affects** | Self, Enemies |
| **Damage Type** | magic |
| **Spell Shield** | False |
| **Spell Effects** | proc |
| **Parry** | True |

**INNATE:** Periodically, **Aatrox** empowers his next basic attack to gain (range) 50 **bonus** range and deal **bonus** magic damage equal to 4 to 8 of the target's **maximum** health, capped at 100 against monsters. *Deathbringer Stance*’s damage applies life steal at 100% effectiveness.

**Aatrox** heals for 100% of the post-mitigation **bonus** damage (Damage calculated after modifiers) dealt, reduced to 25% against minions.

Whenever **Aatrox** hits at least one enemy champion or large monster with a basic attack on-hit or an ability, *Deathbringer Stance*’s **current** cooldown is reduced by 2 seconds, doubled to 4 if he hits with the Sweetspot of *The Darkin Blade*.

**Notes:**

- **Aatrox** will assume stance when an enemy champion is in range of Infernal Chains.
  - The toggle expression (default **CTRL + 5**) switches between stances manually.
- If *Deathbringer Stance* becomes available during a standard attack's windup, it will not be consumed or trigger the **bonus** damage.
- Even if the ability hit is spell shielded *Deathbringer Stance*’s cooldown will still be reduced.
- This ability goes on cooldown on death and refreshes upon respawn.
- The empowered attack will not trigger against structures and wards.

---

### Q: The Darkin Blade

| Attribute | Value |
|-----------|------:|
| **Range** | Varied |
| **Cast Time** | $0.6$ seconds |
| **Cooldown** | 14 / 12 / 10 / 8 / 6 (Cooldown starts on first cast and restarts on each successive recast) seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |

**Aatrox** can activate *The Darkin Blade* three times before the ability goes on cooldown, with a 1-second static cooldown (Unaffected by ability haste) between casts. If **Aatrox** does not recast the ability within 4 seconds of the previous cast, it goes on cooldown.

**ACTIVE:** **Aatrox** performs a strike with his greatsword for each of the three casts, dealing physical damage to enemies hit within an area. Enemies hit within a Sweetspot of the area take 70% **bonus** damage and also knocked up for $0.25$ seconds. Each subsequent cast gains 25% more damage.

**FIRST CAST:** **Aatrox**’s first strike affects a 625×180-unit rectangular area in the target direction, with him centered on the back line and the Sweetspot at the farthest edge.

| Attribute | Value |
|-----------|------:|
| **First Cast Damage** | 10 / 25 / 40 / 55 / 70 (+ 60 / 67.5 / 75 / 82.5 / 90% AD) |
| **First Sweetspot Damage** | 17 / 42.5 / 68 / 93.5 / 119 (+ 102 / 114.75 / 127.5 / 140.25 / 153% AD) |

**SECOND CAST:** **Aatrox**’s second strike affects a trapezoidal area in the target direction, with the Sweetspot at the farthest edge. The hitbox begins 100-units behind **Aatrox** and extends 475-units in front of him, measuring between 300 and 500-units wide from behind to in front.

| Attribute | Value |
|-----------|------:|
| **Second Cast Damage** | 12.5 / 31.25 / 50 / 68.75 / 87.5 (+ 75 / 84.375 / 93.75 / 103.125 / 112.5% AD) |
| **Second Sweetspot Damage** | 21.25 / 53.125 / 85 / 116.875 / 148.75 (+ 127.5 / 143.438 / 159.375 / 175.312 / 191.25% AD) |

**THIRD CAST:** **Aatrox**’s third strike affects a 300-radius circular area centered on a target location that is 200 units in front of him, with a 180-radius Sweetspot within.

| Attribute | Value |
|-----------|------:|
| **Third Cast Damage** | 15 / 37.5 / 60 / 82.5 / 105 (+ 90 / 101.25 / 112.5 / 123.75 / 135% AD) |
| **Third Sweetspot Damage** | 25.5 / 63.75 / 102 / 140.25 / 178.5 (+ 153 / 172.125 / 191.25 / 210.375 / 229.5% AD) |

*The Darkin Blade* deals 55 to 70 damage against minions, and the knock up duration from hitting the Sweetspot is doubled to $0.5$ seconds against monsters.
|leveling6 

**Notes:**

- The first and second casts function like any point-blank AOE (i.e. an effect centered on the caster). An enemy is considered to be hit by the ability based on edge range - i.e. if any part of your gameplay radius is within the hitbox, you are affected.
- The third cast, as well as the Sweetspot for the first and second cast, function like ground-targeted abilities. An enemy is considered to be hit by the ability based on center range - i.e. an enemy's center has to be within the hitbox to be affected.
- As implied by the previous point, the Sweetspot for the first and second cast is implemented as separate areas of effect to the main component of the ability. Enemies must be within both areas to trigger the bonus damage, knock up and Deathbringer Stance’s cooldown reduction.
  - The 'target gets hit' SFX plays whenever an enemy is within the Sweetspot - meaning it is possible to trigger the sound effect without affecting an enemy.
- In the game, Sweetspot damage is incorrectly displayed as a real critical strike.
- Each cast counts as an ability activation for the purposes of on-cast effects such as Spellblade and triggering Force Pulse’s passive.
- The hitbox and **Aatrox**’s model are fixed to the initial target direction.
  - **Aatrox**’s facing-direction, for effects such as Petrifying Gaze, is the direction he is moving, and not the direction the model is facing.
- All damage modifiers stack multiplicatively.
- There's a small period of time in which Aatrox can't declare basic attacks after casting *The Darkin Blade*.
- This ability's damage is calculated based on the caster's current stats and changes dynamically. Effect at cast time end can be cast. Other abilities are disabled.|interrupts=death

---

### W: Infernal Chains

| Attribute | Value |
|-----------|------:|
| **Range** | 825 / er 765 units |
| **Cast Time** | $0.25$ seconds |
| **Width** | 160 units |
| **Speed** | 1800 units/second |
| **Cooldown** | 20 / 18 / 16 / 14 / 12 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | Special |
| **Spell Effects** | spell |
| **Projectile** | True |

**ACTIVE:** **Aatrox** sends a chain in the target direction that deals physical damage to the first enemy hit, doubled against minions, and slowing them for $1.5$ seconds.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 30 / 40 / 50 / 60 / 70 (+ 40% AD) |
| **Minion Damage** | 60 / 80 / 100 / 120 / 140 (+ 80% AD) |

| Attribute | Value |
|-----------|------:|
| **Slow** | 25 / 27.5 / 30 / 32.5 / 35% |

If this hits an enemy champion or large monster, a tether is formed between the target and the ground beneath them for $1.5$ seconds, during which they are revealed.

If the tether is not broken by the end of its duration, the target is dealt the same physical damage again and pulled to the center of the area.

| Attribute | Value |
|-----------|------:|
| **Total Damage** | 60 / 80 / 100 / 120 / 140 (+ 80% AD) |

**Notes:**

- The impact area is oriented relative to **Aatrox**’s position when the projectile hits, not where the projectile originated from.
- The location that the target is dragged to is not at the target's original location, but slightly closer towards **Aatrox**’s position when the zone expires.
- Spell shield will block the chain's application and initial damage but not the aftereffects of one already applied.
- This ability's damage is calculated based on the caster's current stats and changes dynamically. Effect at cast time end

---

### E: Umbral Dash

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 75 (Minimum dash distance) / 300 (Maximum dash distance) / 500 (Maximum increased dash distance across terrain) units |
| **Speed** | 800 (Slightly higher when measured) / Up to 1340 (When dashing through thick terrain) units/second |
| **Cooldown** | 9 / 8 / 7 / 6 / 5 seconds |
| **Targeting** | Location |
| **Affects** | Self |
| **Out of Range Behavior** | cast at max |
| **Terrain Grace** | True |
| **Grounded** | True |
| **Knockdown** | True |

**PASSIVE:** **Aatrox** heals for 16% (+ $1.1$% per 100 **bonus** health) of non-persistent post-mitigation damage (Damage calculated after modifiers) he deals against enemy champions.

**ACTIVE:** **Aatrox** dashes in the target direction.

*Umbral Dash resets **Aatrox**’s basic attack timer and can be cast during his other abilities without cancelling them and vice versa.*

**Notes:**

- The dash distance can be extended to up-to 500 units when targeting across terrain. It will only extend for the distance needed to cross the wall.
  - This may upscale his dash speed so that the dash takes the same total time as the standard maximum-range dash.
- If *Umbral Dash* is cast towards a movement command's end point, *Umbral Dash* will re-issue a movement command to that point when the dash ends.
  - He will be unable to buffer any commands during the dash if it is casted this way.
- **Aatrox**’s model darkens for $1.5$ seconds upon casting *Umbral Dash*, which is a remnant of an attack damage buff he received before it got removed in V9.9.

---

### R: World Ender

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Effect Radius** | 600 (Flee radius) units |
| **Cooldown** | 120 / 110 / 100 / 90 / 80 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |

**ACTIVE:** **Aatrox** unleashes his true form for 10 seconds, fearing nearby enemy minions and monsters for 3 seconds, during which they are gradually slowed by up to 99% over the duration. He also gains **bonus** movement speed that decays by 10% of the **current bonus** every $0.25$ seconds, lasting until *World Ender* has ended.

| Attribute | Value |
|-----------|------:|
| **Bonus Movement Speed** | 60 / 70 / 80 / 90 / 100% |

Whenever **Aatrox** scores a champion takedown, he extends the duration by 5 seconds, up to its original length, and becomes unleashed again.

During *World Ender*, **Aatrox** gains **bonus** attack damage, has 5% increased size, is ghosted, and receives increased self-healing from all sources.

| Attribute | Value |
|-----------|------:|
| **Bonus Attack Damage** | 20 / 25 / 30 / 35 / 40% AD |

| Attribute | Value |
|-----------|------:|
| **Increased Healing** | 50 / 62.5 / 75 / 87.5 / 100% |

**Notes:**

- His resource bar indicates the remaining duration of *World Ender*.
- While *World Ender* is active, the screen will have a red tint. Near the end of the duration, the tint is intensified.
- *World Ender* persists through resurrection effects.
  - *World Ender*’s healing amplification applies to the health gained by resurrecting.
- *World Ender* will cast from wherever **Aatrox** is at the end of the cast time.
- Casting *World Ender* while it is still active removes the timer on the resource bar.
  - If this happens while using Aatrox, the intensified tint effect of *World Ender* will stay on the screen for the rest of the game.

---

## Patch History

### V25.12
- Deathbringer Stance
  - Heal increased to 100% of post-mitigation damage from 80%.
  - Minion heal increased to 25% of post-mitigation damage from 20%.
- Umbral Dash
  - Heal health ratio increased to $1.1$% per 100 **bonus** health from $0.9$%.

### V25.11
- Deathbringer Stance
  - Damage health ratio reduced to 4 to 8 of the target's **maximum** health from 4 to 12.
- The Darkin Blade
  - Sweetspot bonus damage increased to 70% from 60%.
    - First cast sweetspot base damage increased to 17 / 42.5 / 68 / 93.5 / 119 from 16 / 40 / 64 / 88 / 112.
      - Second cast sweetspot base damage increased to 21.25 / 53.125 / 85 / 116.875 / 148.75 from 20 / 50 / 80 / 110 / 140.
      - Third cast sweetspot base damage increased to 25.5 / 63.75 / 102 / 140.25 / 178.5 from 24 / 60 / 96 / 132 / 168.
      - Maximum base damage increased to 63.75 / 159.375 / 255 / 350.625 / 446.25 from 60 / 150 / 240 / 330 / 420.
    - First cast sweetspot AD ratio increased to 102 / 119 / 136 / 153 / 170% AD from 96 / 112 / 128 / 144 / 160% AD.
      - Second cast sweetspot AD ratio increased to 127.5 / 148.75 / 170 / 191.25 / 212.5% AD from 120 / 140 / 160 / 180 / 200% AD.
      - Third cast sweetspot AD ratio increased to 153 / 178.5 / 204 / 229.5 / 255% AD from 144 / 168 / 192 / 216 / 240% AD.
      - Maximum AD ratio increased to 382.5 / 446.25 / 510 / 573.75 / 637.5% AD from 360 / 420 / 480 / 540 / 600% AD.
- Infernal Chains
  - Damage type changed to physical from magic.

### V25.10
- Infernal Chains
  - **Bug Fixes:** Now once again has the proper tooltip description for the debuff after a tooltip change to Aurora in patch 25.09 unintentionally affected it.

### V25.04
- Infernal Chains
  - **Bug Fixes:** Corrected tether visual.

### V25.S1.3#February 6th Hotfix|V25.S1.3
- The Darkin Blade
  - **UNDOCUMENTED / BUG FIX:** Minion damage modifier now properly applies to non-*Sweetspot* hits.
  - **Undocumented:** In the floating text display, now once again uses the incorrect

### V25.S1.3
- The Darkin Blade
  - **Bug Fixes:** In the floating text display, now uses the proper

### V14.23
- Deathbringer Stance
  - **Bug Fixes:** Attack can now trigger both this ability's damage and any and on-hit effects when the attack kills the target. *Deathbringer Stance*’s damage is no longer discarded, previously causing **Aatrox** to lose the healing.

### V14.22
- World Ender
  - Bonus attack damage reduced to 20 / 30 / 40% AD from 20 / 32.5 / 45% AD.

### V14.17
- Umbral Dash
  - **Bug Fixes:** Issued basic attacks no longer unintentionally cancel when dashing towards an enemy.

### V14.14
- Deathbringer Stance
  - **Bug Fixes:** Attack now triggers attack effects if damage originating from *Deathbringer Stance* would have killed the target.
    - *The attack effects' damage now applies first/is prioritized, which may lead to losing the ability's healing.*

## Trivia

- Aatrox is the first champion, since Lulu’s patch introduced champion specific login screens, to receive two login screens and themes released for his Classic skin.
- In-game, Aatrox can toggle his banner-style wings and his sword stance [default: Ctrl + 5].
- This champion has no ability power ratio.
- The icon for World Ender is reused for the Teamfight Tactics item Darkin.
- Aatrox's passive, Deathbringer Stance, is a reference to Final Fantasy IV in both name and function. "Deathbringer" is the name of the last sword you get in the game as a Dark Knight and the icon and ability looks exactly how Cecil holds his sword using his "Darkness" ability.
- Aatrox’s The Darkin Blade, Jax’s Grandmaster-at-Arms, Vayne’s Night Hunter, and Volibear’s The Relentless Storm are the only abilities in *League of Legends* that have the same name as their champion's title.
- Aatrox's Series 1 Eternals make the following references:
  - *Chains Yanked* references the idiom yank one's chain as well as the pull of Infernal Chains.
  - *QQ*’s references the QQ emoticon as well the multi-cast nature of The Darkin Blade.
- Out-of-universe, *Aatrox* is a word play on Latin *atrox* "fierce, savage, cruel" in turn from *ater* "dull black, dark", from root Proto-Indo-European languages **h₂eh₁ter-* "fire";
  - All of these reference the Darkin weapon's physical manifestation and demonic nature (punning on *dark kin*), various early thermal weapons, the scorched earth strategy, and his 'The Art of War' philosophy. Latin *atrox* gives to Anglo-French *atrocity.*
- Enemies hit by Massacre will leave a blood trail between themselves and Aatrox that he will rapidly absorb (this is a remaining visual effect from PBE testing, where Aatrox would gain attack speed based on how many enemies had been hit by Massacre, that was recycled in V5.6).
- Aatrox can still use emotes during Blood Well’s revive.
- Sterak's Gage is speculated to have been **Aatrox**’s missing left arm glove. Its passive was also coincidentally similar to Aatrox's.
- Prior to his rework, on the statement "This champion needs an update", players ranked Aatrox 5th in NA; 4th in BR; 3rd in KR and 14th in CN. He ranked 4th overall.

---
*This page was automatically generated from League of Legends Wiki data.*