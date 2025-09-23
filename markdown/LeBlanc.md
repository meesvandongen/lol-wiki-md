# LeBlanc

## Table of Contents
- [Basic Information](#basic-information)
- [Statistics](#statistics)
  - [Map-specific Stats](#map-specific-stats)
- [Pets](#pets)
- [Abilities](#abilities)
- [Patch History](#patch-history)
- [Trivia](#trivia)

## Basic Information

| Attribute | Value |
|-----------|------:|
| **Champion** | LeBlanc |
| **Title** | the Deceiver |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2010-11-02 |
| **Release Patch** | V1.0.0.104 |
| **Latest Changes** | V25.18 |
| **Roles** | Burst, Assassin |
| **Riot Positions** | Middle |
| **External Positions** | Middle |
| **Blue Essence** | 1575 |
| **Riot Points** | 790 |
| **Difficulty** | 2 |
| **Hero Type** | Assassin |
| **Alt Type** | Mage |
| **Adaptive Type** | Magic |
| **Damage** | 3 |
| **Toughness** | 1 |
| **Control** | 2 |
| **Mobility** | 3 |
| **Utility** | 1 |
| **Style** | 100 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $598.0$ | $+111.0$ |
| **Mana** | $400.0$ | $+25.0$ |
| **Health Regen** | $7.5$ | $+0.55$ |
| **Mana Regen** | $8.0$ | $+0.8$ |
| **Armor** | $22.0$ | $+4.7$ |
| **Magic Resist** | $30.0$ | $+1.3$ |
| **Attack Damage** | $55.0$ | $+2.2$ |
| **Attack Speed** | $0.658$ | |
| **Movement Speed** | $340.0$ | $+0.0$ |
| **Attack Range** | $525.0$ | $+0.0$ |
| **Base Attack Speed** | $0.658$ | |
| **Attack Speed Ratio** | $0.4$ | |
| **Bonus AS per Level** | $2.4\%$ | |
| **Missile Speed** | $1700$ units/second | |
| **Acquisition Radius** | $525$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $100$ units | |
| **Selection Height** | $85$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $110.0\%$ |
| **Damage Taken** | $90.0\%$ |

## Pets

### Mirror Image

| Attribute | Value |
|-----------|------:|
| **Range** | |
| **Health** | 100% of **LeBlanc**’s **current** health |
| **Armor** | 100% of **LeBlanc**’s armor |
| **Magic Resist** | 100% of **LeBlanc**’s magic resistance |
| **Damage** | 0 |
| **Damage Type** | Physical |
| **Attack Speed** | 100% of **LeBlanc**’s attack speed |
| **Move Speed** | 100% of **LeBlanc**’s movement speed |
| **Control** | **R** or **ALT + RIGHT CLICK** |
| **Targeting** | Champion |
| **Spell Effects** | Basic attacks do not apply spell effects. |

---

## Abilities

### Passive: Mirror Image

| Attribute | Value |
|-----------|------:|
| **Static Cooldown** | 60 |
| **Targeting** | Passive |
| **Affects** | Self |

**INNATE:** Periodically, when **LeBlanc** is damaged to health, she creates a clone of herself, after which they both instantly become invisible for 1 second.

The clone can cast basic attacks with no effect and lasts 8 seconds. Upon spawning, it starts moving up to 1900 units toward a random direction in front of **LeBlanc**’s movement, determined at the moment of its spawning.

*See [Pets](#Pets) for more details about **LeBlanc**’s clone.*

**Notes:**

- The clone has a leash range.
- Using a basic attack breaks the stealth at the end of the attack windup.

---

### Q: Sigil of Malice

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 700 units |
| **Speed** | 2000 units/second |
| **Cost** | 50 Mana |
| **Cooldown** | 6 seconds |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Spell |
| **Projectile** | True |
| **Call For Help** | True |

**ACTIVE:** **LeBlanc** projects an orb at the target enemy that deals magic damage and marks them for $3.5$ seconds.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 65 / 90 / 115 / 140 / 165 (+ 40% AP) |

**LeBlanc**’s next damaging ability against the marked target will consume the mark to deal the same magic damage again.

| Attribute | Value |
|-----------|------:|
| **Total Magic Damage** | 130 / 180 / 230 / 280 / 330 (+ 80% AP) |

*Sigil of Malice*’s orb deals 10 to 146 **bonus** magic damage against minions and refunds 100% of its mana cost and 30% of its remaining cooldown if either the orb or its mark's consumption kills the target.

**Notes:**

- *Sigil of Malice* and Mimic: Sigil of Malice can detonate each other's marks.
- The floating damage text upon the mark's detonation currently displays a real critical strike icon instead of a success indicator.

---

### W: Distortion

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 600 (Dash range) / Global (Recast radius) |
| **Effect Radius** | 240 (Damage radius) units |
| **Speed** | 1450 units/second |
| **Cost** | 60 / 70 / 80 / 90 / 100 Mana |
| **Cooldown** | 15 / 13.75 / 12.5 / 11.25 / 10 seconds |
| **Cooldown Start** | on-cast |
| **Queue Time** | $0.5$ (Initial cast) / $0.3$ (Recast) seconds |
| **Targeting** | Location / Auto |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Out of Range Behavior** | Target at maximum range (first cast clamped) |
| **Grounded** | True |
| **Knockdown** | True |

**ACTIVE:** **LeBlanc** dashes to the target location, dealing magic damage to all nearby enemies upon arrival and leaving a return pad at the cast location for 4 seconds. *Distortion* can be recast after $0.2$ seconds of the dash ending for the pad's duration.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 75 / 115 / 155 / 195 / 235 (+ 70% AP) |

**RECAST:** **LeBlanc** blinks to *Distortion*’s return pad, regardless of range.

**Notes:**

- *Distortion* will be buffered and cast as soon as the cooldown ends if the player attempts to cast it within $0.5$ seconds of the cooldown ending.
  - The **RECAST** will be buffered and cast as soon as it becomes available if the player attempts to cast it within $0.3$ seconds of it becoming available to cast. *The initial cast counts as an ability activation for the purposes of on-cast effects such as Spellblade and triggering Force Pulse’s passive. **Recasting the ability does not. **Mimic: Distortion has the same properties.
- *Distortion* and Mimic: Distortion have independent return pads and **LeBlanc** can travel to both while they are active.
- *Distortion*’s pad duration starts once the dash ends.

---

### E: Ethereal Chains

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 950 units |
| **Tether Radius** | 865 (Equivalent to 995 center-to-center range against the average enemy champion) units |
| **Width** | 110 units |
| **Speed** | 1750 units/second |
| **Cost** | 50 Mana |
| **Cooldown** | 14 / 13.25 / 12.5 / 11.75 / 11 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | Special |
| **Spell Effects** | spell |
| **Projectile** | True |

**ACTIVE:** **LeBlanc** flings an illusory chain in the target direction that deals magic damage to the first enemy it hits and forms a tether between **LeBlanc** and the target for $1.5$ seconds, during which they are revealed.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 50 / 70 / 90 / 110 / 130 (+ 40% AP) |

If the tether is not broken by the end of its duration, it fractures to deal magic damage to the target and root them for $1.5$ seconds, during which they are revealed.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 80 / 120 / 160 / 200 / 240 (+ 85% AP) |
| **Total Damage** | 130 / 190 / 250 / 310 / 370 (+ 125% AP) |

**Notes:**

- Both the initial and the delayed damage can detonate Sigil of Malice.
- *Ethereal Chains* and Mimic: Ethereal Chains are separate debuffs, and can both be tethered to the same target without overriding each other. The root duration for either one are unchanged.
- Spell shield will block the tether's application and initial damage but not the aftereffects of one already applied.
- The tether remains for a minimum of $0.25$ seconds regardless of distance. Effect at cast time end

---

### R: Mimic

| Attribute | Value |
|-----------|------:|
| **Cooldown** | 45 / 40 / 35 / 30 / 25 seconds |
| **Queue Time** | $0.5$ (Mimic: Distortion initial cast) / $0.3$ (Mimic: Distortion recast) seconds |
| **Targeting** | Varied |
| **Affects** | Self |
| **Damage Type** | Magic |

**ACTIVE:** **LeBlanc** casts a mimicked version of her most recently used basic ability, applying the same effects and dealing ***modified** magic damage.

** The orb deals modified damage, while the mark consumption deals double that amount.

| Attribute | Value |
|-----------|------:|
| **Orb Magic Damage** | 70 / 105 / 140 / 175 / 210 (+ 40% AP) |
| **Mark Magic Damage** | 140 / 210 / 280 / 350 / 420 (+ 80% AP) |
| **Total Magic Damage** | 210 / 315 / 420 / 525 / 630 (+ 120% AP) |

** Deals modified damage.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 150 / 225 / 300 / 375 / 450 (+ 75% AP) |

** The tether's application deals modified damage, while its fracturing deals double that amount.

| Attribute | Value |
|-----------|------:|
| **Application Magic Damage** | 70 / 105 / 140 / 175 / 210 (+ 40% AP) |
| **Fracture Magic Damage** | 140 / 210 / 280 / 350 / 420 (+ 85% AP) |
| **Total Magic Damage** | 210 / 315 / 420 / 525 / 630 (+ 125% AP) |

**Notes:**

- *Mimic* will default to Mimic: Sigil of Malice if *LeBlanc* ranks up *Mimic* without having used any abilities beforehand (even if she has not yet learned Sigil of Malice).
- Both Mimic: Ethereal Chains and Mimic: Sigil of Malice will apply damage twice if the original ability's condition to do so is met.

---

## Patch History

### V25.18
- Distortion & Mimic: Distortion
  - **Bug Fixes:** Recall props no longer sometimes appear during the dash.

### V25.15
- General
  - **Bug Fixes:** Cape disappearance animation now properly plays on death for certain skins.

### V25.12
- Ethereal Chains
  - Detonation AP ratio increased to 85% AP from 80% AP.
- Mimic: Ethereal Chains
  - Detonation AP ratio increased to 85% AP from 80% AP.
- Mimic
  - Cooldown reduced to 45 / 35 / 25 seconds from 50 / 40 / 30.

### V25.10
- Mimic: Distortion
  - **Bug Fixes:** No longer deals the non-mimicked version's damage when cast very quickly after Distortion.

### V25.07
- General
  - Updated ability icons.
  - Complete visual update across all skins.
    - New splash artwork for LeBlanc, LeBlanc, LeBlanc and LeBlanc.
    - Adjusted splash artwork for LeBlanc, LeBlanc, LeBlanc, LeBlanc, LeBlanc, LeBlanc, LeBlanc, LeBlanc, LeBlanc, and LeBlanc.
  - New voice-over.
  - Updated sound effects.
  - LeBlanc cost increased to from .
  - LeBlanc cost increased to from .

### V25.04
- LeBlanc
  - **Bug Fixes:** Non-damaging displacement effects can no longer be prematurely canceled by using the Toggle expression during the movement.

### V25.S1.2
- LeBlanc
  - Updated skin border to reflect Faker's fifth World Championship title.

### V14.24
- Sigil of Malice
  - Base damage reduced to 65 / 90 / 115 / 140 / 165 from 70 / 95 / 120 / 145 / 170.
- Mimic: Distortion
  - **Bug Fixes:** No longer reveals the user if cast while the cursor is near an enemy champion.

### V14.17
- Sigil of Malice
  - AP ratio reduced to 40% AP from 45% AP.
    - Total AP ratio reduced to 80% AP from 90% AP.
- Distortion
  - AP ratio reduced to 70% AP from 75% AP.

### V14.14
- LeBlanc
  - Mirror Image
    - **Bug Fixes:** Restored clone death SFX.

## Trivia

- LeBlanc is the first champion to have a Loading screen and Portrait icon not focus the actual champion, in her case, focus on her Mimic.
  - The second one is Jhin, focus on the mirror image.
  - The same case applies for some of her skins: LeBlanc, LeBlanc, LeBlanc and LeBlanc.
- Mimic used to be the only ability (currently Relentless Pursuit, Inferno Trigger, Divine Judgement and Purge) that ended up costing 0 mana when fully-ranked.
- LeBlanc is one of a few champions to have multiple textures in one skin. When she uses Mirror Image, her clone will be of a different colour palette which can only be seen by LeBlanc and her allies. To the opposing team, LeBlanc and her clone look exactly the same, including copies of any external buffs and item effects active at the moment the clone is spawned (Prior to patch V5.22, clones did not duplicate external buff/active item effect particles).
  - Four other champions with this feature are Wukong (via Warrior Trickster), Shaco’s (via Hallucinate), Nasus (via Fury of the Sands) and Malphite (via Granite Shield and Thunderclap).
- The Black Rose might have been inspired by and/or might be referencing the Black Hand.
- Her staff appears to be inspired, partially or in full, by that of Tsukasa from .hack//SIGN.
- The name 'LeBlanc' may be a reference to Maurice Leblanc, creator of the character Arsène Lupin. Like Lupin, part of LeBlanc's character is that of a master of disguise.
- For a very short time while her 2025 Visual Update was on PBE (patch V25.07), LeBlanc was illegally able to cast her Sigil of Malice and Ethereal Chains during Distortion’s dash.

---
*This page was automatically generated from League of Legends Wiki data.*