# Galio

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
| **Champion** | Galio |
| **Title** | the Colossus |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2010-08-10 |
| **Release Patch** | V1.0.0.98 |
| **Latest Changes** | V25.18 |
| **Roles** | Warden |
| **Riot Positions** | Middle |
| **External Positions** | Middle, Support |
| **Blue Essence** | 1575 |
| **Riot Points** | 790 |
| **Difficulty** | 2 |
| **Hero Type** | Tank |
| **Alt Type** | Mage |
| **Adaptive Type** | Magic |
| **Damage** | 2 |
| **Toughness** | 3 |
| **Control** | 3 |
| **Mobility** | 2 |
| **Utility** | 1 |
| **Style** | 100 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $600.0$ | $+126.0$ |
| **Mana** | $410.0$ | $+40.0$ |
| **Health Regen** | $8.0$ | $+0.8$ |
| **Mana Regen** | $9.5$ | $+0.7$ |
| **Armor** | $24.0$ | $+4.7$ |
| **Magic Resist** | $32.0$ | $+2.05$ |
| **Attack Damage** | $59.0$ | $+3.5$ |
| **Attack Speed** | $0.625$ | |
| **Movement Speed** | $340.0$ | $+0.0$ |
| **Attack Range** | $150.0$ | $+0.0$ |
| **Base Attack Speed** | $0.625$ | |
| **Attack Speed Ratio** | $0.625$ | |
| **Bonus AS per Level** | $1.5\%$ | |
| **Attack Windup** | $20.6\%$ | |
| **Acquisition Radius** | $600$ units | |
| **Gameplay Radius** | $80$ units | |
| **Pathing Radius** | $50$ units | |
| **Selection Radius** | $125$ units | |
| **Selection Height** | $180$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $90.0\%$ |
| **Damage Taken** | $105.0\%$ |

## Abilities

### Passive: Colossal Smash

| Attribute | Value |
|-----------|------:|
| **Effect Radius** | 250 (pending for confirmation) units |
| **Static Cooldown** | 5 |
| **Targeting** | Passive |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | False |
| **Spell Effects** | spellaoe |
| **Parry** | True |

**INNATE:** Periodically, **Galio**’s next basic attack is empowered to have an uncancellable windup, gain 40% **bonus** attack speed and deal 15 to 115 (+ 100% AD) (+ 40% AP) (+ 60% **bonus** magic resistance) ***modified** magic damage to the target and all enemies near them.

Whenever **Galio** hits at least one enemy champion or epic monster with an ability, *Colossal Smash*’s **current** cooldown is reduced by 3 seconds. This may occur only once per cast.

*Colossal Smash*’s damage based on its AD ratio can critically strike for damage to all targets hit.

**Notes:**

- Spellblade does not get converted to magic damage and it will deal its damage only to the primary target.
- The empowered attack will not trigger against wards.
- Even if the ability hit is spell shielded *Colossal Smash*’s cooldown will still be reduced.

---

### Q: Winds of War

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 825 units |
| **Effect Radius** | cr 150 (Tornado) |
| **Width** | 120 (Curving missile width) units |
| **Speed** | 1400 (Curving missile speed) / 50 (Tornado, estimated) |
| **Cost** | 70 / 75 / 80 / 85 / 90 Mana |
| **Cooldown** | 11 / 10 / 9 / 8 / 7 seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | Special |
| **Spell Effects** | Special |
| **Projectile** | True |
| **Out of Range Behavior** | cast at max |

**ACTIVE:** **Galio** creates two windblasts 250 units to either of him that arc out before converging to the target location, dealing magic damage to all enemies they pass through.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 70 / 105 / 140 / 175 / 210 (+ 70% AP) |

When the blasts meet, the gusts form a tornado that persists for 2 seconds, slowly moving forward and dealing magic damage equal to「 2% (+ 1% per 100 AP) of target's **maximum** health every $0.5$ seconds ⟷ 8% (+ 4% per 100 AP) of target's **maximum** health over the duration 」to enemies within the area, capped at「 150 per tick ⟷ 600 total 」against monsters.

