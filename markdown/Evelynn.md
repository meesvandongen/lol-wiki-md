# Evelynn

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
| **Champion** | Evelynn |
| **Title** | Agony's Embrace |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2009-05-01 |
| **Release Patch** | May 1, 2009 Patch |
| **Latest Changes** | V25.14 |
| **Roles** | Assassin |
| **Riot Positions** | Jungle |
| **External Positions** | Jungle |
| **Blue Essence** | 675 |
| **Riot Points** | 585 |
| **Difficulty** | 2 |
| **Hero Type** | Assassin |
| **Alt Type** | Mage |
| **Adaptive Type** | Magic |
| **Damage** | 2 |
| **Toughness** | 2 |
| **Control** | 1 |
| **Mobility** | 2 |
| **Utility** | 2 |
| **Style** | 75 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $642.0$ | $+98.0$ |
| **Mana** | $315.0$ | $+42.0$ |
| **Health Regen** | $8.5$ | $+0.75$ |
| **Mana Regen** | $8.11$ | $+0.6$ |
| **Armor** | $37.0$ | $+4.7$ |
| **Magic Resist** | $32.0$ | $+2.05$ |
| **Attack Damage** | $61.0$ | $+3.0$ |
| **Attack Speed** | $0.667$ | |
| **Movement Speed** | $335.0$ | $+0.0$ |
| **Attack Range** | $125.0$ | $+0.0$ |
| **Base Attack Speed** | $0.667$ | |
| **Attack Speed Ratio** | $0.667$ | |
| **Bonus AS per Level** | $2.1\%$ | |
| **Attack Windup** | $15.3\%$ | |
| **Acquisition Radius** | $400$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $120$ units | |
| **Selection Height** | $110$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $110.0\%$ |
| **Damage Taken** | $90.0\%$ |

#### URF

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $105.0\%$ |
| **Damage Taken** | $95.0\%$ |

#### Ultimate Spellbook

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $110.0\%$ |
| **Damage Taken** | $100.0\%$ |

## Abilities

### Passive: Demon Shade

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Self |
| **Detection Radius** | 700 |

**INNATE:** **Evelynn** shrouds herself in *Demon Shade* after not performing actions that break stealth for 4 seconds. Attacking or casting abilities ends *Demon Shade* immediately and places it on a 4-second static cooldown (Unaffected by ability haste), reduced to $1.25$ seconds from casting *Last Caress*. Taking damage from champions or turrets interrupts *Demon Shade* and places it on a $1.5$-second static cooldown (Unaffected by ability haste).

**DEMON SHADE:** While below (health) 250 to 590 (+ 250% AP) health, **Evelynn** heals herself for 15 to 150 every second. From level.md) 6 onward, *Demon Shade* also grants camouflage.

**Notes:**

- *Demon Shade* is also placed on a $1.5$ second cooldown upon respawning.
- *Demon Shade*’s cooldown incurs upon starting the attack windup of the basic attack and upon the cast time of the ability.
- *Demon Shade* activates even if **Evelynn** is Recall.
- Upon **Evelynn**’s transition into the stealth, *Demon Shade* will interrupt her in-progress cast commands for unit-targeted spells, excluding *E* and its *empowered* cast.
  - If **Evelynn**’s last issued order was to cast *E*, her transition into the stealth will be followed by an automatic command for casting the ability (if no other orders were issued in the meantime).
- Whenever *Demon Shade* activates, a puff of purple smoke will briefly appear around her.
- Using a basic attack breaks the stealth at the start of the attack windup.

---

### Q: Hate Spike

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.3$ seconds |
| **Target Range** | 800 (Dart range) / 680 (Spike range) units |
| **Effect Radius** | 550 (Recast range, center to edge) units |
| **Width** | 120 (Dart width) / 180 (Spike width) units |
| **Speed** | 2400 (Dart speed) units/second |
| **Cost** | 40 / 45 / 50 / 55 / 60 Mana |
| **Cooldown** | 4 seconds |
| **Cooldown Start** | on-cast |
| **Queue Time** | $0.2$ (Initial cast) seconds |
| **Targeting** | Direction / Proximity |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Special |
| **Projectile** | True |
| **Out of Range Behavior** | The ability will not cast (recast) |
| **Barrage Cooldown (Cooldown in between each spike cast, affected by ability haste)** | $0.5$ |

**ACTIVE:** **Evelynn** launches a dart in the target direction, dealing magic damage to the first enemy hit and marking them for 4 seconds.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 25 / 30 / 35 / 40 / 45 (+ 25% AP) |

**Evelynn**’s next 3 basic attacks or abilities against the marked target deal **bonus** magic damage.

| Attribute | Value |
|-----------|------:|
| **Bonus Magic Damage** | 15 / 25 / 35 / 45 / 55 (+ 25% AP) |
| **Total Bonus Damage** | 45 / 75 / 105 / 135 / 165 (+ 75% AP) |

