# Camille

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
| **Champion** | Camille |
| **Title** | the Steel Shadow |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2016-12-07 |
| **Release Patch** | V6.24 |
| **Latest Changes** | V14.21 |
| **Roles** | Diver |
| **Riot Positions** | Top |
| **External Positions** | Top, Support |
| **Blue Essence** | 2400 |
| **Riot Points** | 880 |
| **Difficulty** | 2 |
| **Hero Type** | Fighter |
| **Alt Type** | Assassin |
| **Adaptive Type** | Physical |
| **Damage** | 3 |
| **Toughness** | 2 |
| **Control** | 2 |
| **Mobility** | 3 |
| **Utility** | 1 |
| **Style** | 40 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $650.0$ | $+99.0$ |
| **Mana** | $339.0$ | $+52.0$ |
| **Health Regen** | $8.5$ | $+0.8$ |
| **Mana Regen** | $8.15$ | $+0.75$ |
| **Armor** | $35.0$ | $+5.0$ |
| **Magic Resist** | $32.0$ | $+2.05$ |
| **Attack Damage** | $68.0$ | $+3.8$ |
| **Attack Speed** | $0.644$ | |
| **Movement Speed** | $340.0$ | $+0.0$ |
| **Attack Range** | $125.0$ | $+0.0$ |
| **Base Attack Speed** | $0.644$ | |
| **Attack Speed Ratio** | $0.644$ | |
| **Bonus AS per Level** | $2.5\%$ | |
| **Attack Windup** | $19.3\%$ | |
| **Acquisition Radius** | $550$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $100$ units | |
| **Selection Height** | $280$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $105.0\%$ |
| **Damage Taken** | $95.0\%$ |
| **Healing** | $120.0\%$ |

## Abilities

### Passive: Adaptive Defenses

| Attribute | Value |
|-----------|------:|
| **Static Cooldown** | 18–10@1–13 |
| **Targeting** | Passive |
| **Affects** | Self |

**INNATE:** Periodically, **Camille**’s next basic attack on-hit against an enemy champion grants her a shield equal to 20% of her **maximum** health, lasting for 2 seconds and absorbing damage from either exclusively physical damage or magic damage, based on which type the target has previously dealt most of against champions as well as on **Camille**’s **current** armor and magic resistance.

**Notes:**

- *Adaptive Defenses* goes on cooldown after the shield expires or is destroyed.
- The shield-typing is determined based on the **total** damage the target has dealt to champions that game (as a percentage) versus **Camille**’s **current** armor and magic resistance.
  - For example, with little **bonus** resistances **Camille** would likely gain the most obvious shield-typing - i.e. physical shield versus champions primarily dealing physical damage and magic shield versus champions primarily dealing magic damage. However, if she has a sufficient amount of armor against a champion dealing both alternates of physical damage and magic damage, she will receive a magic shield as the physical damage threat is already being mitigated by her armor, and vice versa.
- The type of shield that will be granted to **Camille** when she attacks any one enemy champion is shown as an icon beneath their health bar: orange for physical shield and blue for magic shield.
- *Adaptive Defenses* takes priority over all other shields except Black Shield.
- *Adaptive Defenses* Will not grant **Camille** the shield is the attack is dodged, blocked, or if she is blinded.

---

### Q: Precision Protocol

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Cost** | 25 Mana |
| **Cooldown** | 9 / 8 / 7 / 6 / 5 (Starts on first attack) seconds |
| **Cooldown Start** | Special |
| **Targeting** | Auto |
| **Affects** | Self |
| **Damage Type** | Physical True |
| **Spell Shield** | False |
| **Spell Effects** | spell |
| **Parry** | True |

**ACTIVE:** **Camille** empowers her next basic attack within 4 seconds to have an uncancellable windup, gain (range) 50 **bonus** range, deal **bonus** physical damage and grant her (movement speed) **bonus** movement speed for 1 second. This attack cannot critically strike.

| Attribute | Value |
|-----------|------:|
| **Bonus Physical Damage** | 20 / 25 / 30 / 35 / 40% AD |

| Attribute | Value |
|-----------|------:|
| **Bonus Movement Speed** | 25 / 30 / 35 / 40 / 45% |

