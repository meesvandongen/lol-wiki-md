# Udyr

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
| **Champion** | Udyr |
| **Title** | the Spirit Walker |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2009-12-02 |
| **Release Patch** | V1.0.0.61 |
| **Latest Changes** | V25.11 |
| **Roles** | Juggernaut |
| **Riot Positions** | Top, Jungle |
| **External Positions** | Top, Jungle |
| **Blue Essence** | 675 |
| **Riot Points** | 585 |
| **Difficulty** | 1 |
| **Hero Type** | Fighter |
| **Alt Type** | Tank |
| **Adaptive Type** | Physical |
| **Damage** | 2 |
| **Toughness** | 3 |
| **Control** | 2 |
| **Mobility** | 2 |
| **Utility** | 1 |
| **Style** | 35 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $664.0$ | $+92.0$ |
| **Mana** | $271.0$ | $+50.0$ |
| **Health Regen** | $6.0$ | $+0.75$ |
| **Mana Regen** | $7.5$ | $+0.45$ |
| **Armor** | $31.0$ | $+4.7$ |
| **Magic Resist** | $32.0$ | $+2.05$ |
| **Attack Damage** | $62.0$ | $+4.0$ |
| **Attack Speed** | $0.650$ | |
| **Movement Speed** | $350.0$ | $+0.0$ |
| **Attack Range** | $125.0$ | $+0.0$ |
| **Base Attack Speed** | $0.65$ | |
| **Attack Speed Ratio** | $0.65$ | |
| **Bonus AS per Level** | $3.0\%$ | |
| **Acquisition Radius** | $400$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $125$ units | |
| **Selection Height** | $180$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $100.0\%$ |

#### URF

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $110.0\%$ |
| **Damage Taken** | $92.0\%$ |

## Abilities

### Passive: Bridge Between

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Self |
| **Parry** | True |
| **Cooldown (Unaffected by ability haste, but affected by ultimate haste)** | 50–20@1–16 |

**INNATE - AWAKENED SPIRIT:** **Udyr** has no ultimate ability and instead has 4 basic abilities that each incur a (cd) $1.5$-second global cooldown (Affected by ability haste) when cast. Each ability grants a *Stance* that empowers his basic attacks, and switching *Stances* will replace the empowered attacks of the previous *Stance*.

Periodically, after **Udyr** enters a *Stance*, he can cast it again at no cost after $0.25$ seconds and within the following 5 seconds to *Awaken* it, empowering the *Stance* with an additional effect and placing the effect on cooldown.
If **Udyr** does not consume the *Awaken* cast within the duration and has not switched *Stances*, his current *Stance*’s respective ability incurs a $0.25$-second cooldown. *Awakened Spirit* is placed on full cooldown at the start of the game.

**INNATE - MONK TRAINING:** After casting an ability, **Udyr** empowers his next two basic attacks within 4 seconds to gain 30% **bonus** attack speed and refund 5% of *Awakened Spirit*’s **total** cooldown, though not if the remaining cooldown is less than 1 second.

**Notes:**

- *Awakened Spirit*’s cooldown is affected by ah ultimate haste.
- *Awakened* ability casts are special cased to trigger Experimental Hexplate Overdrive, Zeke's Convergence Frostfire Tempest, and possessive=true increased movement speed.
- The *Awakened* casts of *Wilding Claw* and *Wingborne Storm* are special cased to trigger Malignance Hatefog from their damage.
- Axiom Arcanist will affect the *Awakened* cast of all abilities, but does not refund the cooldown of *Awakened Spirit*.
  - Only *Wilding Claw’s* lightning damage is amplified. The on-hit damage is not increased.
- Axiom Arc Flux does not count neither *Awakened Spirit* nor *Wingborne Storm* as ultimate abilities.
- *Awakened Spirit*’s cooldown is tracked on his health bar by an Awakening resource, visible to all teams. This secondary resource bar is non-descript and represents the effect's percentage charge.
  - The bar has a **white** color when *Awakened Spirit* is under 80% charge, a **yellow** color when at 80% charge or above, and a **red** color when an *Awaken* cast is available.
