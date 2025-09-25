# Karma

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
| **Champion** | Karma |
| **Title** | the Enlightened One |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2011-02-01 |
| **Release Patch** | V1.0.0.110 |
| **Latest Changes** | V25.13 |
| **Roles** | Burst, Enchanter |
| **Riot Positions** | Middle, Support |
| **External Positions** | Top, Middle, Support |
| **Blue Essence** | 1575 |
| **Riot Points** | 790 |
| **Difficulty** | 1 |
| **Hero Type** | Mage |
| **Alt Type** | Support |
| **Adaptive Type** | Magic |
| **Damage** | 2 |
| **Toughness** | 1 |
| **Control** | 2 |
| **Mobility** | 1 |
| **Utility** | 2 |
| **Style** | 100 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $630.0$ | $+109.0$ |
| **Mana** | $374.0$ | $+40.0$ |
| **Health Regen** | $5.5$ | $+0.55$ |
| **Mana Regen** | $13.0$ | $+0.8$ |
| **Armor** | $28.0$ | $+5.0$ |
| **Magic Resist** | $30.0$ | $+1.3$ |
| **Attack Damage** | $51.0$ | $+3.3$ |
| **Attack Speed** | $0.625$ | |
| **Movement Speed** | $335.0$ | $+0.0$ |
| **Attack Range** | $525.0$ | $+0.0$ |
| **Base Attack Speed** | $0.625$ | |
| **Attack Speed Ratio** | $0.625$ | |
| **Bonus AS per Level** | $2.3\%$ | |
| **Missile Speed** | $1500$ units/second | |
| **Acquisition Radius** | $525$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $120$ units | |
| **Selection Height** | $145$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $100.0\%$ |
| **Damage Taken** | $100.0\%$ |

## Abilities

### Passive: Gathering Fire

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Self |

**INNATE:** *Mantra’s* **current** cooldown is reduced by 4 seconds for each enemy champion hit by **Karma**’s damaging abilities.

**Notes:**

- Inner Flame / Soulflare reduces Mantra’s cooldown per champion hit by either damage instance (up to a maximum total reduction of 40 seconds if 5 enemy champions are hit by both instances of damage).
- Focused Resolve / Renewal reduces Mantra’s cooldown when initially cast and again after the duration for the tether has expired (for a total reduction of 8 seconds).

---

### Q: Inner Flame

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 950 / er 890 units |
| **Effect Radius** | cr 280 (Collision detonation radius) |
| **Width** | 120 (Missile width) units |
| **Speed** | 1700 (Missile speed) units/second |
| **Cost** | 40 / 50 / 60 / 70 / 80 Mana |
| **Cooldown** | 9 / 8 / 7 / 6 / 5 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |
| **Projectile** | True |

**ACTIVE:** **Karma** fires a bolt in the target direction that explodes on the first enemy hit, dealing magic damage to nearby enemies and slowing them by 40% for $1.5$ seconds.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 60 / 110 / 160 / 210 / 260 (+ 70% AP) |

**Notes:**

- Spell shield will block the initial impact from *Inner Flame*.
- This ability will cast from wherever the caster is at the end of the cast time.
- *Inner Flame*’s effect radius is centered around the location of the missile as it collides.

---

### Q: Soulflare

| Attribute | Value |
|-----------|------:|
| **Effect Radius** | 280 (Field area of effect radius) units |
| **Width** | 160 (Increased missile width) units |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | Special |
| **Spell Effects** | spellaoe |
| **Projectile** | True |

**MANTRA BONUS:** *Inner Flame* deals increased damage, and fires a larger bolt that also explodes at maximum range.

| Attribute | Value |
|-----------|------:|
| **Bonus Magic Damage** | 40 / 100 / 160 / 220 (+ 30% AP) |
| **Total Magic Damage** | 60 / 110 / 160 / 210 / 260 (+ 40 / 100 / 160 / 220) (+ 100% AP) |

The explosion creates a field for $1.5$ seconds that slows enemies within by 50%, which then ruptures to deal magic damage.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 40 / 130 / 220 / 310 (+ 50% AP) |
| **Total Bonus Damage** | 80 / 230 / 380 / 530 (+ 80% AP) |
| **Total Damage** | 60 / 110 / 160 / 210 / 260 (+ 80 / 230 / 380 / 530) (+ 150% AP) |

*Soulflare* scales with *Mantra’s* rank.

**Notes:**