After $0.25$ seconds, *Precision Protocol* can then be recast within the next $3.5$ seconds at no additional cost.

**RECAST:** **Camille** mimics the first cast's effects. If *Precision Protocol* is recast after $1.5$ seconds of the first attack, then the **bonus** damage is doubled, and 40% / 44% / 48% / 52% / 56% / 60% / 64% / 68% / 72% / 76% / 80% / 84% / 88% / 92% / 96% / 100% of the attack's **total** damage will be dealt as true damage.

| Attribute | Value |
|-----------|------:|
| **Increased Mixed Damage** | 40 / 50 / 60 / 70 / 80% AD |

*Both casts of Precision Protocol reset **Camille**’s basic attack timer. **Camille** is able to cast Tactical Sweep, Hookshot, or The Hextech Ultimatum during the windup of Precision Protocol's empowered attack without cancelling it.*

**Notes:**

- Both casts count as ability activations for the purposes of on-cast effects such as Spellblade and triggering Force Pulse’s passive.
- Both enhanced attacks **cannot** critically strike.
- The $1.5$-second delay is "fuzzy", meaning it may take up to another $0.25$ seconds to ready the window for the empowered attack.
  - This syncs up perfectly with Sheen cooldown between casts at 0 item haste, which is also "fuzzy".
- The second attack will also become empowered if it simply starts after the $1.5$-second delay, even if the recast was consumed earlier.
- The empowered **RECAST**'s true damage:
  - Has a special case to factor Spellblade into the true damage conversion of part of her attack's damage.
  - Other on-hit effects do not interact with the true damage conversion.
  - Randomly dependent on buff order, if (e.g. Plated Steelcaps Plating, W) applies first before the conversion, then the resulting damage will be reduced, otherwise if the conversion happens first, then the damage will not be successfully reduced (both effects are pre-mitigation on-take-damage events).
  - Is dealt just prior to the physical damage portion of the attack.
- *Precision Protocol* applies to structures but the recast does not convert to true damage.
- The enhanced attack will still complete and hit the target even if they become untargetable during the attack's windup.
- Warden's Mail flat damage reduction is reduced on the second cast, reducing the physical damage but not the true damage.

---

### W: Tactical Sweep

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 325 (Inner radius) / 650 (Outer radius) units |
| **Angle** | cr 70° |
| **Cost** | 50 / 55 / 60 / 65 / 70 Mana |
| **Cooldown** | 17 / 15.5 / 14 / 12.5 / 11 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | False |

**ACTIVE:** **Camille** sweeps her leg in a cone in the target direction over $1.1$ seconds, during which she is ghosted and unable to declare basic attacks. Afterwards, she deals physical damage to all enemies within.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 50 / 75 / 100 / 125 / 150 (+ 60% **bonus** AD) |

Enemies hit by the outer half of the cone take **additional** physical damage, capped at 300 against monsters, and are slowed by 80% decaying over 2 seconds. **Camille** is healed for 100% of this **additional** damage post-mitigation (After modifiers such as resistances) against enemy champions in the outer half.

| Attribute | Value |
|-----------|------:|
| **Outer Cone Bonus Damage** | 6 / 6.5 / 7 / 7.5 / 8% (+ $2.5$% per 100 **bonus** AD) of target's **maximum** health |

Non-epic monsters take 50% damage from *Tactical Sweep*.

| Attribute | Value |
|-----------|------:|
| **Non-Epic Monster Damage** | 25 / 37.5 / 50 / 62.5 / 75 (+ 30% **bonus** AD) |

| Attribute | Value |
|-----------|------:|
| **Bonus Non-Epic Monster Damage** | 3 / 3.25 / 3.5 / 3.75 / 4% (+ $1.25$% per 100 **bonus** AD) of target's **maximum** health |

*Other abilities can be cast during the animation without cancelling it.*

**Notes:**

- **Camille** can move during the delay, but the hitbox and **Camille**’s visuals are fixed to the initial target direction.
  - **Camille**’s facing-direction, for effects such as Petrifying Gaze, is the direction she is moving, and not the direction the model is facing.
