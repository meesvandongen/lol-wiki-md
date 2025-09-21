# Rumble

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
| **Champion** | Rumble |
| **Title** | the Mechanized Menace |
| **Resource** | Heat |
| **Range Type** | Melee |
| **Release Date** | 2011-04-26 |
| **Release Patch** | V1.0.0.116 |
| **Roles** | Battlemage |
| **Riot Positions** | Top |
| **External Positions** | Top, Middle |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $655.0$ | $+105.0$ | $2440.0$ |
| **Mana** | $150.0$ | $+0.0$ | $150.0$ |
| **Health Regen** | $7.0$ | $+0.6$ | $17.2$ |
| **Armor** | $36.0$ | $+4.7$ | $115.9$ |
| **Magic Resist** | $28.0$ | $+1.55$ | $54.4$ |
| **Attack Damage** | $64.0$ | $+3.2$ | $118.4$ |
| **Attack Speed** | $0.644$ | $+1.9\%$ | $0.847$ |
| **Movement Speed** | $345.0$ | $+0.0$ | $345.0$ |
| **Attack Range** | $125.0$ | $+0.0$ | $125.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.644$ |
| **Attack Speed Ratio** | $0.644$ |
| **Bonus AS per Level** | $1.9\%$ |
| **Acquisition Radius** | $600 units$ |
| **Gameplay Radius** | $80 units$ |
| **Pathing Radius** | $35 units$ |
| **Selection Radius** | $140 units$ |
| **Selection Height** | $170 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Junkyard Titan

**Innate:** **Rumble**’s basic ability casts generate Heat, which gradually expires after a few seconds without generating Heat. Rumble enters the *Danger Zone* while he has at least Heat, and becomes *Overheated* at Heat.

** Danger Zone:** ''Rumble's' basic abilities are empowered.

