# Ashe

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
| **Champion** | Ashe |
| **Title** | the Frost Archer |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2009-02-21 |
| **Release Patch** | Alpha Week 2 |
| **Latest Changes** | V25.18 |
| **Roles** | Marksman |
| **Riot Positions** | Bottom, Support |
| **External Positions** | Bottom, Support |
| **Blue Essence** | 225 |
| **Riot Points** | 260 |
| **Difficulty** | 1 |
| **Hero Type** | Marksman |
| **Alt Type** | Support |
| **Adaptive Type** | Physical |
| **Damage** | 2 |
| **Toughness** | 1 |
| **Control** | 3 |
| **Mobility** | 1 |
| **Utility** | 2 |
| **Style** | 20 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $610.0$ | $+101.0$ |
| **Mana** | $280.0$ | $+35.0$ |
| **Health Regen** | $3.5$ | $+0.55$ |
| **Mana Regen** | $7.0$ | $+0.65$ |
| **Armor** | $26.0$ | $+4.6$ |
| **Magic Resist** | $30.0$ | $+1.3$ |
| **Attack Damage** | $59.0$ | $+3.45$ |
| **Attack Speed** | $0.658$ | |
| **Movement Speed** | $325.0$ | $+0.0$ |
| **Attack Range** | $600.0$ | $+0.0$ |
| **Base Attack Speed** | $0.658$ | |
| **Attack Speed Ratio** | $0.658$ | |
| **Bonus AS per Level** | $3.3\%$ | |
| **Missile Speed** | $2500$ units/second | |
| **Acquisition Radius** | $600$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $120$ units | |
| **Selection Height** | $155$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $105.0\%$ |
| **Damage Taken** | $100.0\%$ |

#### Ultimate Spellbook

| Metric | Value |
|--------|------:|
| **Damage Taken** | $95.0\%$ |

## Abilities

### Passive: Frost Shot

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Parry** | True |

**INNATE:** **Ashe**’s basic attacks deal **bonus** physical damage equal to critical strike chance. Critical strikes do not deal any additional damage.

**INNATE - FROST SHOT:** **Ashe**’s basic attacks and ability hits apply *Frost* to enemies for 2 seconds, which slows them by key=% for the duration.

**INNATE - CRITICAL SLOW:** **Ashe**’s critical strikes double *Frost*’s slow strength to key=%, decaying over the first second of the duration to its normal strength.

**Notes:**

- **Ashe**’s critical strikes are still considered critical strike damage and thus will be reduced by Randuin's Omen Resilience.
- Runaan's Hurricane will not deal additional damage on critical strikes.
- Cheap Shot will trigger on a subsequent basic attack even when the target is no longer slowed.

---

### Q: Ranger's Focus

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Cost** | 30 Mana + 4 Focus |
| **Targeting** | Auto |
| **Affects** | Self |
| **Damage Type** | Physical |
| **Spell Shield** | False |
| **Spell Effects** | Special |
| **Projectile** | True |
| **Parry** | True |

**PASSIVE:** While *Ranger's Focus* is inactive, **Ashe**’s basic attacks on-attack generate a stack of *Focus* for 4 seconds, refreshing on subsequent attacks and stacking up to 4 times. Stacks expire by one every second when the duration ends.

**ACTIVE:** For 6 seconds, **Ashe** gains as and empowers her basic attacks to fire a flurry of five arrows that deal ***modified** physical damage. The flurries apply life steal and Frost Shot per arrow, but apply on-hit effects only once.

| Attribute | Value |
|-----------|------:|
| **Bonus Attack Speed** | 25 / 37.5 / 50 / 62.5 / 75% |

| Attribute | Value |
|-----------|------:|
| **Physical Damage Per Arrow** | 22 / 23.5 / 25 / 26.5 / 28% AD |
| **Total Damage Per Flurry** | 110 / 117.5 / 125 / 132.5 / 140% AD |

*Ranger's Focus resets **Ashe**’s basic attack timer.*

**Notes:**

- The flurries deal 1 instance of basic damage followed by 4 instances of a non-reactive type of damage (either default or proc).
  - The flurries are classified as a basic attack on a script level, but are considered an ability for other effects (e.g. Sap Magic).
- The first flurry of a *Ranger's Focus* cast fires「 one additional arrow, dealing 20% increased total damage. ⟷ arrows, dealing a total of 132 / 141 / 150 / 159 / 168% attack damage. 」
- *Ranger's Focus* works against structures.
- Flat damage reductions (e.g. Tantrum, Nimble Fighter, or Eclipse) apply per arrow, resulting in extremely increased effectiveness against *Ranger's Focus*.
  - The sole exception is Warden's Mail Rock Solid, as that applies per cast instance rather than per damage instance.
