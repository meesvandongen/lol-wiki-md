# Briar

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
| **Champion** | Briar |
| **Title** | the Restrained Hunger |
| **Resource** | Frenzy |
| **Range Type** | Melee |
| **Release Date** | 2023-09-14 |
| **Release Patch** | V13.18 |
| **Latest Changes** | V25.18 |
| **Roles** | Diver |
| **Riot Positions** | Jungle |
| **External Positions** | Jungle |
| **Blue Essence** | 2400 |
| **Riot Points** | 880 |
| **Difficulty** | 2 |
| **Hero Type** | Fighter |
| **Alt Type** | Assassin |
| **Adaptive Type** | Physical |
| **Damage** | 2 |
| **Toughness** | 2 |
| **Control** | 3 |
| **Mobility** | 3 |
| **Utility** | 1 |
| **Style** | 10 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $625.0$ | $+95.0$ |
| **Mana** | $0.0$ | $+0.0$ |
| **Armor** | $30.0$ | $+4.2$ |
| **Magic Resist** | $32.0$ | $+2.05$ |
| **Attack Damage** | $60.0$ | $+2.5$ |
| **Attack Speed** | $0.644$ | |
| **Movement Speed** | $340.0$ | $+0.0$ |
| **Attack Range** | $125.0$ | $+0.0$ |
| **Base Attack Speed** | $0.644$ | |
| **Attack Speed Ratio** | $0.669$ | |
| **Bonus AS per Level** | $2.0\%$ | |
| **Attack Windup** | $19.3\%$ | |
| **Acquisition Radius** | $400$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $120$ units | |
| **Selection Height** | $191.667$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $105.0\%$ |
| **Damage Taken** | $95.0\%$ |
| **Healing** | $115.0\%$ |

## Abilities

### Passive: Crimson Curse

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Enemies, Self |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | default |
| **Parry** | True |

**INNATE:** **Briar**’s basic attacks and abilities inflict a bleed against enemies for 5 seconds, refreshing on subsequent applications, stacking up to 7 times and dealing 25% damage for stacks beyond the first. The bleed deals「 10 to 50 (+ 50% **bonus** AD) **total** physical damage over the duration, increased by 10×0.25 to 50×0.25 (+ 12.5% **bonus** AD) for each subsequent stack and up to a maximum of 10*#expr: 1+( (+ 50*#expr: 1+(. ⟷ 10/(5/0.5) to 50/(5/0.5) (+ 5% **bonus** AD) physical damage every $0.5$ seconds, increased by 10×0.25/(5/0.5) to 50×0.25/(5/0.5) (+ 1.25% **bonus** AD) for each subsequent stack and up to a maximum of 10*#expr: 1+( (+ 50***Briar** heals herself equal to 25% of the pre-mitigation damage (Damage calculated before modifiers) dealt. If a target dies while bleeding, she will heal herself equal to 125% of the remaining bleed dama 」.

**Briar** has no (health regeneration) **base** health regeneration, but she increases healing from all sources by 0%–40%@0–100 (@=**missing** health) (+ 0%–2.5%@0–100 (@=**missing** health) per 100 **bonus** health).

**Notes:**

- Blood Frenzy’s area of effect damage around the target does not apply bleed stacks.

---

### Q: Head Rush

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 475 units |
| **Speed** | 600 to 600+300 units/second |
| **Cost** | 6% **current** Health |
| **Cooldown** | 13 / 12 / 11 / 10 / 9 seconds |
| **Queue Time** | $0.5$ seconds |
| **Targeting** | Unit |
| **Affects** | Enemies, Self |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | spell |
| **Grounded** | True |
| **Knockdown** | True |

**ACTIVE:** **Briar** leaps to the target unit. If the target is an enemy, she deals physical damage, applies on-hit effects, triggers on-attack effects, stuns them for $0.85$ seconds, and reduces their (armor penetration) armor and (magic penetration) magic resistance for 5 seconds.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 60 / 85 / 110 / 135 / 160 (+ 80% **bonus** AD) (+ 60% AP) |