**Innate:** ''Rumble's* basic ability casts generate Heat, which decays by Heat per second after not using any basic ability within 4 seconds or **The Equalizer** within 2 seconds. *'Rumble's* mech enters the *Danger Zone* when at or above Heat, and becomes *Overheated' while at Heat. **Danger Zone:** ''Rumble's* mech enters the *Danger Zone', empowering his basic abilities. **Overheated:** ''Rumble's* mech becomes *Overheated', disabling his abilities as his Heat decays back down to 0 over 4 seconds. During this time, he gains *50+((130-50)/17)*(x-1)*(0.7025+0.0175*(x-1)) *bonus attack speed* and empowers his basic attacks to deal 5 to 40 (+ 25% AP) (+ 4% of the target's **maximum** health) **bonus** magic damage on-hit. The damage based on the target's health ratio is capped at 65+((150-65)/17)*(x-1)*(0.7025+0.0175*(x-1)) against monsters.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Self |
| **Damage Type** | Magic |
| **Spell Effects** | Proc |

**Notes:**

- While **Rumble** is *overheating*, a *Silenced* icon will appear above him to signify he cannot cast abilities. This is visible to all units.
- The attacks do not deal the **bonus** damage against structures.

---

### Q: Flamespitter

**Active:** **Rumble** activates a flamethrower for a short time that continually deals magic damage in a frontal cone.

**Active:** **Rumble** generates Heat to activate his flamethrower for 3 seconds, spewing forth flames in a frontal cone every $0.25$ seconds. Enemies hit by the flame are scorched for $0.5 + 0.1$ seconds, taking magic damage every $0.25$ seconds as well as upon being hit if not currently scorched, refreshing on subsequent inflictions. 'Flamespitter's damage is reduced to $% against minions. **Danger Zone Bonus:** 'Flamespitter's* damage is increased by 50%. *Flamespitter's total damage based on the target's health is capped at 65+((300-65)/17)*(x-1)*(0.7025+0.0175*(x-1)) against monsters.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $10-6$ seconds |
| **Cast Time** | none |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | False |
| **Spell Effects** | AoeDoT |
| **Projectile** | False |

**Scaling:**
- **Minimum Magic Damage:** $( (+ $( (+ $(% of target's **maximum* health)4 round=2 (+ $( (+ $(% of target's **maximum* health)*maximum* health)(15/12) round=2 (+ $ (+ $% of target's
- **maximum** health)
- **Minimum Minion Damage:** $(/12)*3* round=2 (+ $(% AP) (+ $(/12)*3* round=2% of target's **maximum* health)4* round=2 (+ $(% AP) (+ $(/12)*4* round=2% of target's **maximum* health) round=3 (+ $(% AP) (+ $(/12)* round=3% of target's **maximum* health)(15/12)* round=2 (+ $% AP) (+ $*(15/12)* round=2% of target's
- **maximum** health)
- **Minimum Enhanced Damage:** $( (+ $( (+ $(% of target's **maximum* health)41.5*maximum* health)1.5 round=3 (+ $( (+ $(% of target's **maximum* health)(15/12)1.5*maximum
- ** health) ** Minimum Enhanced Minion Damage:** $(/12)*3×1.53×1.5*maximum* health)1.5 to (/12)*4×1.54×1.5*maximum* health)1.5 to (/12)*1.51.5*maximum* health)1.5 to *(15/12)**(15/12)1.5(15/12)*1.5*maximum** health)

**Notes:**

- *Flamespitter* will deal an additional tick of damage to a minion if it would die to its per-tick damage. This is intended.
- **Rumble** will instantly turn to face the cursor if he is not moving when *Flamespitter* is cast. He will also instantly turn when issuing an Attack order on an enemy while *Flamespitter* is active.
  - Issuing an Attack Move order at a location will not cause **Rumble** to instantly turn.
  - If **Rumble** does not path to a location after an Attack or Attack Move order, and the player issues a Stop or Hold order immediately afterwards, **Rumble** will not visually turn completely to face the new direction, but the area will turn to have the correct direction as intended.
- When 'Flamespitter's debuff is applied instead of refreshed, the target is dealt one additional instance of the debuff at the same time as the application. This is done by delaying the actual debuff by $0.1$ seconds.
  - If the target is already affected by the debuff, its duration is refreshed to the maximum ($0.6$ seconds).

---

### W: Scrap Shield

**Active:** **Rumble** briefly shields himself and gains *ms **bonus** movement speed*.

**Active:** **Rumble** generates Heat to grant himself a shield for $1.5$ seconds. **Rumble** also gains *ms **bonus** movement speed* for 1 to $1.32$ seconds. **Danger Zone Bonus:** 'Scrap Shield's shield strength and **bonus** movement speed are increased in effectiveness by 50%.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | 6 seconds |
| **Cast Time** | none |
| **Targeting** | Auto |
| **Affects** | Self |

**Scaling:**
- **Shield Strength:** $ (+ (+
- **Bonus Movement Speed:** $%
- **Enhanced Shield Strength:** $ (+ $ (+ $
- **Enhanced Bonus Movement Speed:** $%

**Notes:**

- The movement speed buff is supposed to last for 1 second, but due to a bug may last longer.
  - Like most buffs, the stat bonus is already only gained at the next stat update (every ). Despite this, the duration still inconsistent.

---

### E: Electro Harpoon

**Active:** **Rumble** shoots a harpoon in the target direction that deals magic damage and briefly slow the first enemy hit, reducing their mpen for a few seconds. These effects can stack with multiple harpoons.

**Rumble** periodically stocks an *Electro Harpoon*, up to a cap.

**Active:** **Rumble** generates Heat to shoot a harpoon in the target direction that deals magic damage to the first enemy hit, inflicting them with magic penetration for 4 seconds and slow them for 2 seconds. These effects stacks additively with multiple harpoons, refreshing their duration and stacking up to 2 times. **Rumble** periodically stocks an *Electro Harpoon* charge, up to a maximum of 2. **Danger Zone Bonus:** 'Electro Harpoon's effects are increased by 50%. *If **Rumble** casts Electro Harpoon before his mech becomes *Overheated*, he may still use another charge within 3 seconds of the initial cast.*

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $0.5$ seconds |
| **Recharge** | 6 seconds |
| **Cast Time** | $0.25$ seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Speed** | 2000 units/second |
| **Spell Shield** | True |
| **Spell Effects** | spell |
| **Projectile** | True |

**Scaling:**
- **Magic Damage:* $ (+ 2 (+ $ **Magic Resistance Reduction:* $%2% **Slow:* $%2%
- **Enhanced Damage:* $ (+ $1.5×2 (+ $ **Enhanced Magic Resistance Reduction:* $%1.5×2% **Enhanced Slow:* $%1.5×2%

**Notes:**

- If *Electro Harpoon* is empowered with *Danger Zone's* bonus and hits a target that is afflicted with the non-enhanced slow and shred applied by previous, non-empowered *Electro Harpoon* casts, the effectiveness of both debuffs on the target are increased to their enhanced values for all stacks of each debuff.
  - This also applies vice versa; a non-empowered *Electro Harpoon* cast will reduce the enhanced slow and shred applied by previous, empowered *Electro Harpoon* casts back to the non-enhanced values, for all stacks.
- A buff with the magic resistance reduction tooltip is applied to minions and pets, but their magic resistance is unaffected by *Electro Harpoon*.
- Each *Electro Harpoon* cast triggers Spellblade and reduces *Force Pulse*’s cooldown.
- 'Electro Harpoon's missile will fail to fire if **Rumble** is suppression during the cast time. Effect at cast time end
- **Rumble** stocks an *Electro Harpoon* charge before the ability is even learned.

---

### R: The Equalizer

**Active:** **Rumble** deploys a barrage of rockets along the target path that create a field which lingers for a few seconds.

*Enemies within the area are continually dealt magic damage and slow.*

**Active:** **Rumble** deploys a barrage of rockets from the sky along the target path in a line over $0.75$ seconds. Each rocket impacts the ground to create a field of fire that lasts $4.5$ seconds. Enemies struck by the impact or within the field are marked *Burning* for 1 second, taking magic damage every $0.25$ seconds and being slow by 35%, refreshing while in the area. Enemies may be *Burning* for up to 5 seconds, for a total of 20 instances of its effect.

| Attribute | Value |
|-----------|-------|
| **Range** | 1700 units |
| **Cooldown** | $130-80$ seconds |
| **Cast Time** | None / $0.5833$ |
| **Targeting** | Vector |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Speed** | 1600 units/second |
| **Spell Shield** | False |
| **Spell Effects** | aoedot |
| **Projectile** | False |

**Scaling:**
- **Magic Damage per Tick:* $ (+ $5 (+ $

**Notes:**

- 'The Equalizer's cumulative hitbox consists of a row of rockets, each having a circular area of effect with a radius of 205 units. Each rocket drops at an offset of 180 units from the previous in the vector of cast.
  - The missile that manages the rockets being deployed starts 150 units behind the target vector origin, and thus drops the first rocket at 30 units in front of the cast vector origin, which is equivalent to a period of from the cast point.
- A given field checks for targets to apply or refresh the debuff on once every $0.25$ seconds.
- Due to each field expiring at different points in time, champions moving in the duration of the rockets' launch as they are expiring may sometimes receive 1 more instance of the debuff at most beyond the maximum.

---

## Patch History

### V25.17
- *Junkyard Titan*
  - Overheat on-hit damage target health ratio reduced to 4% of target's **maximum** health from 5%.
- *Flamespitter*
  - Base damage per tick reduced to $60/12-160/12 round=2$ from $60/12-180/12 round=2$.

### V25.16
- General
  - Each ability's Heat generation amount is now displayed in the HUD when the setting "Show Spell Costs" is enabled. They exist in place of resource costs for most other champions.
- *Flamespitter*
  - AP ratio per tick reduced to $100/12 from $110/12.
  - Target health ratio per tick reduced to $6/12-8/12 from $6/12-9.2/12.
  - **Bug Fixes:** The timing of damage by casters on the red will now match the timing of damage by casters on the blue side.
    - Debuff now affects enemies once at the same time as they are hit when the debuff is applied instead of refreshed.
  - **New Effect:** Upon being affected by 'Flamespitter's debuff, targets now receive damage through a burn that lasts $0.6$ seconds, refreshing on subsequent hits and not stacking with the flame if already debuffed.
    - Now has an internal counter to apply 2 additional instances of damage at most after the final refresh, including the final application instance, as a safety mechanism.
- *The Equalizer*
  - **Bug Fixes:** The timing of damage by casters on the red will now match the timing of damage by casters on the blue side.
  - **Bug Fixes:** Rocket zones no longer start affecting enemies at an up-to $0.25$ second delay (their debuff application is no longer fuzzy).
  - Debuff hit cadence increased to once every $0.25$ seconds from once every $0.5$ seconds.
    - Now has an internal counter to apply 4 additional instances of damage at most after the final refresh, including the final application instance, as a safety mechanism.
      - This may sometimes be inconsistent and apply one more instance at most beyond this maximum after the whole duration, when standing inside overlapping zones.
  - **Bug Fixes:** Each rocket zone now always lasts exactly $4.5$ seconds each instead of all rockets' zones disappearing at once 5 seconds after the first missile lands due to their duration collectively refreshing to the same amount with each subsequent rocket launch.
- *Flamespitter*
  - Target health ratio per tick increased to $6/12-9.2/12 from $6/12-8/12.
    - Total health ratio increased to $6-9.2$% of target's **maximum** health from $6-8$%.
- *Electro Harpoon*
  - Base slow increased to $15-35$% from $10-30$%.

### V25.12
- *Flamespitter*
  - Target health ratio per tick reduced to $6/12-8/12 from $6/12-10/12.
    - Total health ratio reduced to $6-8$% of target's **maximum** health from $6-10$%.
- *Electro Harpoon*
  - Base slow reduced to $10-30$% from $15-35$%.
- *Electro Harpoon*
  - **Bug Fixes:** Magic resistance reduction is once again enhanced while in Danger Zone.
- *Junkyard Titan*
  - **Bug Fixes:** Now once again properly gains all bonus effects while in Danger Zone.
- Rumble
  - *The Equalizer*
    - **Bug Fixes:** Restored all SFX.

### V14.24
- *Flamespitter*
  - AP ratio per tick increased to $110/12 from $100/12.
    - Total AP ratio increased to 110% AP from 100% AP.
- *Scrap Shield*
  - AP ratio increased to 30% AP from 25% AP.

### V14.23
- *Electro Harpoon*
  - **Bug Fixes:** No longer incorrectly gains its *Danger Zone* bonus based on the resulting heat amount instead of the heat amount prior to casting the ability.

### V14.22
- Stats
  - Base health increased to 655 from 625.

### V14.18
- *Flamespitter*
  - Base damage per tick reduced to $60/12-180/12 round=3$ from $80/12-180/12 round=3$.
    - Maximum base damage reduced to $60-180$ from $80-180$.
  - AP ratio per tick reduced to $100/12 from $110/12.
    - Maximum AP ratio reduced to 100% AP from 110% AP.
- *Electro Harpoon*
  - Base damage reduced to $55-155$ from $60-160$.

### V14.17
- *Junkyard Titan*
  - Damage health ratio reduced to 5% of target's **maximum** health from 6%.

### V14.14
- *Flamespitter*
  - Base damage per tick reduced to $80/12-180/12 round=3$ from $125/12-185/12 round=3$.
    - Total base damage reduced to $80-180$ from $125-185$.
  - Minion damage changed to 70% at all ranks from $55-75$%.
  - **Removed:*** No longer prevents minions hit from dying to allied minions if they would die to one tick of damage.
  - **New Effect:** Now deals an additional tick of damage if it would kill a minion.

### V14.9
- Stats
  - Selection radius reduced to 140 units from 165.
- Stats
  - Base attack damage increased to 64 from 61.
- *Electro Harpoon*
  - AP ratio increased to 50% AP from 40% AP.

## Trivia

- Rumble, **Corki**, and **Kled** are the only yordles with a mount (*Tristy*, *ROFL Copter*, and Skaarl, respectively)
- Rumble, , **Blitzcrank**, **Caitlyn**, **Lissandra*, Ci*Sion**, **Varus**, **Vi*, Ci*Xerath**, and **Ziggs** are the only champions who can apply crowd control on themselves.
- Until the release of Rumble, both Rumble and all of his skins have been released on April.

---
*This page was automatically generated from League of Legends Wiki data.*