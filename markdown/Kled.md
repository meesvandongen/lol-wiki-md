# Kled

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
| **Champion** | Kled |
| **Title** | the Cantankerous Cavalier |
| **Resource** | Courage |
| **Range Type** | Melee |
| **Release Date** | 2016-08-10 |
| **Release Patch** | V6.16 |
| **Latest Changes** | V25.16 |
| **Roles** | Skirmisher |
| **Riot Positions** | Top |
| **External Positions** | Top |
| **Blue Essence** | 2400 |
| **Riot Points** | 880 |
| **Difficulty** | 3 |
| **Hero Type** | Fighter |
| **Alt Type** | Tank |
| **Adaptive Type** | Physical |
| **Damage** | 3 |
| **Toughness** | 2 |
| **Control** | 1 |
| **Mobility** | 2 |
| **Utility** | 1 |
| **Style** | 30 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $410.0$ | $+84.0$ |
| **Mana** | $100.0$ | $+0.0$ |
| **Health Regen** | $6.0$ | $+0.75$ |
| **Armor** | $35.0$ | $+5.2$ |
| **Magic Resist** | $28.0$ | $+2.05$ |
| **Attack Damage** | $65.0$ | $+3.5$ |
| **Attack Speed** | $0.625$ | |
| **Movement Speed** | $305.0$ | $+0.0$ |
| **Attack Range** | $250.0$ | $+0.0$ |
| **Base Attack Speed** | $0.625$ | |
| **Attack Speed Ratio** | $0.625$ | |
| **Bonus AS per Level** | $3.5\%$ | |
| **Attack Windup** | $17.5\%$ | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $100$ units | |
| **Selection Height** | $85$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $105.0\%$ |
| **Damage Taken** | $100.0\%$ |

#### URF

| Metric | Value |
|--------|------:|
| **Damage Taken** | $90.0\%$ |

## Pets

### Skaarl

| Attribute | Value |
|-----------|------:|
| **Move Speed** | 335 / 1000 (with bonus movement speed, estimated) |
| **Control** | Autonomous |
| **Targeting** | Untargetable |

---

## Abilities

### Passive: Dismounted Skaarl the Cowardly Lizard

| Attribute | Value |
|-----------|------:|
| **Effect Radius** | 1200 (Enemy champion radius check for granting the movement speed) / cr 1400 (Enemy champion radius check for granting the resistances) units |
| **Targeting** | Passive |
| **Affects** | Self |
| **Parry** | True |

**INNATE:** **Kled** is only vulnerable to death while **DISMOUNTED**. **Kled** has (health) Kled*(x-1)*(0.7025+0.0175*(x-1)) **base** health, which is not improved by sources of **bonus** health, as well as reduced **base** movement speed and increased (attack range) **base** attack range. Effects based on percentage health consider **Kled** and **Skaarl** combined **maximum** health, meaning that a **DISMOUNTED** **Kled** is never considered to be at *full* health.

**Kled** gains 70 to 155 (ms) **bonus** movement speed while facing and after attacking (see notes) nearby visible enemy champions, but his basic attacks against them are reduced to 85%@1; 90%@6; 95%@11; 100%@16 AD physical damage. He also gains **bonus** armor and **bonus** magic resistance equal to 4 (+ 1% of **bonus** health), increased by 30% for each nearby enemy champion, up to a maximum of 10 (+ 2.5% **bonus** health) resistances. Additionally, he gains the ability to restore **Skaarl** Courage to *mount* again.

**INNATE - COURAGE:** **Kled** gains 15 Courage after completing a basic attack against a champion and 5 Courage for minion kills and when basic attacking structures or epic monsters. He can also restore Courage from hitting enemies with *Pocket Pistol*. Upon reaching 100 Courage, **Skaarl** instantly restores 40%–70%@1–16 of her **maximum** health and reunites with **Kled** over $0.25$ seconds, after which he becomes **MOUNTED** and the duo become unable to declare basic attacks or cast their abilities for $0.25$ seconds. *Mounting up* resets the cooldowns of *Bear Trap on a Rope* and *Jousting*.

While at the allied fountain, 25 Courage is generated every $0.25$ seconds. If reuniting outside of the fountain, **Skaarl** gets *Frayed Nerves*, which prevents **Kled** from gaining Courage for 30 seconds. Being within the fountain while **DISMOUNTED** removes *Frayed Nerves*.

*The current status of **Kled**’s own health is preserved between subsequent dismounts.*

**Notes:**

