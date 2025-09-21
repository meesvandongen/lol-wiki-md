# Anivia

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
| **Champion** | Anivia |
| **Title** | the Cryophoenix |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2009-07-10 |
| **Release Patch** | July 10, 2009 Patch |
| **Roles** | Battlemage |
| **Riot Positions** | Middle |
| **External Positions** | Middle |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $550.0$ | $+92.0$ | $2114.0$ |
| **Mana** | $495.0$ | $+45.0$ | $1260.0$ |
| **Health Regen** | $5.5$ | $+0.55$ | $14.9$ |
| **Mana Regen** | $8.0$ | $+0.8$ | $21.6$ |
| **Armor** | $21.0$ | $+4.5$ | $97.5$ |
| **Magic Resist** | $30.0$ | $+1.3$ | $52.1$ |
| **Attack Damage** | $51.0$ | $+3.2$ | $105.4$ |
| **Attack Speed** | $0.658$ | $+1.7\%$ | $0.846$ |
| **Movement Speed** | $325.0$ | $+0.0$ | $325.0$ |
| **Attack Range** | $600.0$ | $+0.0$ | $600.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.658$ |
| **Attack Speed Ratio** | $0.625$ |
| **Bonus AS per Level** | $1.7\%$ |
| **Missile Speed** | $1600 units/second$ |
| **Acquisition Radius** | $600 units$ |
| **Pathing Radius** | $35 units$ |
| **Selection Radius** | $140 units$ |
| **Selection Height** | $140 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Rebirth

**Innate:** Periodically, upon death, **Anivia** will instead revert into an egg. If the egg can survive for a few seconds, she is gloriously resurrection.

**Innate:** Periodically, upon taking death, **Anivia** enters resurrection for 6 seconds and restores all of her *health*. While under resurrection, **Anivia** is lockout and gains -40@1; -25@5; -10@8; 5@12; 20@15 *armor *bonus armor* and *mr **bonus** magic resistance*. If **Anivia** remains alive by the end of the duration, she is revived with her **current** health.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Self |

**Notes:**

- Triggering *Rebirth* will break tether on **Anivia**.
- *Chronoshift* and *Guardian Angel* will take priority over *Rebirth*.
- If *Rebirth* triggers while **Anivia** is channeling Teleport, her channel won't be interrupted.
  - All other channel will be interrupted upon triggering *Rebirth*.
- The following refers for interactions while **Anivia** is resurrecting:

---

### Q: Flash Frost

**Active:** **Anivia** launches a chunk of ice that deals magic damage and slows enemies hit.

*Flash Frost* will recast at max range, or can recast early.

**Active:** **Anivia** launches a chunk of ice in the target direction that deals magic damage to enemies hit and slow them by changedisplay=true for 3 seconds. *Flash Frost* can be recast while the ice is in flight after its cast time, and does so automatically at maximum range. **Recast:** **Anivia** shatters the ice, dealing magic damage to nearby enemies and stun them for a duration, as well as refreshing the slow.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $12-8$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $80-100$ mana |
| **Targeting** | Direction / Auto |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Speed** | 950 units/second |
| **Effect Radius** | 225 units |
| **Spell Shield** | Special |
| **Spell Effects** | spellaoe |
| **Projectile** | True |

**Scaling:**
- **Magic Damage:** $50-130$ (+ 25% AP)
- **Magic Damage:* $60-200$ (+ 45% AP)*Stun Duration:** $1.1-1.5$ seconds

**Notes:**

  - If the ability is not manually recasted, the secondary effect will trigger without being considered as an ability activation.
- *Flash Frost* can only proc *Conqueror* once even if it damages the same target twice.
- *Flash Frost* will fire from wherever **Anivia** was at the start of the cast time.
- Spell shield can only prevent either the collision hit or recast from affecting the target.

---

### W: Crystallize

**Active:** **Anivia** airborne a wall of ice that lingers for a few seconds, blocking movement.

**Active:** **Anivia** summons a wall of ice at the target location perpendicular to her facing, airborne all units away from it, though not through terrain. The wall lingers as impassable terrain for 5 seconds.

| Attribute | Value |
|-----------|-------|
| **Range** | 1000 / 100 units |
| **Cooldown** | 17 seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | 70 mana |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Spell Shield** | False |

**Scaling:**
- **Width:** pathing radius
- **Number of ice segments:** $4-8$ chunks of ice Distance between outermost segments $400-800$ units Distance between individual segments $133.33/125/120/116.67/114.29$ units

**Notes:**

- 'Crystallize's displacement triggers in-combat effects by dealing 0 proc damage true damage, such as drawing turret aggro, *Sudden Impact* and applying *Elixir of Sorcery*.
- Individual ice chunks have both 100 units pathing radius and gameplay radius.
- champion are knocked 120 units to either of a wall segment, non-champions 250 units.
  - Knockback speed?
- Player-generated terrain such as *Crystallize* does not block sight.

---

### E: Frostbite

**Active:** **Anivia** blasts a freezing wind at the target enemy that deals magic damage.

*If the target had been recently hit by **Flash Frost** or **Glacial Storm**, the damage is doubled.*

**Passive:** Enemies hit by **Flash Frost** or a fully formed **Glacial Storm** become *Chilled* for 3 seconds, refreshing on subsequent hits. **Active:** **Anivia** blasts a freezing wind at the target enemy that deals magic damage, doubled if they were *Chilled*.

