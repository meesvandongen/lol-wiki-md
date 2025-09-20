# Azir

## Table of Contents
- [Basic Information](#basic-information)
- [Statistics](#statistics)
- [Pets](#pets)
- [Abilities](#abilities)
- [Patch History](#patch-history)
- [Trivia](#trivia)

## Basic Information

| Attribute | Value |
|-----------|-------|
| **Champion** | Azir |
| **Title** | the Emperor of the Sands |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2014-09-16 |
| **Release Patch** | V4.16 |
| **Roles** | Specialist |
| **Riot Positions** | Middle |
| **External Positions** | Middle |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $575.0$ | $+119.0$ | $2598.0$ |
| **Mana** | $320.0$ | $+40.0$ | $1000.0$ |
| **Health Regen** | $7.0$ | $+0.75$ | $19.8$ |
| **Mana Regen** | $8.0$ | $+0.8$ | $21.6$ |
| **Armor** | $25.0$ | $+5.0$ | $110.0$ |
| **Magic Resist** | $30.0$ | $+1.3$ | $52.1$ |
| **Attack Damage** | $56.0$ | $+3.5$ | $115.5$ |
| **Attack Speed** | $0.625$ | $+5.0\%$ | $1.156$ |
| **Movement Speed** | $330.0$ | $+0.0$ | $330.0$ |
| **Attack Range** | $525.0$ | $+0.0$ | $525.0$ |

## Pets

### Sand Soldiers

| Attribute | Value |
|-----------|-------|
| **Range** | 375 / 425 units |
| **Damage** | 0@1; 5@10; 10@11; 15@12; 20@13; 25@14; 30@15; 35@16; 40@17; 45@18 (+ $50-110$) (+ $35-55$% AP) |
| **Damage Type** | magic |
| **Control** | Attacks whenever **Azir** inputs an attack command on a target near them. |
| **Targeting** | Minion (untargetable) |
| **Spell Effects** | 'Sand Soldiers' attacks apply spell effects as area damage. |
| **On-Hit** | 'Sand Soldiers' attacks are mitigated by block or dodge, likewise while **Azir** is blind. |

**Abilities:**

- **Spear Attack:** *Sand Soldiers* will strike their spear in a line in the target's direction, dealing damage to all enemies within the area. Their attack passes through by 50 units. There is a brief delay between the command and the attack occurring.
- **Lifetime:** *Sand Soldiers* expire twice as fast while within range of a nearby enemy turret, and expires instantly when **Azir** is too far away.
- **Imperative:** Whenever **Azir** declares an attack on a target within 375 units of one of his *Sand Soldiers*, he instead commands all *Sand Soldiers* in range to attack.

---

### Sun Disc

| Attribute | Value |
|-----------|-------|
| **Range** | 750 units |
| **Gold** | 50 |
| **Experience** | 0 |
| **Health** | 1750–3000@0–5 (@=enemy champions in the game) (always 3000 in matchmade games) |
| **Armor** | 30@1; 35–90@7–18 |
| **Magic Resist** | 30@1; 35–90@7–18 |
| **CC Resist** | cc-immune except stasis |
| **Damage** | 230@1; 245–410@7–18 (+ 40% AP) |
| **Damage Type** | Magic |
| **Attack Speed** | $0.833$ attack speed |
| **Control** | Turret (static) |
| **Targeting** | Structure |
| **Spell Effects** | The 'Sun Disc's basic attacks apply spell effects as spell damage. |
| **On-Hit** | The 'Sun Disc's attacks cannot be mitigated by block and dodge. |

**Abilities:**

- **Dodge Piercing:** The 'Sun Disc's attacks cannot be evaded except by effects with homing projectile destruction.
- **Warden's Eye:** The *Sun Disc* gains true sight; it can see stealth units within a 1000 radius of it.
- **Deserter:** If **Azir** is dead or not within approximately 2000 units, the 'Sun Disc's *armor* and *magic resistance* is reduced by 100.

---

## Abilities

### Passive: Shurima's Legacy

**Innate:** **Azir** periodically can summon the Disc of the Sun from the ruins of allied or enemy turret.

*Disc of the Sun functions as a turret that decays over time, and weakens when **Azir** leaves it.*

**Innate:** Whenever a turret outside the enemy team's base is destroyed, a marker is placed above its ruins which can be targeted by **Azir**. **Innate - Disc of the Sun:** **Azir** can select a marker to construct the Sun Disc above the ruins after a $0.5$-second cast time. The Sun Disc will activate after being assembled over 5 seconds. The Sun Disc functions the same as a standard inner turret but deals magic damage, gains 40% AP **bonus attack damage**, and grants its kill credit to **Azir**. The Sun Disc's *health* decays over 45 seconds and loses armor and magic resistance while **Azir** is not alive or is too far away. **Azir cannot construct a Sun Disc with the marker while he is unable to cast abilities.** *See [Pets](#Pets) for more details about the Sun Disc.*

| Attribute | Value |
|-----------|-------|
| **Range** | 700 units |
| **Targeting** | Unit |
| **Affects** | Turret ruins |
| **Damage Type** | Physical |
| **Spell Shield** | False |
| **Spell Effects** | spell |
| **Projectile** | False |

**Notes:**

- No additional notes.

---

### Q: Conquering Sands

**Active:** **Azir** sends all Sand Soldiers to dash towards a location. They deal magic damage and slow enemies they pass through.

**Active:** **Azir** orders all Sand Soldiers to dash toward the target location, dealing magic damage to all enemies they pass through as well as those in front of them upon arrival, and slow them by 25% for 1 second. Enemies hit by subsequent soldiers take no additional damage and are instead slow by an additional 25% per soldier. *A summoned Sand Soldier is required to cast this ability.*

| Attribute | Value |
|-----------|-------|
| **Range** | 740 units |
| **Cooldown** | $14-6$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $70-110$ mana |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | 150 units |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | Special |

**Scaling:**
- **Magic Damage:** $60-140$ (+ 35% AP)

**Notes:**

- Sand Soldiers dash about 50 units further away from Azir than the target location, making them stab targets slightly further away.
- Sand Soldiers will stop their movement upon colliding with *Wind Wall* or *Rebuttal*, but not *Unbreakable*.
- Sand Soldiers will automatically turn to face nearby enemy champions after the dash.
  - This happens even if they're in stealth.

---

### W: Arise!

**Active:** **Azir** summons a Sand Soldier to attack nearby targets for him, replacing his basic attack against enemies within their range. Their attacks deal magic damage to enemies in a line.

*Sand Soldiers expire faster near turrets and instantly when **Azir** leaves them.*

**Active:** **Azir** summons a Sand Soldier at the target location that is untargetable, lasts 10 seconds, and grants sight of its surroundings. The Sand Soldier expires twice as fast while within range of an enemy turret, and does so instantly if **Azir** is too far away. **Azir** periodically stocks a Sand Soldier, up to a maximum of 2. When **Azir** declares a basic attack against an enemy in a soldier's *attack range*, the Sand Soldier attacks in his stead, thrusting their spear in the target's direction to deal magic damage to enemies struck in a line, applying on-hit effects to the primary target, with on-hit damage reduced to 50% effectiveness, and triggering on-attack effects once. Targets hit beyond the closest take key=% damage. Subsequent Sand Soldiers against the same target deal 25% damage. Sand Soldiers cannot attack turret, ward, nor stealthed trap. *See [Pets](#Pets) for more details about Sand Soldiers.*

| Attribute | Value |
|-----------|-------|
| **Range** | 525 units |
| **Cooldown** | $1.5$ seconds |
| **Recharge** | $10-6$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $40-20$ mana + 1 Sand Soldier |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | False |
| **Spell Effects** | aoe |
| **Projectile** | False |

**Scaling:**
- **Magic Damage:** 0@1; 5@10; 10@11; 15@12; 20@13; 25@14; 30@15; 35@16; 40@17; 45@18 (+ $50-110$) (+ $40-60$% AP)

**Notes:**

- Nearsight effects will only allow **Azir** to perform attacks from soldiers within his own sight radius.
- **Azir** must rank *Arise!* as his first ability, and will do so automatically if he reaches level 2 before ranking an ability.
- **Azir** can only command his soldiers to attack if he is able to declare basic attack commands himself, meaning crowd control effects that disable attacking, such as disarm, do not allow him to order his soldiers to attack in his stead.
- Damage to enemy champions from *Sand Soldier* stabs will draw minion aggro even if the enemy champion was not the primary target.
- Commanding a soldier to attack will trigger on-attack effects, but will not apply any of these effects to the target hit.
- Sand Soldiers will attack the closest enemy to **Azir** when using attack move (or **A + RMB** / **Shift + RMB**) or acquisition radius.

---

### E: Shifting Sands

**Active:** **Azir** shield himself and dash to the target Sand Soldier, dealing magic damage to enemies he passes through.

*He will stop upon hitting an enemy champion, gaining a charge of **Arise!**.*

**Active:** **Azir** grants himself a shield for $1.5$ seconds and dashes to the Sand Soldier closest to the cursor, dealing magic damage to enemies within his path. If **Azir** dashes into an enemy champion, he stops and gains a charge of **Arise!**. *A summoned Sand Soldier is required to cast this ability. **Azir** can cast of any of his abilities during the dash.*

| Attribute | Value |
|-----------|-------|
| **Range** | 1100 units |
| **Cooldown** | $22-16$ seconds |
| **Cast Time** | none |
| **Cost** | 60 mana |
| **Targeting** | Unit |
| **Affects** | Self, Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |

**Scaling:**
- **Shield Strength:** $70-230$ (+ 60% AP)
- **Magic Damage:** $60-220$ (+ 40% AP)

**Notes:**

- **Azir** will track the Sand Soldier if they change locations, such as by *Conquering Sands*.

---

### R: Emperor's Divide

**Active:** **Azir** summons a wall of soldiers that charges forward, dealing magic damage and airborne enemies hit.

*The soldiers will linger for a few seconds, acting as terrain against enemies while allowing allies to pass.*

**Active:** **Azir** calls forth a phalanx of soldiers from 175 units behind him, catching enemies from up to 325 units behind him and charging forward 575 units in the target direction. Enemies impacted by the charge are dealt magic damage and airborne over 1 second to a line 650 units in front of **Azir**. Upon finishing their charge, the soldiers stand as a wall for 5 seconds, acting as impassible terrain against enemies and granting sight of their surroundings. **Azir** and his allies can move through these soldiers.

| Attribute | Value |
|-----------|-------|
| **Range** | 400 / -325 units |
| **Cooldown** | $120/105/90$ seconds |
| **Cast Time** | $0.5$ seconds |
| **Cost** | 100 mana |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Speed** | 1000 / 1400 units/second |
| **Effect Radius** | 125 units |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | Special |

**Scaling:**
- **Width:** 6-8 soldiers, 750-950 units
- **Magic Damage:** $200-600$ (+ 75% AP)

**Notes:**

- The soldier units are spawned only 175 units behind **Azir**, but do not control the airborne directly. Each comes with a missile that travels the same path and conveys the damage and knockback effect.
  - The furthest area (from 325 to 175 units behind **Azir**) is covered by a separate set of missiles with the same effects.
  - All of the soldiers and their missiles will end the charge prematurely when one encounters *Wind Wall* / *Unbreakable* / *Blade Whirl* / *Rebuttal* or upon colliding with terrain that they cannot charge through.
- The charge starts at $0.25$ seconds into the cast time.
- When the dash of the soldiers comes to a halt, all not-yet-affected enemies on top of the soldiers' landing locations (125 radius) are also knocked back.
- 'Emperor's Divide' will not be canceled if **Azir** dies during the cast time.
- While the soldiers are charging forward, the wall is not impassable terrain.
  - This means enemies that are immune to airborne effects can pass through the wall while its moving.
- The wall **will** count as terrain for the purposes of 'into terrain' effects (e.g. *Condemn*, *Dredge Line*, *Heroic Charge*). This will only affect enemies, allies will pass through the wall in all circumstances, even if knocked into it by an enemy.

---

## Patch History

### V25.18
- Stats
  - Attack speed growth reduced to 5% from $5.5$%.
- *Arise!*
  - AP ratio reduced to $40-60$% AP from $45-65$% AP.

### V25.15
- *Arise!*
  - **Bug Fixes:** **Azir** when a Soldier attacks their users.
  - **Undocumented:** Soldiers' attacks now only grant 1 *Conqueror* stack.

### V25.14
- Stats
  - Base health increased to 575 from 550.
- *Arise!*
  - AP ratio increased to $45-65$% AP from $40-60$% AP.

### V25.13
- *Emperor's Divide*
  - **Bug Fixes:** Soldiers now once again properly knock back non-champions.

### V25.12
- Stats
  - Base movement speed reduced to 330 from 335.
  - Attack speed growth reduced to $5.5%$ from 6%.

### V14.23
- *Arise!*
  - AP ratio increased to $40-60$% AP from $35-55$% AP.

### V14.21
- General
  - Recommended runes updated.
    - *Fleet Footwork* to *Conqueror*.
- Stats
  - Base attack damage increased to 56 from 52.
  - Base health regeneration increased to 7 from 5.

### V14.18
- *Arise!*
  - Base damage changed to 0@1; 5@10; 10@11; 15@12; 20@13; 25@14; 30@15; 35@16; 40@17; 45@18 from 0@1; 2@10; 7@11; 12@12; 17@13; 29@14; 41@15; 53@16; 65@17; 77@18.
  - AP ratio reduced to $35-55$% AP from 55% AP at all ranks.

### V14.16
- Stats
  - Base health regeneration increased to 5 from $3.5$.
  - Base armor increased to 25 from 22.

### V14.10
- *Arise!*
  - **New Effect:** Basic attacks are now mitigated by dodge and block effects.
  - **Bug Fixes:** Basic attacks now trigger Thorns.
- *Emperor's Divide*
  - **Undocumented:** No longer knocks back non-champions.

## Trivia

- Azir is the first champion to have his first ability point restricted to a single one (*Arise!*) since it's required for the other two (*Conquering Sands* and *Shifting Sands*) to function.
  - He is also the second champion to be unable to rank up a basic ability at Level 1, the first being **Zilean**.
- *Shurima's Legacy* may be referencing the Egyptian solar disc god Aten (depicted as a sphere emanating rays of light each ending with a hand) and titular god of the short-lived monotheistic religion imposed by Pharaoh Akhenaten. His son Tutankhamun later restored the polytheistic religion.
- Azir's dance references Remember The Time by Michael Jackson.
  - A side-by-side comparison can be seen here.
  - If Azir *Arise!* Sand Soldiers and then dances, they will follow him and dance all in sync.
    - His soldiers will keep dance in Azir's stead while if the player types '/dance' while Azir is .
    - If enemies cannot see Azir while he dances, his soldiers will not appear to dance to them.

---
*This page was automatically generated from League of Legends Wiki data.*