- Clones are counted as champions for all of this passive's effects.
- **Kled** will retain the **bonus** movement speed buff for $1.5$ seconds while facing an enemy champion even if that champion is no longer visible.
- After completing a basic attack on a champion, the **bonus** movement speed buff is granted for $1.5$ seconds regardless of his facing direction.
- The **bonus** movement speed buff's value is refreshed when **Kled** levels up.
- **Kled** will neither gain Courage nor **bonus** movement speed if his basic attack is dodged, blocked, missed or parried.
- It is possible to get the Courage from the fountain but have **Kled** and **Skaarl** reunite outside the fountain.
- Sources of increased **maximum** health that additionally restore **current** health will restore **Kled**’s **current** health even if he is **DISMOUNTED**.
- The Courage bar is **white** when under 50 courage, **yellow** when equal to or above 50 courage, and **red** at 100 courage.

---

### Passive: Skaarl the Cowardly Lizard

| Attribute | Value |
|-----------|------:|
| **Speed** | 400 units/second |
| **Targeting** | Passive |
| **Affects** | Self |
| **lunge range** | 200 (estimated) |

**INNATE:** **Kled** rides his semi-trusty mount, **Skaarl**. While **MOUNTED**, all damage dealt to the duo is suffered by **Skaarl**, who has 400 / 1400 **base** health; additional sources of health are applied to **Skaarl** **maximum** health. Effects based on percentage health consider **Kled** and **Skaarl** combined **maximum** health.

Being reduced below (health) 0 health causes **Skaarl** to flee, forcing **Kled** to be **DISMOUNTED**, with all damage in excess of **Skaarl** health being ignored. Upon dismounting, **Kled** lunges in the direction of the allied Nexus, though not through terrain, while cleansing himself of all crowd control and becoming unable to act, untargetable, and immune to crowd control for $0.5$ seconds. *Dismounting* resets the cooldown and charges of *Pocket Pistol*.

While **Kled** is **DISMOUNTED**, his **base** health and **base** movement speed are reduced; however, he gains **bonus** movement speed while moving toward enemy champions. Additionally, his (attack range) **base** attack range is increased and his attacks deal 85%@1; 90%@6; 95%@11; 100%@16 damage to champions. *Bear Trap on a Rope* is replaced with *Pocket Pistol*, while *Jousting* and *Chaaaaaaaarge!!!* become unusable.

While **MOUNTED**, healing and health regeneration will first replenish **Skaarl** health and will only apply to **Kled**’s health while **Skaarl** is at *full* health.

**Notes:**

- **Kled** and **Skaarl** health bar is split in two segments, representing each of them respectively.
  - While **MOUNTED**, **Kled**’s health is visually rounded up while **Skaarl** health is rounded down.
  - While **DISMOUNTED**, **Kled**’s health is visually rounded down while **Skaarl** health is rounded up.
