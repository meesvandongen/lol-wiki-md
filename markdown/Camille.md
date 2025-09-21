# Camille

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
| **Champion** | Camille |
| **Title** | the Steel Shadow |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2016-12-07 |
| **Release Patch** | V6.24 |
| **Roles** | Diver |
| **Riot Positions** | Top |
| **External Positions** | Top, Support |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $650.0$ | $+99.0$ | $2333.0$ |
| **Mana** | $339.0$ | $+52.0$ | $1223.0$ |
| **Health Regen** | $8.5$ | $+0.8$ | $22.1$ |
| **Mana Regen** | $8.15$ | $+0.75$ | $20.9$ |
| **Armor** | $35.0$ | $+5.0$ | $120.0$ |
| **Magic Resist** | $32.0$ | $+2.05$ | $66.8$ |
| **Attack Damage** | $68.0$ | $+3.8$ | $132.6$ |
| **Attack Speed** | $0.644$ | $+2.5\%$ | $0.918$ |
| **Movement Speed** | $340.0$ | $+0.0$ | $340.0$ |
| **Attack Range** | $125.0$ | $+0.0$ | $125.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.644$ |
| **Attack Speed Ratio** | $0.644$ |
| **Bonus AS per Level** | $2.5\%$ |
| **Attack Windup** | $19.3\%$ |
| **Acquisition Radius** | $550 units$ |
| **Pathing Radius** | $35 units$ |
| **Selection Radius** | $100 units$ |
| **Selection Height** | $280 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Adaptive Defenses

**Innate:** Periodically, **Camille**’s next basic attack against a champion will on-hit grant her a shield against their damage type (physical or magical) for a short time.

**Innate:** Periodically, ''Camille's** next basic attack on-hit against an enemy champion grants her a shield equal to *20% of her **maximum** health*, lasting for 2 seconds and absorbing damage from either exclusively physical damage or magic damage, based on which type the target has previously dealt most of against champions as well as on **Camille's** **current'' *armor* and *magic resistance*.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Self |

**Notes:**

- *Adaptive Defenses* goes on cooldown after the shield expires or is destroyed.
- The shield-typing is determined based on the **total** damage the target has dealt to champions that game (as a percentage) versus ''Camille's** **current'' *armor* and *magic resistance*.
  - For example, with little **bonus** resistances **Camille** would likely gain the most obvious shield-typing - i.e. physical shield versus champions primarily dealing physical damage and magic shield versus champions primarily dealing magic damage. However, if she has a sufficient amount of armor against a champion dealing both alternates of physical damage and magic damage, she will receive a magic shield as the physical damage threat is already being mitigated by her armor, and vice versa.
- The type of shield that will be granted to **Camille** when she attacks any one enemy champion is shown as an icon beneath their health bar: orange for physical shield and blue for magic shield.
- *Adaptive Defenses* takes priority over all other shield except *Black Shield*.
- *Adaptive Defenses* Will not grant **Camille** the shield is the attack is dodge, block, or if she is blind.

---

### Q: Precision Protocol

**Active:** **Camille**’s next basic attack deals **bonus** physical damage and grants her *ms **bonus** movement speed*.

*Precision Protocol* can be recast for a short time, dealing significantly increased damage if **Camille** delays the recast. A portion of the attack's damage will be dealt as true damage.

**Active:** **Camille** empowers her next basic attack within 4 seconds to have an uncancellable windup, gain range*bonus** range*, deal **bonus** physical damage and grant her for 1 second. This attack cannot critically strike. After $0.25$ seconds, *Precision Protocol* can then be recast within the next $3.5$ seconds at no additional cost. **Recast:** **Camille** mimics the first cast's effects. If *Precision Protocol* is recast after $1.5$ seconds of the first attack, then the **bonus** damage is doubled, and key=% of the attack's **total** damage will be dealt as . *Both casts of Precision Protocol basic attack reset *'Camille's** basic attack timer. **Camille'' is able to cast *Tactical Sweep*, *Hookshot*, or *The Hextech Ultimatum* during the windup of Precision Protocol's empowered attack without cancelling it.'

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $9-5$ seconds |
| **Cast Time** | none |
| **Cost** | 25 Mana |
| **Targeting** | Auto |
| **Affects** | Self |
| **Damage Type** | Physical True |
| **Spell Shield** | False |
| **Spell Effects** | spell |

**Scaling:**
- **Bonus Physical Damage:** $20-40$% AD
- **Bonus Movement Speed:** $25-45$%
- **Increased Mixed Damage:** $20×2-40×2$% AD

**Notes:**