*Hate Spike* may be recast up to 3 times at no cost until the ability comes off cooldown.

**RECAST:** **Evelynn** unleashes a line of deadly spikes in the direction of the nearest visible enemy that deals magic damage to all enemies struck.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 25 / 30 / 35 / 40 / 45 (+ 25% AP) |
| **Maximum Magic Damage** | 75 / 90 / 105 / 120 / 135 (+ 75% AP) |
| **Total Magic Damage** | 145 / 195 / 245 / 295 / 345 (+ 175% AP) |

*Hate Spike*’s recast prioritizes targeting **Evelynn**’s attack target, then the nearest enemy champion, then the nearest non-champion.

*A nearby visible enemy is required to recast this ability.*

**Notes:**

- Each cast counts as an ability activation for the purposes of on-cast effects such as Spellblade and triggering Force Pulse’s passive.
- *Hate Spike* prioritizes:
  1. The last champion **Evelynn** hit
  1. The last non-champion **Evelynn** hit
  1. The lowest-health champion
  1. The lowest-health non-champion
- The dart applies spell damage and the spikes deal area damage. Effect at cast time start
- Recasts can be buffered up to $0.5$ seconds before becoming castable at a maximum range of approximately 500 units.

---

### W: Allure

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 1200 / 1300 / 1400 / 1500 / 1600 units |
| **Cost** | 60 / 70 / 80 / 90 / 100 Mana |
| **Cooldown** | 15 / 14 / 13 / 12 / 11 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | Special |
| **Spell Effects** | Single target |

**ACTIVE:** **Evelynn** *curses* the target enemy champion or medium or large monster for 5 seconds.

**Evelynn**’s next basic attack or ability against the *accursed* target *expunges* them, slowing them by 45% for $0.75$ seconds.

*Expunging* a target will refund *Allure*’s (mana) mana cost. If the target is *cursed* for at least $2.5$ seconds, the *expunge* also charms them for a few seconds and the duration of the slow is increased to last for this time, as well as applying additional effects based on the target type.

| Attribute | Value |
|-----------|------:|
| **Disable Duration** | 1.25 / 1.5 / 1.75 / 2 / 2.25 seconds |

Against champions, the *expunge* also inflicts (magic penetration) magic resistance reduction for 4 seconds.

| Attribute | Value |
|-----------|------:|
| **Magic Resistance Reduction** | 35 / 37.5 / 40 / 42.5 / 45% |

Against monsters, the *expunge* deals **bonus** magic damage and increases the duration of the charm by 2 seconds.

| Attribute | Value |
|-----------|------:|
| **Bonus Magic Damage** | 250 / 300 / 350 / 400 / 450 (+ 60% AP) |

| Attribute | Value |
|-----------|------:|
| **Monster Disable Duration** | 3.25 / 3.5 / 3.75 / 4 / 4.25 seconds |

*Casting Allure does not break Demon Shade nor its fade into it.*

**Notes:**

- The mark fully forming signifies the $2.5$ seconds have elapsed, meaning the charm will be applied on the triggering attack or ability.
  - An enemy champion cursed by *Allure* is alarmed of her presence and after the mark has fully formed, an arrow will appear next to them pointing towards **Evelynn**’s direction. This is visible to both teams.
- The (magic penetration) magic resistance reduction will be applied to the attack or ability that triggers it.
- *Allure*’s mark application on a target cannot be blocked by spell shield.
  - The expunge will not occur if the ability is blocked by a spell shield. The mark does not trigger and will remain on the target in this case.
- The (magic penetration) magic resistance reduction debuff is applied independently of the Charm debuff when the matured mark is expunged, meaning a target can still have their magic resistance reduced while they are resisting applications of CC debuffs (e.g. Morgana’s Black Shield).
- If the target becomes untargetable, dies, or is too far away or no longer in sight during the cast time, this ability will cancel but still go on cooldown and pay its cost.
  - *Allure* will restart its cooldown whenever the mark debuff is removed.

---

### E: Empowered Whiplash

| Attribute | Value |
|-----------|------:|
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Parry** | Special |
| **Call For Help** | True |
| **Grounded** | True |
| **Knockdown** | True |

**ACTIVE:** **Evelynn** dashes toward the target enemy with her lashers, dealing increased damage to them upon completion as well as to all enemies within her path.

