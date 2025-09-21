# Briar

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
| **Champion** | Briar |
| **Title** | the Restrained Hunger |
| **Resource** | Frenzy |
| **Range Type** | Melee |
| **Release Date** | 2023-09-14 |
| **Release Patch** | V13.18 |
| **Roles** | Diver |
| **Riot Positions** | Jungle |
| **External Positions** | Jungle |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $625.0$ | $+95.0$ | $2240.0$ |
| **Mana** | $0.0$ | $+0.0$ | $0.0$ |
| **Armor** | $30.0$ | $+-1.0$ | $13.0$ |
| **Magic Resist** | $32.0$ | $+2.05$ | $66.8$ |
| **Attack Damage** | $60.0$ | $+2.5$ | $102.5$ |
| **Attack Speed** | $0.644$ | $+2.0\%$ | $0.863$ |
| **Movement Speed** | $340.0$ | $+0.0$ | $340.0$ |
| **Attack Range** | $125.0$ | $+0.0$ | $125.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.644$ |
| **Attack Speed Ratio** | $0.669$ |
| **Bonus AS per Level** | $2.0\%$ |
| **Attack Windup** | $19.3\%$ |
| **Acquisition Radius** | $400 units$ |
| **Pathing Radius** | $35 units$ |
| **Selection Radius** | $120 units$ |
| **Selection Height** | $191.667 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Crimson Curse

**Innate:** **Briar** has no *health regeneration **base** health regeneration* but she gains increased healing from other sources based on her **missing** health.

**Innate:** ''Briar's** basic attacks and abilities inflict a bleed on the target for a few seconds, which may stack up to times. For each stack, the target is dealt physical damage and **Briar'' heal for a portion of it.

**Innate:** ''Briar's* basic attacks and abilities inflict a bleed against enemies for seconds, refreshing on subsequent applications, stack up to times and dealing $% damage for stacks beyond the first. The bleed dealsbonus AD) for each subsequent stack and up to a maximum of bonus AD).bonus AD) physical damage every $ seconds, increased by /) to /(/)bonus AD) for each subsequent stack and up to a maximum of /) to /(/)bonus AD) per tick.**Briar'' heal herself equal to 25% of the pre-mitigation damage dealt. If a target dies while bleeding, she will heal herself equal to 125% of the remaining bleed damage. **Briar** has no *health regeneration **base** health regeneration*, but she increases healing from all sources by key=% (+ .

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Enemies, Self |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | default |

**Notes:**

- *Blood Frenzy*’s area of effect damage around the target does not apply bleed stacks.

---

### Q: Head Rush

**Active:** **Briar** dash to the target unit. If the target is an enemy, she deals magic damage, stun them briefly, and additionally reduces their armor penetration and magic penetration for a few seconds.

**Active:** **Briar** dash to the target unit. If the target is an enemy, she deals physical damage, applies on-hit effects, triggers on-attack effects, stun them for $0.85$ seconds, and reduces their armor penetration and magic penetration for 5 seconds. *Head Rush basic attack reset *'Briar's* basic attack timer.*

| Attribute | Value |
|-----------|-------|
| **Range** | 475 units |
| **Cooldown** | $13-9$ seconds |
| **Cast Time** | none |
| **Cost** | 6% **current** Health |
| **Targeting** | Unit |
| **Affects** | Enemies, Self |
| **Damage Type** | Physical |
| **Speed** | distance |
| **Spell Shield** | True |
| **Spell Effects** | spell |

**Scaling:**
- **Physical Damage:** $60-160$
- *bonus AD) (+ 60% AP)
- **Resistances Reduction:** $10-20$%

**Notes:**

- *Head Rush* can be cast on jungle plants, wards, and stealthed trap.
  - It cannot be cast on structures.
- 'Head Rush's damage benefits from .
- *Head Rush* is a non-following dash.
  - It picks its dash destination ~100 units in front of enemy targets, 75 units in front of enemy targets when already very close to them, 75 units in front of allied targets.
  - If already within 75 units of her target, she dashes to her current location, which takes 0 time but still triggers dash effects such as *Sudden Impact*.
  - It does not force the target's resistance values to update immediately outside of the natural stat update cycle, which means it will typically still be mitigated by the unreduced magic resistance value.

---

### W: Blood Frenzy

**Active:** **Briar** dash and senses her surroundings. If there is a nearby enemy upon her arrival, she gains *Blood Frenzy* for a few seconds, during which she can cast **Snack Attack**.