- Both casts count as ability activations for the purposes of on-cast effects such as Spellblade and triggering *Force Pulse*’s passive.
- Both enhanced attacks **cannot** critical strike.
- The $1.5$-second delay is "fuzzy", meaning it may take up to another $0.25$ seconds to ready the window for the empowered attack.
  - This syncs up perfectly with *Sheen* cooldown between casts at 0 item haste, which is also "fuzzy".
- The second attack will also become empowered if it simply starts after the $1.5$-second delay, even if the recast was consumed earlier.
- The empowered **recast**'s true damage:
  - Has a special case to factor Spellblade into the true damage conversion of part of her attack's damage.
  - Other on-hit effects do not interact with the true damage conversion.
  - Randomly dependent on buff order, if (e.g. *Plated Steelcaps* Plating, *W*) applies first before the conversion, then the resulting damage will be reduced, otherwise if the conversion happens first, then the damage will not be successfully reduced (both effects are pre-mitigation on-take-damage events).
  - Is dealt just prior to the physical damage portion of the attack.
- *Precision Protocol* applies to structures but the recast does not convert to true damage.
- The enhanced attack will still complete and hit the target even if they become untargetable during the attack windup.
- *Warden's Mail* flat damage reduction is reduced on the second cast, reducing the physical damage but not the true damage.

---

### W: Tactical Sweep

**Active:** **Camille** briefly becomes ghosted and sweeps her leg after a delay, dealing physical damage to enemies hit in a cone. Enemies in the outer half are slowed and take increased damage, while also healing her.

**Active:** **Camille** sweeps her leg in a cone in the target direction over $1.1$ seconds, during which she is ghosted and unable to declare basic attacks. Afterwards, she deals physical damage to all enemies within. Enemies hit by the outer half of the cone take **additional** physical damage, capped at 300 against monsters, and are slow by 80% decaying over 2 seconds. **Camille** is healing for 100% of this **additional** damage post-mitigation against enemy champions in the outer half. Non-epic monsters take 50% damage from *Tactical Sweep*. *Other abilities can be cast during the animation without cancelling it.*

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $17-11$ seconds |
| **Cast Time** | none |
| **Cost** | $50-70$ Mana |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Effect Radius** | 325 / 650 units |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | False |

**Scaling:**
- **Physical Damage:** $50-150$ bonus AD)
- **Outer Cone Bonus Damage:** $6-8$%
- **Non-Epic Monster Damage:** $50/2-150/2$ (+ 30% bonus AD)
- **Bonus Non-Epic Monster Damage:** $6/2-8/2$%

**Notes:**

- **Camille** can move during the delay, but the hitbox and ''Camille's' visuals are fixed to the initial target direction.
  - ''Camille's' facing-direction, for effects such as *Petrifying Gaze*, is the direction she is moving, and not the direction the model is facing.
- The 50% damage reduction against non-epic monsters applies after the cap. Because of this, the actual cap against non-epic monsters is 150 damage.
- Because the increased damage in the outer range of the cone is dealt as an **additional** instance of damage (as opposed to **bonus** damage to the original instance), it will trigger effects twice such as being reduced by *Bone Plating* an extra time.
  - It still belongs to the same cast instance, therefore it does not trigger additional stacks of *Conqueror* or *Electrocute*.

---

### E: Hookshot

**Active:** **Camille** fires a grapple, dash onto any terrain she hits and briefly perching onto it.

*While perched, she gains the ability to cast **Wall Dive**.*

**Active:** **Camille** fires a grapple in the target direction. If the grapple collides with terrain, **Camille** will dash toward and attach to the terrain for $0.75$ seconds, during which she gains the ability to cast **Wall Dive**. **Camille will be knockdown by any immobilize or polymorph crowd control during the dash.** *Other abilities can be cast during the ability. Casting *The Hextech Ultimatum* will cause the grapple to disappear if it is in flight.*

| Attribute | Value |
|-----------|-------|
| **Range** | 800 units |
| **Cooldown** | $16-12$ seconds |
| **Cast Time** | none |
| **Cost** | 70 Mana |
| **Targeting** | Direction |
| **Affects** | Terrain, Self |
| **Speed** | 1400 units/second |
| **Projectile** | True |

**Notes:**

- **Camille** may move while the grapple is in flight.
- *Hookshot* can be cast while root but not while ground.
- **Camille** will grapple to terrain even if she is immobilize.
- *Hookshot* can interact with player-generated terrain.
- Sleep does not count for knocking the dash down.

---

### E: Wall Dive

**Active:** **Camille** dashes, dealing physical damage to enemies near the landing location. Dash range is doubled towards enemy champions.

*Upon colliding with an enemy champion, she airborne and stun all nearby enemy champions, gaining *as *bonus attack speed*.*

