# Ashe

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
| **Champion** | Ashe |
| **Title** | the Frost Archer |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2009-02-21 |
| **Release Patch** | Alpha Week 2 |
| **Roles** | Marksman |
| **Riot Positions** | Bottom, Support |
| **External Positions** | Bottom, Support |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $610.0$ | $+101.0$ | $2327.0$ |
| **Mana** | $280.0$ | $+35.0$ | $875.0$ |
| **Health Regen** | $3.5$ | $+0.55$ | $12.9$ |
| **Mana Regen** | $7.0$ | $+0.65$ | $18.1$ |
| **Armor** | $26.0$ | $+4.6$ | $104.2$ |
| **Magic Resist** | $30.0$ | $+1.3$ | $52.1$ |
| **Attack Damage** | $59.0$ | $+3.45$ | $117.7$ |
| **Attack Speed** | $0.658$ | $+3.3\%$ | $1.030$ |
| **Movement Speed** | $325.0$ | $+0.0$ | $325.0$ |
| **Attack Range** | $600.0$ | $+0.0$ | $600.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.658$ |
| **Attack Speed Ratio** | $0.658$ |
| **Bonus AS per Level** | $3.3\%$ |
| **Missile Speed** | $2500 units/second$ |
| **Acquisition Radius** | $600 units$ |
| **Pathing Radius** | $35 units$ |
| **Selection Radius** | $120 units$ |
| **Selection Height** | $155 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Frost Shot

**Innate:** **Ashe**’s basic attacks deal increased damage based on her critical strike chance and critical strike damage.

**Innate - Frost Shot:** **Ashe**’s basic attacks apply a slow on-hit.

**Innate:** ''Ashe's** basic attacks deal **bonus'' physical damage equal to . critical strike do not deal any additional damage. **Innate - Frost Shot:** ''Ashe's* basic attacks and ability hits apply *Frost' to enemies for 2 seconds, which slow them by key=% for the duration. **Innate - Critical Slow:** ''Ashe's* critical strike double *Frost's slow strength to key=%, decaying over the first second of the duration to its normal strength.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |

**Notes:**

- ''Ashe's' critical strikes are still considered critical strike damage and thus will be reduced by *Randuin's Omen* Resilience.
- *Runaan's Hurricane* will not deal additional damage on critical strike.
- *Cheap Shot* will trigger on a subsequent basic attack even when the target is no longer slowed.

---

### Q: Ranger's Focus

**Passive:** **Ashe**’s basic attacks generate stacks of *Focus*, up to a cap. Once the stacks reach the cap, she can cast 'Ranger's Focus'.

**Active:** **Ashe** gains and augments her basic attacks to fire a flurry of arrows.

**Passive:** While 'Ranger's Focus* is inactive, *'Ashe's* basic attacks on-attack generate a stack of *Focus' for 4 seconds, refreshing on subsequent attacks and stacking up to 4 times. Stacks expire by one every second when the duration ends. **Active:** For 6 seconds, **Ashe** gains *as *bonus attack speed* and empowers her basic attacks to fire a flurry of five arrows that deal *modified physical damage. The flurries applies life steal and *Frost Shot* per arrow, but apply on-hit effects only once. 'Ranger's Focus basic attack reset ''Ashe's* basic attack timer.*

| Attribute | Value |
|-----------|-------|
| **Cast Time** | none |
| **Cost** | 30 Mana + 4 Focus |
| **Targeting** | Auto |
| **Affects** | Self |
| **Damage Type** | Physical |
| **Spell Shield** | False |
| **Spell Effects** | Special |
| **Projectile** | True |

**Scaling:**
- **Bonus Attack Speed:** $25-75$%
- **Physical Damage Per Arrow:** $ Total Damage Per Flurry $% AD

**Notes:**

- The flurries deal 1 instance of basic damage followed by 4 instances of a non-reactive type of damage (either default damage or proc damage).
  - The flurries are classified as a basic attack on a script level, but are considered an ability for other effects (e.g. Sap Magic).
