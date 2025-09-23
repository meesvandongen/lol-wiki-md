# Hecarim

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
| **Champion** | Hecarim |
| **Title** | the Shadow of War |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2012-04-18 |
| **Release Patch** | V1.0.0.138 |
| **Latest Changes** | V14.24 |
| **Roles** | Diver |
| **Riot Positions** | Jungle |
| **External Positions** | Jungle |
| **Blue Essence** | 2400 |
| **Riot Points** | 880 |
| **Difficulty** | 2 |
| **Hero Type** | Fighter |
| **Alt Type** | Tank |
| **Adaptive Type** | Physical |
| **Damage** | 2 |
| **Toughness** | 2 |
| **Control** | 2 |
| **Mobility** | 2 |
| **Utility** | 1 |
| **Style** | 65 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $625.0$ | $+106.0$ |
| **Mana** | $280.0$ | $+40.0$ |
| **Health Regen** | $7.0$ | $+0.75$ |
| **Mana Regen** | $7.0$ | $+0.8$ |
| **Armor** | $32.0$ | $+5.45$ |
| **Magic Resist** | $32.0$ | $+2.05$ |
| **Attack Damage** | $66.0$ | $+3.7$ |
| **Attack Speed** | $0.670$ | |
| **Movement Speed** | $345.0$ | $+0.0$ |
| **Attack Range** | $175.0$ | $+0.0$ |
| **Base Attack Speed** | $0.67$ | |
| **Attack Speed Ratio** | $0.67$ | |
| **Bonus AS per Level** | $2.5\%$ | |
| **Gameplay Radius** | $80$ units | |
| **Pathing Radius** | $50$ units | |
| **Selection Radius** | $140$ units | |
| **Selection Height** | $140$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $105.0\%$ |
| **Damage Taken** | $90.0\%$ |
| **Healing** | $120.0\%$ |

## Abilities

### Passive: Warpath

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Self |

**INNATE:** **Hecarim** gains **bonus** attack damage equal to key=% of his **bonus** movement speed.

**Notes:**

- *Warpath*’s **bonus** attack damage will adjust itself to any temporary movement speed buffs and debuffs **Hecarim** is affected by.
  - Since this only affects **bonus** values **Hecarim** will not dip below his regular attack damage if he is slowed below his **base** movement speed.
- The amount of movement speed scaling to **bonus** attack damage is refreshed every $0.25$ seconds.

---

### Q: Rampage

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 375 units |
| **Cost** | 28 / 26 / 24 / 22 / 20 Mana |
| **Cooldown** | 4-0.75*(x-1) seconds |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |

**ACTIVE:** **Hecarim** cleaves his glaive around himself, dealing physical damage to nearby enemies, reduced to 60% against minions.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 60 / 85 / 110 / 135 / 160 (+ 90% **bonus** AD) |
| **Minion damage** | 36 / 51 / 66 / 81 / 96 (+ 54% **bonus** AD) |

If this damages an enemy, **Hecarim** gains a stack of *Rampage* for 8 seconds, refreshing on subsequent damage and stacking up to 3 times. Each stack increases *Rampage*’s damage by 3% (+ 4% per 100 **bonus** AD) and reduces its cd by $0.75$ seconds, up to a 9% (+ 12% per 100 **bonus** AD) damage increase and a $2.25$-second reduction of the **base** cooldown at maximum stacks. Stacks expire by one every 1 second when the duration ends.

**Notes:**

No additional notes.

---

### W: Spirit of Dread

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 525 units |
| **Cost** | 50 / 55 / 60 / 65 / 70 Mana |
| **Cooldown** | 14 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | False |
| **Spell Effects** | AoeDoT |

**ACTIVE:** **Hecarim** surrounds himself with the Spirit of Dread for 4 seconds, dealing magic damage every second to nearby enemies.

| Attribute | Value |
|-----------|------:|
| **Magic Damage Per Tick** | 20 / 30 / 40 / 50 / 60 (+ 20% AP) |
| **Total Magic Damage** | 100 / 150 / 200 / 250 / 300 (+ 100% AP) |

While active, **Hecarim** gains **bonus** armor and **bonus** magic resistance and is healed for 25% of the post-mitigation damage (Damage calculated after modifiers) dealt to enemies within the area from all sources, halved to 12.5% for damage dealt by allies. The healing is capped against minions and monsters.

| Attribute | Value |
|-----------|------:|
| **Bonus Resistances** | 5 / 10 / 15 / 20 / 25 |

| Attribute | Value |
|-----------|------:|
| **Capped Healing** | 120 / 150 / 180 / 210 / 240 |

**Notes:**

- The healing cap against minions accounts for every single one regardless of how many are present and/or affected by *Spirit of Dread* at the time of cast.

---

### E: Devastating Charge

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Cost** | 60 Mana |
| **Cooldown** | 20 / 19 / 18 / 17 / 16 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Auto |
| **Affects** | Self |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | spell |
| **Parry** | Special |
| **Grounded** | Special |
| **Knockdown** | Special |