- A flurry can apply 5 stacks of Black Cleaver Carve to a single target as it deals 5 instances of physical damage.
- *Ranger's Focus* also empowers Runaan's Hurricane Wind's Fury bolts to become a flurry of 5 bolts each as well.
  - The empowered bolts use **Ashe**’s attack projectile, including her missile speed (2500 compared to their default 2000).
  - The bolts are evenly distributed among the targets, but a single target cannot be hit by more than 5 bolts.
  - These flurries also apply on-hit effects only once to each target hit.
- Because *Focus* stacks are generated on-attack, Runaan's Hurricane bolts will not generate any.

---

### W: Volley

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Effect Radius** | 1200 (Not including the offset, see notes) units |
| **Angle** | $27.75$ / $46.25$|Full cone at ranks 1, 2/3 and 4/5, respectively / $4.625$ |
| **Width** | 20 (Each missile) units |
| **Speed** | 2000 units/second |
| **Cost** | 75 / 70 / 65 / 60 / 55 Mana |
| **Cooldown** | 18 / 14.5 / 11 / 7.5 / 4 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | aoe |
| **Projectile** | True |

**ACTIVE:** **Ashe** shoots a volley of arrows in a cone in the target direction, each dealing physical damage to the first enemy hit, and applying *Critical Slow* to enemy champions hit.

| Attribute | Value |
|-----------|------:|
| **Arrows** | 7 / 8 / 9 / 10 / 11 |

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 60 / 95 / 130 / 165 / 200 (+ 110% **bonus** AD) |

Enemies can intercept multiple arrows but do not take damage from any beyond the first.

**Notes:**

- The missiles are spawned in a straight horizontal line 75 units in front of **Ashe**, with a total width of 75/100/100/123/123 units.
  - Between each missile spawn location is a distance of $12.5$ units, except for the two outermost ones at the last two ranks (which are $11.5$ units from the closest other one), and the two centermost missiles at ranks 2 and 4 (which have 20 units between them and 15 units to the next missile, respectively).
  - This inconsistent behaviour of spawn locations means the angle between each missile can vary off of $4.625$ slighty. Effect at cast time end

---

### E: Hawkshot

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | Global |
| **Effect Radius** | 1000 (Vision bubble radiuses) units |
| **Speed** | 1400 units/second |
| **Cost** | 1 Charge |
| **Cooldown** | 5 seconds |
| **Recharge** | 90 / 80 / 70 / 60 / 50 seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Spell Shield** | False |
| **Projectile** | Special |

**ACTIVE:** **Ashe** sends a hawk spirit to a location, granting sight of the area along its path「 repeatedly for $0.5$ seconds after every 100 units traveled ⟷ for effectively up-to $1.8$ seconds at any location 」and at its destination for 5 seconds.

**Ashe** periodically stocks a *Hawkshot* charge, up to a maximum of 2.

**Notes:**

- *Hawkshot*’s recharge timer seems to start at the beginning of the game, even prior to ranking the ability, but upon actually ranking the ability, it starts with one charge and the recharge timer starts from 0.
- *Hawkshot* will ping enemy champions it spots if allies didn't already have vision of them, and put a 'revealed' visual effect on them for $2.75 seconds$ (Estimated), but does not actually reveal the units in particular, and is removed if the champion leaves the area.
- **Ashe** marks enemy champions who were previously unseen but were revealed *Hawkshot* in order to gain assist credit, lasting for the standard credit timer.
- *Hawkshot* will trigger upon colliding with Blade Whirl or Wind Wall but not Unbreakable.
- *Hawkshot* will grant 0.33 points of vision score for each champion revealed.

---

### R: Enchanted Crystal Arrow

| Attribute | Value |
|-----------|------:|
| **Range** | Global |
| **Cast Time** | $0.25$ seconds |
| **Effect Radius** | 400 (Radius of explosion) / sight 350 (Missile sight radius) units |
| **Width** | 260 (Missile width) units |
| **Speed** | 1500 to 2100 by 200 units/second |
| **Cost** | 100 Mana |
| **Cooldown** | 100 / 90 / 80 / 70 / 60 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | aoe |
| **Projectile** | True |