- The first flurry of a 'Ranger's Focus' cast fires attack damage.
- 'Ranger's Focus' works against structures.
- Flat damage reductions (e.g. Tantrum, Nimble Fighter, or Eclipse) apply per arrow, resulting in extremely increased effectiveness against 'Ranger's Focus'.
  - The sole exception is *Warden's Mail* Rock Solid, as that applies per cast instance rather than per damage instance.
- A flurry can apply 5 stacks of *Black Cleaver* Carve to a single target as it deals 5 instances of physical damage.
- 'Ranger's Focus' also empowers *Runaan's Hurricane* Wind's Fury bolts to become a flurry of 5 bolts each as well.
  - The empowered bolts use ''Ashe's' attack projectile, including her missile speed (2500 compared to their default 2000).
  - The bolts are evenly distributed among the targets, but a single target cannot be hit by more than 5 bolts.
  - These flurries also apply on-hit effects only once to each target hit.
- Because *Focus* stacks are generated on-attack, *Runaan's Hurricane* bolts will not generate any.

---

### W: Volley

**Active:** **Ashe** fires arrows in a cone dealing physical damage and applying *Critical Slow* to the first enemies hit.

**Active:** **Ashe** shoots a volley of arrows in a cone in the target direction, each dealing physical damage to the first enemy hit, and applying **Critical Slow** to enemy champions hit. Enemies can intercept multiple arrows but do not take damage from any beyond the first.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $18-4$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $75-55$ Mana |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Speed** | 2000 units/second |
| **Effect Radius** | 1200 units |
| **Spell Shield** | True |
| **Spell Effects** | aoe |
| **Projectile** | True |

**Scaling:**
- **Arrows:** $7-11$
- **Physical Damage:** $60-200$ bonus AD)

**Notes:**

- The missiles are spawned in a straight horizontal line 75 units in front of **Ashe**, with a total width of 75/100/100/123/123 units.
  - Between each missile spawn location is a distance of $12.5$ units, except for the two outermost ones at the last two ranks (which are $11.5$ units from the closest other one), and the two centermost missiles at ranks 2 and 4 (which have 20 units between them and 15 units to the next missile, respectively).
  - This inconsistent behaviour of spawn locations means the angle between each missile can vary off of $4.625$ slighty. Effect at cast time end

---

### E: Hawkshot

**Active:** **Ashe** sends a hawk spirit to a location, expose the area along its path and a large area at its destination.

**Ashe** periodically stocks a charge of *Hawkshot*, up to a cap.

**Active:** **Ashe** sends a hawk spirit to a location, granting sight of the area along its pathand at its destination for 5 seconds. **Ashe** periodically stocks a *Hawkshot* charge, up to a maximum of 2.

| Attribute | Value |
|-----------|-------|
| **Range** | Global |
| **Cooldown** | 5 seconds |
| **Recharge** | $90-50$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | 1 Charge |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Speed** | 1400 units/second |
| **Effect Radius** | 1000 units |
| **Spell Shield** | False |
| **Projectile** | Special |

**Notes:**

- 'Hawkshot's recharge timer seems to start at the beginning of the game, even prior to ranking the ability, but upon actually ranking the ability, it starts with one charge and the recharge timer starts from 0.
- *Hawkshot* will ping enemy champions it spots if allies didn't already have vision of them, and put a 'revealed' visual effect on them for $2.75 seconds$, but does not actually reveal the units in particular, and is removed if the champion leaves the area.
- **Ashe** marks enemy champions who were previously unseen but were revealed *Hawkshot* in order to gain assist credit, lasting for the standard credit timer.
- *Hawkshot* will trigger upon colliding with *Blade Whirl* or *Wind Wall* but not *Unbreakable*.
- *Hawkshot* will grant 0.33 points of vision score for each champion revealed.

---

### R: Enchanted Crystal Arrow

**Active:** **Ashe** launches a crystal arrow of ice that stun the first enemy champion hit, dealing magic damage. The arrow accelerates initially and its stun duration increases the farther the arrow has traveled.

*Surrounding enemies take magic damage and are *slowed*.*