| Attribute | Value |
|-----------|------:|
| **Empowered Magic Damage** | 80 / 120 / 160 / 200 / 240 (+ 4% (+ $2.5$% per 100 AP) of target's **maximum** health) |

*Empowered Whiplash* only applies on-hit effects to the primary target.

**Notes:**

- **Evelynn** will track the target if they change locations.
  - She will dash to the target's previous location if the target moves 2000 (Estimated) or more units away.
  - She will not stop tracking the target until she reaches them, as there is no maximum tracking distance.
- There is no time-out period on *Empowered Whiplash*.
- *Empowered Whiplash* can be blocked but cannot be dodged and/or missed if **Evelynn** is blinded.

---

### E: Whiplash

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 210 units |
| **Cost** | 40 / 45 / 50 / 55 / 60 Mana |
| **Cooldown** | 8 seconds |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Spell |
| **Projectile** | False |
| **Parry** | Special |
| **Call For Help** | True |

**ACTIVE:** **Evelynn** whips the target enemy with her lashers, dealing magic damage and applying on-hit effects.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 60 / 90 / 120 / 150 / 180 (+ 3% (+ $1.5$% per 100 AP) of target's **maximum** health) |

**Evelynn** also gains (ms) **bonus** movement speed for 2 seconds.

| Attribute | Value |
|-----------|------:|
| **Bonus Movement Speed** | 30 / 35 / 40 / 45 / 50% |

Gaining *Demon Shade* resets *Whiplash*’s cooldown and *empowers* its next cast. The damage based on the target's health ratio for both *Whiplash* and its *empowered* cast deals a minimum of 25 and is capped at 450 against monsters.

**Notes:**

- *Whiplash* can be dodged and blocked but may not miss if **Evelynn** is blinded.

---

### R: Last Caress

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.35$ seconds |
| **Effect Radius** | 500 (Estimated) units |
| **Angle** | 180° |
| **Cost** | 100 Mana |
| **Cooldown** | 120 / 110 / 100 / 90 / 80 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies / Self |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |
| **Projectile** | False |
| **Grounded** | True |

**ACTIVE:** **Evelynn** reveals her true form, becoming untargetable at the start of the cast time and then unleashing her lashers in a cone in the target direction, dealing magic damage to enemies struck within. After $0.5$ seconds (From end of cast time), she blinks 700 units in the opposite direction and becomes targetable again.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 125 / 187.5 / 250 / 312.5 / 375 (+ 75% AP) |

*Last Caress* is empowered to deal 240% damage against enemies below (health) 30% of their **maximum** health.

| Attribute | Value |
|-----------|------:|
| **Empowered Damage** | 300 / 450 / 600 / 750 / 900 (+ 180% AP) |

**Notes:**

- *Last Caress* can be buffered during Empowered Whiplash.
- The casting input controls the direction *Last Caress' * damage is dealt from, with the movement automatically happening in the reverse direction.
- Enemy champions below the health threshold are marked for **Evelynn**, indicating the target will receive *Last Caress' * increased damage.
- The screen will direct to **Evelynn**’s position after she blinks.

#### Channel Behavior (cast)

| Aspect | State / Notes |
|--------|---------------|
| **Summoner Spells** | Allowed / Disabled |

---

## Patch History

### V25.14
- General
  - **Bug Fixes:** Restored VO for various events.

### V25.S1.3
- Whiplash
  - Base damage increased to 60 / 90 / 120 / 150 / 180 from 55 / 70 / 85 / 100 / 115.
- Empowered Whiplash
  - Base damage increased to 80 / 120 / 160 / 200 / 240 from 75 / 100 / 125 / 150 / 175.

### V25.S1.2
- Whiplash
  - Bonus movement speed increased to 30 / 35 / 40 / 45 / 50% from 30% at all ranks.
- Last Caress
  - AP ratio increased to 75% AP from 65% AP.
    - Maximum AP ratio increased to 180% AP from 156% AP.

### V14.11
- Hate Spike
  - **Bug Fixes:** SFX once again properly plays for **Evelynn** upon hitting a target at max range with the first spike.

### V14.9
- Allure
  - Slow reduced to 45% from 65%.
- Last Caress
  - AP ratio reduced to 65% AP from 75% AP.
    - Maximum AP ratio reduced to 156% AP from 180% AP.

### V14.8
- General
  - **Bug Fixes:** VO lines for first encounter with Brand and Dr. Mundo are no longer incorrectly swapped.
- Last Caress
  - **Bug Fixes:** Now properly triggers her VO lines.

### V14.5
- Hate Spike
  - Dart AP ratio reduced to 25% AP from 30% AP.
  - Spike AP ratio reduced to 25% AP from 30% AP.

### V14.2
- Allure
  - **Removed:*** Can no longer target small monsters.
- Last Caress
  - Now checks if enemies are below the health threshold much more often.

### V13.24
- Hate Spike
  - **Bug Fixes:** SFX now properly plays even if the target is in close proximity.

### V13.19
- Hate Spike
  - **Bug Fixes:** Now properly plays its SFX even if it hits a target at maximum range.

## Trivia

- Evelynn's dance references Tina Tech's dance from the movie Flashdance.
  - A side-by-side comparison can be seen here.
- Evelynn is one of five champions which had a difficulty rating of 10 on the old scale, the others being Anivia, Cassiopeia, Rumble, and Yasuo.
  - She was not originally given this rating.
- Evelynn's Series 2 Eternals make the following references:
  - *Bad Romance* is a reference to the eponymous song from the Lady Gaga.

---
*This page was automatically generated from League of Legends Wiki data.*