**Notes:**

- Deals area damage on the gust and applies persistent area damage for the tornado.
- Spell shield will block the gusts damage but not the tornado's.
- Yasuo’s Wind Wall and Braum’s Unbreakable can block each windblast individually.
  - The tornado will not form if only one windblast reaches the target location. Effect at cast time end

---

### W: Shield of Durand

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 175 / 196.88 / 218.75 / 240.62 / 262.5 / 284.38 / 306.25 / 328.12 / 350 units |
| **Cost** | 50 Mana |
| **Cooldown** | 18 / 17 / 16 / 15 / 14 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Auto |
| **Affects** | Self / Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Silence** | True |

**PASSIVE:** **Galio** gains *Anti-Magic Bulwark*, and restores it after 12@1; 10@6; 8@11 seconds without taking damage.

**ANTI-MAGIC BULWARK:** Gain a shield that absorbs magic damage.

| Attribute | Value |
|-----------|------:|
| **Magic Shield Strength** | 7.5 / 9 / 10.5 / 12 / 13.5% of **maximum** health |

**ACTIVE:** **Galio** charges for up to 2 seconds, slowing himself by 15%, and gaining physical and magic damage reduction; charging increases *Shield of Durand*’s radius, damage and taunt duration over the first $1.25$ seconds of the channel.

| Attribute | Value |
|-----------|------:|
| **Physical Damage Reduction** | 12.5 / 15 / 17.5 / 20 / 22.5% (+ 1.5% per 100 AP) (+ 4% per 100 **bonus** magic resistance) (+ 0.5% per 100 **bonus** health) |

| Attribute | Value |
|-----------|------:|
| **Magic Damage Reduction** | 25 / 30 / 35 / 40 / 45% (+ 4% per 100 AP) (+ 8% per 100 **bonus** magic resistance) (+ 1% per 100 **bonus** health) |

*Shield of Durand* can be recast within the duration and does so automatically afterwards or if it is interrupted.

**RECAST:** **Galio** refreshes the damage reduction for 2 seconds and deals magic damage to nearby enemy champions, increased by 0% / 25% / 50% / 75% / 100% / 125% / 150% / 175% / 200%, as well as taunts them for 0.5 / 0.62 / 0.75 / 0.88 / 1 / 1.12 / 1.25 / 1.38 / 1.5 seconds and, after a brief delay, sets their (ms) movement speed to a static 60 for the same duration but slightly longer.

| Attribute | Value |
|-----------|------:|
| **Minimum Magic Damage** | 20 / 30 / 40 / 50 / 60 (+ 30% AP) |
| **Maximum Magic Damage** | 60 / 90 / 120 / 150 / 180 (+ 90% AP) |

**Notes:**

- **Galio** is locked out of attacking and casting for $0.4$ seconds after recasting.
  - **Galio** may still buffer his actions during this time.
- *Static movement speed* cannot be modified by (movement speed) **bonus** movement speed or slow resist.
  - This effect is negated only if the target is slow-immune.
- The ability key does not need to be held down when buffered in other abilities except Winds of War.
- The following table refers for interactions while **Galio** is channeling:

#### Channel Behavior (charge)

| Aspect | State / Notes |
|--------|---------------|
| **Attacking** | Disabled |
| **Movement** | Allowed |
| **Abilities** | Disabled |
| **Items** | Interrupts / Allowed / Disabled |
| **Summoner Spells** | Allowed / Disabled / Interrupts |
| **Consumables** | Usable |
| **Interrupted by** | death, silence |

---

### E: Justice Punch

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.4$ seconds |
| **Target Range** | 250 (Minimum dash distance) / 650 (Maximum dash distance) units |
| **Width** | 400 units |
| **Speed** | 2300 (dash speed) units/second |
| **Cost** | 50 Mana |
| **Cooldown** | 11 / 10 / 9 / 8 / 7 seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Effects** | spellaoe |
| **Out of Range Behavior** | cast at max |
| **Grounded** | True |
| **Knockdown** | Special |

**ACTIVE:** **Galio** steps backwards in the opposite direction over the cast time, then dashes to the target location until he hits an enemy champion or terrain.