**Active:** **Ashe** fires a massive arrow of ice in the target direction, granting sight of the area it flies through each for 1 second. The arrow shatters upon hitting an enemy champion, dealing them magic damage, stun them for type=distance traveled seconds, and granting sight of the area around them for 1 second. Enemies surrounding the primary target are dealt the same damage and afflicted with **Frost Shot**.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $100-60$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | 100 Mana |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Speed** | 1500 to 2100 by 200 units/second |
| **Effect Radius** | 400 / sight 350 units |
| **Spell Shield** | True |
| **Spell Effects** | aoe |
| **Projectile** | True |

**Scaling:**
- **Magic Damage:** $200-600$ (+ 120% AP)

**Notes:**

- 'Enchanted Crystal Arrow's projectile has an icon on the mini-map while it is in flight. It can be seen by only **Ashe** and her allies. Effect at cast time start
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
- *Ranger's Focus*
  - **New Effect:** Can now generate stacks against structures.

### V25.10
- *Hawkshot*
  - **Bug Fixes:** Reveal VFX particles no longer disappear if the player's camera has not followed or switched away from the explosion location.

### V25.09
- *Ranger's Focus*
  - Bonus attack speed increased to $25-75$% from $25-65$%.
  - AD ratio changed to $110-140$% AD from $111-135$% AD.

### V25.05
- *Enchanted Crystal Arrow*
  - Base damage reduced to $200-600 3$ from $250-650 3$.

### V25.04
- *Ranger's Focus*
  - AD ratio increased to $111-135$% AD from $110-130$% AD.
- *Volley*
  - AD ratio increased to 110% *bonus AD from 100%.
- *Enchanted Crystal Arrow*
  - Base damage increased to $250-650 3$ from $200-600 3$.
  - **Removed:*** No longer deals 50% less damage to secondary targets.
  - **Bug Fixes:** Hit SFX now properly originates from the hit location instead of the caster's.
- *Frost Shot*
  - **New Effect:** ''Ashe's basic attacks deal bonus damage equal to (75% + ii) critical strike chanceFrost' to enemies for 2 seconds, which slow them by key=% for the duration.
  - **Old Effect:** ''Ashe's* basic attack and ability hits apply *Frost* to enemies for 2 seconds, which slow them by key=% for the duration. Basic attacks against enemies affected by *Frost' are modified to deal 115% (+ (75% + damage.
- *Ranger's Focus*
  - Bonus attack speed increased to $25-65$% from $25-55$%.
  - Damage per flurry increased to $110-130$% AD from $105-125$% AD.
- *Volley*
  - Base damage increased to $60-200$ from $20-80$.
  - AD ratio changed to 100% *bonus AD from 100% **total** AD.

### V14.17
- Stats
  - Base health reduced to 610 from 640.

### V14.12
- *Frost Shot*
  - Bonus damage reduced to 115% from 120%.

### V14.10
- *Enchanted Crystal Arrow*
  - Updated missile visual effect to more accurately match the hitbox for all skins.
    - Trail VFX has been added to the ground below the arrow.
    - Direction arrow has been moved slightly further.
- Ashe, Ashe, Ashe
  - *Enchanted Crystal Arrow*
    - Added more volume to the arrow trails and glow to better match the hitbox.

## Trivia

- Ashe's dance references J'en Ai Marre! by Alizée.
  - A by comparison can be seen here.
- Ashe shares similarities with from Final Fantasy XII. Their names are similar and both became queens, use ranged weapons (Ashelia uses hand bombs in Final Fantasy XII: Revenant Wings) and were once hunted down to prevent them from leading their respective peoples.
- Ashe is the first (and so far the only) champion to possess two global-ranged abilities: *Hawkshot* and *Enchanted Crystal Arrow*.
  - *Timewinder* and *Chronobreak*’s ranges can be global, albeit situationally.
- Ashe was named for Marc 'Tryndamere' Merrill's wife Ashley.
- As of December 9, 2017 Ashe has featured on the free champion rotation more than any other champion (76 times).
- In Summoner's Rift, *Thornmail* was once recommended as a situational item for **Ashe**, this was most likely a reference to the old Basic Tutorial where **Ashe** is pre-selected as the player's champion and *Thornmail* is the only purchasable item.
- In the now-removed official League of Legends forums, the old icon of Focus.png was used to represent the "New Player Forum" section.

---
*This page was automatically generated from League of Legends Wiki data.*