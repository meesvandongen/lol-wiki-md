# Dr._Mundo

## Table of Contents
- [Basic Information](#basic-information)
- [Abilities](#abilities)
- [Patch History](#patch-history)
- [Trivia](#trivia)

## Basic Information

| Attribute | Value |
|-----------|-------|
| **Champion** | Dr. Mundo |

## Abilities

### Passive: Goes Where He Pleases

**Innate:** **Dr. Mundo** an additional portion of his **maximum** health.

**Passive:** Periodically, **Dr. Mundo** gains immunity to the next hostile immobilize effect to affect him, and upon resisting one, he loses a portion of his **current** health and drops a canister nearby.

**Innate:** **Dr. Mundo** an additional every 5 seconds. 0.4/10 to 0.65/10 for 6–2.3/10 of his every $0.5$ seconds. **Passive:** Periodically, **Dr. Mundo** gains immunity to the next hostile immobilize effect to affect him. Upon resisting one, **Dr. Mundo** pays a health cost equal to 4% of his **current** health and propels a canister that lands 525 units in the general direction of its source, remaining on the ground for 7 seconds. **Dr. Mundo** can move near the canister to consume it, heal himself for 4% of his **maximum** health and reducing the *cooldown* of *Goes Where He Pleases* by 15 seconds. Enemy champion can move near it to destroy it. 'Goes Where He Pleases' cooldown resets upon respawning.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Self |
| **Projectile** | False |

**Notes:**

- Upon being triggered by a hostile immobilize effect, *Goes Where He Pleases* will also grant **Dr. Mundo** immunity to additional immobilizing effects from the same cast instance.
  - Abilities where non-immobilizing effects and damage are nested into immobilizing ones will have them also prevented. Non-immobilizing effects and damage applied separately (the vast majority of cases) are not prevented.
- Spell shield and *Black Shield* take priority over *Goes Where He Pleases*.
- The canister, whether in flight and on the ground, will transition in and out of Realm of Death alongside *Dr. Mundo*.
- The canister cannot be interacted with while untargetable.

---

### Q: Infected Bonesaw

**Active:** **Dr. Mundo** throws an infected bonesaw in the target direction that deals magic damage and slow the first enemy hit.

*If the bonesaw hits an enemy champion or monster, **Dr. Mundo** heals for the entire health cost, reduced to half health cost when hitting anything else.*

**Active:** **Dr. Mundo** throws an infected bonesaw in the target direction that deals magic damage to the first enemy hit and slow them by 40% for 2 seconds. *Infected Bonesaw* has a minimum damage threshold, and is capped against monsters. If the bonesaw hits an enemy, **Dr. Mundo** heals for 50% of the health cost, increased to 100% against champions or monsters.

| Attribute | Value |
|-----------|-------|
| **Range** | 1050 / er 990 units |
| **Cooldown** | 4 seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $50-90$ Health |
| **Targeting** | Direction |
| **Affects** | Self, Enemies |
| **Damage Type** | magic |
| **Speed** | 2000 units/second |
| **Spell Shield** | True |
| **Spell Effects** | spell |
| **Projectile** | True |

**Scaling:**
- **Magic Damage:** $20-30$
- **current** health
- **Minimum Damage:** $80-280$
- **Capped Monster Damage:** $350-650$

**Notes:**

- Spell shield blocking the ability's effects on the enemy does not prevent **Dr. Mundo** from receiving the health cost refund.

---

### W: Heart Zapper

**Active:** **Dr. Mundo** charges up a defibrillator for a few seconds, continually dealing magic damage to nearby enemies and storing a portion of the post-mitigation damage he takes.

*Heart Zapper* can be recast within the duration, and does so automatically after the duration.

**Active:** **Dr. Mundo** charges up a defibrillator for up to 3 seconds, dealing magic damage every $0.25$ seconds to nearby enemies and storing 80 to 95 of post-mitigation damage he takes as grey health on his health bar, reduced to 25% after the first $0.75$ seconds. *Heart Zapper* can be recast after $0.5$ seconds within the duration, and does so automatically after the duration. **Recast:** **Dr. Mundo** detonates the defibrillator, dealing magic damage to nearby enemies and heal for 50% of grey health, increased to 100% if at least one enemy champion or large monster is hit.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $17-15$ seconds |
| **Cast Time** | None / None |
| **Cost** | 8% **current** health |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | magic |
| **Effect Radius** | 325 units |
| **Spell Shield** | Special |
| **Spell Effects** | Special |

**Scaling:**
- **Magic Damage per Tick:* $5-20$16-20×16$
- **Magic Damage:** $20-80$
- **bonus** health)

**Notes:**

- The charge up applies persistent area damage and the detonation deals area damage.
- Spell shield will only block the detonation.
- *Heart Zapper* will occasionally deal an additional tick of damage.
- *Heart Zapper* and its recast will buffer and cast at the end of *Infected Bonesaw*’s cast time if attempted to cast during it.
  - As with all buffering of this type, another input such as a movement command can override the buffering of the spell again.

---

### E: Blunt Force Trauma

**Passive:** **Dr. Mundo** gains **bonus attack damage** based on his **maximum** health.

**Active:** ''Dr. Mundo's** next basic attack deals **bonus** physical damage, increased based on his **missing'' health. If the target dies, their corpse is sent flying away in a line, dealing physical damage to enemies it passes through.

**Passive:** **Dr. Mundo** gains **bonus attack damage**. **Active:** **Dr. Mundo** empowers his next basic attack within 4 seconds to have an uncancellable windup, gain *50 **bonus** range*, and deal **bonus** physical damage, increased by key=%100-100 for 11*bonus** damage. *Blunt Force Trauma* as well as the triggering attack's damage is increased to % against minions and % against monsters. *Blunt Force Trauma basic attack reset *'Dr. Mundo's* basic attack timer.*

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $9-6$ seconds |
| **Cast Time** | none |
| **Cost** | $10-70$ Health |
| **Targeting** | Auto |
| **Damage Type** | physical |
| **Effect Radius** | 155 units |
| **Spell Shield** | False |
| **Spell Effects** | Special |

**Scaling:**
- **Bonus Attack Damage:** $2-2.8$%
- **maximum** health
- **Minimum Bonus Physical Damage:* $ (+ (+ $% **bonus** health)
- **Minimum Minion Physical Damage:** $* (+ $% **bonus* health) to
- ** (+ $% ** bonus
- ** health) ** Minimum Monster Physical Damage:** $* (+ $% **bonus* health) to
- ** (+ $% ** bonus** health)

**Notes:**

- 'Blunt Force Trauma's attack deals basic damage but also triggers spell effects by also being tagged as spell damage, while the target's body being shoved deals area damage to enemies it passes through.
  - The basic attack itself is also considered a part of *Blunt Force Trauma*.
- 'Blunt Force Trauma's attack works against structures, consuming the buff and dealing its full damage.
- *Blunt Force Trauma* cannot knock back wards, structures or epic monsters it kills, nor champions that enter a zombie state upon dying.
- Targets flung away are rendered untargetable in the process.
- The damage of the enemy's corpse colliding with enemies does not benefit from 'Blunt Force Trauma's attack critical strike.

---

### R: Maximum Dosage

**Active:** **Dr. Mundo** injects himself with chemicals, gaining **increased base health** and *ms **bonus** movement speed*, in addition to health regeneration health over the duration.

**Active:** **Dr. Mundo** injects himself with chemicals to become enhanced for 10 seconds, gaining increased *health **base** health*, *ms **bonus** movement speed*, and **bonus health regeneration**. At rank 3, 'Maximum Dosage's increased **base** health and **bonus** health regeneration are both increased by 5% for each enemy champion within 1200 units at the time of cast.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | 120 seconds |
| **Cast Time** | none |
| **Targeting** | Auto |
| **Affects** | Self |

**Scaling:**
- **Increased Base Health:** $15-25$%
- **missing** health
- **Bonus Movement Speed:** $15-35$%
- **Bonus Health Regeneration:** $20/2-60/2$% **maximum* health*maximum
- ** health ** Total Health Regenerated:
- ** $20-60$% ** maximum** health

**Notes:**

- The health regeneration granted by *Maximum Dosage* adjusts dynamically to ''Dr. Mundo's** **maximum'' health.
- The health regeneration granted by *Maximum Dosage* is special cased to be increased by *Axiom Arcanist*.
- *Maximum Dosage* will end prematurely upon entering resurrection.
- Against 5 champions, Rank 3 *Maximum Dosage* will heal 85% **Maximum** health regeneration & 50% **Missing** Bonus health

---

## Patch History

### V25.05
- Stats
  - Base health increased to 640 from 613.
- *Maximum Dosage*
  - **New Effect:** Now is special-cased to gain increased health regeneration from *Axiom Arcanist* (despite it not affecting regeneration, but heal).

### V14.24
- *Blunt Force Trauma*
  - Bonus attack damage reduced to $2-2.8$% **maximum** health from $2-3$%.

### V14.20
- *Blunt Force Trauma*
  - Bonus attack damage reduced to $2-3$% **maximum** health from $2-3.4$%.

### V14.15
- *Goes Where He Pleases*
  - **Bug Fixes:** No longer ignores **Aurora**’s *Between Worlds*’s boundary collision push.

### V13.22
- *Blunt Force Trauma*
  - Bonus attack damage increased to $2-3.4$% **maximum** health from $2-3$%.

### V13.21
- *Blunt Force Trauma*
  - Bonus attack damage reduced to $2-3$% **maximum** health from $2-4$%.
  - Bonus damage amplifier reduced to key=% from key=%.
    - Maximum bonus base damage reduced to $5×1.4-45×1.4$ from $5×1.6-45×1.6$.
    - Maximum bonus damage health ratio reduced to $7×1.4$% **bonus** health from $7×1.6$%.

### V13.16
- *Infected Bonesaw*
  - **Bug Fixes:** The missile's visuals now correspond to its hitbox better.

### V13.6
- Stats
  - Base health regeneration increased to 7 from $6.5$.
  - Health regeneration growth reduced to $0.5$ from $0.55$.
- *Heart Zapper*
  - Recast timer reduced to $0.5$ seconds from 1.
- *Blunt Force Trauma*
  - Damage against monsters increased to 200% from 150%.

### V13.5
- *Maximum Dosage*
  - **Bug Fixes:** Now properly gains the base health instantly upon cast, rather than after a 0-$0.25$-second delay.

### V13.1
- Stats
  - Base health reduced to 613 from 653.
  - Armor growth reduced to $3.7$ from $4.2$.
- *Blunt Force Trauma*
  - Bonus attack damage reduced to $2-4$% **maximum** health from $2.5-4.5$%.
  - **Bug Fixes:** No longer deals more damage to monsters than intended.
- Stats
  - Health regeneration growth reduced to $0.55$ to $0.75$.
- *Goes Where He Pleases*
  - Health regeneration reduced to 0.4 to 0.65 for 6–2.3 **maximum** health from 0.4 to 0.65 for 6–2.5.
  - Health cost increased to 4% **current** health from 3%.
- *Infected Bonesaw*
  - Health cost changed to $50-90$ from 60 at all ranks.
- *Blunt Force Trauma*
  - Health cost changed to $10-70$ from $20-60$.
- *Maximum Dosage*
  - Cooldown increased to 120 seconds from 110.

## Trivia

- "Mundo" has two real world origins: the Proto-Germanic **mundo* "hand, protection" & unrelated Ibero-Romance languages *mundo* "world" (like in Spanish, Portuguese, etc.).
  - He is named after Edmundo 'odnumde' Sanchez, by Brandon 'Ryze' Beck.
- Dr. Mundo's dance references the titular character from House, M.D. in turn dancing to Fight the Power by Public Enemy (band).
  - A side-by-side comparison be seen here.
- In the V1.0.0.115 April Fools' Day patch, the following change regarding Dr. Mundo were jokingly listed：
  - Dr. Mundo decided that he enjoys the color blue.
  - Dr. Mundo now has mana.
  - None of his ability costs have changed.
- *Masochism* references the Masochism for enjoyment derived from being inflicted pain, named after real-life writer Leopold von Sacher-Masoch.
  - This is also mechanically referenced in his kit by gaining a higher AD bonus the lower his health is.
  - The ability's name also fits for the Masochism_old.png of his E ability, which caused Dr. Mundo to be healed instead of being damaged from physical attacks for a duration.
- *Sadism* references the Sadism for enjoyment derived from inflicting pain on others, named after real-life writer Marquis de Sade.
  - The name chosen for this ability made the most sense for the Kiss of Death.png of his R ability, which damaged nearby enemies for their current health.
- This champion has no ability power ratio.
- Dr. Mundo is one of champions who use health as a resource for abilities, the other five being **Briar**, **Olaf**, **Soraka**, **Vladimir** and **Zac**.
- His dance is a Pec Flex which involves flexing his pectoral muscles.
- His death animation shows him assume the morgue pose whilst adding a toe tag to himself.
- *Goes Where He Pleases* is among a few abilities that were named after a quote of their champion. In this case, it references his "Mundo goes where he pleases" quote which he had before the patch he got the ability, V11.12.
  - The others are League of Draven and End of the Line.
- Following his rework, his kit was given a higher emphasis on medical equipment and terms, thus making several references to them.
  - *Infected Bonesaw* was renamed from *Infected Cleaver* to have Dr. Mundo utilize the Bone cutter.
  - *Heart Zapper* is based on the act of defibrillation.
  - *Blunt Force Trauma* references the term for the blunt trauma.

---
*This page was automatically generated from League of Legends Wiki data.*