**Galio** deals magic damage to enemies he passes through, reduced to 80% against non-champions, and knocks them up for $0.75$ seconds as well as reveals them for the same duration.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 90 / 130 / 170 / 210 / 250 (+ 90% AP) |
| **Non-Champion Damage** | 72 / 104 / 136 / 168 / 200 (+ 72% AP) |

**Notes:**

- **Galio** will initiate the dash even if his step backwards is knocked down.
- **Galio**’s second dash will always dash the distance of the target direction. Having the step backwards cut short has no effect on the total distance gained.
- The following table refers for interactions during *Justice Punch*’s cast time and dash:

#### Channel Behavior (cast)

| Aspect | State / Notes |
|--------|---------------|
| **Summoner Spells** | Allowed / Disabled |

---

### R: Hero's Entrance

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 4000 / 4750 / 5500 units |
| **Effect Radius** | 650 units |
| **Cost** | 100 Mana |
| **Cooldown** | 180 / 170 / 160 / 150 / 140 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Unit |
| **Affects** | Allies, Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Area of Effect |
| **Grounded** | True |
| **Knockdown** | False |

**ACTIVE:** **Galio** prepares to make an entrance, channeling for $2.75$ seconds and designating the target allied champion's location at the time of cast as his landing spot. Additionally, he resets *Shield of Durand’s* passive shield for himself and grants it to all allied champions within the area for 5 seconds.

After channeling for $1.25$ seconds, he gains crowd control immunity for the remaining duration, becomes untargetable, and leaps into the for $0.8 seconds$ (Estimated) before dashing to his destination over $0.2 seconds$ (Estimated). Afterwards, he lands and becomes targetable again, dealing magic damage to all nearby enemies upon impact, knocking them back 100 units over $0.75$ seconds, and remaining in place for $0.5 seconds$ (Estimated).

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 150 / 250 / 350 (+ 70% AP) |

**Notes:**

- *Hero's Entrance* does not resist allied crowd control.
- *Hero's Entrance* has a forgiveness radius of 175 units.
- **Galio** cannot cast *Hero's Entrance* on allies he cannot see while nearsighted.
- **Galio** will still cause an impact to occur at the destination even if he is unable to travel the full distance with the dash (e.g. by The Hextech Ultimatum’s and Realm of Death’s borders and terrain). **Galio** will dash as far as he can go, and thus, does not end the dash prematurely.
- **Galio** will always dash 100 units minimum, and dash *over* his target's location if they are less than 100 units away from him.
- The following table refers for interactions while **Galio** is channeling:

#### Channel Behavior (channel)

| Aspect | State / Notes |
|--------|---------------|
| **Attacking** | Disabled |
| **Movement** | Disabled |
| **Abilities** | Disabled |
| **Items** | Disabled / Allowed |
| **Summoner Spells** | Allowed / Disabled |
| **Interrupted by** | death, root, silence, ground |
- *Hero's Entrance*’s channel can only be interrupted by crowd control in the first $1.25$ seconds.

---

## Patch History

### V25.18
- Colossal Smash
  - AP ratio reduced to 40% AP from 45% AP.
- Winds of War
  - Tornado target health ratio per tick reduced to 2% of target's **maximum** health from $2.5$%.
    - Total health ratio reduced to 8% of target's **maximum** health from 10%.

### V25.S1.3
- Stats
  - Base health reduced to 600 from 632.

### V14.21
- Stats
  - Base mana reduced to 410 from 500.

### V14.20
- Hero's Entrance
  - **New Effect:** Cast now has a forgiveness radius of 175 units.

### V14.8
- Stats
  - Base movement speed increased to 340 from 335.
- Winds of War
  - Cooldown reduced to 11 / 10 / 9 / 8 / 7 seconds from 12 / 10.75 / 9.5 / 8.25 / 7.

### V14.7
- Colossal Smash
  - AP ratio increased to 45% AP from 40% AP.