- The 50% damage reduction against non-epic monsters applies after the cap. Because of this, the actual cap against non-epic monsters is 150 damage.
- Because the increased damage in the outer range of the cone is dealt as an **additional** instance of damage (as opposed to **bonus** damage to the original instance), it will trigger effects twice such as being reduced by Bone Plating an extra time.
  - It still belongs to the same cast instance, therefore it does not trigger additional stacks of Conqueror or Electrocute.

---

### E: Hookshot

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 800 units |
| **Width** | 100 units |
| **Speed** | 1400 units/second |
| **Cost** | 70 Mana |
| **Cooldown** | 16 / 15 / 14 / 13 / 12 seconds |
| **Targeting** | Direction |
| **Affects** | Terrain, Self |
| **Projectile** | True |
| **Grounded** | Special |
| **Knockdown** | True |

**ACTIVE:** **Camille** fires a grapple in the target direction. If the grapple collides with terrain, **Camille** will dash toward and attach to the terrain for $0.75$ seconds, during which she gains the ability to cast *Wall Dive*.

***Camille** will be knocked down by any immobilizing or polymorphing crowd control during the dash.*

*Other abilities can be cast during the ability. Casting The Hextech Ultimatum will cause the grapple to disappear if it is in flight.*

**Notes:**

- **Camille** may move while the grapple is in flight.
- *Hookshot* can be cast while rooted but not while grounded.
- **Camille** will grapple to terrain even if she is immobilized.
- *Hookshot* can interact with player-generated terrain.
- Sleep does not count for knocking the dash down.

---

### E: Wall Dive

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 400 (Basic) / 800 (Champion) units |
| **Effect Radius** | 130 units |
| **Speed** | 1050 + 100% movement speed |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Terrain Grace** | True |
| **Grounded** | Special |
| **Knockdown** | True |

**ACTIVE:** **Camille** dashes in the target direction, dealing physical damage to enemies near the landing location. *Wall Dive*’s range is doubled towards enemy champions within 1400 units. **Camille** grants ghosting to non-champions she passes through for 4 seconds.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 60 / 90 / 120 / 150 / 180 (+ 75% **bonus** AD) |

**Camille** stops prematurely upon colliding with an enemy champion, knocking back all nearby enemy champions, though not through terrain, as well as stunning them for $0.75$ seconds, while also gaining **bonus** attack speed for 5 seconds.

| Attribute | Value |
|-----------|------:|
| **Bonus Attack Speed** | 40 / 45 / 50 / 55 / 60% |

*Wall Dive may be also cast with movement or attack commands. Other abilities besides The Hextech Ultimatum can be cast while grappled or during the dash.*

**Notes:**

- *Wall Dive* count as an ability activation for the purposes of on-cast effects such as Spellblade and triggering Force Pulse’s passive.
- Despite what one might assume from a wall-jumping ability, *Wall Dive* **can** be used to go through terrain (including the terrain she is bound to).
- Casting Flash during *Wall Dive*’s dash will interrupt it but **Camille** will affect enemies around the new location.
- *Wall Dive* is disabled while grounded or rooted.
  - *Wall Dive* will not automatically cast from movement or attack commands in both cases. While grounded, however, **Camille** is able to move while bound. She may still dash away from her current location once the grounding finishes.
- **Camille** will only gain a range indicator towards each valid champion within the extended dash range (800), but will still gain the extended dash range without the indicator against enemies within the 1400 range.
  - She does not have to see any enemy champions to gain the range indicator or extended range.
- Displacement immunity will not resist the application of the stun.
- If Tactical Sweep is cast during *Wall Dive*’s dash on its opposite direction, the target wil be knockback further and to a different direction.
- The following table refers for interactions while **Camille** is bound to terrain:
  - When **Camille** is interrupted, she will automatically cast *Wall Dive*. The only exception is if she was interrupted by casting Flash or Recall, in which case *Wall Dive* cancels.

| Aspect | State / Notes |
|--------|---------------|
| **Attacking** | Initiates the dash towards attack target. |
| **Movement** | Initiates the dash towards the commanded location. |
| **Abilities** | Precision Protocol and Tactical Sweep are usable. The Hextech Ultimatum is disabled. |
| **Items** | Interrupted by: All items |
| **Summoner Spells** | Disabled: Barrier, Clarity, Cleanse, Exhaust, Ghost, Heal, Ignite, Smite, Teleport, Hexflash; Interrupted by: Flash, Recall |
| **Consumables** | Interrupts |