**ACTIVE:** **Hecarim** becomes ghosted and gains ms for 4 seconds.

During this time, **Hecarim** empowers his next basic attack to gain type=distance traveled range and cause him to dash in the target's direction, revealing them for 1 second. If the target remains nearby during the dash, the ghosting and **bonus** movement speed ends prematurely and **Hecarim** knocks them back type=distance traveled units, though not through terrain, stuns them for $0.25$ seconds, and deals them ***modified** physical damage, increased by type=distance traveled.

| Attribute | Value |
|-----------|------:|
| **Minimum Physical Damage** | 30 / 45 / 60 / 75 / 90 (+ 50% **bonus** AD) |
| **Maximum Physical Damage** | 60 / 90 / 120 / 150 / 180 (+ 100% **bonus** AD) |

*Devastating Charge* can critically strike for AD **bonus** physical damage.

*Devastating Charge resets **Hecarim**’s basic attack timer. **Hecarim** can cast any of his abilities during the dash. Devastating Charge's duration is paused during Onslaught of Shadows.*

**Notes:**

- **Hecarim** can cast any of his abilities while dashing.
  - Onslaught of Shadows will interrupt the dash.
- If the target does not remain nearby during the dash, *Devastating Charge*’s effect will not be consumed.
- The **bonus** movement speed stacks multiplicatively with other sources of movement speed boosts.
- The bonus attack range stacks additively with Rapid Firecannon.
- *Devastating Charge* will still apply its effects to the target even if the dash is interrupted, but not if he goes into resurrection.
- *Devastating Charge*’s damage will apply life steal and will affect structures.
- *"Distance traveled"* can be simple movement as well as dashing and blinking.
- *Devastating Charge*’s damage will not bypass block or dodge but the knock back will still be applied.
  - *Devastating Charge* will still deal damage while blinded.
- While grounded or rooted, **Hecarim** loses the **bonus** attack range from *Devastating Charge*.
  - **Hecarim** will not use the empowered attack while rooted.
- The dash does not follow targets. The target's position at the time of the enhanced attack is the direction **Hecarim** will leap to.
- Displacement immunity will not resist the application of the stun.

---

### R: Onslaught of Shadows

| Attribute | Value |
|-----------|------:|
| **Range** | 1510 (Spectral riders missile range, estimated) units |
| **Cast Time** | none |
| **Target Range** | 300 (Minimum range) / 1000 (Maximum range) units |
| **Effect Radius** | 315 (Fear radius around dash end) units |
| **Width** | 80 (Individual spectral riders missile width) / 480 (Width of full effect given by outermost riders) units |
| **Speed** | 1100 (Both dash and spectral riders missile speed) units/second |
| **Cost** | 100 Mana |
| **Cooldown** | 140 / 120 / 100 seconds |
| **Targeting** | Location |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | True |
| **Out of Range Behavior** | cast at max |
| **Grounded** | True |
| **Knockdown** | False |

**ACTIVE:** **Hecarim** dashes with displacement immunity to the target location and summons 5 spectral riders in an arrow formation (see notes) that charge alongside him in the target direction, dealing magic damage to all enemies in their path and revealing them for $2.5$ seconds.

| Attribute | Value |
|-----------|------:|
| **Magic damage** | 150 / 200 / 250 / 300 / 350 (+ 100% AP) |

Upon arrival, he fears nearby enemies for type=distance traveled seconds and slows them by type=distance from **Hecarim**.

*The wave of spectral riders travels independently of **Hecarim** and will always charge at the same distance.*

**Notes:**

- The spectral rider missiles originate at **Hecarim**’s location, 145 units behind and 100 units to either of him, and 290 units behind and 200 units to either side. They travel 1510 units forward parallel to Hecarim from their origin each.
- Only the spectral riders deal damage to enemies they pierce, **Hecarim** only dashes and fears enemies at the end of it.
- The slow improperly counts as an additive percent movement speed modifier rather than a multiplicative one.
  - The strength is unaffected by slow resist as a result of this behavior.
  - This means it can unintentionally stack with the effects of other slows.

---

## Patch History

### V14.24
- Onslaught of Shadows
  - **Bug Fixes:** No longer reveals the user if cast while the cursor is near an enemy champion.

### V14.14
- Stats
  - Health growth increased to 106 from 99.
- Spirit of Dread
  - Healing increased to 25% from 20%.
    - Healing from ally damage increased to $12.5$% from 10%.
  - Duration reduced to 4 seconds from 5.
  - Cooldown reduced to 14 seconds at all ranks from 16 / 15.5 / 15 / 14.5 / 14.
- Devastating Charge
  - Cooldown changed to 20 / 19 / 18 / 17 / 16 seconds from 18 at all ranks.

