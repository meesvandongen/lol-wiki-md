# Soraka

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
| **Champion** | Soraka |
| **Title** | the Starchild |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2009-02-21 |
| **Release Patch** | Alpha Week 2 |
| **Roles** | Enchanter |
| **Riot Positions** | Support |
| **External Positions** | Support |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $605.0$ | $+88.0$ | $2101.0$ |
| **Mana** | $425.0$ | $+40.0$ | $1105.0$ |
| **Health Regen** | $2.5$ | $+0.5$ | $11.0$ |
| **Mana Regen** | $11.5$ | $+0.4$ | $18.3$ |
| **Armor** | $32.0$ | $+5.0$ | $117.0$ |
| **Magic Resist** | $30.0$ | $+1.3$ | $52.1$ |
| **Attack Damage** | $50.0$ | $+3.0$ | $101.0$ |
| **Attack Speed** | $0.625$ | $+2.1\%$ | $0.852$ |
| **Movement Speed** | $325.0$ | $+0.0$ | $325.0$ |
| **Attack Range** | $550.0$ | $+0.0$ | $550.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.625$ |
| **Attack Speed Ratio** | $0.625$ |
| **Bonus AS per Level** | $2.1\%$ |
| **Missile Speed** | $1000 units/second$ |
| **Acquisition Radius** | $600 units$ |
| **Pathing Radius** | $44.2 units$ |
| **Selection Radius** | $110 units$ |
| **Selection Height** | $145 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Salvation

**Innate:** **Soraka** gains tremendous *ms **bonus** movement speed* while facing nearby allied champions that are at low health.

**Innate:** **Soraka** gains ms*bonus** movement speed* while facing nearby allied champions that are below 40% of their **maximum** health.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Self |
| **Effect Radius** | 2500 units |

**Notes:**

- **Soraka** will see an arrow indicating the direction in which she can gain the bonus movement speed.

---

### Q: Starcall

**Active:** **Soraka** calls down a star upon the target location that deals magic damage and briefly slows enemies hit. If this hits at least one enemy champion, the star dust returns to **Soraka** to grant her *Rejuvenation* for a short time.

*While **Soraka** has *Rejuvenation*, **Astral Infusion** will also grant *Rejuvenation* to the target ally.*

**Active:** **Soraka** calls down a star upon the target location that grants sight of the area before landing after type=target range seconds, dealing magic damage to enemies hit and slow them by 30% for $1.5$ seconds. If this hits at least one enemy champion, star dust returns to **Soraka**, granting her *Rejuvenation* for $2.5$ seconds. While **Soraka** has *Rejuvenation*, casting **Astral Infusion** will also grant *Rejuvenation* to the target for the same duration. **Rejuvenation**: Heal every $0.2$ seconds and gain *ms **bonus** movement speed* that decays over the duration.

| Attribute | Value |
|-----------|-------|
| **Range** | 800 units |
| **Cooldown** | $8-4$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $45-65$ Mana |
| **Targeting** | Location |
| **Affects** | Self, Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | 265 units |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | True |

**Scaling:**
- **Magic Damage:** $85-225$ (+ 35% AP)
- **Total Heal:* $60-120$ (+ 30% AP)*Bonus Movement Speed:** $20-30$%

**Notes:**

- *Starcall* counts as hitting even if it gets blocked by spell shield, granting *Rejuvenation*.
- Both the initial hit and the *Rejuvenation* projectile can be projectile.
- *Rejuvenation* heals over 12 ticks, with the first 4 each healing for about 15% of the heal, the next 4 ticks for about $5.5$% each, and the last 4 for about $4.5$% each.
- *Rejuvenation* won't be granted to **Soraka** and she won't be able to make use of the health cost reduction on *Astral Infusion* until the projectile gets to her, but she is able to *Rejuvenate* allies as soon as the target is hit.

---

### W: Astral Infusion

**Active:** **Soraka** consumes a portion of her **maximum** health to heal the target allied champion.

*If this is cast while under **Rejuvenation**, the health cost is reduced.*

**Active:** **Soraka** heals the target allied champion. If cast while under **Rejuvenation**, the health cost will be reduced by a percentage. *Astral Infusion* cannot be cast if **Soraka** is below .

| Attribute | Value |
|-----------|-------|
| **Range** | 550 units |
| **Cooldown** | $6-2$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | 10% **Maximum** Health + $40-60$ Mana |
| **Targeting** | Unit |
| **Affects** | Allies / Self |
| **Projectile** | False |

**Scaling:**
- **Heal:** $90-170$ (+ 50% AP)
- **Health Cost Reduction:* $80-100$%*maximum** health

**Notes:**

- *Astral Infusion* can also be targeted on allied clone.
- *Astral Infusion* can be cast even if **Soraka** doesn't have the sufficient amount of health to pay for the health cost while above .
  - As with all abilities with health costs, *Astral Infusion* will not reduce ''Soraka's' health below 1.
- *Spirit Visage* will only increase the health donated to allies if the target ally possesses it, although owning it *will* boost the heal that **Soraka** receives from *Starcall* and *Wish*.
- 'Astral Infusion's effects occur before the cast time.
- *Astral Infusion* has a forgiveness radius of 175 units.

---

### E: Equinox

**Active:** **Soraka** creates a celestial zone at the target location that deals magic damage to enemy champions within upon casting. The zone then persists for a brief moment and silence enemies within.

*Afterwards, the zone erupts to deal magic damage to enemy champions within and briefly root them.*