---

### R: The Hextech Ultimatum

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 475 units |
| **Effect Radius** | 425 (Estimated) units |
| **Cost** | 100 Mana |
| **Cooldown** | 140 / 127.5 / 115 / 102.5 / 90 seconds |
| **Targeting** | Unit |
| **Affects** | Enemies, Self |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Proc |
| **Grounded** | True |
| **Knockdown** | False |

**ACTIVE:** **Camille** becomes untargetable and leaps with displacement immunity towards the target enemy champion over $0.5$ seconds, revealing them for the duration and disrupting their ongoing channels.

Upon landing on the target, she becomes targetable again and creates a hexagonal zone around her current location for a duration, knocking away all other nearby enemies on impact, though not through terrain.

| Attribute | Value |
|-----------|------:|
| **Zone Duration** | 2.5 / 2.875 / 3.25 / 3.625 / 4 seconds |

The target cannot escape the zone through any means. While within the zone, **Camille**’s basic attacks against the target deal **bonus** magic damage.

| Attribute | Value |
|-----------|------:|
| **Bonus Magic Damage** | 4 / 5 / 6 / 7 / 8% of target's **current** health |

*The Hextech Ultimatum* ends automatically within 1 second if **Camille** leaves the area or dies.

**Notes:**

- The disrupt is 'nested' into a $0.4$-second Silence debuff on the target, but unlike normal silences, this debuff does not actually make the target *unable to cast*. Instead, this makes sure that the *disrupt* is prevented by immunity to silences.
- **Camille** will attempt to basic attack the target upon landing.
- **Camille** will track the target if they change locations.
  - She will always land and create the zone after $0.5$ seconds.
- If the target becomes untargetable, dies, or moves 2000 (Estimated) or more units away during the dash, it will be interrupted and the ability's cooldown and mana cost will not be refunded.
- Knockaway speed is 1000.
- If the target manages to be outside of the boundaries, they will immediately be pulled back towards the center of the hexagon.
- The boundaries are not considered terrain (Unstoppable Onslaught exception), and enemies that try to move out of the area will be displaced back inside.
  - Dashes, blinks, displacements, and other forms of movement will cap their effect range to the boundary. Targets will stop at the boundary even after it expired.
  - Ekko’s Chronobreak will still deal damage at the hologram's location.
  - An active Cease and Desist on a target outside the perimeter will cause Vi to continue her charge until *The Hextech Ultimatum* ends. She can knockback and damage secondary targets multiple times.
  - If the target attaches to a unit, they will only be displaced back inside once they detach. *The Hextech Ultimatum* does not end nor break the bind to the victim.
    - Kayn’s Umbral Trespass allows him to cast it even if his target is outside the area, dashing through the borders and attaching despite being unable to escape.
    - Yuumi’s You and Me! allows her to bypass the borders if the unit she attaches to leaves the area.
    - Tristana’s Rocket Jump does not allow her to escape, but she applies the magic damage and slow.
    - Sett’s The Show Stopper does not allow him to pass through the borders and the dash will end prematurely when he encounters them.
      - If the target of The Show Stopper is also the target of *The Hextech Ultimatum*, they can go through the borders while attached to him.
- If **Camille** enters resurrection during the initial cast, the dash is stopped and the zone is formed immediately.
  - *The Hextech Ultimatum* ends if the target enters *resurrection*, but not if **Camille** does.
- *The Hextech Ultimatum* ends immediately if the target is dragged by Fear Beyond Death, or displaced outside of the zone by Dragon's Rage or Keeper's Verdict.
- The following table refers for interactions while **Camille** is dashing.

| Aspect | State / Notes |
|--------|---------------|
| **Attacking** | Disabled |
| **Movement** | Disabled |
| **Abilities** | Disabled |
| **Items** | Disabled: All items |
| **Summoner Spells** | Disabled: All summoner spells |
| **Consumables** | Disabled |
| **Interrupted by** | Death |

---

## Patch History

### V14.21
- Stats
  - Base health reduced to 650 from 670.
- Tactical Sweep
  - Base damage reduced to 50 / 75 / 100 / 125 / 150 from 70 / 100 / 130 / 160 / 190.
  - Tooltip updated to reflect the 300 damage cap against monsters on the outer half.