**ACTIVE:** **Ashe** fires a massive arrow of ice in the target direction, granting sight of the area (Cannot grant sight through terrain and can only grant sight into brush when the missile flies through that brush) it flies through each for 1 second. The arrow shatters upon hitting an enemy champion, dealing them magic damage, stunning them for type=distance traveled seconds, and granting sight of the area around them for 1 second.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 200 / 300 / 400 / 500 / 600 (+ 120% AP) |

Enemies surrounding the primary target are dealt the same damage and afflicted with *Frost Shot*.

**Notes:**

- *Enchanted Crystal Arrow*’s projectile has an icon on the mini-map while it is in flight. It can be seen by only **Ashe** and her allies. Effect at cast time start
- The effect will be centered on the first champion struck, not at the location the missile collided with them.

---

## Patch History

### V25.18
- Stats
  - Attack damage growth increased to $3.45$ from $2.95$.

### V25.15
- Ashe
  - English voice-over added.

### V25.11
- Ranger's Focus
  - **New Effect:** Can now generate stacks against structures.

### V25.10
- Hawkshot
  - **Bug Fixes:** Reveal VFX particles no longer disappear if the player's camera has not followed or switched away from the explosion location.

### V25.09
- Ranger's Focus
  - Bonus attack speed increased to 25 / 37.5 / 50 / 62.5 / 75% from 25 / 35 / 45 / 55 / 65%.
  - AD ratio changed to 110 / 117.5 / 125 / 132.5 / 140% AD from 111 / 117 / 123 / 129 / 135% AD.

### V25.05
- Enchanted Crystal Arrow
  - Base damage reduced to 200 / 400 / 600 from 250 / 450 / 650.

### V25.04
- Ranger's Focus
  - AD ratio increased to 111 / 117 / 123 / 129 / 135% AD from 110 / 115 / 120 / 125 / 130% AD.
- Volley
  - AD ratio increased to 110% **bonus** AD from 100%.
- Enchanted Crystal Arrow
  - Base damage increased to 250 / 450 / 650 from 200 / 400 / 600.
  - **Removed:*** No longer deals 50% less damage to secondary targets.
  - **Bug Fixes:** Hit SFX now properly originates from the hit location instead of the caster's.

### V25.S1.1
- Frost Shot
  - **New Effect:** **Ashe**’s basic attacks deal bonus damage equal to (75% + Infinity Edge) critical strike chance. Additionally, her basic attacks and ability hits apply *Frost* to enemies for 2 seconds, which slows them by key=% for the duration.
  - **OLD EFFECT:** **Ashe**’s basic attack and ability hits apply *Frost* to enemies for 2 seconds, which slows them by key=% for the duration. Basic attacks against enemies affected by *Frost* are modified to deal 115% (+ (75% + Infinity Edge) critical strike chance) damage.
- Ranger's Focus
  - Bonus attack speed increased to 25 / 35 / 45 / 55 / 65% from 25 / 32.5 / 40 / 47.5 / 55%.
  - Damage per flurry increased to 110 / 115 / 120 / 125 / 130% AD from 105 / 110 / 115 / 120 / 125% AD.
- Volley
  - Base damage increased to 60 / 95 / 130 / 165 / 200 from 20 / 35 / 50 / 65 / 80.
  - AD ratio changed to 100% **bonus** AD from 100% **total** AD.

### V14.17
- Stats
  - Base health reduced to 610 from 640.

### V14.12
- Frost Shot
  - Bonus damage reduced to 115% from 120%.

## Trivia

- Ashe's dance references J'en Ai Marre! by Alizée.
  - A by comparison can be seen here.
- Ashe shares similarities with from Final Fantasy XII. Their names are similar and both became queens, use ranged weapons (Ashelia uses hand bombs in Final Fantasy XII: Revenant Wings) and were once hunted down to prevent them from leading their respective peoples.
- Ashe is the first (and so far the only) champion to possess two global-ranged abilities: Hawkshot and Enchanted Crystal Arrow.
  - Timewinder and Chronobreak’s ranges can be global, albeit situationally.
- Ashe was named for Marc 'Tryndamere' Merrill's wife Ashley.
- As of December 9, 2017 Ashe has featured on the free champion rotation more than any other champion (76 times).
- In Summoner's Rift, Thornmail was once recommended as a situational item for Ashe, this was most likely a reference to the old Basic Tutorial where Ashe is pre-selected as the player's champion and Thornmail is the only purchasable item.
- In the now-removed official League of Legends forums, the old icon of Focus.png was used to represent the "New Player Forum" section.

---
*This page was automatically generated from League of Legends Wiki data.*