# Hecarim

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
| **Champion** | Hecarim |
| **Title** | the Shadow of War |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2012-04-18 |
| **Release Patch** | V1.0.0.138 |
| **Roles** | Diver |
| **Riot Positions** | Jungle |
| **External Positions** | Jungle |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $625.0$ | $+106.0$ | $2427.0$ |
| **Mana** | $280.0$ | $+40.0$ | $960.0$ |
| **Health Regen** | $7.0$ | $+0.75$ | $19.8$ |
| **Mana Regen** | $7.0$ | $+0.8$ | $20.6$ |
| **Armor** | $32.0$ | $+5.45$ | $124.7$ |
| **Magic Resist** | $32.0$ | $+2.05$ | $66.8$ |
| **Attack Damage** | $66.0$ | $+3.7$ | $128.9$ |
| **Attack Speed** | $0.670$ | $+2.5\%$ | $0.955$ |
| **Movement Speed** | $345.0$ | $+0.0$ | $345.0$ |
| **Attack Range** | $175.0$ | $+0.0$ | $175.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.67$ |
| **Attack Speed Ratio** | $0.67$ |
| **Bonus AS per Level** | $2.5\%$ |
| **Gameplay Radius** | $80 units$ |
| **Pathing Radius** | $50 units$ |
| **Selection Radius** | $140 units$ |
| **Selection Height** | $140 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Warpath

**Innate:** **Hecarim** gains **bonus attack damage** equal to a portion of his **bonus movement speed**.

**Innate:** **Hecarim** gains **bonus attack damage** equal to key=% of his .

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Self |

**Notes:**

- 'Warpath's **bonus** attack damage will adjust itself to any temporary movement speed buffs and debuffs **Hecarim** is affected by.
  - Since this only affects **bonus** values **Hecarim** will not dip below his regular attack damage if he is slow below his **base** movement speed.
- The amount of movement speed scaling to **bonus** attack damage is refreshed every $0.25$ seconds.

---

### Q: Rampage

**Active:** **Hecarim** cleaves his glaive to deal physical damage to nearby enemies.

*Whenever *Rampage* hits at least one enemy, he gains a stack, up to a cap.*

**Active:** **Hecarim** cleaves his glaive around himself, dealing physical damage to nearby enemies, reduced to 60% against minions. If this damages an enemy, **Hecarim** gains a stack of *Rampage* for 8 seconds, refreshing on subsequent damage and stacking up to 3 times. Each stack increases 'Rampage's damage by 3% (+ 4% per 100 *bonus AD) and reduces its *cd **base** cooldown* by $0.75$ seconds, up to a 9% (+ 12% per 100 *bonus AD) damage increase and a $2.25$-second reduction of the **base** cooldown at maximum stacks. Stacks expire by one every 1 second when the duration ends.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | 4-0.75*(x-1) seconds |
| **Cast Time** | none |
| **Cost** | $28-20$ Mana |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical |
| **Effect Radius** | 375 units |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |

**Scaling:**
- **Physical Damage:** $60-160$ (+ 90% bonus AD); pass-through: $60-160$ × 0.6 (+ $90×0.6$% bonus AD)

**Notes:**

No additional notes.

---

### W: Spirit of Dread

**Active:** **Hecarim** continually deals magic damage to nearby enemies for a short time.

*During this time, he gains **bonus** resistances and heal for a portion of any damage he and his allies deal to nearby enemies, with reduced healing on ally damage.*

**Active:** **Hecarim** surrounds himself with the Spirit of Dread for 4 seconds, dealing magic damage every second to nearby enemies. While active, **Hecarim** gains **bonus armor** and **bonus magic resistance** and is heal for 25% of the post-mitigation damage dealt to enemies within the area from all sources, halved to $25/2$% for damage dealt by allies. The healing is capped against minions and monsters.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | 14 seconds |
| **Cast Time** | none |
| **Cost** | $50-70$ Mana |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | 525 units |
| **Spell Shield** | False |
| **Spell Effects** | AoeDoT |

**Scaling:**
- **Magic Damage Per Tick:* $20-60$ (+ 20% AP)5-60×5$ (+ $20×5$% AP)
- **Bonus Resistances:** $5-25$
- **Capped Healing:** $120-240$

**Notes:**

- The healing cap against minions accounts for every single one regardless of how many are present and/or affected by *Spirit of Dread* at the time of cast.

---

### E: Devastating Charge

**Active:** **Hecarim** becomes ghosted and builds up **bonus movement speed** over a short time.

*During this time, ''Hecarim's** next basic attack will gain *range **bonus** range* and cause him to dash. Upon arrival, he deals **modified'' physical damage and airborne the target, briefly stun them afterward.*