- Shield of Durand
  - Base magic damage reduction increased to 25 / 30 / 35 / 40 / 45% from 20 / 25 / 30 / 35 / 40%.
    - Base physical damage reduction increased to 12.5 / 15 / 17.5 / 20 / 22.5% from 10 / 12.5 / 15 / 17.5 / 20%.
- Justice Punch
  - Base damage increased to 90 / 130 / 170 / 210 / 250 from 75 / 115 / 155 / 195 / 235.

### V14.6
- Colossal Smash
  - **Removed:*** Cooldown is no longer affected by ability haste.
  - **New Effect:** Cooldown is now reduced by 3 seconds upon hitting an enemy champion or epic monster with an ability, once per cast.
  - Base damage reduced to 15 to 115 from 15 to 200.
  - AP ratio reduced to 40% AP from 50% AP.
  - **New Effect:** Empowered attack now gains 40% **bonus** attack speed during its windup.
  - **Bug Fixes:** Now properly executes minions when using the support item with charges.
- Winds of War
  - Cooldown reduced to 12 / 10.75 / 9.5 / 8.25 / 7 seconds from 12 / 11 / 10 / 9 / 8.
  - Initial hit AP ratio reduced to 70% AP from 75% AP.
- Shield of Durand
  - Minimum base damage reduced to 20 / 30 / 40 / 50 / 60 from 20 / 35 / 50 / 65 / 80.
    - Maximum base damage reduced to 60 / 90 / 120 / 150 / 180 from 60 / 105 / 150 / 195 / 240.
  - Base magic damage reduction reduced to 20 / 25 / 30 / 35 / 40% from 25 / 30 / 35 / 40 / 45%.
    - Physical damage reduction reduced to 10 / 12.5 / 15 / 17.5 / 20% from 12.5 / 15 / 17.5 / 20 / 22.5%.
  - Magic damage reduction AP ratio reduced to 4% per 100 AP from 5% per 100 AP.
    - Physical damage reduction AP ratio reduced to 1.5% per 100 AP from 2.5% per 100 AP.
  - Magic damage reduction magic resistance ratio reduced to 8% per 100 **bonus** magic resistance from 12%.
    - Physical damage reduction magic resistance ratio reduced to 4% per 100 **bonus** magic resistance from 6%.
  - **New Effect:** Magic damage reduction now scales with 1% per 100 **bonus** health.
    - **New Effect:** Physical damage reduction now scales with $0.5$% per 100 **bonus** health.
- Justice Punch
  - Base damage reduced to 75 / 115 / 155 / 195 / 235 from 90 / 130 / 170 / 210 / 250.
  - Non-champion damage increased to 80% from 50%.

### V13.24
- Winds of War
  - Cooldown reduced to 12 / 11 / 10 / 9 / 8 seconds from 12 / 11.5 / 11 / 10.5 / 10.

### V13.20
- Shield of Durand
  - Cooldown reduced to 18 / 17 / 16 / 15 / 14 seconds from 18 / 17.5 / 17 / 16.5 / 16.
  - Shield timer reduced to 12@1; 10@6; 8@11 seconds from 12 at all ranks.

### V13.19
- Hero's Entrance
  - Cooldown reduced to 180 / 160 / 140 seconds from 200 / 180 / 160.

## Trivia

- The name Galio may be inspired by the name Galileo, most commonly known for the Renaissance polymath Galileo Galilei.
  - The name Galileo means "man of Galilee", ultimately from Semitic root Gilgal "to roll".Huehnergard, J. *2011 Proto-Semitic Language & Culture; Semitic Roots* p. 2073
- Galio's dance references Beauty and the Beast (1991 film) by Walt Disney.
- Galio retains his dance post-rework, which references Beauty and the Beast by Walt Disney.
- He references his previous alias while attacking a Blue Sentinel: **"YOU'RE A SENTINEL? I'M A SENTINEL! WE SHOULD FIGHT!"**
  - In this case however, this interaction has literal implications: Galio being a textbook-definition sentinel for Demacia, and Galio perhaps sharing similar origins with the Blue Sentinel, both being rock/mineral beings empowered/given life by magic.

---
*This page was automatically generated from League of Legends Wiki data.*