# Dr._Mundo

## Table of Contents
- [Basic Information](#basic-information)
- [Abilities](#abilities)
- [Patch History](#patch-history)
- [Trivia](#trivia)

## Basic Information

| Attribute | Value |
|-----------|------:|
| **Champion** | Dr. Mundo |

## Abilities

### Passive: Goes Where He Pleases

| Attribute | Value |
|-----------|------:|
| **Static Cooldown** | 60@1; 52.5@3; 45@6; 37.5@8; 30@11; 22.5@13; 15@16 |
| **Targeting** | Passive |
| **Affects** | Self |
| **Projectile** | False |

**INNATE:** **Dr. Mundo** health regeneration an additional「 0.4 to 0.65 for 6 / 0.75 to 1.25 for 6 / 1.4 / 1.55 / 1.7 / 1.9 / 2.1 / 2.3 of his **maximum** health every 5 seconds. ⟷ 0.4/10 to 0.65/10 for 6 / 0.75/10 to 1.25/10 for 6 / 1.4/10 / 1.55/10 / 1.7/10 / 1.9/10 / 2.1/10 / 2.3/10 of his **maximum** health every $0.5$ seconds. 」

**PASSIVE:** Periodically, **Dr. Mundo** gains immunity to the next hostile immobilizing effect to affect him. Upon resisting one, **Dr. Mundo** pays a health cost equal to 4% of his **current** health and propels a canister that lands 525 units in the general direction of its source, remaining on the ground for 7 seconds.

**Dr. Mundo** can move near the canister to consume it, healing himself for 4% of his **maximum** health and reducing the cooldown of *Goes Where He Pleases* by 15 seconds. Enemy champions can move near it to destroy it.

*Goes Where He Pleases' * cooldown resets upon respawning.

**Notes:**

- Upon being triggered by a hostile immobilizing effect, *Goes Where He Pleases* will also grant **Dr. Mundo** immunity to additional immobilizing effects from the same cast instance.
  - Abilities where non-immobilizing effects and damage are nested into immobilizing ones will have them also prevented. Non-immobilizing effects and damage applied separately (the vast majority of cases) are not prevented.
- Spell shield and Black Shield take priority over *Goes Where He Pleases*.
- The canister, whether in flight and on the ground, will transition in and out of Realm of Death alongside *Dr. Mundo*.
- The canister cannot be interacted with while untargetable.

---

### Q: Infected Bonesaw

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 1050 / er 990 units |
| **Width** | 120 units |
| **Speed** | 2000 units/second |
| **Cost** | 50 / 60 / 70 / 80 / 90 Health |
| **Cooldown** | 4 seconds |
| **Targeting** | Direction |
| **Affects** | Self, Enemies |
| **Damage Type** | magic |
| **Spell Shield** | True |
| **Spell Effects** | spell |
| **Projectile** | True |

**ACTIVE:** **Dr. Mundo** throws an infected bonesaw in the target direction that deals magic damage to the first enemy hit and slows them by 40% for 2 seconds.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 20 / 22.5 / 25 / 27.5 / 30% of target's **current** health |

*Infected Bonesaw* has a minimum damage threshold, and is capped against monsters.

| Attribute | Value |
|-----------|------:|
| **Minimum Damage** | 80 / 130 / 180 / 230 / 280 |

| Attribute | Value |
|-----------|------:|
| **Capped Monster Damage** | 350 / 425 / 500 / 575 / 650 |

If the bonesaw hits an enemy, **Dr. Mundo** heals for 50% of the health cost, increased to 100% against champions or monsters.

**Notes:**

- Spell shield blocking the ability's effects on the enemy does not prevent **Dr. Mundo** from receiving the health cost refund.

---

### W: Heart Zapper

| Attribute | Value |
|-----------|------:|
| **Cast Time** | None (Initial cast) / None (Recast) |
| **Effect Radius** | 325 units |
| **Cost** | 8% **current** health |
| **Cooldown** | 17 / 16.5 / 16 / 15.5 / 15 seconds |
| **Cooldown Start** | post-effect |
| **Queue Time** | $0.275$ (Initial cast) / $0.275$ (Recast) seconds |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | magic |
| **Spell Shield** | Special |
| **Spell Effects** | Special |

**ACTIVE:** **Dr. Mundo** charges up a defibrillator for up to 3 seconds, dealing magic damage every $0.25$ seconds to nearby enemies and storing 80 to 95 of post-mitigation damage (Damage calculated after modifiers) he takes as grey health on his health bar, reduced to 25% after the first $0.75$ seconds.

| Attribute | Value |
|-----------|------:|
| **Magic Damage per Tick** | 5 / 8.75 / 12.5 / 16.25 / 20 |
| **Total Magic Damage** | 80 / 140 / 200 / 260 / 320 |

*Heart Zapper* can be recast after $0.5$ seconds within the duration, and does so automatically after the duration.

**RECAST:** **Dr. Mundo** detonates the defibrillator, dealing magic damage to nearby enemies and healing for 50% of grey health, increased to 100% if at least one enemy champion or large monster is hit.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 20 / 35 / 50 / 65 / 80 (+ 7% **bonus** health) |

**Notes:**

- The charge up applies persistent area damage and the detonation deals area damage.
- Spell shield will only block the detonation.
- *Heart Zapper* will occasionally deal an additional tick of damage.
- *Heart Zapper* and its recast will buffer and cast at the end of Infected Bonesaw’s cast time if attempted to cast during it.
  - As with all buffering of this type, another input such as a movement command can override the buffering of the spell again.

---

### E: Blunt Force Trauma

| Attribute | Value |
|-----------|------:|
| **Range** | 650 (Corpse push distance) units |
| **Cast Time** | none |
| **Effect Radius** | 155 (Corpse size) units |
| **Cost** | 10 / 25 / 40 / 55 / 70 Health |
| **Cooldown** | 9 / 8.25 / 7.5 / 6.75 / 6 seconds |
| **Cooldown Start** | post-effect |
| **Targeting** | Auto |
| **Damage Type** | physical |
| **Spell Shield** | False |
| **Spell Effects** | Special |
| **Parry** | True |

**PASSIVE:** **Dr. Mundo** gains **bonus** attack damage.

| Attribute | Value |
|-----------|------:|
| **Bonus Attack Damage** | 2 / 2.2 / 2.4 / 2.6 / 2.8% **maximum** health |

**ACTIVE:** **Dr. Mundo** empowers his next basic attack within 4 seconds to have an uncancellable windup, gain 50 **bonus** range, and deal **bonus** physical damage, increased by key=%. If the target dies or is a small monster, they are sent flying away in a line, though not through terrain, causing all enemies they pass through to take 100% AD physical damage plus *Blunt Force Trauma*’s minimum **bonus** damage.

| Attribute | Value |
|-----------|------:|
| **Minimum Bonus Physical Damage** | 5 / 15 / 25 / 35 / 45 (+ 7% **bonus** health) |
| **Maximum Minion Physical Damage** | 7 / 21 / 35 / 49 / 63 (+ 9.8% **bonus** health) |

*Blunt Force Trauma* as well as the triggering attack's damage is increased to 140% against minions and 200% against monsters.

| Attribute | Value |
|-----------|------:|
| **Minimum Minion Physical Damage** | 7 / 21 / 35 / 49 / 63 (+ 9.8% **bonus** health) |
| **Maximum Minion Physical Damage** | 9.8 / 29.4 / 49 / 68.6 / 88.2 (+ 13.72% **bonus** health) |

| Attribute | Value |
|-----------|------:|
| **Minimum Monster Physical Damage** | 10 / 30 / 50 / 70 / 90 (+ 14% **bonus** health) |
| **Maximum Monster Physical Damage** | 14 / 42 / 70 / 98 / 126 (+ 19.6% **bonus** health) |

*Blunt Force Trauma resets **Dr. Mundo**’s basic attack timer.*

**Notes:**

- *Blunt Force Trauma*’s attack deals basic damage but also triggers spell effects by also being tagged as spell damage, while the target's body being shoved deals area damage to enemies it passes through.
  - The basic attack itself is also considered a part of *Blunt Force Trauma*.
- *Blunt Force Trauma*’s attack works against structures, consuming the buff and dealing its full damage.
- *Blunt Force Trauma* cannot knock back wards, structures or epic monsters it kills, nor champions that enter a zombie state upon dying.
- Targets flung away are rendered untargetable in the process.
- The damage of the enemy's corpse colliding with enemies does not benefit from *Blunt Force Trauma*’s attack critically striking.

---

### R: Maximum Dosage

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Cooldown** | 120 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Auto |
| **Affects** | Self |

**ACTIVE:** **Dr. Mundo** injects himself with chemicals to become enhanced for 10 seconds, gaining increased health, ms, and **bonus** health regeneration.

| Attribute | Value |
|-----------|------:|
| **Increased Base Health** | 15 / 17.5 / 20 / 22.5 / 25% **missing** health |

| Attribute | Value |
|-----------|------:|
| **Bonus Movement Speed** | 15 / 20 / 25 / 30 / 35% |

| Attribute | Value |
|-----------|------:|
| **Bonus Health Regeneration** | 10 / 15 / 20 / 25 / 30% **maximum** health |
| **Health Regenerated per $0.5$ Seconds** | 1 / 1.5 / 2 / 2.5 / 3% **maximum** health |

| Attribute | Value |
|-----------|------:|
| **Total Health Regenerated** | 20 / 30 / 40 / 50 / 60% **maximum** health |

At rank 3, *Maximum Dosage*’s increased **base** health and **bonus** health regeneration are both increased by 5% for each enemy champion within 1200 units at the time of cast.

**Notes:**

- The health regeneration granted by *Maximum Dosage* adjusts dynamically to **Dr. Mundo**’s **maximum** health.
- The health regeneration granted by *Maximum Dosage* is special cased to be increased by Axiom Arcanist.
- *Maximum Dosage* will end prematurely upon entering resurrection.
- Against 5 champions, Rank 3 *Maximum Dosage* will heal 85% **Maximum** health regeneration & 50% **Missing** Bonus health

---

## Patch History

### V25.05
- Stats
  - Base health increased to 640 from 613.

### V25.S1.2
- Maximum Dosage
  - **New Effect:** Now is special-cased to gain increased health regeneration from Axiom Arcanist (despite it not affecting regeneration, but healing).

### V14.24
- Blunt Force Trauma
  - Bonus attack damage reduced to 2 / 2.2 / 2.4 / 2.6 / 2.8% **maximum** health from 2 / 2.25 / 2.5 / 2.75 / 3%.

### V14.20
- Blunt Force Trauma
  - Bonus attack damage reduced to 2 / 2.25 / 2.5 / 2.75 / 3% **maximum** health from 2 / 2.35 / 2.7 / 3.05 / 3.4%.

### V14.15
- Goes Where He Pleases
  - **Bug Fixes:** No longer ignores Aurora’s Between Worlds’s boundary collision push.

### V13.22
- Blunt Force Trauma
  - Bonus attack damage increased to 2 / 2.35 / 2.7 / 3.05 / 3.4% **maximum** health from 2 / 2.25 / 2.5 / 2.75 / 3%.

### V13.21
- Blunt Force Trauma
  - Bonus attack damage reduced to 2 / 2.25 / 2.5 / 2.75 / 3% **maximum** health from 2 / 2.5 / 3 / 3.5 / 4%.
  - Bonus damage amplifier reduced to key=% from key=%.
    - Maximum bonus base damage reduced to 7 / 21 / 35 / 49 / 63 from 8 / 24 / 40 / 56 / 72.
    - Maximum bonus damage health ratio reduced to 9.8% **bonus** health from 11.2%.

### V13.16
- Infected Bonesaw
  - **Bug Fixes:** The missile's visuals now correspond to its hitbox better.

### V13.6
- Stats
  - Base health regeneration increased to 7 from $6.5$.
  - Health regeneration growth reduced to $0.5$ from $0.55$.
- Heart Zapper
  - Recast timer reduced to $0.5$ seconds from 1.
- Blunt Force Trauma
  - Damage against monsters increased to 200% from 150%.

### V13.5
- Maximum Dosage
  - **Bug Fixes:** Now properly gains the base health instantly upon cast, rather than after a 0-$0.25$-second delay.

## Trivia

- "Mundo" has two real world origins: the Proto-Germanic **mundo* "hand, protection" & unrelated Ibero-Romance languages *mundo* "world" (like in Spanish, Portuguese, etc.).
  - He is named after Edmundo 'odnumde' Sanchez, by Brandon 'Ryze' Beck.
- Dr. Mundo's dance references the titular character from House, M.D. in turn dancing to Fight the Power by Public Enemy (band).
  - A side-by-side comparison be seen here.
- In the V1.0.0.115 April Fools' Day patch, the following change regarding Dr. Mundo were jokingly listed：
  - Dr. Mundo decided that he enjoys the color blue.
  - Dr. Mundo now has mana.
  - None of his ability costs have changed.
- Masochism references the Masochism for enjoyment derived from being inflicted pain, named after real-life writer Leopold von Sacher-Masoch.
  - This is also mechanically referenced in his kit by gaining a higher AD bonus the lower his health is.
  - The ability's name also fits for the Masochism_old.png of his E ability, which caused Dr. Mundo to be healed instead of being damaged from physical attacks for a duration.
- Sadism references the Sadism for enjoyment derived from inflicting pain on others, named after real-life writer Marquis de Sade.
  - The name chosen for this ability made the most sense for the Kiss of Death.png of his R ability, which damaged nearby enemies for their current health.
- This champion has no ability power ratio.
- Dr. Mundo is one of champions who use health as a resource for abilities, the other five being Briar, Olaf, Soraka, Vladimir and Zac.
- His dance is a Pec Flex which involves flexing his pectoral muscles.
- His death animation shows him assume the morgue pose whilst adding a toe tag to himself.
- Goes Where He Pleases is among a few abilities that were named after a quote of their champion. In this case, it references his "Mundo goes where he pleases" quote which he had before the patch he got the ability, V11.12.
  - The others are League of Draven and End of the Line.
- Following his rework, his kit was given a higher emphasis on medical equipment and terms, thus making several references to them.
  - Infected Bonesaw was renamed from Infected Cleaver to have Dr. Mundo utilize the Bone cutter.
  - Heart Zapper is based on the act of defibrillation.
  - Blunt Force Trauma references the term for the blunt trauma.

---
*This page was automatically generated from League of Legends Wiki data.*