**Active:** **Hecarim** becomes ghosted and gains ms*bonus total** movement speed* for 4 seconds. During this time, **Hecarim** empowers his next basic attack to gain type=distance traveled *range **bonus** range* and cause him to dash in the target's direction, standard sight them for 1 second. If the target remains nearby during the dash, the ghosting and **bonus** movement speed ends prematurely and **Hecarim** airborne type=distance traveled units, though not through terrain, stun them for $0.25$ seconds, and deals them *modified physical damage, increased by type=distance traveled. *Devastating Charge can critically strike for critical damage*bonus** physical damage. *Devastating Charge basic attack reset *'Hecarim's** basic attack timer. **Hecarim'' can cast any of his abilities during the dash. Devastating Charge's duration is paused during *Onslaught of Shadows*.'

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $20-16$ seconds |
| **Cast Time** | none |
| **Cost** | 60 Mana |
| **Targeting** | Auto |
| **Affects** | Self |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | spell |

**Scaling:**
- **Minimum Physical Damage:** $30-90$ (+ 50% bonus AD)2-90×2$ (+ $50×2$% bonus AD)

**Notes:**

- **Hecarim** can cast any of his abilities while dash.
  - *Onslaught of Shadows* will interrupt the dash.
- If the target does not remain nearby during the dash, 'Devastating Charge's effect will not be consumed.
- The **bonus** movement speed stacks multiplicatively with other sources of movement speed boosts.
- The bonus attack range stacks additively with *Rapid Firecannon*.
- *Devastating Charge* will still apply its effects to the target even if the dash is interrupt, but not if he goes into resurrection.
- 'Devastating Charge's damage will apply life steal and will affect structures.
- *"Distance traveled"* can be simple movement as well as dash and blink.
- 'Devastating Charge's damage will not bypass block or dodge but the airborne will still be applied.
  - *Devastating Charge* will still deal damage while blind.
- While ground or root, **Hecarim** loses the **bonus attack range** from *Devastating Charge*.
  - **Hecarim** will not use the empowered attack while root.
- The dash does not follow targets. The target's position at the time of the enhanced attack is the direction **Hecarim** will leap to.
- Displacement immunity will not resist the application of the stun.

---

### R: Onslaught of Shadows

**Active:** **Hecarim** summons spectral riders and dash in the target direction, dealing magic damage to enemies they pass through.

*Upon arrival, he fear and gradually slow enemies for a duration based on the distance traveled.*

**Active:** **Hecarim** dash with displacement immunity to the target location and summons 5 spectral riders in an arrow formation that charge alongside him in the target direction, dealing magic damage to all enemies in their path and standard sight them for $2.5$ seconds. Upon arrival, he fear nearby enemies for type=distance traveled seconds and slow them by type=distance from **Hecarim**. *The wave of spectral riders travels independently of **Hecarim** and will always charge at the same distance.*

| Attribute | Value |
|-----------|-------|
| **Range** | 300 / 1000 units |
| **Cooldown** | $140/120/100$ seconds |
| **Cast Time** | none |
| **Cost** | 100 Mana |
| **Targeting** | Location |
| **Damage Type** | Magic |
| **Speed** | 1100 units/second |
| **Effect Radius** | 315 units |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | True |

**Scaling:**
- **Magic damage:** $150-350$ (+ 100% AP)

**Notes:**

- The spectral rider missiles originate at ''Hecarim's' location, 145 units behind and 100 units to either of him, and 290 units behind and 200 units to either side. They travel 1510 units forward parallel to Hecarim from their origin each.
- Only the spectral riders deal damage to enemies they pierce, **Hecarim** only dashes and fears enemies at the end of it.
- The slow improperly counts as an additive percent movement speed modifier rather than a multiplicative one.
  - The strength is unaffected by slow resist as a result of this behavior.
  - This means it can unintentionally stack with the effects of other slows.

---

## Patch History

### V14.24
- *Onslaught of Shadows*
  - **Bug Fixes:** No longer reveals the user if cast while the cursor is near an enemy champion.

### V14.14
- Stats
  - Health growth increased to 106 from 99.
- *Spirit of Dread*
  - Healing increased to 25% from 20%.
    - Healing from ally damage increased to $12.5$% from 10%.
  - Duration reduced to 4 seconds from 5.
  - Cooldown reduced to 14 seconds at all ranks from $16-14$.
- *Devastating Charge*
  - Cooldown changed to $20-16$ seconds from 18 at all ranks.

### V14.1
- *Devastating Charge*
  - **Bug Fixes:** No longer sometimes critically strikes with 0% critical strike chance.