**Blood Frenzy:** **Briar** breaks free from her pillory, turning berserk against the nearest enemy, prioritizing champions, and gaining ghosting, **bonus attack speed** and **bonus movement speed**. During this time, she also empowers her basic attacks to deal physical damage to enemies around her target.

**Active:** **Briar** dash in the target direction. If there is a nearby enemy upon her arrival, she gains *Blood Frenzy* for 5 seconds, during which she can cast **Snack Attack**. **Blood Frenzy:** **Briar** breaks free from her pillory, causing her to become forced to basic attack the nearest enemy. She standard sight the target and gains ghosting, **bonus attack speed** and **bonus movement speed**, as well as empowering her basic attacks to have an uncancelable windup and deal physical damage to enemies surrounding her target. This damage is affected by critical strike modifiers. **Briar** prioritizes attacking champions, then large monsters or minions, and then any other unit. If she casts **Head Rush** on a non-champion, she will shift her target priority to large monsters or minions, then champions, and then any other unit. *Blood Frenzy causes **Briar** to become unable to receive movement and attack commands. The frenzy ends early if there are no longer any nearby valid targets or *Chilling Scream* is cast. Blood Frenzy basic attack reset *'Briar's* basic attack timer. *Head Rush* and *Certain Death* can be cast during the dash.*

| Attribute | Value |
|-----------|-------|
| **Range** | 75 - 300 / 650 units |
| **Cooldown** | $14-10$ seconds |
| **Cast Time** | none |
| **Cost** | 6% **current** Health |
| **Targeting** | Location |
| **Affects** | Enemies, Self |
| **Damage Type** | Physical |
| **Speed** | 1200 units/second |
| **Effect Radius** | 1000 / 275 units |
| **Spell Effects** | default |

**Scaling:**
- **Bonus Attack Speed:** $55-95$%
- **Bonus Movement Speed:** $24-60$%
- **Physical Damage:** $60-100$

**Notes:**

- *Blood Frenzy* acquires targets regardless of if they are visible or not.
  - This does not apply to stealth targets.
- If the main attack critically strike, the cleave damage will do so as well.
- **Briar** uses Frenzy as a resource to indicate the remaining duration of the frenzy, in seconds.
- The following table refers for interactions while **Briar** is in her frenzy:
  - While in the frenzy, **Briar** cannot control her movement nor declare who she attacks. She will automatically acquire a nearby enemy as her attack target based on a priority system, becoming *forced* to basic attack the target and consequently move into her attack range of them to do so. *** Disarming crowd control as well as any other lockout that would disable basic attacking will disable the forced attacks. *** Forced action crowd control will cause her forced attacks to be either overridden or disabled, depending on the actions being forced by the debuff.

---

### W: Snack Attack

**Active:** **Briar** empowers her next attack during *Blood Frenzy* to take a bite out of the target enemy. The attack gains increased *range*, deals **bonus** physical damage based on the target's **missing** health and heal **Briar** for a portion of the damage dealt.

**Active:** **Briar** empowers her next basic attack within 5 seconds during **Blood Frenzy** to take a bite out of the target enemy, gaining range*bonus** range*, dealing **bonus** physical damage and heal her for 5% of her **maximum** health plus a percentage of the post-mitigation damage dealt. *Snack Attack* deals 110% damage against minions and monsters, with the damage based on the target's health ratio being capped at 400. *Snack Attack basic attack reset *'Briar's* basic attack timer.*

| Attribute | Value |
|-----------|-------|
| **Cast Time** | none |
| **Targeting** | Auto |
| **Affects** | Enemies, Self |
| **Damage Type** | Physical |
| **Spell Shield** | False |
| **Spell Effects** | proc |