| Attribute | Value |
|-----------|-------|
| **Range** | 600 units |
| **Cooldown** | 4 seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | 50 mana |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Speed** | 1600 units/second |
| **Spell Shield** | True |
| **Spell Effects** | spell |
| **Projectile** | True |

**Scaling:**
- **Magic Damage:* $50-150$ (+ 55% AP)2-150×2$ (+ $55×2$% AP)

**Notes:**

- The damage of *Frostbite* is calculated once it hits. If the target's mark from being hit by *Flash Frost* or a fully formed *Glacial Storm* wears off while the projectile is traveling, the damage is not doubled.
- *Frostbite* has a different sound effect when it hits a target for double damage.

---

### R: Glacial Storm

**Toggle:** **Anivia** summons a driving rain of ice and hail at the target location that continually deals magic damage and slow enemies within.

*Glacial Storm* is deactivated when **Anivia** leaves it or gets hit by crowd control.

**Active:** **Anivia** calls forth a driving rain of ice and hail at the target location, dealing magic damage every $0.5$ seconds to enemies within and slow them for 1 second, refreshing every $0.5$ seconds while they remain inside. The blizzard increases in over $1.5$ seconds. At maximum size, *Glacial Storm* is empowered to deal 300% damage and increase the effectiveness of its slow by 50%, which also instead lasts $1.5$ seconds and refreshes every $0.25$ seconds. *Glacial Storm* can be recast after 1 second, and does so automatically if **Anivia** is no longer in range or unable to pay the *mana* cost, or becomes affected by any form of interrupt crowd control. **Recast:** **Anivia** ends *Glacial Storm*, dealing one last tick of damage.

| Attribute | Value |
|-----------|-------|
| **Range** | 750 units |
| **Cooldown** | $4-2$ seconds |
| **Cast Time** | none |
| **Cost** | 60 mana+$35-55 3$ per second |
| **Targeting** | Location / Auto |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | time active |
| **Spell Shield** | False |
| **Spell Effects** | aoedot |

**Scaling:**
- **Magic Damage per Tick:** $15-30$ (+ $6.25$% AP)
- **Slow:** $20-40$%
- **Empowered Damage per Tick:** $15×3-30×3$ (+ $6.25×3$% AP)
- **Empowered Slow:** $20×1.5-40×1.5$%

**Notes:**

- 'Glacial Storm's slow leaves a trail that is visible even if the target is stealthed.
- *Glacial Storm* deals 3 half ticks at $200/267/333$ radius for a total of $1.5$ normal damage ticks before it starts dealing empowered damage at 400 radius.
- Stasis (buff) via *Zhonya's Hourglass* doesn't interrupt *Glacial Storm*.
  - Devour **does** interrupt (allied and enemy) *Glacial Storm*.

---

## Patch History

### V25.13
- *Rebirth*
  - **Bug Fixes:** Dying to Nexus Obelisk damage no longer triggers 'Rebirth's resurrection.

### V14.22
- Stats
  - Armor growth reduced to $4.5$ from $4.9$.
- *Frostbite*
  - AP ratio reduced to 55% AP from 60% AP.
    - Chilled AP ratio reduced to 110% AP from 120% AP.

### V13.22
- Stats
  - Base attack speed increased to $0.658$ from $0.625$.

### V13.13
- *Rebirth*
  - **Bug Fixes:** Now properly reverts back to her normal resistance values immediately after reviving, instead of after the first automatic stat update.

### V13.4
- Stats
  - Health growth reduced to 92 from 96.
  - Armor growth reduced to $4.9$ from $5.2$.
- *Flash Frost*
  - Cooldown increased to $12-8$ seconds from $11-7$.
- *Frostbite*
  - Base damage reduced to $50-150$ from $50-170$.

### V12.10
- Stats
  - Base health increased to 550 from 480.
  - Health growth increased to 96 from 82.
  - Armor growth increased to $5.2$ from 4.
  - Magic resistance growth increased to $1.3$ from $0.5$.

### V11.20
- General
  - Base voice lines have been remastered to sound clearer, cleaner, and smoother.

### V11.8
- General
  - **Bug Fixes:** Joke and laugh voice lines have been restored.

### V11.6
- *Glacial Storm*
  - **Bug Fixes:** No longer improperly toggles off from activating *Hextech Rocketbelt* Supersonic.

### V11.3
- *Flash Frost*
  - Explosion base damage reduced to $60-200$ from $70-210$.
  - Explosion AP ratio reduced to 45% AP from 50% AP.
- *Glacial Storm*
  - Cooldown increased to $4-2 3$ seconds from $4-1 3$.

## Trivia

- Anivia's dance references the Chicken Dance, a fad dance.
  - A side-by-side comparison can be seen here.
  - She shares this dance with Galio.
- The quote in her lore references Fire and Ice (poem) by Robert Frost.
- Anivia is the first female entirely-non-humanoid champion, the second being **Rek'Sai**.
- As stated by Brian 'FeralPony' Feeney, *Flash Frost* resembles the very first Nexus Crystal.
- From the previous rating system, Anivia was one of five champions with a difficulty of 10, the others being **Cassiopeia**, **Evelynn**, **Rumble**, and **Yasuo**.
  - However, Anivia has not been given this rating originally.
    - Anivia also had an ability of 10, which made her the only champion that scored two 10 ratings.

---
*This page was automatically generated from League of Legends Wiki data.*