- Spell shield will block the initial impact from either ability but will not block *Soulflare*’s field damage.
- *Soulflare*’s detonation radius is centered around the location of the missile as it collides, while the lingering field will always be created at the impacted enemy's center instead.
- *Soulflare* will cast from wherever **Karma** is at the end of the cast time.

---

### W: Focused Resolve

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 675 units |
| **Tether Radius** | 825 units |
| **Cost** | 50 / 55 / 60 / 65 / 70 Mana |
| **Cooldown** | 12 seconds |
| **Targeting** | Unit |
| **Affects** | Enemies / Self |
| **Damage Type** | Magic |
| **Spell Shield** | Special |
| **Spell Effects** | spell |
| **Projectile** | False |
| **Call For Help** | True |

**ACTIVE:** **Karma** deals magic damage to the target enemy champion, monster or pet and forms a tether between her and them for 2 seconds, during which they are revealed.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 40 / 65 / 90 / 115 / 140 (+ 45% AP) |

If the tether is not broken by the end of its duration, the target is dealt the same magic damage again and is rooted for a duration, during which they are revealed.

| Attribute | Value |
|-----------|------:|
| **Total Magic Damage** | 80 / 130 / 180 / 230 / 280 (+ 90% AP) |

| Attribute | Value |
|-----------|------:|
| **Root Duration** | 1.6 / 1.7 / 1.8 / 1.9 / 2 seconds |

**Notes:**

- Spell shield will block the tether's application and damage but not the aftereffects of one already applied.

---

### W: Renewal

| Attribute | Value |
|-----------|------:|
| **Targeting** | Unit |
| **Affects** | Enemies / Self |
| **Damage Type** | Magic |
| **Spell Shield** | Special |
| **Spell Effects** | spell |
| **Projectile** | False |
| **Call For Help** | True |

**MANTRA BONUS:** *Focused Resolve*’s root duration is increased. 

**Karma** heals for (health) 17% (+ 1% per 100 AP) of her **missing** health once on-cast, and again once the tether lasts its full duration or the target dies while tethered.

| Attribute | Value |
|-----------|------:|
| **Root Duration Increase** | 0.5 / 0.75 / 1 / 1.25 seconds |
| **Total Root Duration** | 1.6 / 1.7 / 1.8 / 1.9 / 2 (+ 0.5 / 0.75 / 1 / 1.25) seconds |

*Renewal* scales with *Mantra’s* rank.

**Notes:**

- Spell shield will block the tether's application and damage but not the aftereffects of one already applied.

---

### E: Defiance

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 700 (Shield spread radius to allied champions around primary target) units |
| **Targeting** | Unit |
| **Affects** | Allies |

**MANTRA - ACTIVE:** *Inspire* grants a bonus amount of shielding.

*Inspire* spreads to surrounding allied champions, granting them a shield for $2.5$ seconds and (ms) 15% **bonus** movement speed for 2 seconds.

| Attribute | Value |
|-----------|------:|
| **Bonus Primary Target Shield** | 50 / 100 / 150 / 200 (+ 45% AP) |
| **Total Primary Target Shield** | 80 / 130 / 180 / 230 / 280 (+ 50 / 100 / 150 / 200) (+ 105% AP) |

| Attribute | Value |
|-----------|------:|
| **Secondary Target Shield** | 50 / 100 / 150 / 200 (+ 45% AP) |

*Defiance* scales with *Mantra’s* rank.

**Notes:**

- *Defiance* has a forgiveness radius of 175 units.

---

### E: Inspire

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 800 units |
| **Cost** | 50 / 55 / 60 / 65 / 70 Mana |
| **Cooldown** | 10 / 9.5 / 9 / 8.5 / 8 seconds |
| **Targeting** | Unit |
| **Affects** | Allies |

**ACTIVE:** **Karma** grants a shield to herself or the target allied champion for $2.5$ seconds as well as (ms) 40% **bonus** movement speed for 2 seconds.

| Attribute | Value |
|-----------|------:|
| **Shield Strength** | 80 / 130 / 180 / 230 / 280 (+ 60% AP) |

**Notes:**

- *Inspire* has a forgiveness radius of 175 units.

---

### R: Mantra

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Cooldown** | 40 / 38 / 36 / 34 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Auto |
| **Affects** | Self |

**ACTIVE:** **Karma** empowers her next basic ability within 8 seconds for an additional effect. 

*Mantra can be used while affected by cast-inhibiting crowd control.*

**Karma** begins the game with one rank in *Mantra*. Her empowered abilities scale based on *Mantra*’s rank.

**Notes:**

