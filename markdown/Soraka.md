# Soraka

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
| **Champion** | Soraka |
| **Title** | the Starchild |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2009-02-21 |
| **Release Patch** | Alpha Week 2 |
| **Latest Changes** | V14.24 |
| **Roles** | Enchanter |
| **Riot Positions** | Support |
| **External Positions** | Support |
| **Blue Essence** | 225 |
| **Riot Points** | 260 |
| **Difficulty** | 1 |
| **Hero Type** | Support |
| **Alt Type** | Mage |
| **Adaptive Type** | Magic |
| **Damage** | 1 |
| **Toughness** | 1 |
| **Control** | 2 |
| **Mobility** | 1 |
| **Utility** | 3 |
| **Style** | 100 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $605.0$ | $+88.0$ |
| **Mana** | $425.0$ | $+40.0$ |
| **Health Regen** | $2.5$ | $+0.5$ |
| **Mana Regen** | $11.5$ | $+0.4$ |
| **Armor** | $32.0$ | $+5.0$ |
| **Magic Resist** | $30.0$ | $+1.3$ |
| **Attack Damage** | $50.0$ | $+3.0$ |
| **Attack Speed** | $0.625$ | |
| **Movement Speed** | $325.0$ | $+0.0$ |
| **Attack Range** | $550.0$ | $+0.0$ |
| **Base Attack Speed** | $0.625$ | |
| **Attack Speed Ratio** | $0.625$ | |
| **Bonus AS per Level** | $2.1\%$ | |
| **Missile Speed** | $1000$ units/second | |
| **Acquisition Radius** | $600$ units | |
| **Pathing Radius** | $44.2$ units | |
| **Selection Radius** | $110$ units | |
| **Selection Height** | $145$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $100.0\%$ |
| **Damage Taken** | $100.0\%$ |
| **Healing** | $100.0\%$ |

#### URF

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $105.0\%$ |
| **Damage Taken** | $92.0\%$ |

## Abilities

### Passive: Salvation

| Attribute | Value |
|-----------|------:|
| **Effect Radius** | 2500 units |
| **Targeting** | Passive |
| **Affects** | Self |

**INNATE:** **Soraka** gains ms while facing nearby allied champions that are below 40% of their **maximum** health.

**Notes:**

- **Soraka** will see an arrow indicating the direction in which she can gain the bonus movement speed.

---

### Q: Starcall

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 800 units |
| **Effect Radius** | 265 units |
| **Cost** | 45 / 50 / 55 / 60 / 65 Mana |
| **Cooldown** | 8 / 7 / 6 / 5 / 4 seconds |
| **Targeting** | Location |
| **Affects** | Self, Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | True |
| **Out of Range Behavior** | walk to location |

**ACTIVE:** **Soraka** calls down a star upon the target location that grants sight of the area before landing after type=target range seconds, dealing magic damage to enemies hit and slowing them by 30% for $1.5$ seconds.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 85 / 120 / 155 / 190 / 225 (+ 35% AP) |

If this hits at least one enemy champion, star dust returns to **Soraka**, granting her *Rejuvenation* for $2.5$ seconds. While **Soraka** has *Rejuvenation*, casting *Astral Infusion* will also grant *Rejuvenation* to the target for the same duration.

**REJUVENATION**: Heal every $0.2$ seconds and gain ms that decays over the duration.

| Attribute | Value |
|-----------|------:|
| **Total Heal** | 60 / 75 / 90 / 105 / 120 (+ 30% AP) |
| **Heal per Tick** | 5 / 6.25 / 7.5 / 8.75 / 10 (+ 2.5% AP) |

| Attribute | Value |
|-----------|------:|
| **Bonus Movement Speed** | 20 / 22.5 / 25 / 27.5 / 30% |

**Notes:**