### V13.21
- Stats
  - Attack damage growth increased to $3.7$ from $3.2$.
- *Spirit of Dread*
  - Duration increased to 5 seconds from 4.

### V13.17
- Stats
  - Base mana increased to 280 from 277.
  - Mana growth reduced to 40 from 60.
  - Base mana regeneration increased to 7 from $6.5$.
  - Mana regeneration growth increased to $0.8$ from $0.6$.
- *Rampage*
  - Mana cost reduced to $28-20$ from 30 at all ranks.
- *Spirit of Dread*
  - Mana cost reduced to $50-70$ from $50-90$.
  - Base heal reduced to 20% from 25%.
  - **Removed:*** Healing no longer scales with 2% per 100 *bonus AD.

### V13.14
- *Spirit of Dread*
  - Base bonus resistances reduced to $5-25$ from $15-35$.

### V13.6
- General
  - New splash artwork for Hecarim.
- *Rampage*
  - Base damage amplification per stack reduced to 3% from 4%.
    - Maximum base damage amplification reduced to 9% from 12%.
  - Damage amplification bonus AD ratio per stack reduced to 4% per 100 *bonus AD from 5%.
    - Maximum damage amplification bonus AD ratio reduced to 12% per 100 *bonus AD from 15%.

### V12.18
- Stats
  - Base health reduced to 625 from 650.
- *Rampage*
  - Damage bonus AD ratio reduced to 90% *bonus AD from 95%.
  - Damage amplification bonus AD ratio per stack reduced to 5% per 100 *bonus AD from 6%.
    - Maximum damage amplification bonus AD ratio reduced to 15% per 100 *bonus AD from 18%.
  - Winddown time reduced to 0 seconds from 1.
- *Spirit of Dread*
  - Base heal reduced to 25% from 30%
  - Heal bonus AD ratio reduced to 2% per 100 *bonus AD from $2.5$%.

### V12.17
- Stats
  - Base armor reduced to 32 from 35.
  - Health growth reduced to 99 from 104.
- *Rampage*
  - Base damage reduced to $60-160$ from $60-180$.
  - Damage bonus AD ratio increased to 95% *bonus AD from 90%.
  - Number of stacks increased to 3 from 2.
  - Cooldown reduction per stack reduced to $0.75$ seconds from 1.
    - Maximum cooldown reduction increased to $2.25$ seconds from 2.
  - Base damage amplification per stack increased to 4% from 2%.
    - Maximum base damage amplification increased to 12% from 4%.
  - Damage amplification bonus AD ratio per stack increased to 6% per 100 *bonus AD from 3%.
    - Maximum damage amplification bonus AD ratio increased to 18% per 100 *bonus AD from 6%.
  - Mana cost changed to 30 at all ranks from $28-40$.
  - **Undocumented:** Stack duration increased to 8 seconds from 6.
  - **New Effect:** Stacks now expire by one every 1 second instead of all at once upon expiration.
- *Spirit of Dread*
  - Cooldown increased to $16-14$ seconds from 14 at all ranks.
  - **New Effect:** Now grants him $15-35$ **bonus armor and **bonus** magic resistance while active.
- *Devastating Charge*
  - Minimum bonus AD ratio reduced to 50% *bonus AD from 55%.
    - Maximum bonus AD ratio reduced to 100% *bonus AD from 110%.
  - Knock back distance reduced to type=distance traveled units from type=distance traveled.
  - Cooldown changed to 18 seconds at all ranks from $20-16$.
- *Onslaught of Shadows*
  - Fear duration reduced to type=distance traveled seconds from type=distance traveled.

### V12.10
- Stats
  - Base health increased to 650 from 580.
  - Health growth increased to 104 from 90.
  - Armor growth increased to $5.45$ from $4.25$.
  - Magic resistance growth increased to $2.05$ from $1.25$.

## Trivia

- Hecarim might have been inspired by the Four Horsemen of the Apocalypse (specifically War) and/or Assault Rider from Kingdom Hearts.
- His dance references 'Dope Zebra' by 'rhettandlink'.
  - A side-by-side comparison can be seen here.
- *Warpath* is one of the two abilities in the game with damage scaling with movement speed. The other being *Tailwind*.
- *Onslaught of Shadows* might be referencing the fight with in The Legend of Zelda: Twilight Princess.
  - The spectral riders used to be members of the Iron Order Hecarim was Knight Commander of and, together with him, became undead when the Ruination of the Blessed Isles was unleashed.
- Hecarim was deemed overpowered in Ultra Rapid Fire (2014 edition) and was ultimately disabled in non-custom games.

---
*This page was automatically generated from League of Legends Wiki data.*