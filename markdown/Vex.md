# Vex

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
| **Champion** | Vex |
| **Title** | the Gloomist |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2021-09-23 |
| **Release Patch** | V11.19 |
| **Roles** | Burst |
| **Riot Positions** | Middle |
| **External Positions** | Middle |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $590.0$ | $+104.0$ | $2358.0$ |
| **Mana** | $490.0$ | $+32.0$ | $1034.0$ |
| **Health Regen** | $6.5$ | $+0.6$ | $16.7$ |
| **Mana Regen** | $6.0$ | $+0.8$ | $19.6$ |
| **Armor** | $23.0$ | $+4.45$ | $98.7$ |
| **Magic Resist** | $28.0$ | $+1.3$ | $50.1$ |
| **Attack Damage** | $54.0$ | $+2.75$ | $100.8$ |
| **Attack Speed** | $0.669$ | $+1.0\%$ | $0.783$ |
| **Movement Speed** | $335.0$ | $+0.0$ | $335.0$ |
| **Attack Range** | $550.0$ | $+0.0$ | $550.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.669$ |
| **Attack Speed Ratio** | $0.625$ |
| **Bonus AS per Level** | $1.0\%$ |
| **Missile Speed** | $2000 units/second$ |
| **Acquisition Radius** | $750 units$ |
| **Gameplay Radius** | $55 units$ |
| **Pathing Radius** | $30 units$ |
| **Selection Radius** | $100 units$ |
| **Selection Height** | $100 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Doom 'n Gloom

**Passive - Doom:** Periodically, **Vex** empowers her next basic ability to knockdown and briefly flee enemies hit.

**Innate - Gloom:** Nearby enemies that dash or blink become marked with *Gloom* for a few seconds. ''Vex's** next basic attack or basic ability hit will consume the mark to deal **bonus'* magic damage and cdr *Doom's* cooldown. **Looming Darkness** will also inflict *Gloom', but cannot detonate it.

**Passive - Doom:** Periodically, **Vex** empowers her next basic ability to knockdown and fear enemies hit for 0.75@1; 1@6; 1.25@9; 1.5@13 seconds, during which they are slow by type=distance from **Vex**. If **Looming Darkness** triggers *Doom*, enemies hit will flee from the epicenter instead. 'Doom's *cooldown* resets upon respawning. **Innate - Gloom:** Nearby enemy champions and monsters that dash or blink will be marked with *Gloom* for 6 seconds. ''Vex's* next basic attack, which becomes projectile, or basic ability hit against an enemy with *Gloom* will detonate the mark. **Looming Darkness** will also inflict *Gloom', but cannot detonate it. 'Gloom's* detonation deals 40 to 150 (+ 25% AP) **bonus** magic damage and refunds 25% of *Doom's* cooldown. Against non-champions, this instead deals 40@1; 45@6; 50@9; 55@13; 60@16 (@=%) damage and refunds 10% of *Doom's cooldown.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Self, Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | 1600 units |
| **Spell Shield** | Special |
| **Spell Effects** | proc |
| **Projectile** | False |

**Notes:**

- *Gloom* will mark Rift Herald and Rift Scuttler when they use their dashes.
- *Gloom* can mark clone, but not other pet.
- 'Gloom's mark has a very brief cooldown and refreshes on subsequent dashes or blinks nearby enemies use.
- 'Gloom's mark duration will refresh to $0.5$ seconds when **Vex** starts an attack windup against a target that has a mark which is about to expire.
- *Gloom* will mark enemies even if they are untargetable.
- *Gloom* will mark enemies that are inside the detection radius when they blink, but will not mark those that blink inside from far away.
- 'Doom's cooldown starts as soon as the basic ability is cast.
- 'Doom's flee will not be removed when **Vex** dies, unlike any other flee in the game.
- Non-champions (e.g. Rift Scuttler) are not knockdown by the fear-empowered ability.
- Spell shield will block all of 'Doom's* effects but not *Gloom's empowered attack nor mark.
- Nearby enemies that become displaced do not count for being marked by *Gloom*.
- recast does not cause the caster to be marked by *Gloom*.
- ''Vex's* basic abilities use a different icon when *Doom' is ready:
  - Mistral Bolt
  - Personal Space
  - Looming Darkness

---

### Q: Mistral Bolt

**Active:** **Vex** launches a wave of mist in the target direction that deals magic damage to enemies hit. After a short delay, the wave accelerates but also narrows.

**Active:** **Vex** launches a wave of mist in the target direction that deals magic damage to enemies hit. After travelling 500 units, the wave accelerates but also narrows itself.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $8-4$ seconds |
| **Cast Time** | $0.15$ seconds |
| **Cost** | $45-65$ Mana |
| **Targeting** | Direction |
| **Affects** | Ememies |
| **Damage Type** | Magic |
| **Speed** | 600 / 3200 units/second |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | True |

**Scaling:**
- **Magic Damage:** $70-250$ (+ 70% AP)

**Notes:**

  - *Mistral Bolt* will fire from wherever **Vex** is at the end of the cast time, towards the originally targeted location or 1200 units in the originally targeted direction if cast beyond that.

---

### W: Personal Space

**Active:** **Vex** emits a shockwave, dealing magic damage to nearby enemies and shield herself for a short time.

**Active:** **Vex** emits a shockwave around her before the cast time, dealing magic damage to nearby enemies and granting herself a shield for $2.5$ seconds. '**Vex** can move during Personal Space's cast time.'

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $16-12$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | 75 mana |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | 475 / 550 units |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |

**Scaling:**
- **Shield Strength:** $50-150$ (+ 75% AP)
- **Magic Damage:** $80-240$ (+ 30% AP)

**Notes:**

- *Personal Space* can be buffered $0.1$ seconds before it comes off cooldown or becomes available otherwise.

---

### E: Looming Darkness

**Active:** **Vex** tosses her *Shadow* to explode at the target location, dealing magic damage and slow enemies hit for a short time. The explosion widens based on cast distance.

**Active:** **Vex** tosses her *Shadow* to explode at the target location, dealing magic damage to enemies hit and slow them for 2 seconds. The explosion's radius increases based on cast distance. Killing an enemy with *Looming Darkness* refunds 10% of '*Doom's*' *cooldown*, increased to 25% for champion kills.

| Attribute | Value |
|-----------|-------|
| **Range** | 800 units |
| **Cooldown** | 13 seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $70-110$ mana |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Speed** | 1300 units/second |
| **Effect Radius** | cast distance |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | True |

**Scaling:**
- **Magic Damage:** $50-130$
- **Slow:** $30-50$%

**Notes:**

- No additional details.

---

### R: Shadow Surge

**Active:** **Vex** sends her *Shadow* in the target direction that deals magic damage to enemies hit. *Shadow* stops upon hitting an enemy champion, marking and true sight them for a few seconds. *Shadow Surge* can be recast while the target is marked.

**Recast:** **Vex** dashes towards the marked target. Upon arrival, the mark is consumed to deal magic damage.

**Active:** **Vex** sends her *Shadow* in the target direction that grants sight around its trajectory and deals magic damage to enemies hit. *Shadow* stops upon hitting an enemy champion to mark them for 4 seconds, during which they are true sight. *Shadow Surge* can be recast while the target is marked. **Recast:** **Vex** dashes towards the marked target with displacement immunity. Upon arrival, she consumes their mark and deals magic damage. If **Vex** scores a takedown against 'Shadow Surge's* marked target within 6 seconds of its application, **Vex** can cast *Shadow Surge' again within 12 seconds at no cost after $0.5$ seconds.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $140-100$ seconds |
| **Cast Time** | $0.25$ / None |
| **Cost** | 100 mana |
| **Targeting** | Direction / Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | Magic |
| **Speed** | 1600 / 2200 units/second |
| **Effect Radius** | sight650 / Global |
| **Spell Shield** | True |
| **Spell Effects** | spell |
| **Projectile** | True |

**Scaling:**
- **Magic Damage:** $75-175$ (+ 20% AP)
- **Magic Damage:** $150-350$ (+ 50% AP)

**Notes:**

- ''Vex's' dash will track the target if they change locations.
  - She will not stop tracking the target until she reaches them, as there is no maximum tracking distance.
- *Shadow Surge* grants sight of the area along its path for $0.4 seconds each.
- *Shadow Surge* cannot be recast while ground or root, or if the target is untargetable.
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
- *Doom 'n Gloom*
  - Base damage increased to 40 to 150 from 30 to 140.
  - AP ratio increased to 25% AP from 20% AP.
  - **New Effect:** Doom's cooldown is now also partially refunded upon killing units with *Looming Darkness*.
    - Champion kills refund the cooldown by 25% and non-champion kills do so by 10%.

### V13.21
- *Shadow Surge*
  - **Bug Fixes:** Now properly damages minions and monsters hit by the missile.
    - *Note: This fix was already in effect in the previous patch.*

### V13.17
- *Mistral Bolt*
  - Base damage increased to $70-250$ from $60-240$.
- *Personal Space*
  - Cooldown reduced to $16-12$ seconds from $20-12$.
- *Shadow Surge*
  - **New Effect:** Pinging the ability now displays visible targets in chat.

### V13.13
- *Doom 'n Gloom*
  - **Bug Fixes:** Now properly applies *Gloom* to a nearby enemy **Galio** casting *Justice Punch*.

### V13.6
- *Doom 'n Gloom*
  - **Bug Fixes:** Can no longer mark an enemy **Aurelion Sol** dashing from *Astral Flight* with *Gloom* more than once.

### V13.1
- *Doom 'n Gloom*
  - **Bug Fixes:** Ability icon is now properly displayed for its buff icon.

### V12.13
- *Mistral Bolt*
  - AP ratio increased to 70% AP from 60% AP.
  - Cooldown reduced to $8-4$ seconds from $9-5$.

### V12.10
- Stats
  - Base health increased to 590 from 520.
  - Health growth increased to 104 from 90.
  - Armor growth increased to $4.45$ from $3.25$.
  - Magic resistance growth increased to $1.3$ from $0.5$.
- *Personal Space*
  - Base shield reduced to $50-150$ from $50-170$.
  - Shield AP ratio reduced to 75% AP from 80% AP.
- *Doom 'n Gloom*
  - **Bug Fixes:** Fixed a bug where marked enemies would sometimes fail to get feared by her Doom-empowered *Looming Darkness*.

## Trivia

- Vex was the third of three champions released in 2021 tied to **Viego**’s return and a continuation of the Ruined King's story.
- Vex's Shadow cannot be disabled in game.
- Vex shares a few similarities with from *Guilty Gear* series.
  - Both fight with a shadow companion. Her shadow baring a slight resemblance to design in the game.
- Vex's laugh animation references the Laughing Spongebob meme.

---
*This page was automatically generated from League of Legends Wiki data.*