- Switching *Stances* will not cause any additional effects granted by the previous one to end prematurely.
- The global cooldown will not incur on abilities that are currently on a greater cooldown.
- The empowered attacks will not get consumed if they are parried.
- *Awakened Spirit*’s cast also activates *Monk Training* and empowers the next 2 attacks by the stances unawakened effects as well, overriding unused unawakened stance attacks.

---

### Q: Wilding Claw

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 450 (Lightning bounce radius) units |
| **Cost** | 20 Mana |
| **Cooldown** | 6 (Starts on-cast, but recasting refreshes the cooldown) seconds |
| **Cooldown Start** | on-cast |
| **Queue Time** | $0.15$ seconds |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical Magic |
| **Spell Shield** | Special |
| **Spell Effects** | Special |
| **Projectile** | True |
| **Parry** | True |

**ACTIVE - STANCE:** **Udyr** enters *Claw Stance*, empowering his next two basic attacks to gain (range) 50 **bonus** range and deal **bonus** physical damage, capped at 15 (+ 100% **bonus** AD) (+ 50% AP) against monsters.

| Attribute | Value |
|-----------|------:|
| **Bonus Physical Damage** | 3 / 4 / 5 / 6 / 7 / 8% (+ 4% per 100 **bonus** AD) of target's **maximum** health |
| **Total Physical Damage** | 6 / 8 / 10 / 12 / 14 / 16% (+ 8% per 100 **bonus** AD) of target's **maximum** health |

Additionally, **Udyr** gains **bonus** attack speed and deals **bonus** physical damage on-hit for 4 seconds.

| Attribute | Value |
|-----------|------:|
| **Bonus Attack Speed** | 20 / 32 / 44 / 56 / 68 / 80% |

| Attribute | Value |
|-----------|------:|
| **Bonus Physical Damage On-Hit** | 5 / 11 / 17 / 23 / 29 / 35 (+ 25% **bonus** AD) |

**RECAST - AWAKEN:** **Udyr** gains an additional 20 to 70 **bonus** attack speed. The empowered attacks deal an additional 2 to 4 (+ 3% per 100 **bonus** AD) of the target's **maximum** health **bonus** physical damage. **Udyr**’s next two basic attacks in any *Stance* within 4 seconds are each empowered to strike with lightning「 6 times over 1 second, ⟷ every $0.2$ seconds (First strike is applied instantly, while the remaining 5 are applied every 0.2 seconds thereafter) over 1 second, 」dealing **bonus** magic damage equal to 1.5 to 3 (+ $0.8$% per 100 AP) of the target's **maximum** health per hit, chaining to up to 3 nearby visible enemies and being able to hit the same target multiple times, for a total of 1.5×6 to 3×6 (+ 4.8% per 100 AP) of the target's **maximum** health. Each lightning strike deals a minimum of 40 to 160 against minions and is capped at 15 (+ 100% **bonus** AD) (+ 50% AP) against monsters.

**Notes:**

- Deals spell damage to the primary target and applies area damage on the lightning strikes.
- The lightning strikes can deal up to 1.5×6 to 3×6 (+ 4.8% per 100 AP) of the target's **maximum** health **bonus** magic damage per empowered basic attack against an isolated target. **If both empowered attacks are used on the same isolated target, the lightning strikes can deal up to 3×6 to 6×6 (+ 9.6% per 100 AP) of the target's **maximum** health **bonus** magic damage.
- The lightning strikes from two separate empowered attacks can overlap with each other on the same target for additional damage.
- Spell shield will only block an empowered attack's application of the lightning strikes.
- The empowered attacks will not trigger nor be consumed if they hit a structure or ward or are parried.

---

### W: Iron Mantle

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Cost** | 40 to 23 Mana |
| **Cooldown** | 6 (Starts on-cast, but recasting refreshes the cooldown) seconds |
| **Cooldown Start** | on-cast |
| **Queue Time** | $0.15$ seconds |
| **Targeting** | Auto |
| **Affects** | Self |
| **Spell Effects** | spell |
| **Parry** | True |

**ACTIVE - STANCE:** **Udyr** enters *Mantle Stance*, empowering his next two basic attacks to gain life steal and heal him for $1.2$% of his **maximum** health (+ 8% AP) on-hit.