| Attribute | Value |
|-----------|------:|
| **Resistances Reduction** | 10 / 12.5 / 15 / 17.5 / 20% |

*Head Rush resets **Briar**’s basic attack timer.*

**Notes:**

- *Head Rush* can be cast on jungle plants, wards, and traps.
  - It cannot be cast on structures.
- *Head Rush*’s damage benefits from life steal.
- *Head Rush* is a non-following dash.
  - It picks its dash destination ~100 units in front of enemy targets, 75 units in front of enemy targets when already very close to them, 75 units in front of allied targets.
  - If already within 75 units of her target, she dashes to her current location, which takes 0 time but still triggers dash effects such as Sudden Impact.
  - It does not force the target's resistance values to update immediately outside of the natural stat update cycle, which means it will typically still be mitigated by the unreduced magic resistance value.

---

### W: Blood Frenzy

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 75 (Minimum dash distance) - 300 (Maximum dash distance) / 650 (Maximum extended dash distance across terrain) units |
| **Effect Radius** | 1000 (Frenzy target radius) / 275 (Attack cleave radius around primary target) units |
| **Speed** | 1200 (Dash speed) units/second |
| **Cost** | 6% **current** Health |
| **Cooldown** | 14 / 13 / 12 / 11 / 10 seconds |
| **Cooldown Start** | on-cast |
| **Queue Time** | $0.5$ seconds |
| **Targeting** | Location |
| **Affects** | Enemies, Self |
| **Damage Type** | Physical |
| **Spell Effects** | default |
| **Out of Range Behavior** | cast at max |
| **Terrain Grace** | True |
| **Parry** | True |
| **Grounded** | True |
| **Knockdown** | True |

**ACTIVE:** **Briar** dashes in the target direction. If there is a nearby enemy upon her arrival, she gains *Blood Frenzy* for 5 seconds, during which she can cast *Snack Attack*.

**BLOOD FRENZY:** **Briar** breaks free from her pillory, causing her to become forced (See notes) to basic attack the nearest enemy. She reveals the target and gains ghosting, **bonus** attack speed and **bonus** movement speed, as well as empowering her basic attacks to have an uncancelable windup and deal physical damage to enemies surrounding her target. This damage is affected by critical strike modifiers.

| Attribute | Value |
|-----------|------:|
| **Bonus Attack Speed** | 55 / 65 / 75 / 85 / 95% |

| Attribute | Value |
|-----------|------:|
| **Bonus Movement Speed** | 24 / 33 / 42 / 51 / 60% |

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 60 / 70 / 80 / 90 / 100% AD |

**Briar** prioritizes attacking champions, then large monsters or minions, and then any other unit. If she casts *Head Rush* on a non-champion, she will shift her target priority to large monsters or minions, then champions, and then any other unit.

*Blood Frenzy causes **Briar** to become unable to receive movement and attack commands. The frenzy ends early if there are no longer any nearby valid targets or Chilling Scream is cast. Blood Frenzy resets **Briar**’s basic attack timer. Head Rush and Certain Death can be cast during the dash.*

**Notes:**

- *Blood Frenzy* acquires targets regardless of if they are visible or not.
  - This does not apply to stealthed targets.
- If the main attack critically strikes, the cleave damage will do so as well.
- **Briar** uses Frenzy as a resource to indicate the remaining duration of the frenzy, in seconds.
- The following table refers for interactions while **Briar** is in her frenzy:
  - While in the frenzy, **Briar** cannot control her movement nor declare who she attacks. She will automatically acquire a nearby enemy as her attack target based on a priority system, becoming *forced* to basic attack the target and consequently move into her attack range of them to do so.
    - Disarming crowd control as well as any other lockout that would disable basic attacking will disable the forced attacks.
    - Forced action crowd control will cause her forced attacks to be either overridden or disabled, depending on the actions being forced by the debuff. * can only be cast on targets within its radius. ** interrupts the frenzy. ** can be cast during the frenzy without interrupting it.|items=true|consume=true|spells=true,true,false,false|interrupts=death

---