- *Starcall* counts as hitting even if it gets blocked by spell shield, granting *Rejuvenation*.
- Both the initial hit and the *Rejuvenation* projectile can be intercepted.
- *Rejuvenation* heals over 12 ticks, with the first 4 each healing for about 15% of the heal, the next 4 ticks for about $5.5$% each, and the last 4 for about $4.5$% each.
- *Rejuvenation* won't be granted to **Soraka** and she won't be able to make use of the health cost reduction on Astral Infusion until the projectile gets to her, but she is able to *Rejuvenate* allies as soon as the target is hit.

---

### W: Astral Infusion

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 550 units |
| **Cost** | 10% **Maximum** Health + 40 / 45 / 50 / 55 / 60 Mana |
| **Cooldown** | 6 / 5 / 4 / 3 / 2 seconds |
| **Targeting** | Unit |
| **Affects** | Allies / Self |
| **Projectile** | False |

**ACTIVE:** **Soraka** heals the target allied champion.

| Attribute | Value |
|-----------|------:|
| **Heal** | 90 / 110 / 130 / 150 / 170 (+ 50% AP) |

If cast while under *Rejuvenation*, the health cost will be reduced by a percentage.

| Attribute | Value |
|-----------|------:|
| **Health Cost Reduction** | 80 / 85 / 90 / 95 / 100% |
| **Reduced Health Cost** | 2 / 1.5 / 1 / 0.5 / 0% of **maximum** health |

*Astral Infusion* cannot be cast if **Soraka** is below 5% of her **maximum** health.

**Notes:**

- *Astral Infusion* can also be targeted on allied decoys.
- *Astral Infusion* can be cast even if **Soraka** doesn't have the sufficient amount of health to pay for the health cost while above health.
  - As with all abilities with health costs, *Astral Infusion* will not reduce **Soraka**’s health below 1.
- Spirit Visage will only increase the health donated to allies if the target ally possesses it, although owning it *will* boost the heal that **Soraka** receives from Starcall and Wish.
- *Astral Infusion*’s effects occur before the cast time.
- *Astral Infusion* has a forgiveness radius of 175 units.

---

### E: Equinox

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 925 units |
| **Effect Radius** | 260 (Silence field which appears to affect in a very slightly larger radius than the damage and root radius checks cover) units |
| **Cost** | 70 / 75 / 80 / 85 / 90 Mana |
| **Cooldown** | 20 / 19 / 18 / 17 / 16 seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | Special |
| **Spell Effects** | spellaoe |
| **Out of Range Behavior** | walk to location |

**ACTIVE:** **Soraka** creates a celestial zone at the target location that deals magic damage to enemy champions within at the time of cast. The zone then persists for $1.5$ seconds and silences enemies within.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 70 / 95 / 120 / 145 / 170 (+ 40% AP) |

Afterwards, the zone erupts to deal the same damage to enemy champions within and root them for a duration.

| Attribute | Value |
|-----------|------:|
| **Total Magic Damage** | 140 / 190 / 240 / 290 / 340 (+ 80% AP) |

| Attribute | Value |
|-----------|------:|
| **Root Duration** | 1 / 1.25 / 1.5 / 1.75 / 2 seconds |

**Notes:**

- *Equinox*’s silence is marked as non-dispellable, so it is not removed by most cleanses. It is however allowed to be removed by cleanses that **also** grant immunity to it, such as Ragnarok.
- Spell shield will not block the silence.

---

### R: Wish

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Effect Radius** | Global |
| **Cost** | 100 Mana |
| **Cooldown** | 150 / 142.5 / 135 / 127.5 / 120 seconds |
| **Targeting** | Auto |
| **Affects** | Allies |

**ACTIVE:** **Soraka** calls upon the stars, healing herself and all allied champions, increased by 50% on targets below 40% of their **maximum** health.

| Attribute | Value |
|-----------|------:|
| **Heal** | 150 / 200 / 250 / 300 / 350 (+ 50% AP) |
| **Increased Heal** | 225 / 300 / 375 / 450 / 525 (+ 75% AP) |

**Notes:**

