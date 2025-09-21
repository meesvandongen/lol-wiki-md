# Milio

## Table of Contents
- [Basic Information](#basic-information)
- [Statistics](#statistics)
  - [Advanced Stats](#advanced-stats)
- [Abilities](#abilities)
- [Patch History](#patch-history)

## Basic Information

| Attribute | Value |
|-----------|-------|
| **Champion** | Milio |
| **Title** | The Gentle Flame |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2023-03-23 |
| **Release Patch** | V13.6 |
| **Roles** | Enchanter |
| **Riot Positions** | Support |
| **External Positions** | Support |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $560.0$ | $+88.0$ | $2056.0$ |
| **Mana** | $365.0$ | $+43.0$ | $1096.0$ |
| **Health Regen** | $5.0$ | $+0.5$ | $13.5$ |
| **Mana Regen** | $11.5$ | $+0.4$ | $18.3$ |
| **Armor** | $26.0$ | $+4.6$ | $104.2$ |
| **Magic Resist** | $30.0$ | $+1.3$ | $52.1$ |
| **Attack Damage** | $48.0$ | $+3.2$ | $102.4$ |
| **Attack Speed** | $0.625$ | $+3.0\%$ | $0.944$ |
| **Movement Speed** | $330.0$ | $+0.0$ | $330.0$ |
| **Attack Range** | $525.0$ | $+0.0$ | $525.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.625$ |
| **Attack Speed Ratio** | $0.625$ |
| **Bonus AS per Level** | $3.0\%$ |
| **Attack Windup** | $17.1\%$ |
| **Missile Speed** | $1900 units/second$ |
| **Acquisition Radius** | $550 units$ |
| **Gameplay Radius** | $55 units$ |
| **Pathing Radius** | $30 units$ |
| **Selection Radius** | $100 units$ |
| **Selection Height** | $200 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Fired Up!

**Innate:** **Milio**’s abilities enchant allied champions on-contact, causing their next basic attack or ability hit shortly after to apply a burst and then a short burn to the target enemy, both parts dealing magic damage.

**Innate:** ''Milio's' ability hits on himself and allied champions grant an enchantment for 4 seconds, which causes the next basic attack or ability hit against enemies to deal 7@1; 11@6; 15@9 (@=%) of enchanted target's AD **bonus** magic damage and apply a burn that deals

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Allies, Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | Special |
| **Spell Effects** | periodic |

**Notes:**

- Both the burst and the burn respect enchantment redirection.
- Subsequent applications of *Fired Up!* only refresh the duration.
- Spell shield will not block the effect if applied by a basic attack.

---

### Q: Ultra Mega Fire Kick

**Active:** **Milio** kicks a fireball in the target direction that briefly airborne and stun the first enemy hit. The ball then bounces once toward the target and explodes, dealing magic damage and slow enemies hit.

*If the primary target is a non-champion, the ball knocks back further and creates a larger explosion.*

**Active:** After a $0.25$-second delay, **Milio** kicks a fireball in the target direction that grants sight of its path and airborne and stun the first enemy it hits over 1 second. Upon collision, the ball bounces once in the same direction from the target's location, granting sight of the area before exploding in the same radius after a brief delay, dealing magic damage to enemies hit and slow them for $1.5$ seconds. If the primary target is a non-champion, the ball knocks back further and creates a larger explosion. *Ultra Mega Fire Kick* refunds 50% of its *mana cost* if it hits at least one champion with the fireball or explosion. '**Milio** cannot cast other abilities during Ultra Mega Fire Kick's delay.'

| Attribute | Value |
|-----------|-------|
| **Cooldown** | 10 seconds |
| **Cast Time** | none |
| **Cost** | $50-70$ Mana |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Speed** | 1200 units/second |
| **Effect Radius** | 100 / cr 250 / 275 / 190 units |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | True |

**Scaling:**
- **Magic Damage:** $80-320$ (+ 120% AP)
- **Slow:** $40-60$% (+ 5% per 100 AP)

**Notes:**

No additional notes.

---

### W: Cozy Campfire

**Active:** **Milio** summons a fuemigo that lasts for 6 seconds and follows the closest allied champion. Allied champions near the fuemigo gain increased *attack range* and are continually heal.

**Recast:** The fuemigo follows the target allied champion.

**Active:** **Milio** summons a fuemigo at the target location or upon the target allied champion for 6 seconds that follows the nearest allied champion and grants sight of its surroundings. Allied champions near the fuemigo gain **bonus attack range** equal to a percentage of their **base** attack range and heal every over the duration. *Cozy Campfire* can be recast after $0.5$ seconds within the duration. **Recast:** **Milio** commands the fuemigo to follow the target allied champion to within cr 150-units, placing the recast on a cd-second static cooldown. **Milio counts as an allied champion for this ability. Cozy Campfire may grant **Fired Up!* upon being summoned and at most once every 3 seconds thereafter.*

| Attribute | Value |
|-----------|-------|
| **Range** | 650 / 3000 units |
| **Cooldown** | $29-21$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $90-130$ Mana |
| **Targeting** | Location / Unit |
| **Affects** | Self, Allies |
| **Effect Radius** | 415 units |

**Scaling:**
- **Base Attack Range Scaling:** $10-20$%
- **Heal per Tick:** $70/25-150/25$ (+ $15/25$% AP) Total Heal $70-150$ (+ 15% AP)

**Notes:**

- 'Cozy Campfire's initial cast and recast have a forgiveness radius of 175 units for their unit-targeted version.
  - The summoned fuemigo will still follow the nearest allied champion in range, even if the initial cast was targeted on a different ally.
- The *attack range* increase lingers on allies for the entire duration of *Cozy Campfire*, even if they leave the zone.

---

### E: Warm Hugs

**Active:** **Milio** envelops himself or the target ally in protective flames, granting them a shield and **bonus move speed** for a short time.

**Milio** periodically stocks a *Warm Hugs* charge, up to a maximum of 2. The effects can stack up to 2 times.

**Active:** **Milio** envelops himself or the target allied champion in protective flames, granting the target a shield and **bonus movement speed** for $2.5$ seconds. **Milio** periodically stocks a *Warm Hugs* charge, up to a maximum of 2. 'Warm Hugs' effects can stack up to 2 times.'

| Attribute | Value |
|-----------|-------|
| **Range** | 650 units |
| **Recharge** | $17-13$ seconds |
| **Cast Time** | none |
| **Cost** | $50-90$ mana |
| **Targeting** | Unit |
| **Affects** | Self, Allies |

**Scaling:**
- **Shield Strength:** $45-165$ (+ 45% AP)
- **Bonus Movement Speed:** $12-20$%

**Notes:**

- *Warm Hugs* has a forgiveness radius of 175 units.

---

### R: Breath of Life

**Active:** **Milio** explodes in soothing flames, heal and cleanse himself and nearby allied champion of non-airborne crowd control and granting them tenacity for a short time.

**Active:** **Milio** explodes in soothing flames, heal and cleanse himself and nearby allied champions of non-airborne crowd control, and granting them 65% tenacity for 3 seconds. '**Milio** cannot cast his other abilities for $0.75$ seconds after Breath of Life's activation. Breath of Life cannot be used while affected by cast-inhibiting crowd control.'

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $160-130$ seconds |
| **Cast Time** | none |
| **Cost** | 100 Mana |
| **Targeting** | Auto |
| **Affects** | Self, Allies |
| **Effect Radius** | 700 units |

**Scaling:**
- **Heal:** $150-350$ (+ 50% AP)

**Notes:**

- *Breath of Life* affects untargetable units.

---

## Patch History

### V25.18
- *Fired Up!*
  - **Bug Fixes:** No longer instantly kills Cell Division blobs the first time it triggers or refreshes after the blobs spawn.

### V25.13
- *Fired Up!*
  - **Bug Fixes:** No longer is able to stack and trigger *Electrocute* on allies.

### V14.20
- *Cozy Campfire*
  - **New Effect:** Cast & recast now have a forgiveness radius of 175 units.
- *Warm Hugs*
  - **New Effect:** Cast now has a forgiveness radius of 175 units.

### V14.14
- *Fired Up!*
  - AD ratio reduced to 7@1; 11@6; 15@9 (@=%) of enchanted target's AD from 15% of enchanted target's AD at all levels.
  - Base damage reduced to 10 to 50 from 15 to 50.
- *Ultra Mega Fire Kick*
  - **New Effect:** Now refunds 50% of its mana cost if either the missile or the area of effect hits at least one enemy champion.
  - Range increased to 1200 units from 1000.
  - Cooldown reduced to 10 seconds from 12.
- *Cozy Campfire*
  - Recast range increased to 3000 units from 700.
  - **New Effect:** The campfire is now slower when following targets that are more than 2500 units from him.
- *Warm Hugs*
  - Base shield reduced to $45-165$ from $60-180$.
  - AP ratio increased to 45% AP from 30% AP.

### V14.9
- Stats
  - Gameplay radius reduced to 55 units from 65.
  - Pathing radius reduced to 30 units from 35.
  - Selection radius reduced to 100 units from 125.

### V13.21
- *Fired Up!*
  - **New Effect:** Allies now retain kill credit if his damage triggers his *Summon Aery* rune.
- *Breath of Life*
  - **Bug Fixes:** Now properly cleanses berserk.

### V13.20
- *Fired Up!*
  - **New Effect:** Bonus damage is now considered ''Milio's' instead of his allies'.
    - Does not affect kill credit.
- *Warm Hugs*
  - Base shield increased to $60-180$ from $60-160$.
  - Recharge timer reduced to $17-13$ seconds from $18-14$.
- *Breath of Life*
  - AP ratio increased to 50% AP from 30% AP.

### V13.16
- *Ultra Mega Fire Kick*
  - Base damage changed to $80-320$ from $90-270$.
  - Damage AP ratio increased to 120% AP from 90% AP.
- *Warm Hugs*
  - Base shield increased to $60-160$ from $60-140$.
  - AP ratio increased to 30% AP from 25% AP.

### V13.14
- *Fired Up!*
  - Total base damage reduced to 15 to 50 from 25 to 80.
- *Cozy Campfire*
  - **Bug Fixes:** Can no longer trigger *Guardian* from outside of the rune's intended range by "affecting" an ally by targeting the cast on them from far away.
- *Warm Hugs*
  - Bonus movement speed reduced to $12-20$% from $15-25$%.

### V13.12
- Stats
  - Base armor reduced to 26 from 28.
  - Armor growth reduced to $4.6$ from $4.9$.
- *Fired Up!*
  - AD ratio reduced to 15% AD at all levels from 15 to 35 AD.

---
*This page was automatically generated from League of Legends Wiki data.*