### W: Snack Attack

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Targeting** | Auto |
| **Affects** | Enemies, Self |
| **Damage Type** | Physical |
| **Spell Shield** | False |
| **Spell Effects** | proc |
| **Parry** | True |

**ACTIVE:** **Briar** empowers her next basic attack within 5 seconds during *Blood Frenzy* to take a bite out of the target enemy, gaining (range) 50 **bonus** range, dealing **bonus** physical damage and healing her for 5% of her **maximum** health plus a percentage of the post-mitigation damage (Damage calculated after modifiers) dealt.

| Attribute | Value |
|-----------|------:|
| **Bonus Physical Damage** | 5 / 20 / 35 / 50 / 65 (+ 5% AD) (+ 9% (+ $2.5$% per 100 **bonus** AD) of the target's **missing** health) |

| Attribute | Value |
|-----------|------:|
| **Heal Percentage** | 24 / 28 / 32 / 36 / 40% |

*Snack Attack* deals 110% damage against minions and monsters, with the damage based on the target's health ratio being capped at 400.

| Attribute | Value |
|-----------|------:|
| **Non-Champion Bonus Damage** | 5.5 / 22 / 38.5 / 55 / 71.5 (+ 5.5% AD) (+ 9.9% (+ 2.75% per 100 **bonus** AD) of the target's **missing** health) |

*Snack Attack resets **Briar**’s basic attack timer.*

**Notes:**

- *Snack Attack* can only be used during *Blood Frenzy*; if the frenzy ends at any point, the empowered attack will be lost immediately.
- *Snack Attack* can be cast while **Briar** is charmed or taunted.

---

### E: Chilling Scream

| Attribute | Value |
|-----------|------:|
| **Range** | -100 (Backwards rectangle reach) / cr 400 to 600 units |
| **Cast Time** | None (Immediately starts charging) / $0.15$ (Recast time) |
| **Width** | 380 units |
| **Speed** | 1900 (Missile speed) / 1800 (Knockback speed) units/second |
| **Cost** | 6% **current** Health |
| **Cooldown** | 16 seconds |
| **Cooldown Start** | post-effect |
| **Queue Time** | $0.5$ seconds |
| **Targeting** | Direction / Auto |
| **Affects** | Enemies, Self |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | True |
| **Silence** | False |

**ACTIVE:** **Briar** prepares to unleash a scream in the target direction, charging for up to 1 second, during which she increases *Chilling Scream*’s damage and range, and gains 35% damage reduction and heals herself every $0.25$ seconds.

| Attribute | Value |
|-----------|------:|
| **Heal Per Tick** | 2.5 / 2.875 / 3.25 / 3.625 / 4% **maximum** health |
| **Maximum Heal** | 10 / 11.5 / 13 / 14.5 / 16% **maximum** health |

*Chilling Scream* can be recast within the duration, and does so automatically afterwards. *Chilling Scream*’s charge cannot be interrupted by crowd control.

**RECAST:** **Briar** unleashes the scream in the direction she targeted at the time of cast, dealing magic damage to enemies hit and slowing them by 80% for $0.5$ seconds.

| Attribute | Value |
|-----------|------:|
| **Maximum Magic Damage** | 80 / 115 / 150 / 185 / 220 (+ 100% **bonus** AD) (+ 100% AP) |
| **Minimum Magic Damage** | 2 / 2.875 / 3.75 / 4.625 / 5.5 (+ 2.5% **bonus** AD) (+ 2.5% AP) |

If *Chilling Scream* was charged for its full duration, enemies hit are also knocked back 575 units. If they collide with terrain, they will rebound to take **bonus** magic damage and become knocked up for $0.5$ seconds and stunned for $1.5$ seconds.

| Attribute | Value |
|-----------|------:|
| **Bonus Magic Damage** | 140 / 215 / 290 / 365 / 440 (+ 240% **bonus** AD) (+ 240% AP) |

| Attribute | Value |
|-----------|------:|
| **Total Magic Damage** | 220 / 330 / 440 / 550 / 660 (+ 340% **bonus** AD) (+ 340% AP) |

**Notes:**

- *Chilling Scream* increases its damage by its minimum every $0.025$ seconds over the duration. - This ability will cast from wherever the caster is at the end of the cast time.
- The following table refers for interactions while **Briar** is channeling:

#### Channel Behavior (charge)

| Aspect | State / Notes |
|--------|---------------|
| **Attacking** | Disabled |
| **Movement** | Disabled |
| **Abilities** | Disabled |
| **Items** | Disabled |
| **Summoner Spells** | Allowed / Disabled / Recasts |
| **Consumables** | Disabled |
| **Interrupted by** | death |
| **Notes** | but can still use trinkets |

---

### R: Certain Death

| Attribute | Value |
|-----------|------:|
| **Range** | 12000 (Missile range) units |
| **Cast Time** | 1 (Time before kicking hemolith) / $1.25$ (Time before dashing to target) seconds |
| **Effect Radius** | Global (Second cast dash range and frenzy marked/non-marked target radius (see notes)) / cr 1500 (Frenzy non-marked target radius) / er 575 (Fear radius on impact around target) |
| **Width** | 320 (Missile width) units |
| **Speed** | 2000 (Missile speed) / 2500 – 5000 (Dash speed) units/second |
| **Cost** | 6% **current** Health |
| **Cooldown** | 120 / 110 / 100 / 90 / 80 seconds |
| **Cooldown Start** | on-cast |
| **Queue Time** | $0.5$ seconds |
| **Targeting** | Direction |
| **Affects** | Enemies, Self |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | True |
| **Grounded** | True |
| **Knockdown** | False |

**ACTIVE:** **Briar** kicks her pillory's hemolith in the target direction, briefly granting sight of its surroundings (Can grant sight of area into brush and through terrain) as it travels (creates vision bubbles every 0.25 seconds) and marking the first enemy champion hit as her prey. The mark's application disrupts the target's ongoing channels. While the target is marked, they are revealed.

If a target is hit, **Briar** cleanses herself from all crowd control and becomes immune to them over a cast time, afterwards dashing to them with displacement immunity. Upon arrival, she creates an explosion around the marked target that deals magic damage to them and nearby enemies and fears all non-marked targets for $1.5$ seconds, during which they are slowed by 35%. She then enters a state of *Hematomania*.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 150 / 200 / 250 / 300 / 350 (+ 130% AP) |

**HEMATOMANIA:** **Briar** gains all effects of *Blood Frenzy* as well as **bonus** armor and **bonus** magic resistance equal to 20% AD, life steal, and (ms) **additional bonus** movement speed.

| Attribute | Value |
|-----------|------:|
| **Life Steal** | 10 / 12.5 / 15 / 17.5 / 20% |

| Attribute | Value |
|-----------|------:|
| **Additional Bonus Movement Speed** | 10 / 15 / 20 / 25 / 30% |

While in the empowered frenzy, **Briar** prioritizes attacking the marked target over all other units and regardless of range. If that target becomes invalid, she will shift back to her normal targeting priority (nearest non-marked champion, then large monster or minion, and then any other unit) until the marked target can be attacked again. *Hematomania* lasts until the mark is dispelled by any means, including **Briar**’s or the target's death.

*Casting Chilling Scream removes the target's mark and ends Hematomania early. Head Rush’s cast does not shift her targeting priority during Hematomania.*

**Notes:**

- **Briar** will track the target with the dash if they change locations.
  - She will not stop tracking the target until she reaches them, as there is no maximum tracking distance.
- **Briar** lands 100 units in front of her target at the end of the dash.
- **Briar** will not initiate the second cast if she is being suppressed, is under resurrection, or is in the Realm of Death when her target is hit by the missile in the normal realm.
  - Since *Certain Death* cleanses herself from all crowd control when the first cast hits, she will be able to start the second cast due to removing the suppression effect.
- **Briar** will still dash to the target even if they die before she reaches them.
- **Briar** will transition from having total crowd control immunity during the cast time of the dash, to having displacement immunity during the dash.
- **Briar** will place herself onto the ground and interrupt displacements affecting her upon starting the second cast time.
- *Hematomania*’s targeting priorities differ slightly from that of *Blood Frenzy’s*:
  - She will prioritize the marked target regardless of range and over all other units as long as it can be attacked.
    - If the marked target is not available, then she shifts her priority to the nearest other champion, then large monster or minion, and then any other unit.
      - If neither the marked target is valid or any other valid targets are close nearby, then she will prioritize targeting the nearest other champion, then large monster or minion, and then any other unit within a global radius.
- Starting the second cast interrupts any spells that **Briar** is channeling.
- The mark will expire if *Hematomania*’s buff is dispelled.
- Casting *Blood Frenzy* during *Hematomania* will not grant any additional bonuses nor will it override *Hematomania*’s effects.
  - Gaining *Hematomania* while *Blood Frenzy* is active will override the previous buff.
- The disrupt is 'wrapped' into a status effect that says the target is Silenced for $0.3$ seconds, but it does not actually *silence*. It however makes sure that the *disrupt* is prevented by immunity to silences. - This ability will cast from wherever the caster is at the start of the cast time.
- The following table refers for interactions while **Briar** is in either the first or second cast time:

- The following table refers for interactions while **Briar** is dashing:

- The following table refers for interactions while **Briar** is in *Hematomania*: * can only be cast on targets within its radius. ** interrupts the frenzy. ** is disabled.|items=true|consume=true|spells=true,true,false,false|interrupts=death

---

## Patch History

### V25.18
- Head Rush
  - Damage type changed to physical from magic.
  - Base damage reduced to 60 / 85 / 110 / 135 / 160 from 60 / 90 / 120 / 150 / 180.
- Blood Frenzy
  - **Bug Fixes:** Manual attack orders issued on a target during the Frenzy can now properly persist upon exiting the state and are no longer automatically dropped.
- Certain Death
  - Global audio warning now plays after the cast time of the missile's cast instead of at its start.
  - **Removed:*** Explosion damage no longer scales off of 50% **bonus** AD.
  - Explosion damage AP ratio increased to 130% AP from 120% AP.
  - Missile range increased to 12000 units from 10000.

### V25.17
- Blood Frenzy
  - **Bug Fixes:** No longer sometimes has a brief delay before aggroing a valid target.

### V25.14
- Blood Frenzy
  - **Bug Fixes:** Casting Head Rush on an enemy champion who is standing near an allied minion no longer unintentionally causes her to prioritize enemy minions.
- Certain Death
  - Disable immunity during the cast time for the dash changed to total crowd control immunity from displacement immunity.
  - **Bug Fixes:** **Briar** no longer becomes unable to cast any spell after Tempered Fate’s stasis expires if she was affected by it prior to *Certain Death*’s dagger landing.

### V25.13#June 27th Hotfix|V25.13
- Stats
  - Attack damage growth reduced to $2.5$ from 3.
- Crimson Curse
  - Maximum number of stacks reduced to 7 from 9.

### V25.13
- Blood Frenzy
  - **New Effect:** All attacks during her *Blood Frenzy* now have an uncancelable windup.
- Chilling Scream
  - Recast time changed to $0.15$ seconds from 100% of her attack windup.

### V25.11
- Chilling Scream
  - **Bug Fixes:** Issuing an attack-move command during the charge no longer incorrectly causes the ability to recast.

### V25.09
- General
  - **Bug Fixes:** If feared during her frenzy state (Blood Frenzy / Certain Death), no longer becomes unable to move and attack acquired targets, standing still until exiting the state instead.

### V25.05
- Certain Death
  - **Bug Fixes:** Cast time to begin the travel is now properly canceled upon her False Life triggering.

### V25.04
- Certain Death
  - **Bug Fixes:** Frenzy no longer continues through the end of her False Life.

### V14.22
- Stats
  - Base health increased to 625 from 590.

## Trivia

- Briar is one of champions who use health as a resource for abilities, the other five being Dr. Mundo, Olaf, Soraka, Vladimir, and Zac.

---
*This page was automatically generated from League of Legends Wiki data.*