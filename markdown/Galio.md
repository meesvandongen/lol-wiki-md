# Galio

## Table of Contents
- [Basic Information](#basic-information)
- [Statistics](#statistics)
  - [Advanced Stats](#advanced-stats)
- [Abilities](#abilities)
- [Patch History](#patch-history)
- [Trivia](#trivia)

## Basic Information

| Attribute | Value |
|-----------|-------|
| **Champion** | Galio |
| **Title** | the Colossus |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2010-08-10 |
| **Release Patch** | V1.0.0.98 |
| **Roles** | Warden |
| **Riot Positions** | Middle |
| **External Positions** | Middle, Support |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $600.0$ | $+126.0$ | $2742.0$ |
| **Mana** | $410.0$ | $+40.0$ | $1090.0$ |
| **Health Regen** | $8.0$ | $+0.8$ | $21.6$ |
| **Mana Regen** | $9.5$ | $+0.7$ | $21.4$ |
| **Armor** | $24.0$ | $+4.7$ | $103.9$ |
| **Magic Resist** | $32.0$ | $+2.05$ | $66.8$ |
| **Attack Damage** | $59.0$ | $+3.5$ | $118.5$ |
| **Attack Speed** | $0.625$ | $+1.5\%$ | $0.784$ |
| **Movement Speed** | $340.0$ | $+0.0$ | $340.0$ |
| **Attack Range** | $150.0$ | $+0.0$ | $150.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.625$ |
| **Attack Speed Ratio** | $0.625$ |
| **Bonus AS per Level** | $1.5\%$ |
| **Attack Windup** | $20.6\%$ |
| **Acquisition Radius** | $600 units$ |
| **Gameplay Radius** | $80 units$ |
| **Pathing Radius** | $50 units$ |
| **Selection Radius** | $125 units$ |
| **Selection Height** | $180 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Colossal Smash

**Innate:** Every few seconds, **Galio**’s next basic attack is modified to gain as and deal magic damage in an area, increased based on his *mr **bonus** magic resist*.

*''Galio's* ability hits against enemy champions or epic monsters ah *Colossal Smash's cd.*

**Innate:** Periodically, ''Galio's** next basic attack is empowered to have an uncancellable windup, gain *40% *bonus attack speed* and deal 15 to 115 (+ 100% AD) (+ 40% AP) (+ 60% **bonus** magic resistance) **modified'' magic damage to the target and all enemies near them. Whenever **Galio** hits at least one enemy champion or epic monster with an ability, 'Colossal Smash's **current cooldown** is reduced by 3 seconds. This may occur only once per cast. 'Colossal Smash's damage based on its AD ratio can critical strike for critical damage to all targets hit.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | 250 units |
| **Spell Shield** | False |
| **Spell Effects** | spellaoe |

**Notes:**

- Spellblade does not get converted to magic damage and it will deal its damage only to the primary target.
- The empowered attack will not trigger against wards.
- Even if the ability hit is spell shield 'Colossal Smash's cooldown will still be reduced.

---

### Q: Winds of War

**Active:** **Galio** fires two windblasts that converge to the target location, dealing magic damage to enemies hit.

*Upon converging, the blasts form a tornado that continually deals magic damage to enemies within, based on each target's **maximum** health.*

**Active:** **Galio** creates two windblasts 250 units to either of him that arc out before converging to the target location, dealing magic damage to all enemies they pass through. When the blasts meet, the gusts form a tornado that persists for 2 seconds, slowly moving forward and dealing magic damage equal to every $0.5$ seconds*maximum** health over the durationto enemies within the area, capped atagainst monsters.

| Attribute | Value |
|-----------|-------|
| **Range** | 825 units |
| **Cooldown** | $11-7$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $70-90$ Mana |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Speed** | 1400 / 50 units/second |
| **Effect Radius** | 150 units |
| **Spell Shield** | Special |
| **Spell Effects** | Special |
| **Projectile** | True |

**Scaling:**
- **Magic Damage:** $70-210$ (+ 70% AP)

**Notes:**

- Deals area damage on the gust and applies persistent area damage for the tornado.
- Spell shield will block the gusts damage but not the tornado's.
- Wind Wall and Unbreakable can block each windblast individually.
  - The tornado will not form if only one windblast reaches the target location. Effect at cast time end

---

### W: Shield of Durand

**Passive:** **Galio** gains a shield that absorbs magic damage based on his **maximum** health, and restores it while out of combat.

**Active:** **Galio** channel a defensive stance, moving slow while gaining damage reduction. *Shield of Durand* can be recast within the duration and does so automatically afterward.

**Passive:** **Galio** gains *Anti-Magic Bulwark*, and restores it after 12@1; 10@6; 8@11 seconds without taking damage. **Anti-Magic Bulwark:** Gain a shield that absorbs magic damage. **Active:** **Galio** channel for up to 2 seconds, slow himself by 15%, and gaining physical and magic damage reduction; charging increases 'Shield of Durand's radius, damage and taunt duration over the first $1.25$ seconds of the channel. *Shield of Durand* can be recast within the duration and does so automatically afterwards or if it is interrupt. **Recast:** **Galio** refreshes the damage reduction for 2 seconds and deals magic damage to nearby enemy champions, increased by type=channel time, as well as taunt them for type=channel time seconds and, after a brief delay, sets their ms to a static 60 for the same duration but slightly longer.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $18-14$ seconds |
| **Cast Time** | none |
| **Cost** | 50 Mana |
| **Targeting** | Auto |
| **Affects** | Self / Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | channel time |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |

**Scaling:**
- **Magic Shield Strength:** $7.5-13.5$% of
- **maximum** health
- **Physical Damage Reduction:** $25/2-45/2$% (+ $1.5$% per 100 AP) (+ $8/2$% per 100
- **bonus** magic resistance) (+ $1/2$% per 100
- **bonus** health)
- **Magic Damage Reduction:** $25-45$% (+ 4% per 100 AP) (+ 8% per 100
- **bonus** magic resistance) (+ $1$% per 100
- **bonus** health)

**Notes:**

- **Galio** is lockout of attacking and casting for $0.4$ seconds after recasting.
  - **Galio** may still buffer his actions during this time.
- *Static movement speed* cannot be modified by *movement speed **bonus** movement speed* or slow resist.
  - This effect is negated only if the target is slow-immune.
- The ability key does not need to be held down when buffered in other abilities except *Winds of War*.
- The following table refers for interactions while **Galio** is channel:

---

### E: Justice Punch

**Active:** **Galio** will briefly lunge back before dash to the target location, dealing magic damage and airborne enemies he passes through. He will stop upon hitting an enemy champion.

**Active:** **Galio** lunge backwards in the opposite direction over the cast time, then dash to the target location until he hits an enemy champion or terrain. **Galio** deals magic damage to enemies he passes through, reduced to 80% against non-champions, and airborne for $0.75$ seconds as well as standard sight them for the same duration.

| Attribute | Value |
|-----------|-------|
| **Range** | 250 / 650 units |
| **Cooldown** | $11-7$ seconds |
| **Cast Time** | $0.4$ seconds |
| **Cost** | 50 Mana |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Speed** | 2300 units/second |
| **Spell Effects** | spellaoe |

**Scaling:**
- **Magic Damage:* $90-250$ (+ 90% AP)0.8-250×0.8$ (+ $90×0.8$% AP)

**Notes:**

- **Galio** will initiate the dash even if his step backwards is knockdown.
- ''Galio's' second dash will always dash the distance of the target direction. Having the step backwards cut short has no effect on the total distance gained.
- The following table refers for interactions during 'Justice Punch's cast time and dash:

---

### R: Hero's Entrance

**Active:** **Galio** guards the target allied champion, channels for a short time and granting *his* magic damage shield to himself and all allied champions in the area.

*After a short delay, **Galio** rises cc-immune and untargetable before dash down onto the target's initial location, dealing magic damage to nearby enemies and airborne.*

**Active:** **Galio** prepares to make an entrance, channel for $2.75$ seconds and designating the target allied champion location at the time of cast as his landing spot. Additionally, he resets **Shield of Durand*’s* passive shield for himself and grants it to all allied champions within the area for 5 seconds. After channeling for $1.25$ seconds, he gains cc-immune for the remaining duration, becomes untargetable, and leaps into the for $0.8 seconds$ before dash to his destination over $0.2 seconds$. Afterwards, he lands and becomes targetable again, dealing magic damage to all nearby enemies upon impact, airborne 100 units over $0.75$ seconds, and remaining in place for $0.5 seconds$.

| Attribute | Value |
|-----------|-------|
| **Range** | $4000/4750/5500$ units |
| **Cooldown** | $180-140$ seconds |
| **Cast Time** | none |
| **Cost** | 100 Mana |
| **Targeting** | Unit |
| **Affects** | Allies, Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | 650 units |
| **Spell Shield** | True |
| **Spell Effects** | Area of Effect |

**Scaling:**
- **Magic Damage:** $150/250/350$ (+ 70% AP)

**Notes:**

- 'Hero's Entrance' does not resist allied crowd control.
- 'Hero's Entrance' has a forgiveness radius of 175 units.
- **Galio** cannot cast 'Hero's Entrance' on allies he cannot sight while Nearsight.
- **Galio** will still cause an impact to occur at the destination even if he is unable to travel the full distance with the dash (e.g. by *The Hextech Ultimatum*’s and *Realm of Death*’s borders and terrain). **Galio** will dash as far as he can go, and thus, does not end the dash prematurely.
- **Galio** will always dash 100 units minimum, and dash *over* his target's location if they are less than 100 units away from him.
- The following table refers for interactions while **Galio** is channel:
- 'Hero's Entrance's channel can only be interrupt by crowd control in the first $1.25$ seconds.

---

## Patch History

### V25.18
- *Colossal Smash*
  - AP ratio reduced to 40% AP from 45% AP.
- *Winds of War*
  - Tornado target health ratio per tick reduced to 2% of target's **maximum** health from $2.5$%.
    - Total health ratio reduced to $2×4$% of target's **maximum** health from $2.5×4$%.
- Stats
  - Base health reduced to 600 from 632.

### V14.21
- Stats
  - Base mana reduced to 410 from 500.

### V14.20
- *Hero's Entrance*
  - **New Effect:** Cast now has a forgiveness radius of 175 units.

### V14.8
- Stats
  - Base movement speed increased to 340 from 335.
- *Winds of War*
  - Cooldown reduced to $11-7$ seconds from $12-7$.

### V14.7
- *Colossal Smash*
  - AP ratio increased to 45% AP from 40% AP.
- *Shield of Durand*
  - Base magic damage reduction increased to $25-45$% from $20-40$%.
    - Base physical damage reduction increased to $25×0.5-45×0.5$% from $20×0.5-40×0.5$%.
- *Justice Punch*
  - Base damage increased to $90-250$ from $75-235$.

### V14.6
- *Colossal Smash*
  - **Removed:*** Cooldown is no longer affected by ability haste.
  - **New Effect:** Cooldown is now reduced by 3 seconds upon hitting an enemy champion or epic monster with an ability, once per cast.
  - Base damage reduced to 15 to 115 from 15 to 200.
  - AP ratio reduced to 40% AP from 50% AP.
  - **New Effect:** Empowered attack now gains 40% *bonus attack speed during its windup.
  - **Bug Fixes:** Now properly executes minions when using the support item with charges.
- *Winds of War*
  - Cooldown reduced to $12-7$ seconds from $12-8$.
  - Initial hit AP ratio reduced to 70% AP from 75% AP.
- *Shield of Durand*
  - Minimum base damage reduced to $20-60$ from $20-80$.
    - Maximum base damage reduced to $20×3-60×3$ from $20×3-80×3$.
  - Base magic damage reduction reduced to $20-40$% from $25-45$%.
    - Physical damage reduction reduced to $20×0.5-40×0.5$% from $25×0.5-45×0.5$%.
  - Magic damage reduction AP ratio reduced to 4% per 100 AP from 5% per 100 AP.
    - Physical damage reduction AP ratio reduced to $3×0.5$% per 100 AP from $5×0.5$% per 100 AP.
  - Magic damage reduction magic resistance ratio reduced to 8% per 100 **bonus** magic resistance from 12%.
    - Physical damage reduction magic resistance ratio reduced to $8×0.5$% per 100 **bonus** magic resistance from $12×0.5$%.
  - **New Effect:** Magic damage reduction now scales with 1% per 100 **bonus** health.
    - **New Effect:** Physical damage reduction now scales with $0.5$% per 100 **bonus** health.
- *Justice Punch*
  - Base damage reduced to $75-235$ from $90-250$.
  - Non-champion damage increased to 80% from 50%.

### V13.24
- *Winds of War*
  - Cooldown reduced to $12-8$ seconds from $12-10$.

### V13.20
- *Shield of Durand*
  - Cooldown reduced to $18-14$ seconds from $18-16$.
  - Shield timer reduced to 12@1; 10@6; 8@11 seconds from 12 at all ranks.

### V13.19
- *Hero's Entrance*
  - Cooldown reduced to $180-140 3$ seconds from $200-160 3$.

### V13.6
- *Shield of Durand*
  - Base magic damage reduction increased to $25-45$% from $20-40$%.
    - Base physical damage reduction increased to $12.5-22.5$% from $10-20$%.
  - Magic damage reduction magic resistance ratio increased to 12% per 100 **bonus** magic resistance from 8%.
    - Physical damage reduction magic resistance ratio increased to 6% per 100 **bonus** magic resistance from 4%.
- *Justice Punch*
  - Cooldown reduced to $11-7$ seconds from $12-8$.

## Trivia

- The name Galio may be inspired by the name Galileo, most commonly known for the Renaissance polymath Galileo Galilei.
  - The name Galileo means "man of Galilee", ultimately from Semitic root Gilgal "to roll".Huehnergard, J. *2011 Proto-Semitic Language & Culture; Semitic Roots* p. 2073
- Galio's dance references Beauty and the Beast (1991 film) by Walt Disney.
- Galio retains his dance post-rework, which references Beauty and the Beast by Walt Disney.
- He references his previous alias while attacking a Blue Sentinel: **"You're a sentinel? I'm a sentinel! We should fight!"**
  - In this case however, this interaction has literal implications: Galio being a textbook-definition sentinel for Demacia, and Galio perhaps sharing similar origins with the Blue Sentinel, both being rock/mineral beings empowered/given life by magic.

---
*This page was automatically generated from League of Legends Wiki data.*