**Scaling:**
- **Bonus Physical Damage:** $5-65$ (+ 5% AD) (+ 9%
- **Heal Percentage:** $24-40$%
- **Non-Champion Bonus Damage:** $5×1.1-65×1.1$ (+ $5×1.1$% AD) (+ $9×1.1$%

**Notes:**

- *Snack Attack* can only be used during **Blood Frenzy**; if the frenzy ends at any point, the empowered attack will be lost immediately.
- *Snack Attack* can be cast while **Briar** is charm or taunt.

---

### E: Chilling Scream

**Active:** **Briar** channel to increase 'Chilling Scream's* damage and range, gaining damage reduction and heal herself. *Chilling Scream' can be recast within the duration, and does so automatically afterwards. The charge cannot be interrupt.

**Recast:** **Briar** unleashes a scream in the target direction that deals magic damage to enemies hit and briefly slow them.

**Active:** **Briar** prepares to unleash a scream in the target direction, channel for up to 1 second, during which she increases 'Chilling Scream's damage and range, and gains 35% damage reduction and heal herself every $0.25$ seconds. *Chilling Scream* can be recast within the duration, and does so automatically afterwards. 'Chilling Scream's charge cannot be interrupt by crowd control. **Recast:** **Briar** unleashes the scream in the direction she targeted at the time of cast, dealing magic damage to enemies hit and slow them by 80% for $0.5$ seconds. If *Chilling Scream* was charged for its full duration, enemies hit are also airborne 575 units. If they collide with terrain, they will rebound to take **bonus** magic damage and become airborne for $0.5$ seconds and stun for $1.5$ seconds.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | 16 seconds |
| **Cast Time** | None / $0.15$ |
| **Cost** | 6% **current** Health |
| **Targeting** | Direction / Auto |
| **Affects** | Enemies, Self |
| **Damage Type** | Magic |
| **Speed** | 1900 / 1800 units/second |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | True |

**Scaling:**
- **Heal Per Tick:** $10/4-16/4$% **maximum* health*maximum** health
- **Maximum Magic Damage:** $80-220$ (+ 100% bonus AD) (+ 100% AP)0.025-220×0.025$ (+ $100×0.025$%
- *bonus AD) (+ $100×0.025$% AP)

**Notes:**

- *Chilling Scream* increases its damage by its minimum every $0.025$ seconds over the duration. Effect at cast time end
- The following table refers for interactions while **Briar** is channel:

---

### R: Certain Death

**Active:** **Briar** kicks her pillory's hemolith a large distance toward the target direction which marks the first enemy champion hit and true sight them.

*If a target is hit, **Briar** destroys her pillory and dash to them. Upon arrival, she creates an explosion that deals magic damage to the marked champion and enemies around them, and also flee all non-marked enemies. **Briar** then enters *Hematomania*.*

**Active:** **Briar** kicks her pillory's hemolith in the target direction, briefly granting sight of its surroundings as it travels and marking the first enemy champion hit as her prey. The mark's application disrupt the target's ongoing channel. While the target is marked, they are true sight. If a target is hit, **Briar** cleanse herself from all crowd control and becomes cc-immune over a cast time, afterwards dash to them with displacement immunity. Upon arrival, she creates an explosion around the marked target that deals magic damage to them and nearby enemies and flee all non-marked targets for $1.5$ seconds, during which they are slow by 35%. She then enters a state of *Hematomania*. **Hematomania:** **Briar** gains all effects of **Blood Frenzy** as well as **bonus armor** and **bonus magic resistance** equal to 20% AD, , and *ms **additional bonus** movement speed*. While in the empowered frenzy, **Briar** prioritizes attacking the marked target over all other units and regardless of range. If that target becomes invalid, she will shift back to her normal targeting priority until the marked target can be attacked again. *Hematomania* lasts until the mark is dispelled by any means, including ''Briar's' or the target's death. 'Casting *Chilling Scream* removes the target's mark and ends Hematomania early. *Head Rush*’s cast does not shift her targeting priority during Hematomania.'

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $120-80$ seconds |
| **Cast Time** | 1 / $1.25$ seconds |
| **Cost** | 6% **current** Health |
| **Targeting** | Direction |
| **Affects** | Enemies, Self |
| **Damage Type** | Magic |
| **Speed** | 2000 / 2500 – 5000 units/second |
| **Effect Radius** | Global / cr 1500 / er 575 |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | True |

**Scaling:**
- **Magic Damage:** $150-350$ (+ 130% AP)
- **Life Steal:** $10-20$%
- **Additional Bonus Movement Speed:** $10-30$%

**Notes:**

- **Briar** will track the target with the dash if they change locations.
  - She will not stop tracking the target until she reaches them, as there is no maximum tracking distance.
- **Briar** lands 100 units in front of her target at the end of the dash.
- **Briar** will not initiate the second cast if she is being suppression, is under resurrection, or is in the *Realm of Death* when her target is hit by the missile in the normal realm.
  - Since *Certain Death* cleanses herself from all crowd control when the first cast hits, she will be able to start the second cast due to removing the suppression effect.
- **Briar** will still dash to the target even if they die before she reaches them.
- **Briar** will transition from having cc-immune during the cast time of the dash, to having displacement immunity during the dash.
- **Briar** will place herself onto the ground and interrupt airborne affecting her upon starting the second cast time.
- 'Hematomania's* targeting priorities differ slightly from that of **Blood Frenzy*’s':
  - She will prioritize the marked target regardless of range and over all other units as long as it can be attacked. *** If the marked target is not available, then she shifts her priority to the nearest other champion, then large monster or minion, and then any other unit. **** If neither the marked target is valid or any other valid targets are close nearby, then she will prioritize targeting the nearest other champion, then large monster or minion, and then any other unit within a global radius.
- Starting the second cast interrupts any spells that **Briar** is channel.
- The mark will expire if 'Hematomania's buff is dispelled.
- Casting **Blood Frenzy** during *Hematomania* will not grant any additional bonuses nor will it override 'Hematomania's effects.
  - Gaining *Hematomania* while *Blood Frenzy* is active will override the previous buff.
- The disrupt is 'wrapped' into a status effect that says the target is Silence for $0.3$ seconds, but it does not actually *silence*. It however makes sure that the *disrupt* is prevented by Cc-immune. Effect at cast time start
- The following table refers for interactions while **Briar** is in either the first or second cast time:
- The following table refers for interactions while **Briar** is dashing:
- The following table refers for interactions while **Briar** is in *Hematomania*:

---

## Patch History

### V25.18
- *Head Rush*
  - Damage type changed to physical from magic.
  - Base damage reduced to $60-160$ from $60-180$.
- *Blood Frenzy*
  - **Bug Fixes:** Manual attack orders issued on a target during the Frenzy can now properly persist upon exiting the state and are no longer automatically dropped.
- *Certain Death*
  - Global audio warning now plays after the cast time of the missile's cast instead of at its start.
  - **Removed:*** Explosion damage no longer scales off of 50% *bonus AD.
  - Explosion damage AP ratio increased to 130% AP from 120% AP.
  - Missile range increased to 12000 units from 10000.

### V25.17
- *Blood Frenzy*
  - **Bug Fixes:** No longer sometimes has a brief delay before aggroing a valid target.

### V25.14
- *Blood Frenzy*
  - **Bug Fixes:** Casting *Head Rush* on an enemy champion who is standing near an allied minion no longer unintentionally causes her to prioritize enemy minions.
- *Certain Death*
  - Disable immunity during the cast time for the dash changed to total cc-immune from displacement immunity.
  - **Bug Fixes:** **Briar** no longer becomes unable to cast any spell after stasis expires if she was affected by it prior to 'Certain Death's dagger landing.
- Stats
  - Attack damage growth reduced to $2.5$ from 3.
- *Crimson Curse*
  - Maximum number of stacks reduced to 7 from 9.

### V25.13
- *Blood Frenzy*
  - **New Effect:** All attacks during her *Blood Frenzy* now have an uncancelable windup.
- *Chilling Scream*
  - Recast time changed to $0.15$ seconds from 100% of her attack windup.

### V25.11
- *Chilling Scream*
  - **Bug Fixes:** Issuing an attack-move command during the charge no longer incorrectly causes the ability to recast.

### V25.09
- General
  - **Bug Fixes:** If Flee during her frenzy state (*Blood Frenzy* / *Certain Death*), no longer becomes unable to move and attack acquired targets, standing still until exiting the state instead.

### V25.05
- *Certain Death*
  - **Bug Fixes:** Cast time to begin the travel is now properly canceled upon her False Life triggering.

### V25.04
- *Certain Death*
  - **Bug Fixes:** Frenzy no longer continues through the end of her False Life.

### V14.22
- Stats
  - Base health increased to 625 from 590.

### V14.9
- Stats
  - Gameplay radius increased to 65 units from 55.
  - Selection radius increased to 120 units from $111.11109924316406$.
- *Certain Death*
  - **Bug Fixes:** No longer lasts indefinitely if the acquired target is a *Vessel* created by Test of Spirit.

## Trivia

- Briar is one of champions who use health as a resource for abilities, the other five being **Dr. Mundo**, **Olaf**, **Soraka**, **Vladimir**, and **Zac**.

---
*This page was automatically generated from League of Legends Wiki data.*