### V14.1
- Devastating Charge
  - **Bug Fixes:** No longer sometimes critically strikes with 0% critical strike chance.

### V13.21
- Stats
  - Attack damage growth increased to $3.7$ from $3.2$.
- Spirit of Dread
  - Duration increased to 5 seconds from 4.

### V13.17
- Stats
  - Base mana increased to 280 from 277.
  - Mana growth reduced to 40 from 60.
  - Base mana regeneration increased to 7 from $6.5$.
  - Mana regeneration growth increased to $0.8$ from $0.6$.
- Rampage
  - Mana cost reduced to 28 / 26 / 24 / 22 / 20 from 30 at all ranks.
- Spirit of Dread
  - Mana cost reduced to 50 / 55 / 60 / 65 / 70 from 50 / 60 / 70 / 80 / 90.
  - Base heal reduced to 20% from 25%.
  - **Removed:*** Healing no longer scales with 2% per 100 **bonus** AD.

### V13.14
- Spirit of Dread
  - Base bonus resistances reduced to 5 / 10 / 15 / 20 / 25 from 15 / 20 / 25 / 30 / 35.

### V13.6
- General
  - New splash artwork for Hecarim.

### V12.23#December 14th Hotfix|V12.23
- Rampage
  - Base damage amplification per stack reduced to 3% from 4%.
    - Maximum base damage amplification reduced to 9% from 12%.
  - Damage amplification bonus AD ratio per stack reduced to 4% per 100 **bonus** AD from 5%.
    - Maximum damage amplification bonus AD ratio reduced to 12% per 100 **bonus** AD from 15%.

### V12.18
- Stats
  - Base health reduced to 625 from 650.
- Rampage
  - Damage bonus AD ratio reduced to 90% **bonus** AD from 95%.
  - Damage amplification bonus AD ratio per stack reduced to 5% per 100 **bonus** AD from 6%.
    - Maximum damage amplification bonus AD ratio reduced to 15% per 100 **bonus** AD from 18%.
  - Winddown time reduced to 0 seconds from 1.
- Spirit of Dread
  - Base heal reduced to 25% from 30%
  - Heal bonus AD ratio reduced to 2% per 100 **bonus** AD from $2.5$%.

### V12.17
- Stats
  - Base armor reduced to 32 from 35.
  - Health growth reduced to 99 from 104.
- Rampage
  - Base damage reduced to 60 / 85 / 110 / 135 / 160 from 60 / 90 / 120 / 150 / 180.
  - Damage bonus AD ratio increased to 95% **bonus** AD from 90%.
  - Number of stacks increased to 3 from 2.
  - Cooldown reduction per stack reduced to $0.75$ seconds from 1.
    - Maximum cooldown reduction increased to $2.25$ seconds from 2.
  - Base damage amplification per stack increased to 4% from 2%.
    - Maximum base damage amplification increased to 12% from 4%.
  - Damage amplification bonus AD ratio per stack increased to 6% per 100 **bonus** AD from 3%.
    - Maximum damage amplification bonus AD ratio increased to 18% per 100 **bonus** AD from 6%.
  - Mana cost changed to 30 at all ranks from 28 / 31 / 34 / 37 / 40.
  - **Undocumented:** Stack duration increased to 8 seconds from 6.
  - **New Effect:** Stacks now expire by one every 1 second instead of all at once upon expiration.
- Spirit of Dread
  - Cooldown increased to 16 / 15.5 / 15 / 14.5 / 14 seconds from 14 at all ranks.
  - **New Effect:** Now grants him 15 / 20 / 25 / 30 / 35 **bonus** armor and **bonus** magic resistance while active.
- Devastating Charge
  - Minimum bonus AD ratio reduced to 50% **bonus** AD from 55%.
    - Maximum bonus AD ratio reduced to 100% **bonus** AD from 110%.
  - Knock back distance reduced to type=distance traveled units from type=distance traveled.
  - Cooldown changed to 18 seconds at all ranks from 20 / 19 / 18 / 17 / 16.
- Onslaught of Shadows
  - Fear duration reduced to type=distance traveled seconds from type=distance traveled.

## Trivia

- Hecarim might have been inspired by the Four Horsemen of the Apocalypse (specifically War) and/or Assault Rider from Kingdom Hearts.
- His dance references 'Dope Zebra' by 'rhettandlink'.
  - A side-by-side comparison can be seen here.
- Warpath is one of the two abilities in the game with damage scaling with movement speed. The other being Tailwind.
- Onslaught of Shadows might be referencing the fight with in The Legend of Zelda: Twilight Princess.
  - The spectral riders used to be members of the Iron Order Hecarim was Knight Commander of and, together with him, became undead when the Ruination of the Blessed Isles was unleashed.
- Hecarim was deemed overpowered in Ultra Rapid Fire (2014 edition) and was ultimately disabled in non-custom games.

---
*This page was automatically generated from League of Legends Wiki data.*