**Active:** **Soraka** creates a celestial zone at the target location that deals magic damage to enemy champions within at the time of cast. The zone then persists for $1.5$ seconds and silence enemies within. Afterwards, the zone erupts to deal the same damage to enemy champions within and root them for a duration.

| Attribute | Value |
|-----------|-------|
| **Range** | 925 units |
| **Cooldown** | $20-16$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $70-90$ Mana |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | 260 units |
| **Spell Shield** | Special |
| **Spell Effects** | spellaoe |

**Scaling:**
- **Magic Damage:** $70-170$ (+ 40% AP)
- **Total Magic Damage:** $70×2-170×2$ (+ 80% AP)
- **Root Duration:** $1-2$ seconds

**Notes:**

- 'Equinox's silence is marked as non-dispellable, so it is not removed by most cleanse. It is however allowed to be removed by cleanses that **also** grant immunity to it, such as Ragnarok.
- Spell shield will not block the silence.

---

### R: Wish

**Active:** **Soraka** calls upon the stars, heal herself and all allied champions, regardless of distance. The healing is increased on targets at low health.

**Active:** **Soraka** calls upon the stars, heal herself and all allied champions, increased by 50% on targets below 40% of their **maximum** health.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $150-120$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | 100 Mana |
| **Targeting** | Auto |
| **Affects** | Allies |
| **Effect Radius** | Global |

**Scaling:**
- **Heal:* $150-350$ (+ 50% AP)1.5-350×1.5$ (+ 75% AP)

**Notes:**

- Due to several character stats needing to be recalculated each time a character spawns, casting *Wish* immediately upon respawning will cause its cooldown to be unaffected by ability haste, and the healing will not be increased by ''Soraka's' ability power.
- Untargetable does not invalidate the targeting of the heal.
- *Wish* will not credit **Soraka** for assisting in a champion kill if the target ally is at full health.
- *Wish* will apply heal effects (such as *Summon Aery*) prioritizing targets based on the *Spawn ID*, which is the order in which units were added to an ongoing game. For champions, this is the order they appear in from left to right in the loading screen.
- 'Wish's effects occur before the cast time.
- *Wish* will apply to units that are affected by **Mordekaiser**'s R even if **Soraka** is not in the same realm.

---

## Patch History

### V14.24
- *Equinox*
  - **Bug Fixes:** No longer reveals the user if cast while the cursor is near an enemy champion.

### V14.20
- *Astral Infusion*
  - **New Effect:** Cast now has a forgiveness radius of 175 units.

### V14.19
- General
  - Updated visual effects.
  - The following skins are affected: Soraka, Soraka, Soraka, Soraka, Soraka, Soraka.

### V14.4
- *Salvation*
  - Bonus movement speed increased to 90% from 70%.
- *Starcall*
  - Base heal increased to $60-120$ from $50-110$.
- *Equinox*
  - **Bug Fixes:** Now silences enemies in the area instantly rather than after a variable delay.
- *Wish*
  - Cooldown reduced to $150-120 3$ seconds from $160-130 3$.

### V13.17
- Soraka
  - *Starcall*
    - **Bug Fixes:** VFX for increased movement speed buff has been restored.

### V12.16
- General
  - Adjusted splash artwork for Soraka.

### V12.13
- *Wish*
  - Tooltip now clarifies who is healed.

### V12.12
- *Wish*
  - Base heal increased to $150-350 3$ from $125-275 3$.
  - **Removed:*** No longer cleanses Grievous Wounds before healing.

### V12.10
- Stats
  - Base health increased to 605 from 535.
  - Health growth increased to 88 from 74.
  - Armor growth increased to 5 from $3.8$.
  - Magic resistance growth increased to $1.3$ from $0.5$.
- *Astral Infusion*
  - Base heal reduced to $90-170$ from $100-220$.
  - Heal AP ratio reduced to 50% AP from 65% AP.
- *Wish*
  - Base heal reduced to $125-275 3$ from $130-300 3$.
  - Heal AP ratio reduced to 50% AP from 55% AP.

### V12.8
- *Astral Infusion*
  - Base heal reduced to $100-220$ from $100-240$.
  - AP ratio reduced to 65% AP from 70% AP.

## Trivia

- Soraka has the slowest attack missile of all champions.
- Out-of-universe, "Soraka" is the Romanization of Japanese of Japanese 空&nbsp;香 "celestial fragrance", a reference to Soraka's celestial nature.
- Soraka was one of the champions available in the Ionian pool during the Ionia vs. Noxus match.
  - Soraka was in the previous match and had apparently performed quite poorly.
- In the Mac launch video, there is a banana in the weapons cabinet. This is a homage to Soraka's basic attack particle and horn, the look of which often have been compared to a banana.
- Soraka's dance references a commercial for Chiquita bananas.
  - A side-by-side comparison can be seen here.
- In the Ultra Rapid Fire game mode (available in April 2014), Soraka was deemed overpowered/obnoxious, and was ultimately disabled in non-custom games. However, she was enabled in future URF and ARURF events.
- *Wish* could be a reference to the high-level mage spell "Wish" in Dungeons & Dragons. One of the functions of the Wish spell is to heal the entire party, which is very much alike to Soraka's ability.
- The ward skin Starcall Ward.png references her.
- Soraka is one of champions who use health as resource for their abilities, the other five being , **Dr. Mundo*, Ci*Vladimir**, and **Zac**.

---
*This page was automatically generated from League of Legends Wiki data.*