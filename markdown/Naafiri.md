# Naafiri

## Table of Contents
- [Basic Information](#basic-information)
- [Statistics](#statistics)
  - [Map-specific Stats](#map-specific-stats)
- [Pets](#pets)
- [Abilities](#abilities)
- [Patch History](#patch-history)
- [Trivia](#trivia)

## Basic Information

| Attribute | Value |
|-----------|------:|
| **Champion** | Naafiri |
| **Title** | the Hound of a Hundred Bites |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2023-07-19 |
| **Release Patch** | V13.14 |
| **Latest Changes** | V25.11 |
| **Roles** | Assassin |
| **Riot Positions** | Middle |
| **External Positions** | Middle |
| **Blue Essence** | 675 |
| **Riot Points** | 585 |
| **Difficulty** | 1 |
| **Hero Type** | Assassin |
| **Alt Type** | Fighter |
| **Adaptive Type** | Physical |
| **Damage** | 3 |
| **Toughness** | 1 |
| **Control** | 1 |
| **Mobility** | 3 |
| **Utility** | 1 |
| **Style** | 70 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $610.0$ | $+105.0$ |
| **Mana** | $400.0$ | $+55.0$ |
| **Health Regen** | $7.5$ | $+0.7$ |
| **Mana Regen** | $7.5$ | $+1.0$ |
| **Armor** | $28.0$ | $+4.2$ |
| **Magic Resist** | $32.0$ | $+2.05$ |
| **Attack Damage** | $55.0$ | $+2.0$ |
| **Attack Speed** | $0.663$ | |
| **Movement Speed** | $340.0$ | $+0.0$ |
| **Attack Range** | $125.0$ | $+0.0$ |
| **Base Attack Speed** | $0.663$ | |
| **Attack Speed Ratio** | $0.625$ | |
| **Bonus AS per Level** | $2.1\%$ | |
| **Acquisition Radius** | $400$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $120$ units | |
| **Selection Height** | $191.667$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $110.0\%$ |

## Pets

### Packmate

| Attribute | Value |
|-----------|------:|
| **Range** | 125 units |
| **Gold** | 3 |
| **Experience** | 0 |
| **Health** | 80 / 301 |
| **Armor** | 0 / 30.6 |
| **Magic Resist** | 0 / 30.6 |
| **Damage** | 10 to 20 (+ 4% **bonus** AD) |
| **Damage Type** | Physical |
| **Attack Speed** | 100% of **Naafiri**’s attack speed |
| **Move Speed** | |
| **Control** | **Naafiri**’s basic attacks and ability hits against enemies command *Packmates* to attack the target for 2 seconds. Subsequent hits against the same target will refresh the duration. Hitting a different target will switch their focus to them from the current target.
- *Darkin Daggers* will command them to attack the first target hit by the missile that is not a minion.
  - The damage over time effect does not count for triggering this attack command, and thus will not refresh it.
- *Eviscerate* will command them to attack the nearest target hit by the flurry, prioritizing champions.
- *Hounds' Pursuit* will command them to attack the target upon collision of them. |
| **Targeting** | Minion, does not count towards the minion kill tracking score |
| **Spell Effects** | *Packmates' *attacks apply spell effects as area damage. |
| **On-Hit** | *Packmates' *attacks are mitigated by dodge, block, and blind. They also deal 135% damage to monsters and 50% damage to structures. |

**Abilities:**

- **Prey Drive:** After **Naafiri** casts an ability, *Packmates* deal「 30% increased damage ⟷ 10×1.3 to 20×1.3 (+ 5.2% **bonus** AD) physical damage 」on their next basic attack within 3 seconds. They can store up to 3 empowered attacks at a time.
- **Pouncing Assault:** *Packmates* will dash to visible targets hit by **Naafiri**’s basic attacks and abilities within 1300 units. They will also charge alongside **Naafiri** when she uses *Hounds' Pursuit*, becoming untargetable in the process.
- **Recalling of the Pack:** When **Naafiri** casts *The Call of the Pack* or *Eviscerate*, *Packmates* vanish immediately. They will then reappear near her shortly afterwards.
- **Replenish:** *Packmates* are healed to full health when they reappear from being vanished by *Eviscerate*.
- **Agility:** *Packmates* gain **bonus** movement speed during *The Call of the Pack*.
- **Minion Hunter:** *Packmates* will occasionally score the killing blow on nearby enemy minions that are below 25–55@1–11 (@=**Naafiri**’s level) health. 
- This has an internal cooldown (Not visible to the player and not affected by ability haste.) of 10 seconds.
- There is a minimum range of 300 units and a maximum range of 800 units around each *Packmate* for this effect to occur.
- *Packmates* will not perform this action while **Naafiri** is in brush.
  - Enemy vision of the brush may impact whether this behavior is enabled.

---

## Abilities

### Passive: We Are More

| Attribute | Value |
|-----------|------:|
| **Static Cooldown** | 30 to 10 |
| **Targeting** | Passive |
| **Affects** | Self, Enemies |

**INNATE:** Periodically, **Naafiri** summons a *Packmate* that fights alongside her. She may have up to 2@1; 3@9; 4@12; 5@15 *Packmates* at a time. **Naafiri** summons the maximum number of *Packmates* when the game starts and upon respawning.

Hitting champions or large monsters with abilities reduces the cooldown by 4 seconds. Killing enemies reduces it by 1 second.

*See [Pets](#Pets) for full details on Packmates.*

**Notes:**

- *Packmates* do not count as abilities for triggering the cooldown reduction.
- Only the initial hit of *Darkin Daggers* counts for reducing *We Are More*’s cooldown; the damage over time will not trigger the cooldown reduction.
- Both hits of *Eviscerate* will trigger the cooldown reduction.

---

### Q: Darkin Daggers

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 900 units |
| **Width** | 50 units |
| **Speed** | 1700 units/second |
| **Cost** | 55 / 60 / 65 / 70 / 75 Mana |
| **Cooldown** | 9 / 8.5 / 8 / 7.5 / 7 seconds |
| **Cooldown Start** | post-effect |
| **Targeting** | Direction |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |
| **Projectile** | True |

**ACTIVE:** **Naafiri** hurls a dagger in the target direction that deals physical damage to enemies hit and inflicts them with a bleed that deals **bonus** physical damage every $0.5$ seconds for 5 seconds. The bleed executes minions and non-epic monsters if they would be damaged below (health) 30 / 34 / 38 / 42 / 46 / 50 / 54 / 58 / 62 / 66 / 70 health. *Darkin Daggers* can be recast after $0.5$ seconds and within 4 seconds at no additional cost.

| Attribute | Value |
|-----------|------:|
| **Initial Physical Damage** | 35 / 40 / 45 / 50 / 55 (+ 20% **bonus** AD) |

| Attribute | Value |
|-----------|------:|
| **Bleed Physical Damage per Tick** | 3.5 / 6 / 8.5 / 11 / 13.5 (+ 8% **bonus** AD) |
| **Total Bleed Physical Damage** | 35 / 60 / 85 / 110 / 135 (+ 80% **bonus** AD) |
| **Total Physical Damage** | 70 / 100 / 130 / 160 / 190 (+ 100% **bonus** AD) |

If the dagger hits a target that is already bleeding, they are instead dealt the remaining bleed damage plus **additional bonus** physical damage, with the **base bonus** damage increased by 0%–100%@0–100 (@=target's **missing** health) and the *scaling* for the **bonus** damage increased by 0%–250%@0–100 (@=target's **missing** health). If that target is also a champion or large monster, **Naafiri** heals herself.

| Attribute | Value |
|-----------|------:|
| **Minimum Bonus Physical Damage** | 30 / 45 / 60 / 75 / 90 (+ 40% **bonus** AD) |
| **Maximum Bonus Physical Damage** | 60 / 90 / 120 / 150 / 180 (+ 140% **bonus** AD) |

| Attribute | Value |
|-----------|------:|
| **Minimum Total Physical Damage** | 100 / 145 / 190 / 235 / 280 (+ 140% **bonus** AD) |
| **Maximum Total Physical Damage** | 130 / 190 / 250 / 310 / 370 (+ 240% **bonus** AD) |

| Attribute | Value |
|-----------|------:|
| **Heal** | 45 / 60 / 75 / 90 / 105 (+ 40% **bonus** AD) |

**RECAST:** **Naafiri** mimics the first cast's effects.

**Notes:**

- *Darkin Daggers* calculates its additional damage based on missing health before applying the remaining bleed damage.
- *Darkin Daggers' * execute against minions considers both the damage of the initial hit of the dagger and any bleed tick for putting the target under the threshold.

---

### W: The Call of the Pack

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.75$ seconds |
| **Cost** | 60 Mana |
| **Cooldown** | 26 / 24 / 22 / 20 / 18 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Auto |
| **Affects** | Self |

**ACTIVE:** **Naafiri** goes on the hunt for the next 5 seconds (Hunt begins from the start of the cast time), becoming untargetable for the first 1 second and vanishing all of her active *Packmates* in the first $0.9$ seconds. Her active *Packmates* will reappear near her after their vanish.

After $1.25$ seconds into the hunt, **Naafiri** summons 2 additional *Packmates* that last for the remaining duration and can exceed *We Are More’s* summon cap.

While on the hunt, **Naafiri** gains 20% AD **bonus** attack damage and grants herself and all *Packmates* **bonus** movement speed.

| Attribute | Value |
|-----------|------:|
| **Bonus Movement Speed** | 20 / 22.5 / 25 / 27.5 / 30% |

*Casting Hounds' Pursuit extends The Call of the Pack's hunt duration by $1.75$ seconds, up to its original duration.*

***Naafiri** can move during The Call of the Pack's cast time.*

**Notes:**

- While *The Call of the Pack* is active, it increases *We Are More’s* summon cap to 4@1; 5@9; 6@12; 7@15.
- The order of additional *Packmates* that despawn is predetermined, most likely their internal spawn IDs. The ability does not consider which *Packmates* are the closest or healthiest. Effect at cast time end

---

### E: Eviscerate

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 250 – 450 (Minimum & maximum dash range) / 650 (Maximum extended dash range through terrain) units |
| **Collision Radius** | 50 units |
| **Effect Radius** | 230 (Flurry radius) units |
| **Speed** | 900 units/second |
| **Cost** | 35 Mana |
| **Cooldown** | 11 / 10 / 9 / 8 / 7 seconds |
| **Targeting** | Location |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Out of Range Behavior** | cast at max |
| **Terrain Grace** | True |
| **Grounded** | True |
| **Knockdown** | True |

**ACTIVE:** **Naafiri** dashes in the target direction and deals physical damage to enemies she passes through. Upon arrival, she explodes in a flurry of blades to deal physical damage to nearby enemies.

| Attribute | Value |
|-----------|------:|
| **Dash Physical Damage** | 15 / 25 / 35 / 45 / 55 (+ 40% **bonus** AD) |

| Attribute | Value |
|-----------|------:|
| **Flurry Physical Damage** | 60 / 85 / 110 / 135 / 160 (+ 80% **bonus** AD) |

| Attribute | Value |
|-----------|------:|
| **Total Physical Damage** | 75 / 110 / 145 / 180 / 215 (+ 120% **bonus** AD) |

*Packmates* vanish during the dash and reappear near **Naafiri** on her arrival. They are also healed to full health upon their reappearance.

**Notes:**

- *Packmates* will not be affected by *Eviscerate* while under the effect of *Hounds' Pursuit*.

---

### R: Hounds' Pursuit

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 900 units |
| **Effect Radius** | 2100 (Sight radius) units |
| **Speed** | 1800 units/second |
| **Cost** | 100 Mana |
| **Cooldown** | 110 / 102.5 / 95 / 87.5 / 80 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Unit |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | Special |
| **Spell Effects** | spell |
| **Grounded** | True |
| **Knockdown** | True |
| **Silence** | True |

**ACTIVE:** **Naafiri** and her *Packmates* channel for $0.75$ seconds to single out the target enemy champion, revealing them in the duration. Upon completion of the channel, they dash to the target; upon arrival **Naafiri** deals physical damage and slows the target by 99% for $0.25$ seconds, whereas each *Packmate* deals 10% of that damage. *Packmates* become untargetable during their channel and dash.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 150 / 200 / 250 / 300 / 350 (+ 120% **bonus** AD) |

| Attribute | Value |
|-----------|------:|
| **Physical Damage per *Packmate*** | #expr:#var:rd_b1 to #expr:#var:rd_b3 (+ |

If **Naafiri** scores a champion takedown within 7 seconds of cast, she triggers a pulse that grants sight of the surrounding area (Can see through terrain) for 1 second and reveals enemy champions within for 4 seconds. She can also recast *Hounds' Pursuit* once from triggering this effect within 12 seconds at no cost.

**RECAST:** **Naafiri** immediately grants herself a shield for 3 seconds before mimicking the first cast's effects (Excluding the takedown effects, which can trigger only once).

| Attribute | Value |
|-----------|------:|
| **Shield** | 100 / 125 / 150 / 175 / 200 (+ 150% **bonus** AD) |

**Notes:**

- *Hounds' Pursuit*’s theoretical **total** physical damage from *Packmates* **only**, assuming **Naafiri** has the maximum number of them active and *Hounds' Pursuit* is always ranked at the earliest experience permitted (ranks 1 / 2 / 3 at levels 6 / 11 / 16):
  - **LEVELS 6–8:** #expr: (+ #expr:&emsp;/&emsp;**icononly=true** #expr: (+ #expr:.
  - **LEVELS 9–10:** #expr: (+ #expr:&emsp;/&emsp;**icononly=true** #expr: (+ #expr:.
  - **LEVEL 11:** (#expr: (+ #expr:&emsp;/&emsp;**icononly=true** (#expr: (+ #expr:.
  - **LEVELS 12–14:** (#expr: (+ #expr:&emsp;/&emsp;**icononly=true** (#expr: (+ #expr:.
  - **LEVEL 15:** (#expr: (+ #expr:&emsp;/&emsp;**icononly=true** (#expr: (+ #expr:.
  - **LEVELS 16–18:** #expr: (+ #expr:&emsp;/&emsp;**icononly=true** #expr: (+ #expr:.
- **Naafiri** marks the enemy champion targeted by *Hounds' Pursuit* in order to gain assist credit on them, lasting for the standard credit timer.
- Spell shield will block **Naafiri**’s collision damage but cannot prevent collision damage from *Packmates*.
- **Naafiri** will track the target if they change locations, up to a maximum distance.
  - She will dash to the target's previous location if they move too far away during the dash.
- **Naafiri** will be ordered to basic attack the target at the end of the dash.
- **Naafiri** will windup a basic attack while dashing.
- *Packmates* hitting the same target as **Naafiri** will count as dealing "additional" damage.
- The target is revealed for another $1.5$ seconds after the channel completes.
- The following table refers for interactions while **Naafiri** is channeling:
  - If **Naafiri**’s channel is interrupted, the *Packmates' * channels will be canceled as well.
  - The channel is cancelled if the target dies or moves too far away, but not if they become untargetable.

---

## Patch History

### V25.11
- We Are More
  - **Bug Fixes:** Move orders no longer persist on Packmates if the player issues a move order within a tiny radius of their current position, allowing the ordered Packmates to freely leave Naafiri's radius of influence and remain at their destination location indefinitely so long as the player does not issue a new move order (which would subsequently cause the Packmates to re-calculate their distance from Naafiri's radius of influence and return to her).
- Hounds' Pursuit
  - **Bug Fixes:** Naafiri and her Packmates no longer ignore collision with Emperor's Divide during their dash, which would previously allow them to bypass the displacement.

### V25.10
- We Are More
  - Packmate monster damage reduced to 135% from 155%.

### V25.09
- The Call of the Pack
  - Cooldown increased to 26 / 24 / 22 / 20 / 18 seconds from 22 / 21 / 20 / 19 / 18.
- Eviscerate
  - Dash AD ratio reduced to 40% **bonus** AD from 50%.
- Hounds' Pursuit
  - ***UNDOCUMENTED - REMOVED:*** Her own and her Packmates' dash speed no longer scales with 100% of Naafiri's **total** movement speed.
  - ***UNDOCUMENTED - REMOVED:*** Tooltip no longer mentions the window for unlocking the recast is timed. *Actual effect unchanged.*

### V25.07#April 2nd Hotfix|V25.07
- Darkin Daggers
  - Initial base damage reduced to 35 / 40 / 45 / 50 / 55 from 35 / 45 / 55 / 65 / 75.
  - Total bleed base damage changed to 35 / 60 / 85 / 110 / 135 from 30 / 60 / 90 / 120 / 150.
    - Bleed base damage per tick changed to 3.5 / 6 / 8.5 / 11 / 13.5 from 3 / 6 / 9 / 12 / 15.
- Eviscerate
  - Flurry base damage reduced to 60 / 85 / 110 / 135 / 160 from 60 / 90 / 120 / 150 / 180.
  - Cooldown increased to 11 / 10 / 9 / 8 / 7 seconds from 9 / 8.5 / 8 / 7.5 / 7.

### V25.07
- Stats
  - Base health reduced to 610 from 635.
  - Base armor reduced to 28 from 30.
- We Are More
  - Packmate monster damage reduced to 155% from 165%.
  - **Bug Fixes:** Packmates no longer sometimes fail to dash alongside her during her abilities.
- Darkin Daggers
  - Minion damage increased to 100% from 80%.
- The Call of the Pack
  - Cooldown increased to 22 / 21 / 20 / 19 / 18 seconds from 20 / 19.5 / 19 / 18.5 / 18.
  - **Bug Fixes:** No longer creates a redundant buff icon.
- Hounds' Pursuit
  - **Bug Fixes:** Now properly crosses terrain when the target is beyond it.

### V25.06#March 19th Hotfix|V25.06
- Stats
  - Base attack damage reduced to 55 from 57.
  - Base health regeneration reduced to $7.5$ from 9.
  - Health regeneration growth reduced to $0.7$ from $0.9$.

### V25.06
- Stats
  - Base attack damage increased to 57 from 55.
  - Attack damage growth reduced to 2 from $2.1$.
  - Armor growth reduced to $4.2$ from $4.7$.
  - Health growth reduced to 105 from 120.
- We Are More
  - Maximum Packmates increased to 2@1; 3@9; 4@12; 5@15 from 2@1; 3@9.
  - Packmate base attack damage reduced to 10 to 20 from 12 to 32.
  - Packmate attack damage bonus AD ratio reduced to 4% **bonus** AD from 5%.
  - Packmate health reduced to 80 / 301 from 80 / 352.
  - Packmate resistances reduced to 0 / 30.6 from 0 / 34.
  - Packmate incoming area damage modifier increased to 76%–55%@1–15 from 76%–50%@1–14.
  - Packmate monster damage increased to 165% from 100%.
  - Packmate attack command duration reduced to 2 seconds from 3.
  - **Undocumented:** Packmate structure damage increased to 50% from 25%.
- Darkin Daggers
  - Minion damage increased to 80% from 60%.
- The Call of the Pack (W)
  - *Swapped with Hounds' Pursuit.*
  - **New Effect:** Naafiri now becomes untargetable in the first second of the hunt's duration.
  - **Removed:*** No longer despawns any active Packmates.
  - **Removed:*** No longer spawns the maximum number of Packmates We Are More allows before spawning additional ones.
  - **New Effect:** Any active Packmates are now vanished in the first $0.9$ seconds of the hunt.
  - Additional Packmates summoned reduced to 2 at all ranks from 2 / 3 / 4.
  - Hunt duration reduced to 5 seconds from 15.
  - Hunt duration now begins at the start of the cast time, rather than afterwards.
  - **Removed:*** Bonus movement speed no longer decays over 4 seconds.
  - **Removed:*** Bonus movement speed is no longer reduced by 50% upon receiving non-minion damage.
  - Bonus movement speed reduced to 20 / 22.5 / 25 / 27.5 / 30% from 70 / 85 / 100%.
  - Bonus movement speed duration increased to 5 seconds from 4.
  - **Removed:*** No longer grants a base amount of 5 / 15 / 25 **bonus** attack damage.
  - Bonus attack damage AD ratio changed to 20% AD at all ranks from 8 / 16 / 24% AD.
  - Cooldown reduced to 20 / 19.5 / 19 / 18.5 / 18 seconds from 120 / 110 / 100.
  - Mana cost reduced to 60 from 100.
  - **Removed:*** Her first basic attack or ability hit against a champion no longer grants her a 125 / 325 / 525 (+ 50% **bonus** AD) shield for 3 seconds.
  - **Removed:*** No longer refreshes the hunt's duration and reapplies the effects (excluding Packmate summoning) upon scoring the first champion takedown within the duration.
  - **Removed:*** No longer grants sight of the surrounding 2100 units for 2 seconds and reveals enemy champions within for 4 seconds.
  - **Removed:*** No longer increases Packmates' **maximum** health by 25%.
  - **Removed:*** No longer restores Packmates' health to full upon cast and upon the hunt ending.
  - **Removed:*** No longer reduces We Are More’s cooldown by 50%.
  - **Removed:*** No longer increases Hounds' Pursuit’s cast range by 80 / 160 / 240.
- Eviscerate
  - Cooldown reduced to 9 / 8.5 / 8 / 7.5 / 7 seconds from 10 / 9.5 / 9 / 8.5 / 8.
  - Target range increased to 450 units from 350.
    - **New Effect:** Now dashes to cursor instead of always to maximum range.
  - Terrain grace increased to 200 units from 150.
  - Dash base damage reduced to 15 / 25 / 35 / 45 / 55 from 35 / 50 / 65 / 80 / 95.
- Hounds' Pursuit (R)
  - *Swapped with The Call of the Pack.*
  - **New Effect:** If Naafiri scores a champion takedown within 7 seconds of casting, she grants sight over a 2100 unit radius for 1 second and reveals enemy champions within for 4 seconds. Additionally, she is allowed to recast Hounds' Pursuit once in the next 12 seconds.
  - **New Effect:** Upon recasting, Naafiri gains a 100 / 150 / 200 (+ 150 **bonus** AD) shield for 3 seconds.
  - **New Effect:** Casting/recasting the ability now extends The Call of the Pack’s duration by $1.75$ seconds (up to its maximum).
  - **New Effect:** Dash speed now scales with 100% of her movement speed.
  - Cooldown increased to 110 / 95 / 80 seconds from 22 / 20 / 18 / 16 / 14.
  - Mana cost increased to 100 at all ranks from 70 / 60 / 50 / 40 / 30.
  - Target range changed to 900 units at all ranks from 700 / 760 / 820 / 880 / 940.
  - Base damage changed to 150 / 250 / 350 from 30 / 70 / 110 / 150 / 190.
    - Packmate base damage changed to 15 / 25 / 35 from 3 / 7 / 11 / 15 / 19.
  - Damage bonus AD ratio increased to 120% **bonus** AD from 80%.
    - Packmate damage bonus AD ratio increased to 12% **bonus** AD from 8%.
  - **Removed:*** Naafiri and her Packmates no longer collide with the first champion hit; they can only collide with the targeted enemy.
  - **Removed:*** Can no longer be cast on non-champion units.
  - **Removed:*** No longer refunds 50% of its cooldown if the channel is interrupted.

### V25.S1.3
- We Are More
  - **Bug Fixes:** Packmate attacks against champions no longer trigger minion call-for-help against the summoner (if in valid range).

### V25.S1.2
- We Are More
  - **Bug Fixes:** Teleport’s / Unleashed Teleport’s dash now transports her Packmates to the target location.

### V14.22
- Darkin Daggers
  - **Bug Fixes:** Missile longer fails to apply its effects after **Naafiri** dies.

## Trivia

- This champion has no ability power ratio.
- Naafiri has a special joke animation which comes in a sequence of three animations, playing the next animation in the sequence once the appropriate key combination is pressed just as the previous animation ends.
  - The first animation plays by pressing Ctrl + 1 twice, which shows Naafiri being pet on the head by a hand resembling the Legacy cursor.
  - The second animation plays by pressing Ctrl + 2 as the first animation ends, with Naafiri being petted on the neck before falling on her back.
  - The third animation plays by pressing Ctrl + 3 as the second animation ends, showing Naafiri being rubbed on the belly while still lying down. Pressing Ctrl + 3 again during the end of the animation will allow it to be repeated again as many times as the player keeps pressing the key.
- Naafiri's Series 2 Eternals make the following references:
  - *Let the Dogs Out* is a reference to the song called Who Let The Dogs Out by Baha Men.
  - *What the dog doin?* is a reference to the What the Dog Doin' meme.

---
*This page was automatically generated from League of Legends Wiki data.*