- Wall Dive
  - Bonus AD ratio reduced to 75% **bonus** AD from 90%.
- The Hextech Ultimatum
  - **Removed:*** No longer has a base on-hit damage of 5 / 10 / 15.

### V14.19
- Tactical Sweep
  - **Bug Fixes:** No longer fails to grant the healing if the damage is mitigated by a shield.

### V14.15
- Precision Protocol
  - **Bug Fixes:** True damage is no longer mitigated by some effects.

### V14.11
- Precision Protocol
  - Bonus movement speed reduced to 25 / 30 / 35 / 40 / 45% from 30 / 35 / 40 / 45 / 50%.
- The Hextech Ultimatum
  - Base damage reduced to 5 / 10 / 15 from 20 / 30 / 40.

### V14.7
- Precision Protocol
  - Bonus movement speed increased to 30 / 35 / 40 / 45 / 50% from 30 / 32.5 / 35 / 37.5 / 40%.
  - Cooldown reduced to 9 / 8 / 7 / 6 / 5 seconds from 9 / 8.25 / 7.5 / 6.75 / 6.
- Tactical Sweep
  - Outer cone base damage increased to 6 / 6.5 / 7 / 7.5 / 8% of target's **maximum** health from 5 / 5.5 / 6 / 6.5 / 7%.
- Wall Dive
  - Base damage reduced to 60 / 90 / 120 / 150 / 180 from 80 / 110 / 140 / 170 / 200.

### V14.2
- Stats
  - Base health increased to 670 from 646.
- Adaptive Defenses
  - Cooldown reduced to 18–10@1–13 from 20–10@1–13.
- Precision Protocol
  - Movement speed increased to 30 / 32.5 / 35 / 37.5 / 40% from 20 / 25 / 30 / 35 / 40%.

### V13.17
- Adaptive Defenses
  - **Bug Fixes:** Shield type indicator now uses the correct color for an enemy Neeko disguised as a champion that uses a different damage type from hers.

### V13.15
- Stats
  - Attack damage growth increased to $3.8$ from $3.5$.
- The Hextech Ultimatum
  - Base damage increased to 20 / 30 / 40 from 5 / 10 / 15.

### V12.23
- Camille
  - **Bug Fixes:** Voice lines will no longer be played globally for allies and enemies as long as she was visible to them.

### V12.22
- Stats
  - Base magic resistance reduced to 32 from $32.1$.

## Trivia

- This champion has no ability power ratio.
- Camille was the last champion released in 2016, designed to make players 'feel like in-control badasses'.
- Her design bears a very strong resemblance to Lin Beifong from the series Legend of Korra, with similar appearance, profession, and fighting style. She may have also been influenced by Gazelle from the movie Kingsman: The Secret Service.
- Camille’s Adaptive Defenses was the first instance of a shield that exclusively protects from physical damage. It was the only one until the introduction of Armored Advance Noxian Endurance on patch V25.S1.1.
- *Camille* is the French derivative of Latin *Camilla*, feminine form of cognomen *Camillus*.
  - *Camillus/a* means 'temple attendant,' and has no Latin root. Earlier forms include Greek Cadmus-Κάμιλλος-Κᾰσμῖλος-Κᾰδμῖλος, loaned from Pre-Greek **Hatʸmilʸ-*.
  - The name Camille could also be a reference to the mythological warrior Camilla(mythology) who appears in the second half of Vergil's Aeneid on the of the Rutuli. She is a warrior virgin queen of the Volsci and serves under the goddess Diana. She was said to be so swift-footed she could run through a wheat field without breaking the tops of the stalks and run across the ocean without wetting her feet.
  - Her clan's name *Ferros* probably references the Latin word for iron "iron".
- Her dance references the Airflare, a breakdance move.
  - A side-by-side comparison can be seen here.
    - She shares this dance with Kennen.
    - Her dance is very similar to Vayne, whose dance is a similar breakdance move.
      - A side-by-side comparison of her and Vayne dances can be seen here.
- She was first mentioned in pre-rework Evelynn’s color story The Shadows Beckon which is no longer canon.

---
*This page was automatically generated from League of Legends Wiki data.*