- Due to several character stats needing to be recalculated each time a character spawns, casting *Wish* immediately upon respawning will cause its cooldown to be unaffected by ability haste, and the healing will not be increased by **Soraka**’s ability power.
- Untargetability does not invalidate the targeting of the heal.
- *Wish* will not credit **Soraka** for assisting in a champion kill if the target ally is at full health.
- *Wish* will apply heal effects (such as Summon Aery) prioritizing targets based on the *Spawn ID*, which is the order in which units were added to an ongoing game. For champions, this is the order they appear in from left to right in the loading screen.
- *Wish*’s effects occur before the cast time.
- *Wish* will apply to units that are affected by **Mordekaiser**'s R even if **Soraka** is not in the same realm.

---

## Patch History

### V14.24
- Equinox
  - **Bug Fixes:** No longer reveals the user if cast while the cursor is near an enemy champion.

### V14.20
- Astral Infusion
  - **New Effect:** Cast now has a forgiveness radius of 175 units.

### V14.19
- General
  - Updated visual effects.
  - The following skins are affected: Soraka, Soraka, Soraka, Soraka, Soraka, Soraka.

### V14.4
- Salvation
  - Bonus movement speed increased to 90% from 70%.
- Starcall
  - Base heal increased to 60 / 75 / 90 / 105 / 120 from 50 / 65 / 80 / 95 / 110.
- Equinox
  - **Bug Fixes:** Now silences enemies in the area instantly rather than after a variable delay.
- Wish
  - Cooldown reduced to 150 / 135 / 120 seconds from 160 / 145 / 130.

### V13.17
- Soraka
  - Starcall
    - **Bug Fixes:** VFX for increased movement speed buff has been restored.

### V12.16
- General
  - Adjusted splash artwork for Soraka.

### V12.13
- Wish
  - Tooltip now clarifies who is healed.

### V12.12
- Wish
  - Base heal increased to 150 / 250 / 350 from 125 / 200 / 275.
  - **Removed:*** No longer cleanses Grievous Wounds before healing.

### V12.10
- Stats
  - Base health increased to 605 from 535.
  - Health growth increased to 88 from 74.
  - Armor growth increased to 5 from $3.8$.
  - Magic resistance growth increased to $1.3$ from $0.5$.
- Astral Infusion
  - Base heal reduced to 90 / 110 / 130 / 150 / 170 from 100 / 130 / 160 / 190 / 220.
  - Heal AP ratio reduced to 50% AP from 65% AP.
- Wish
  - Base heal reduced to 125 / 200 / 275 from 130 / 215 / 300.
  - Heal AP ratio reduced to 50% AP from 55% AP.

### V12.8
- Astral Infusion
  - Base heal reduced to 100 / 130 / 160 / 190 / 220 from 100 / 135 / 170 / 205 / 240.
  - AP ratio reduced to 65% AP from 70% AP.

## Trivia

- Soraka has the slowest attack missile of all champions.
- Out-of-universe, "Soraka" is the Romanization of Japanese of Japanese 空 (Old Japanese sʷora)&nbsp;香 (Old Japanese ka) "celestial fragrance", a reference to Soraka's celestial nature.
- Soraka was one of the champions available in the Ionian pool during the Ionia vs. Noxus match.
  - Soraka was in the previous match and had apparently performed quite poorly.
- In the Mac launch video, there is a banana in the weapons cabinet. This is a homage to Soraka's basic attack particle and horn, the look of which often have been compared to a banana.
- Soraka's dance references a commercial for Chiquita bananas.
  - A side-by-side comparison can be seen here.
- In the Ultra Rapid Fire game mode (available in April 2014), Soraka was deemed overpowered/obnoxious, and was ultimately disabled in non-custom games. However, she was enabled in future URF and ARURF events.
- Wish could be a reference to the high-level mage spell "Wish" in Dungeons & Dragons. One of the functions of the Wish spell is to heal the entire party, which is very much alike to Soraka's ability.
- The ward skin Starcall Ward.png references her.
- Soraka is one of champions who use health as resource for their abilities, the other five being Briar, Dr. Mundo, Olaf, Vladimir, and Zac.

---
*This page was automatically generated from League of Legends Wiki data.*