- Excess damage dealt to **Skaarl** counts for the purposes of effects based on damage dealt (eg. Death's Dance Ignore Pain, Stormsurge Stormraider, item damage trackers).
- If **Skaarl** health is reduced below 0 by an attack or ability with multiple damage instances, subsequent damage instances from the same cast instance are reduced to 0 damage.
- The Collector Death is special cased to not affect **Kled** while he is **MOUNTED** regardless of the duo's combined **current** health.
- If **Skaarl** health is reduced below 0 by Eclipse Ever Rising Moon, **Kled** will take the triggering attack's damage.
- While untargetable, **Kled** rapidly destroys enemy projectiles targeting him.
- **Kled** does not receive damage from Illaoi’s Test of Spirit, or Kayn’s Umbral Trespass while dismounting.
- Dismounting removes Speed Shrine's movement speed buff from **Kled** and prevent him from regaining it for the next 5 seconds, even if he remounts or enters the shrine again.
- The lunge can pass through Yorick’s Dark Procession.
- Buying items while **Kled** and **Skaarl** combined **current** health is lower than **Kled**’s **maximum** health will cause **Kled** to dismount upon undoing the purchase and potentially lose health.
- Dismounting with a spell shield sometimes consumes the spell shield and negates the effects of dismounting.
- The following table refers for interactions while **Kled** is lunging:

---

### Q: Bear Trap on a Rope

| Attribute | Value |
|-----------|------:|
| **Range** | 800 / 60 (Center-to-edge radius check for enemies within the projectile's origin point, prioritizing the closest unit that can be affected by the tether.) units |
| **Cast Time** | $0.25$ seconds |
| **Tether Radius** | 625 (Minimum radius) units |
| **Width** | 90 units |
| **Speed** | 1600 units/second |
| **Cooldown** | 11 / 10 / 9 / 8 / 7 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | Special |
| **Projectile** | True |

**ACTIVE:** **Kled** throws a bear trap in the target direction that deals physical damage to enemies hit, increased to 150% against minions, small pets, and small monsters. The bear trap collides with the first enemy champion, large pet, or large monster hit, revealing them for 2.5 seconds, and forming a tether between **Kled** and the target for $1.75$ seconds.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 30 / 55 / 80 / 105 / 130 (+ 60% **bonus** AD) |
| **Minion and Small Monster Damage** | 45 / 82.5 / 120 / 157.5 / 195 (+ 90% **bonus** AD) |

The tether's radius shrinks over its duration, and if it is not broken before then, **Kled** pulls the target 150 (Estimated) units toward him, deals physical damage and slows them for $2.5$ seconds.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 60 / 110 / 160 / 210 / 260 (+ 120% **bonus** AD) |
| **Total Physical Damage** | 90 / 165 / 240 / 315 / 390 (+ 180% **bonus** AD) |

| Attribute | Value |
|-----------|------:|
| **Slow** | 30 / 35 / 40 / 45 / 50% |

*Bear Trap on a Rope can be cast while Jousting.*

**Notes:**

- Applies spell damage to enemy champions, large monsters and large pets, while applying area damage to all other enemy units. Effect at cast time end
  - The visual effect of *Bear Trap on a Rope* will appear at **Kled**’s height at the end of cast time and may not portray its projectile accurately if **Kled** is *dismounting* at the end of cast time.
    - Is the same true if **Kled** is airborne?
- In addition to its projectile, *Bear Trap on a Rope* will check for enemy champions, large monsters and large pets within a 60-unit center-to-edge radius area from the projectile's origin point.
  - The projectile interacts with projectile-blocking effects but the area check doesn't.
- Tibbers, Daisy and The Maiden are considered large pets.
  - All other pets are considered small pets for the purposes of this ability.
- *Bear Trap on a Rope* will not pull the target if they are in stasis.
- The tether cannot be broken within the first $0.25$ (estimated) seconds of its application.
- Rek'Sai’s Void Rush will break the tether upon the beginning of its cast time.
- The pull damage is updated dynamically based on **Kled**’s' current attack damage.
- Kled has no sound cue for when the tether is broken.
- The damage is also increased to 150% against Neeko disguised as a ward.

---

### Q: Pocket Pistol

| Attribute | Value |
|-----------|------:|
| **Range** | 700 / 55 (Center-to-edge radius check for enemies within the projectiles' origin point.) units |
| **Cast Time** | $0.25$ seconds |
| **Angle** | 20°, -5°, 0°, 5° and 10° from the cast direction |
| **Width** | 80 units |
| **Speed** | 3000 (projectile speed) / 1025 (dash speed, estimated) units/second |
| **Cost** | 1 Charge |
| **Cooldown** | 3 seconds |
| **Recharge** | 18 / 16 / 14 / 12 / 10 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | Special |
| **Projectile** | True |
| **Grounded** | Special |
| **Knockdown** | True |

**ACTIVE:** **Kled** sprays a cone of five *pellets* in the target direction while recoiling 300 units in the opposite direction, dealing physical damage to enemies hit.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 35 / 50 / 65 / 80 / 95 (+ 65% **bonus** AD) |

**Kled** periodically stocks a *Pocket Pistol* charge, up to a maximum of 2. He will restore both charges after *dismounting*.

*Pellets* collide with the first enemy champion they hit, and deal 20% damage per *pellet* beyond the first. Each *pellet* that hits an enemy champion or epic monster restores *Courage*, by 5 and $2.5$ respectively, for up to a total restore of 25 against champions and $12.5$ versus epic monsters.

| Attribute | Value |
|-----------|------:|
| **Reduced Damage** | 7 / 10 / 13 / 16 / 19 (+ 13% **bonus** AD) |
| **Maximum Damage** | 63 / 90 / 117 / 144 / 171 (+ 117% **bonus** AD) |

**Notes:**

- Applies spell damage to champions and area damage to non-champions.
- The backwards dash from *Pocket Pistol* can pass terrain.
- *Pocket Pistol* can be cast while grounded or rooted but **Kled** will not dash backwards. Effect at cast time end
- In addition to its projectiles, *Pocket Pistol* will check for enemy champions within a 55-unit center-to-edge radius area from the projectiles' origin point.
  - The projectiles interact with projectile-blocking effects but the area check doesn't.
- Black Cleaver will specifically apply a stack for every pellet hit.
- Spell shield will negate multiple pellets if they hit at the same time.
  - Even if the first pellet is negated, subsequent non-negated pellets will still only deal 20% of the damage.
- Pellets will restore Courage when colliding with units protected by spell shield, but will not restore Courage when colliding with Fiora’s Riposte.
- *Pocket Pistol* will have no charges if **Kled** dismounts and triggers Transcendence at the same time.
- *Pocket Pistol* will falsely visually appear to have another charge ready if the end of its cast time coincides with a charge being stocked.

---

### W: Violent Tendencies

| Attribute | Value |
|-----------|------:|
| **Cooldown** | 13 / 12 / 11 / 10 / 9 (Starts after the attacks time out or are consumed) seconds |
| **Cooldown Start** | post-effect |
| **Targeting** | Passive |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | False |
| **Spell Effects** | Special |
| **Parry** | True |

**PASSIVE:** **Kled** periodically gains *Violent Tendencies*, which starts expiring upon his next basic attack.

**VIOLENT TENDENCIES:** **Kled** gains (attack speed) 150% **bonus** attack speed on his next 4 basic attacks within 4 seconds. The fourth attack deals **bonus** physical damage. The damage based on the target's health is capped at 200 against monsters.

*Violent Tendencies' * **current** cooldown is reduced by $0.5$ seconds on-hit, increased to $1.5$ against champions.

| Attribute | Value |
|-----------|------:|
| **Bonus Physical Damage** | 20 / 30 / 40 / 50 / 60 (+ 4.5 / 5 / 5.5 / 6 / 6.5% (+ 2% per 100 **bonus** AD) (+ $0.4$% per 100 **bonus** health) of target's **maximum** health) |

Against structures, the fourth attack deals modified **bonus** physical damage.

| Attribute | Value |
|-----------|------:|
| **Structure Bonus Damage** | 62 / 91.75 (+ 0 / 15 / 30 / 45 / 60) |

**Notes:**

- *Violent Tendencies** bonus damage is applied as a separate damage instance from the attack that triggers it.
  - This causes Black Cleaver Carve to be applied twice.
- *Violent Tendencies** bonus damage is tagged as basic damage and spell damage, which allows it to trigger spell effects.
- Attacking a plant or ward will not activate *Violent Tendencies*.
  - Attacking either while *Violent Tendencies* is already active will still consume an attack.
- While affected by Silence or Forced Actions, the animations and sounds of **Kled**’s first 3 *Violent Tendencies* attacks are replaced with default ones.
  - The first and third attack will instead play critical strike animations if they strike critically.
- Learning *Violent Tendencies* will cancel **Kled**’s ongoing attack windup.
  - In contrast, *Violent Tendencies* coming off cooldown during an ongoing attack windup will not cancel it.
    - Successfully finishing the basic attack will then cause *Violent Tendencies* to begin expiring, but will not count as one of the 4 attacks.

---

### E: Jousting

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 550 (first dash) / cr 765 (second dash, estimated) units |
| **Cooldown** | 13 / 12 / 11 / 10 / 9 (Starts after the first dash ends, see notes) seconds |
| **Cooldown Start** | Special |
| **Targeting** | Direction / Auto |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | aoe |
| **Grounded** | True |
| **Knockdown** | True |

**ACTIVE:** **Skaarl** dashes in the target direction, though not through terrain, dealing physical damage to enemies he passes through and pulling minions, pets and small monsters hit towards them.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 35 / 60 / 85 / 110 / 135 (+ 55% **bonus** AD) |

Upon hitting an enemy champion or large monster, **Skaarl** dashes a fixed 200 (estimated) units through them, marking them for 3 seconds, during which they are revealed. After the dash ends, the duo gain (ms) 50% **bonus** movement speed for 1 second. *Jousting* can be recast after $0.5$ seconds of the first dash ending while the target is marked.

**RECAST:** **Skaarl** consumes the mark to dash through the marked target, applying the same effects as the first cast though being able to dash through terrain.

| Attribute | Value |
|-----------|------:|
| **Total Physical Damage** | 70 / 120 / 170 / 220 / 270 (+ 110% **bonus** AD) |

The mark will remain even if the target dies, and ends prematurely if the duo are too far away or **Kled** *dismounts*.

*Bear Trap on a Rope and Chaaaaaaaarge!!! can be cast during either of the dashes.*

**Notes:**

- *Jousting* will go on cooldown at the end of its first dash, but its cooldown cannot be reduced by cooldown refunding effects such as Transcendence while its mark is active.
- Both casts count as ability activations for the purposes of on-cast effects such as Spellblade and triggering Force Pulse’s passive.
- Casting Chaaaaaaaarge!!! during *Jousting* will not interrupt the dash.
- The mark is removed at 765 range center-to-edge, which allows **Kled** to attempt to cast *Jousting* while outside of maximum range.
  - Attempting to recast *Jousting* while outside of maximum range will make **Kled** path 550 units in the direction of the mark and consume the mark without casting the ability.
- When recasting *Jousting*, **Skaarl** will Dash to the marked target's location at the time of recasting and will then Dash another 200 (estimated) units.
  - The 200 (estimated) unit dash cannot pass through terrain.
  - If the marked target's location at the time of recasting happens to be inside terrain, *Jousting* will end upon reaching terrain.
- The first dash can enter terrain within a short radius.
  - This allows **Kled** to sometimes pass through Yorick’s Dark Procession with the first cast of *Jousting*.
  - the first dash's terrain forgiveness radius
- *Jousting* can be recast even while the target is untargetable or dead.
- *Jousting*’s hitbox
- The following table refers for interactions while **Kled** is dashing:

---

### E: Dismounted Jousting

| Attribute | Value |
|-----------|------:|
| **Targeting** | N/A |

This ability is unusable while *dismounted*.

**Notes:**

- No additional details.

---

### R: Chaaaaaaaarge!!!

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 3500 / 3750 / 4000 / 4250 / 4500 units |
| **Speed** | 1000 (dash speed) / 600 (enemy knockback speed) units/second |
| **Cooldown** | 140 / 132.5 / 125 / 117.5 / 110 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | location |
| **Affects** | Enemies |
| **Damage Type** | magic |
| **Spell Shield** | True |
| **Spell Effects** | spell |
| **Out of Range Behavior** | walk to location |
| **Grounded** | True |
| **Knockdown** | Special |
| **minimum target range** | 500 (estimated) |
| **search range** | er 500 (enemy champions (estimated)) / er 800 (visible enemy champions (estimated)) |

**ACTIVE:** **Kled** & **Skaarl** charge toward the target location, automatically navigating terrain along the way, during which they are ghosted, immune to crowd control and revealed to enemy champions in a er 1250 (estimated) radius. If the charge does not complete within 15 seconds, it will end prematurely. The charge is interrupted instantly if **Kled** *dismounts* during it.

While charging, the duo gain a shield for every $0.25$ seconds of traveling, up to 10% / 20% / 30% / 40% / 50% / 60% / 70% / 80% / 90% / 100% of the **maximum** shield amount over $2.25$ seconds. The shield lasts for $1.75 – 2$ (random) seconds upon finishing the charge. Additionally, the duo trails a directional draft in their wake, lasting 9 seconds. Allied units other than lane minions and Voidmite following the draft gain *Mr. Kled's Wild Ride*. After $0.5$ seconds of charging, the duo gain (ms) **bonus** movement speed every $0.25$ seconds, up to over 3 seconds.

| Attribute | Value |
|-----------|------:|
| **Minimum Shield** | 20 / 25 / 30 / 35 / 40 (+ 30% **bonus** AD) |
| **Maximum Shield** | 200 / 250 / 300 / 350 / 400 (+ 300% **bonus** AD) |

**MR. KLED'S WILD RIDE:** While in champion combat, gain (ms) 40% (estimated) **bonus** movement speed. After not being in champion combat for 3 seconds, if **Kled** is charging, gain (ms) **bonus** movement speed up to **Kled's current** movement speed; otherwise gain up to .

**Kled** & **Skaarl** will dash toward the first visible enemy champion in range of the charge (see notes), revealing the target during the dash and colliding with the first visible enemy champion in their path to deal magic damage, increased by 0%–200%@0–4 (@=charge time), knocking them back 150 (Estimated) units and ending the charge.

| Attribute | Value |
|-----------|------:|
| **Minimum Magic Damage** | 4 / 5 / 6 / 7 / 8% (+ 3% per 100 **bonus** AD) of target's **maximum** health |
| **Maximum Magic Damage** | 12 / 15 / 18 / 21 / 24% (+ 9% per 100 **bonus** AD) of target's **maximum** health |

**Notes:**

- During the charge the duo gain (950(**starting** movement speed (movement speed at the beginning of the charge; after movement speed soft caps)))12×(time$0.25$) **bonus** movement speed
  - The bonus movement speed is not affected by movement speed caps.
  - The charge will not give any movement speed to the duo if they have more than 「 1440 raw movement speed ⟷ 950 after movement speed soft caps 」.
- The trail will not give **Kled** and his allies any movement speed out-of-champion-combat if they have more than 「 840 raw movement speed ⟷ 650 after movement speed soft caps 」.
- Movement speed is granted by facing the direction of the draft and lasts for upon exiting the trail or facing another direction.
- **Kled** will re-path if his current path is blocked by a newly created terrain (either player created or by the Mountain Drake).
- The charge will continue towards the target location even if **Kled**’s location changes drastically.
- **Kled** will mark his path with an indicator visible to him and his allies.
  - **Kled**’s pathing indicator will not account for changes in **Kled**’s pathing.
    - **Kled**’s pathing indicator will sometimes display a path different from his real path.
- Axiom Arcanist only increases the first instance of shield gained.
- While charging, if there are enemy champions within 500 (estimated) range in **Kled**’s facing direction, **Kled** will lock onto the closest visible enemy champion within 800 (estimated) range in his facing direction and begin dashing after a brief delay (0.05 to 0.11, estimated).
  - If the charge ends before (because of the delay before the dash begins), or during the dash (because of the charge ending after 15 seconds), the dash will not:
    - Generate a shield;
    - Trail a directional draft, applying *Mr. Kled's Wild Ride*;
    - Disable summoner spells and items.
- Zeke's Convergence Frostfire Tempest and Experimental Hexplate Overdrive trigger at the end of the charge or at the end of the dash, whichever comes last, even if the charge ends before or during the dash.
- **Kled** and **Skaarl** will track the target if they change locations.
  - They will dash to the target's previous location if the target is too far away or moves beyond 2500 (Estimated) units of the dash's beginning location.
  - If the target moves 2500 (Estimated) or more units away from **Kled** and **Skaarl** during the dash, it will be interrupted.
  - If the dash does not complete within 9 seconds, **Kled** and **Skaarl** will continue to track their target and be able to cast spells, but will not be able to collide with enemy champions.
- While dashing, **Kled** will mark his target with an indicator visible to both teams.
- The damage dealt is based on time spent traveling before the beginning of the dash.
- It is possible for **Kled** to briefly lose his crowd control immunity while colliding with an enemy champion, which can cause the dash, and its subsequent damage and displacement to be interrupted.
- *Chaaaaaaaarge!!!*’s knockback distance and speed may be impeded by terrain.
- The airborne debuff is removed early when the forced movement stops.
- The following table refers for interactions while **Kled** is charging:
  - Ghost is also disabled, contrary to the table below (technical limitation (hopefully temporary)).
- The following table refers for interactions while **Kled** is dashing:
- Every summoner spell is disabled, but summoner spells that aren't disabled while **Kled** is charging will not visually appear disabled.
- No summoner spells or items are disabled if the charge ends before or during the dash. **Buffering Interactions**
- Nothing can be buffered during the charge.
- Movement and attack commands, as well as Bear Trap on a Rope and Jousting can be buffered during the dash.

---

### R: Dismounted Chaaaaaaaarge!!!

| Attribute | Value |
|-----------|------:|
| **Targeting** | N/A |

This ability is unusable while *dismounted*.

**Notes:**

- No additional details.

---

## Patch History

### V25.16#August 13th Hotfix|V25.16
- Bear Trap on a Rope
  - **Bug Fixes:** No longer uses Pocket Pistol’s cooldown after being cast for the first time after remounting in the fountain.

### V25.16
- Bear Trap on a Rope
  - **UNDOCUMENTED / BUG FIX:** No longer gains two charges after **Kled** remounts via the fountain.

### V25.15
- Skaarl the Cowardly Lizard
  - **New Effect:** While dismounted, **Kled** now also gains 4 **bonus** armor and magic resistance, increased for each nearby enemy champion.
  - Bonus resistances per nearby enemy increased to 30% from 20%.
    - Maximum bonus resistances increased to 250% of the minimum from 200%.
  - **Bug Fixes:** Refunding an item during the remount sequence no longer causes **Kled** to perform the remount sequence but without actually switching the player to the mounted ability set.
- Bear Trap on a Rope
  - **UNDOCUMENTED / BUG FIX:** Is no longer able to damage minions, small pets, and small monsters separately with its projectile and area check, resulting in double the intended damage in such case.

### V25.14
- General
  - While out of combat, **Skaarl** will now tilt its head to look at nearby visible jungle monsters.
  - Resolved an error that sometimes caused players to have reduced performance when looking at **Kled** after he has re-mounted.
- Skaarl the Cowardly Lizard
  - **New Effect:** Remount health is now granted instantly upon beginning to remount, rather than after $0.5$ seconds.
  - **New Effect:** **Kled** now gains **bonus** armor and **bonus** magic resistance equal to 1% of **bonus** health, increased by 20% per nearby enemy champion within cr 1400 units, up to a maximum of 2% **bonus** health.
  - Courage from minion kills increased to 5 from 4.
  - **Removed:*** Champion takedowns within 3 seconds of damaging them no longer grants 20 Courage.
  - Dismounted attack damage penalty reduced to 15%@1; 10%@6; 5%@11; 0%@16 from 20% at all levels.
  - Dismounted movement speed penalty reduced to 40 from 60.
  - Dismounted bonus movement speed toward champions reduced to 70 to 155 from 100 to 185.
  - Dismounted bonus movement speed range increased to 1200 units from 1000.
  - **Skaarl** base health reduced to 400 / 1400 from 400 to 1550. *Now uses stat growth instead of linear growth.*
  - Remount health reduced to 40%–70%@1–16 of **Skaarl's maximum** health from 45%–75%@1–16.
  - **New Effect:** Granted bonus movement speed while dismounted now updates on level-up.
  - **Bug Fixes:** Additional instances of damage applied by a spell that causes the dismount are now properly ignored upon dismounting instead of unintentionally affecting **Kled**.
  - **Bug Fixes:** Dismount dash and its accompanying lockout are now exactly $0.5$ seconds instead of lasting up to $0.25$ seconds more based on engine ticks (they are no longer "fuzzy").
  - **Bug Fixes:** Allied projectiles are no longer destroyed by the untargetability while dismounting.
  - **Bug Fixes:** Courage bar now always turns yellow when equal to or above 50 Courage instead of only when at 50–59 Courage.
  - **Bug Fixes:** Kled's health no longer sometimes shows an incorrect amount after remounting.
  - **Bug Fixes:** Taking fatal damage while affected by Renata Glasc’s Bailout no longer breaks **Kled**’s health bar for the rest of the game.
  - **Bug Fixes:** **Skaarl**’s health is no longer unintentionally displayed while only **Kled** is affected by the protection of Renata Glasc’s Bailout after taking fatal damage.
  - **Bug Fixes:** Lulu’s Wild Growth no longer grants **Kled** more health than the intended health bonus based on **Skaarl**’s base health.
  - **Bug Fixes:** Health regeneration is now properly calculated upon remounting and no longer uses incorrect numbers for up to $0.25$ seconds after remounting (it is no longer "fuzzy").
  - **Bug Fixes:** No longer unintentionally loses a minuscule amount of attack damage while dismounted.
- Bear Trap on a Rope
  - Tether bonus AD ratio reduced to 60% **bonus** AD from 65%.
  - **Removed:*** No longer applies Grievous Wounds for 5 seconds after successfully pulling the target.
  - Slow reduced to 30 / 35 / 40 / 45 / 50% from 40 / 45 / 50 / 55 / 60%.
  - Slow duration increased to $2.5$ seconds from $1.5$.
  - **New Effect:** Damage calculations now update again for the pull instead of using the values at the time of the tether connecting.
  - **New Effect:** Now checks for enemy units within a 60-unit radius (center-to-edge) from the projectile's origin point, prioritizing the closest unit that can be affected by the tether.
  - **Bug Fixes:** Pull timer is now exactly $1.75$ seconds instead of lasting up to $0.25$ seconds more based on engine ticks (it is no longer "fuzzy").
    - *[Note: This was documented as fixed in 14.2, but no actual change was made.]*
- Pocket Pistol
  - Recharge timer changed to 18 / 16 / 14 / 12 / 10 seconds from 20 to 7.25.
  - Bonus AD ratio reduced to 65% **bonus** AD from 80%.
  - **New Effect:** Now checks for enemy champions within a 55-unit radius (center-to-edge) from the projectiles' origin point.
  - **New Effect:** Now deals spell damage instead of area damage against champion targets.
- Violent Tendencies
  - Cooldown increased to 13 / 12 / 11 / 10 / 9 seconds from 11 / 9.5 / 8 / 6.5 / 5.
  - **New Effect:** Basic attacks now reduces its cooldown by $1.5$ seconds, reduced to $0.5$ against non-champions.
  - Bonus AD ratio reduced to 2% per 100 **bonus** AD from 5%.
  - **New Effect:** Now scales with $0.4$% per 100 **bonus** health.
  - **New Effect:** Attacking wards now maintains bonus attack speed rather than consuming the effect.
  - **Bug Fixes:** While dismounted, the fourth attack is no longer invalidated by silence and forced action effects.
- Jousting
  - Bonus AD ratio reduced to 55% **bonus** AD from 65%.
  - **New Effect:** Recast timer is now tracked on the ability's icon in the HUD.
  - **New Effect:** Recast now has a range indicator.
  - **Bug Fixes:** No longer is able to hit units twice when they re-enter its area check during the dash.
- Chaaaaaaaarge!!!
  - Damage type changed to magic from physical.
  - Minimum base damage increased to 4 / 6 / 8% of target's **maximum** health from 4 / 5 / 6%.
    - Maximum base damage increased to 12 / 18 / 24% of target's **maximum** health from 12 / 15 / 18%
  - Minimum damage bonus AD ratio reduced to 3% per 100 **bonus** AD from 4%.
    - Maximum damage bonus AD ratio reduced to 9% per 100 **bonus** AD from 12%.
  - Increased the cadence of checking for valid targets.
  - **New Effect:** Pinging the ability now shows targets within range in chat.
  - **Bug Fixes:** CC-immunity is no longer unintentionally removed at the end point of the charge shortly before the dash to a target begins.
  - **Bug Fixes:** No longer causes **Kled** to sometimes lose Elixir of Iron trail.
  - Rescripted the check for valid targets at the start of cast.
    - *[Note: This was intended to prevent failure to detect valid targets in range at the start of the charge, but did not.]*

### V25.11
- Skaarl the Cowardly Lizard
  - **Bug Fixes:** No longer causes Kled to unintentionally instantly die when a damage instance that triggers The Collector Death execute on Skaarl exceeds Kled's current health.

### V14.9
- Violent Tendencies
  - **New Effect:** Now also does spell damage in addition to basic damage.

### V14.4
- Bear Trap on a Rope
  - Tether update cadence for valid range conditions has been updated to check every $0.05$ seconds instead of every $0.25$ seconds.

### V13.23
- **Undocumented:** Added a new voice line.

### V13.13
- Chaaaaaaaarge!!!
  - **Bug Fixes:** Can now properly cast an auto-targeted Heal during the charge.

### V13.12
- Chaaaaaaaarge!!!
  - **Bug Fixes:** Attempting to cast Primal Smite on a target outside of its range no longer causes **Kled** to redirect the path and charge towards the target enemy.

## Trivia

- This champion has no ability power ratio.
- Kled and **Skaarl** are the first champion with two health bars, combining to form the highest in-game base health, with Kled's being the lowest.
  - Kled is also the first champion and one of the two champions in the game whose health cannot be improved except through growth per level, the second being Pyke.
- Kled, while dismounted, has the lowest base movement speed of all champions at 305.
- When using the joke emote, Kled will randomly play one of two joke animations.
- Mr. Kled's Wild Ride is most likely a reference to the Disneyland attraction from the animated film , originally adapted from the children's novel by Kenneth Grahame.
  - Kled and **Skaarl**, as and respectively.
    - J. Thaddeus Toad And Cyril Proudbottom
- Mr. Kled's Wild Ride is a probable reference to "MR BONES WILD RIDE", a custom roller coaster built in RollerCoaster Tycoon 2 called that took in game years to complete its course. Then, once a passenger finally got off the ride, it led back to the start of the ride, creating an never-ending cycle of riding the attraction. One of the most famous quotes that a ton of riders thought was "I want to get off 'MR BONES WILD RIDE'".
- The name Skaarl the Cowardly Lizard may be a reference to the *Courage the Cowardly Dog* Cartoon Network cartoon.
- Kled is the first champion to swear (albeit censored) in-game.
- Kled is the only champion to celebrate takedowns with a unique animation.
- Kled's dismounted dance references the Moonwalk_(dance), a famous dance move.
  - A side-by-side comparison can be seen here.
- The song used in the short cinematic The Reunion is Edvard Grieg, Peer Gynt - Morning Mood.
- While out of combat, **Skaarl** will tilt her head to look at nearby visible jungle monsters.

---
*This page was automatically generated from League of Legends Wiki data.*