| Attribute | Value |
|-----------|------:|
| **Life Steal** | 15 / 16 / 17 / 18 / 19 / 20% |

Additionally, **Udyr** grants himself a shield for 4 seconds.

| Attribute | Value |
|-----------|------:|
| **Shield Strength** | 45 / 60 / 75 / 90 / 105 / 120 (+ 40% AP) (+ 2 / 2.3 / 2.6 / 2.9 / 3.2 / 3.5% **maximum** health) |

**RECAST - AWAKEN:** **Udyr** increases the shield's strength, stacking with the remaining shield from the first cast, and heals every $0.25$ seconds over the next 4 seconds, as well as causes the empowered attacks to instead gain doubled life steal and heal him for $2.4$% of his **maximum** health (+ 16% AP) on-hit.

| Attribute | Value |
|-----------|------:|
| **Increased Shield Strength** | 20 to 150 (+ 45 / 60 / 75 / 90 / 105 / 120) (+ 65% AP) (+ 8% **maximum** health) |

| Attribute | Value |
|-----------|------:|
| **Total Healing** | 20/2 to 150/2 (+ 22.5 / 30 / 37.5 / 45 / 52.5 / 60) (+ 32.5% AP) (+ 4% **maximum** health) |
| **Heal per Tick** | (+ 45/#expr: #var:w_sd/2 to 120/#expr: #var:w_sd/2 6) (+ 65/#expr: (+ 8/
\| Attribute \| Value \|
\|-----------\|------:\|
\| **Increased Life Steal** \| 30 / 32 / 34 / 36 / 38 / 40% \| |

Healing on-hit is reduced to 60% against minions.

**Notes:**

- The empowered attacks apply spell effects to the target.
- The empowered attacks will not trigger nor be consumed if they hit a structure or ward or are parried.
- The increased life steal is a buff that expires after two valid attacks have been completed, as opposed to an on-attack modifier for your next two attacks. This means that while the buff is active, Ravenous Hydra will heal at the increased life steal value.

---

### E: Blazing Stampede

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Cost** | 40 to 23 Mana |
| **Cooldown** | 6 (Starts on-cast, but recasting refreshes the cooldown) seconds |
| **Cooldown Start** | on-cast |
| **On-target CD Static** | 6 / 5.6 / 5.2 / 4.8 / 4.4 / 4 |
| **Queue Time** | $0.15$ seconds |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Spell Shield** | True |
| **Parry** | True |
| **Grounded** | Special |
| **Knockdown** | Special |

**ACTIVE - STANCE:** **Udyr** enters *Stampede Stance*, empowering his basic attacks to have an uncancellable windup and to stun them for $0.75$ seconds. This cannot affect the same target more than once every few seconds.

Additionally, **Udyr** becomes ghosted and gains **bonus** movement speed for 4 seconds, which decays to 30% effectiveness over $1.5$ seconds.

| Attribute | Value |
|-----------|------:|
| **Bonus Movement Speed** | 25 / 31 / 37 / 43 / 49 / 55% |

| Attribute | Value |
|-----------|------:|
| **Decayed Bonus Movement Speed** | 7.5 / 9.3 / 11.1 / 12.9 / 14.7 / 16.5% |

**RECAST - AWAKEN:** **Udyr** gains (range) 75 **bonus** attack range and an additional 30 to 40 **bonus** movement speed. He also gains immunity to crowd control for $1.5$ seconds.

**Notes:**

- **Udyr** cannot dash while grounded or rooted, but he will still declare the empowered attack.
- The empowered attack will still apply its effects even if the dash is interrupted.
- The empowered attacks will not trigger nor be consumed if they hit a structure or ward or are parried.
  - The immunity to the stun is not applied in any case.

---

### R: Wingborne Storm

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 370 (Storm and empowered attack radius, center to edge) / cr 1300 (Storm lock-on radius) units |
| **Speed** | 109.5 / 109.68 / 109.85 / 111.47 / 120.29 / 129.12 / 137.94 / 146.76 / 155.59 / 164.41 / 173.24 / 182.06 / 190.88 / 199.71 / 208.53 / 217.35 / 232.35 / 250 units/second |
| **Cost** | 40 to 23 Mana |
| **Cooldown** | 6 (Starts on-cast, but recasting refreshes the cooldown) seconds |
| **Cooldown Start** | on-cast |
| **Queue Time** | $0.15$ seconds |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | magic |
| **Spell Shield** | False |
| **Spell Effects** | aoedot |
| **Parry** | True |

**ACTIVE - STANCE:** **Udyr** enters *Storm Stance*, empowering his next two basic attacks to deal 10 to 40 (+ 35% AP) magic damage to nearby enemies.

Additionally, **Udyr** summons a blizzard around himself for 4 seconds that deals magic damage every $0.5$ seconds to nearby enemies and slows them while they remain within. Minions take 50%–80%@1–16 damage.

| Attribute | Value |
|-----------|------:|
| **Magic Damage per Tick** | 10 / 18 / 26 / 34 / 42 / 50 (+ $17.5$% AP) |
| **Total Magic Damage** | 80 / 144 / 208 / 272 / 336 / 400 (+ 140% AP) |

| Attribute | Value |
|-----------|------:|
| **Slow** | 15 / 18 / 21 / 24 / 27 / 30% |

**RECAST - AWAKEN:** **Udyr** ends the blizzard if it is active and unleashes a glacial storm for 4 seconds that is able to move on its own, as well as causes the empowered attacks to instead deal their damage to enemies within that storm. The storm applies the same effects as the blizzard but deals **bonus** magic damage equal to「 8/8 to 14/8 (+ 0.4375% per 100 AP) of the target's **maximum** health per tick ⟷ 8 to 14 (+ $3.5$% per 100 AP) of the target's **maximum** health over the duration 」and slows by an additional 5%. Against monsters, the storm deals a minimum of 「 40/8 to 280/8 per tick, and is capped at 80/8 to 400/8 ⟷ 40 to 280 over the duration, and is capped at 80 to 400 」.

| Attribute | Value |
|-----------|------:|
| **Increased Slow** | 20 / 23 / 26 / 29 / 32 / 35% |

The storm grants sight of its surroundings (Does not permeate through terrain or brush) and prioritizes following the last enemy **Udyr** attacked while it was active, then enemy champions, then non-champions, then **Udyr**. The storm can only follow visible enemies.

**Notes:**

- The storm that **Udyr** surrounds himself with is independent from the unleashed storm.
  - He can still cast *Wingborne Storm* to surround himself with a storm while an unleashed storm from an *Awakened* cast is active.
  - Using the *Awakened* cast ends the storm around **Udyr** if it is active and summons a separate storm that is able to move on its own.
- The slow debuff has a duration equal to the storm's remaining duration and is removed immediately when the unit leaves the storm's area of effect.
  - Units receive the debuff whenever they are in the storm, even after having it removed from leaving the area.
- If the target **Udyr** has most recently attacked is invalid and there are no nearby enemies then the storm will follow him instead.
  - The storm can follow **Udyr** regardless of how far away he is from it.
- The storm will remain active even if **Udyr** dies. Any commands it was issued to follow a target will not be interrupted.
- The storm will follow its target even if they are not visible, but only for up to 1 second.
  - If it does not gain vision of its target within that time, it changes its target.
- The storm is obstructed by terrain. If it encounters terrain, then it will automatically navigate its path towards the target.
- The empowered attacks will not trigger nor be consumed if they hit a structure or ward or are parried.

---

## Patch History

### V25.11
- Wingborne Storm
  - Flame attack base damage increased to 10 to 40 from 10 to 30.
  - Flame attack AP ratio increased to 35% AP from 30% AP.
  - Storm base damage per tick increased to 10 / 18 / 26 / 34 / 42 / 50 from 10 / 17 / 24 / 31 / 38 / 45.
    - Total base damage increased to 80 / 144 / 208 / 272 / 336 / 400 from 80 / 136 / 192 / 248 / 304 / 360.

### V25.S1.3
- Bridge Between
  - **Bug Fixes:** Axiom Arcanist now correctly refunds Awaken's cooldown.

### V14.22
- General
  - **Bug Fixes:** Non-Awakened ability casts no longer trigger Experimental Hexplate Overdrive and Zeke's Convergence Frostfire Tempest.
- Wingborne Storm
  - Base damage per tick reduced to 10 / 17 / 24 / 31 / 38 / 45 from 10 / 19 / 28 / 37 / 46 / 55.
    - Total base damage reduced to 80 / 136 / 192 / 248 / 304 / 360 from 80 / 152 / 224 / 296 / 368 / 440.
  - Awaken health ratio reduced to 8/8 to 14/8 target **maximum** health from 8/8 to 16/8.
  - Monster damage cap per tick changed to 10 to 50 per tick from 5 to 35.
    - Total monster damage cap changed to 10×8 to 50×8 per tick from 5×8 to 35×8.

### V14.20
- Wilding Claw
  - Awaken base monster damage reduced to 15 from 20.
- Blazing Stampede
  - Bonus movement speed reduced to 25 / 31 / 37 / 43 / 49 / 55% from 30 / 36 / 42 / 48 / 54 / 60%.
- Wingborne Storm
  - Monster damage cap per tick reduced to 5 to 35 from 10 to 50.
    - Total monster damage cap reduced to 5×8 to 35×8 from 10×8 to 50×8.

### V14.19
- Blazing Stampede
  - **Bug Fixes:** Removed a redundant buff on the status bar while under the effects of the ability.

### V14.3
- Bridge Between
  - **New Effect:** Cooldown is now affected by Ultimate haste.
  - **New Effect:** Entering a stance is now special-cased to trigger Experimental Hexplate Overdrive and possessive=true increased movement speed.
  - **New Effect:** *Awakened* Wilding Claw and Wingborne Storm are now special-cased to trigger Malignance Hatefog.

### V13.18
- Blazing Stampede
  - **Bug Fixes:** Spell shield no longer blocks the application of the on-target cooldown marker for the empowered attack.

### V13.14
- Blazing Stampede
  - **Bug Fixes:** Hail of Blades now properly triggers from the empowered attack.

### V13.4
- Stats
  - Health growth reduced to 92 from 98.
  - Base armor reduced to 31 from 34.
- Wilding Claw
  - On-hit bonus AD ratio reduced to 25% **bonus** AD from 30%.
- Wingborne Storm
  - Empowered slow reduced to 20 / 23 / 26 / 29 / 32 / 35% from 25 / 28 / 31 / 34 / 37 / 40%.

### V13.1b
- Wilding Claw
  - Bonus AD ratio reduced to 4% per 100 **bonus** AD from 5%.
  - Bonus attack speed increased to 20 / 32 / 44 / 56 / 68 / 80% from 20 / 30 / 40 / 50 / 60 / 70%.
- Wingborne Storm
  - Base damage per tick reduced to 10 / 19 / 28 / 37 / 46 / 55 from from 10 / 20 / 30 / 40 / 50 / 60.
    - Total base damage reduced to 80 / 152 / 224 / 296 / 368 / 440 from 80 / 160 / 240 / 320 / 400 / 480.

## Trivia

- Udyr's dance references the Drunken Master Kung Fu Style.
  - A side-by-side comparison can be seen here.
- He is the only character that cannot max out all of his skills, as each of his skills have  levels and the max character level is 18 (18 points for a total of 24 possible ranks).
  - However, in the Ultra Rapid Fire (2015) featured mode, there is no limit to a champion's level, so Udyr is able to have 6 levels for his abilities.
- He is one of the few characters without an ultimate along with Jayce. Udyr has 4 skills that can be leveled up to the maximum rank of 6.
- Udyr is one of a few champions to have multiple textures in one skin. Udyr however has more different textures in one skin than any other champion, as he has a different one for each of his stances.
- Udyr is one of two champions that went into the weekly champion rotation on the day of their release. The other one is Poppy.
- Udyr was one of the champions chosen for the Ionian pool available during the Ionia vs. Noxus match.
  - He was one of the selected champions, and was voted the MVP.
- In the V1.0.0.115 April Fools' Day patch, he was jokingly listed to receive a skin called **"Urfdyr"**, referencing Urf.

---
*This page was automatically generated from League of Legends Wiki data.*