**Active:** **Camille** dashes in the target direction, dealing physical damage to enemies near the landing location. 'Wall Dive's range is doubled towards enemy champions within 1400 units. **Camille** grants ghosted to non-champions she passes through for 4 seconds. **Camille** stops prematurely upon colliding with an enemy champion, airborne all nearby enemy champions, though not through terrain, as well as stun them for $0.75$ seconds, while also gaining **bonus attack speed** for 5 seconds. *Wall Dive may be also cast with movement or attack commands. Other abilities besides *The Hextech Ultimatum* can be cast while grappled or during the dash.*

| Attribute | Value |
|-----------|-------|
| **Range** | 400 / 800 units |
| **Cast Time** | none |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Speed** | 1050 + 100% movement speed |
| **Effect Radius** | 130 units |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |

**Scaling:**
- **Physical Damage:** $60-180$ bonus AD)
- **Bonus Attack Speed:** $40-60$%

**Notes:**

- *Wall Dive* count as an ability activation for the purposes of on-cast effects such as Spellblade and triggering *Force Pulse*’s passive.
- Despite what one might assume from a wall-jumping ability, *Wall Dive* **can** be used to go through terrain (including the terrain she is bound to).
- Casting Flash during 'Wall Dive's dash will interrupt it but **Camille** will affect enemies around the new location.
- *Wall Dive* is disabled while grounded or root.
  - *Wall Dive* will not automatically cast from movement or attack commands in both cases. While grounded, however, **Camille** is able to move while bound. She may still dash away from her current location once the grounding finishes.
- **Camille** will only gain a range indicator towards each valid champion within the extended dash range (800), but will still gain the extended dash range without the indicator against enemies within the 1400 range.
  - She does not have to Sight any enemy champions to gain the range indicator or extended range.
- Displacement immunity will not resist the application of the stun.
- If *Tactical Sweep* is cast during 'Wall Dive's dash on its opposite direction, the target wil be knockback further and to a different direction.
- The following table refers for interactions while **Camille** is bound to terrain:
  - When **Camille** is interrupted, she will automatically cast *Wall Dive*. The only exception is if she was interrupted by casting Flash or Recall, in which case *Wall Dive* cancels.

---

### R: The Hextech Ultimatum

**Active:** **Camille** dash onto an enemy champion, creating a zone around the target and airborne all other enemies.

*While within the zone, the target cannot escape through any means, and ''Camille's** basic attacks deal **bonus'' magic damage against them.*

**Active:** **Camille** becomes untargetable and dash with displacement immunity towards the target enemy champion over $0.5$ seconds, standard sight them for the duration and disrupt their ongoing channel. Upon landing on the target, she becomes targetable again and creates a hexagonal zone around her current location for a duration, airborne all other nearby enemies on impact, though not through terrain. The target cannot escape the zone through any means. While within the zone, ''Camille's** basic attacks against the target deal **bonus'' magic damage. *The Hextech Ultimatum* ends automatically within 1 second if **Camille** leaves the area or dies.

| Attribute | Value |
|-----------|-------|
| **Range** | 475 units |
| **Cooldown** | $140-90$ seconds |
| **Cast Time** | none |
| **Cost** | 100 Mana |
| **Targeting** | Unit |
| **Affects** | Enemies, Self |
| **Damage Type** | Magic |
| **Effect Radius** | 425 units |
| **Spell Shield** | True |
| **Spell Effects** | Proc |

**Scaling:**
- **Zone Duration:** $2.5-4$ seconds
- **Bonus Magic Damage:** $4-8$% of target's
- **current** health

**Notes:**

- The disrupt is 'nested' into a $0.4$-second Silence on the target, but unlike normal silences, this debuff does not actually make the target *unable to cast*. Instead, this makes sure that the *disrupt* is prevented by Cc-immune.
- **Camille** will attempt to basic attack the target upon landing.
- **Camille** will track the target if they change locations.
  - She will always land and create the zone after $0.5$ seconds.