No additional notes.

---

## Patch History

### V25.13
- General
  - **Bug Fixes:** The symbol on her cape piece above her now properly plays its animations after the player has reconnected.

### V25.06
- Inner Flame
  - Base damage reduced to 60 / 110 / 160 / 210 / 260 from 70 / 120 / 170 / 220 / 270.

### V25.S1.2
- Soulflare
  - **Bug Fixes:** Now properly benefits from Axiom Arcanist.

### V14.24
- Mantra
  - **Bug Fixes:** No longer reveals the user if cast while the cursor is near an enemy champion.

### V14.20
- Inspire
  - **New Effect:** Cast now has a forgiveness radius of 175 units.
- Defiance
  - **New Effect:** Cast now has a forgiveness radius of 175 units.

### V14.13
- Mantra
  - **Bug Fixes:** Cooldown is now refunded if Soulflare kills a target.

### V14.12
- Inspire
  - Bonus movement speed duration increased to 2 seconds from $1.5$.
  - Base shield increased to 80 / 130 / 180 / 230 / 280 from 80 / 125 / 170 / 215 / 260.
- Defiance
  - Bonus base shield increased to 50 / 100 / 150 / 200 from 50 / 90 / 130 / 170.

### V14.9
- Inner Flame
  - Slow increased to 40% from 35%.
- Defiance
  - Secondary target bonus movement speed increased to 15% from 12%.
  - Secondary target shielding increased to 100% of bonus shield from 90%.
    - Secondary target base shield increased to 50 / 90 / 130 / 170 from 45 / 81 / 117 / 153.
    - Secondary target AP ratio increased to 45% AP from 40.5% AP.

### V14.7
- Focused Resolve
  - Root duration increased to 1.6 / 1.7 / 1.8 / 1.9 / 2 seconds from 1.4 / 1.55 / 1.7 / 1.85 / 2.
- Defiance
  - Secondary target total shield strength changed to 90% of *Defiance*’s bonus shield from 30% of Inspire’s base shield plus 30% of *Defiance*’s bonus shield.
    - Secondary target base shield strength changed to 45 / 81 / 117 / 153 from 24 / 37.5 / 51 / 64.5 / 78 (+ 15 / 27 / 39 / 51).
    - Secondary target AP ratio increased to 40.5% AP from 31.5% AP.

### V14.6
- Stats
  - Mana growth reduced to 40 from 50.
  - Mana regeneration growth increased to $0.8$ from $0.5$.
- Gathering Fire
  - Cooldown reduction changed to 4 seconds from 5.
- Inner Flame
  - Mana cost changed to 40 / 50 / 60 / 70 / 80 from 45 at all ranks.
- Defiance
  - Bonus base shield changed to 50 / 90 / 130 / 170 from 25 / 75 / 125 / 175.
- Mantra
  - Cooldown increased to 40 / 38 / 36 / 34 seconds from 40 / 37 / 34 / 31.

## Trivia

- Karma was named after Karma.
  - Her title, *The Enlightened One*, references Gautama Buddha & Enlightenment in Buddhism.
- Mantra was named after Mantra, from Proto-Indo-Iranian language **mántram* < Proto-Indo-European language **m* "instrument of thought".
  - The 'translation' for Mantra’s Ancient Ionian language was provided by Player Support Escalation Specialist 'Marowe'.
- Karma was the first champion to feature an 'Art Spotlight' before release and the first to be fully relaunched.
  - She was the fifth to have more than four abilities (the others being Elise, Gnar, Heimerdinger, Jayce, Lee Sin, and Nidalee).
- Like Jayce, Karma used to have ranks on each basic ability and only one on her ultimate.
- Renewal’s root has the longest duration in-game ($3.25$ seconds with Focused Resolve and Mantra both fully-ranked).
- In the minigame Astro Teemo, the most expensive upgrade in the game that Teemo’s can purchase, Soul Shield, is a reference to one of Karma's pre-rework skills of the same name.
- In Karma's most current lore, Karma's spirit has lived countless lives only to be re-incarnated within new bodies, inspired by Reincarnation in Indian religions.
  - The name of her current incarnation, *Darha*, comes from Sanskrit adjective < ", steadfast", sharing Proto-Indo-European language root *dʰer-* "to uphold" with Darius.
- Karma was the fourth champion to feature two 'Champion Spotlights' due to significant gameplay changes (the others being Ezreal, Katarina, Lee Sin, and Sivir)

---
*This page was automatically generated from League of Legends Wiki data.*