- If the target becomes untargetable, death, or moves 2000 or more units away during the dash, it will be interrupted and the ability's *cooldown* and *mana* cost will not be refunded.
- Airborne speed is 1000.
- If the target manages to be outside of the boundaries, they will immediately be pulled back towards the center of the hexagon.
- The boundaries are not considered terrain (*Unstoppable Onslaught* exception), and enemies that try to move out of the area will be displaced back inside.
  - Dash, blink, airborne, and other forms of movement will cap their effect range to the boundary. Targets will stop at the boundary even after it expired.
  - Chronobreak will still deal damage at the hologram's location.
  - An active *Cease and Desist* on a target outside the perimeter will cause **Vi** to continue her charge until *The Hextech Ultimatum* ends. She can knockback and damage secondary targets multiple times.
  - If the target attached to a unit, they will only be displaced back inside once they detach. *The Hextech Ultimatum* does not end nor break the bind to the victim. *** Umbral Trespass allows him to cast it even if his target is outside the area, dashing through the borders and attaching despite being unable to escape. *** You and Me! allows her to bypass the borders if the unit she attaches to leaves the area. *** Rocket Jump does not allow her to escape, but she applies the magic damage and slow. *** The Show Stopper does not allow him to pass through the borders and the dash will end prematurely when he encounters them. **** If the target of *The Show Stopper* is also the target of *The Hextech Ultimatum*, they can go through the borders while attached to him.
- If **Camille** enters resurrection during the initial cast, the dash is stopped and the zone is formed immediately.
  - *The Hextech Ultimatum* ends if the target enters *resurrection*, but not if **Camille** does.
- *The Hextech Ultimatum* ends immediately if the target is dragged by *Fear Beyond Death*, or displaced outside of the zone by *Dragon's Rage* or *Keeper's Verdict*.
- The following table refers for interactions while **Camille** is dashing.

---

## Patch History

### V14.21
- Stats
  - Base health reduced to 650 from 670.
- *Tactical Sweep*
  - Base damage reduced to $50-150$ from $70-190$.
  - Tooltip updated to reflect the 300 damage cap against monsters on the outer half.
- *Wall Dive*
  - Bonus AD ratio reduced to 75% *bonus AD from 90%.
- *The Hextech Ultimatum*
  - **Removed:*** No longer has a base on-hit damage of $5-15 3$.

### V14.19
- *Tactical Sweep*
  - **Bug Fixes:** No longer fails to grant the healing if the damage is mitigated by a shield.

### V14.15
- *Precision Protocol*
  - **Bug Fixes:** True damage is no longer mitigated by some effects.

### V14.11
- *Precision Protocol*
  - Bonus movement speed reduced to $25-45$% from $30-50$%.
- *The Hextech Ultimatum*
  - Base damage reduced to $5-15 3$ from $20-40 3$.

### V14.7
- *Precision Protocol*
  - Bonus movement speed increased to $30-50$% from $30-40$%.
  - Cooldown reduced to $9-5$ seconds from $9-6$.
- *Tactical Sweep*
  - Outer cone base damage increased to $6-8$% of target's **maximum** health from $5-7$%.
- *Wall Dive*
  - Base damage reduced to $60-180$ from $80-200$.

### V14.2
- Stats
  - Base health increased to 670 from 646.
- *Adaptive Defenses*
  - Cooldown reduced to 18–10@1–13 from 20–10@1–13.
- *Precision Protocol*
  - Movement speed increased to $30-40$% from $20-40$%.

### V13.17
- *Adaptive Defenses*
  - **Bug Fixes:** Shield type indicator now uses the correct color for an enemy **Neeko** disguised as a champion that uses a different damage type from hers.

### V13.15
- Stats
  - Attack damage growth increased to $3.8$ from $3.5$.
- *The Hextech Ultimatum*
  - Base damage increased to $20-40 3$ from $5-15 3$.

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
- Adaptive Defenses was the first instance of a shield that exclusively protects from physical damage. It was the only one until the introduction of *Armored Advance* Noxian Endurance on patch V25.S1.1.
- *Camille* is the French derivative of Latin *Camilla*, feminine form of cognomen *Camillus*.
  - *Camillus/a* means 'temple attendant,' and has no Latin root. Earlier forms include Greek Cadmus-Κάμιλλος-Κᾰσμῖλος-Κᾰδμῖλος, loaned from Pre-Greek **Hatʸmilʸ-*.
  - The name Camille could also be a reference to the mythological warrior Camilla(mythology) who appears in the second half of Vergil's Aeneid on the of the Rutuli. She is a warrior virgin queen of the Volsci and serves under the goddess Diana. She was said to be so swift-footed she could run through a wheat field without breaking the tops of the stalks and run across the ocean without wetting her feet.
  - Her clan's name *Ferros* probably references the Latin word for iron "iron".
- Her dance references the Airflare, a breakdance move.
  - A side-by-side comparison can be seen here.
    - She shares this dance with **Kennen**.
    - Her dance is very similar to Vayne, whose dance is a similar breakdance move.
      - A side-by-side comparison of her and Vayne dances can be seen here.
- She was first mentioned in pre-rework **Evelynn**’s color story The Shadows Beckon which is no longer canon.

---
*This page was automatically generated from League of Legends Wiki data.*