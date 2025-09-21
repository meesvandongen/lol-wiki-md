# Rell

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
| **Champion** | Rell |
| **Title** | the Iron Maiden |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2020-12-10 |
| **Release Patch** | V10.25 |
| **Roles** | Vanguard |
| **Riot Positions** | Support |
| **External Positions** | Support |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $620.0$ | $+104.0$ | $2388.0$ |
| **Mana** | $320.0$ | $+40.0$ | $1000.0$ |
| **Health Regen** | $7.5$ | $+0.85$ | $21.9$ |
| **Mana Regen** | $7.0$ | $+0.7$ | $18.9$ |
| **Armor** | $30.0$ | $+4.0$ | $98.0$ |
| **Magic Resist** | $28.0$ | $+1.8$ | $58.6$ |
| **Attack Damage** | $55.0$ | $+3.0$ | $106.0$ |
| **Attack Speed** | $0.625$ | $+2.0\%$ | $0.838$ |
| **Movement Speed** | $315.0$ | $+0.0$ | $315.0$ |
| **Attack Range** | $175.0$ | $+0.0$ | $175.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.625$ |
| **Attack Speed Ratio** | $0.625$ |
| **Bonus AS per Level** | $2.0\%$ |
| **Attack Windup** | $21.0\%$ |
| **Acquisition Radius** | $400 units$ |
| **Pathing Radius** | $35 units$ |
| **Selection Radius** | $120 units$ |
| **Selection Height** | $150 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Break the Mold

**Innate:** **Rell**’s basic attacks deal **bonus** magic damage on-hit equal to a percentage of her **total** armor and **total** magic resistance.

*''Rell's* basic attacks and abilities apply a stack of *Break The Mold' to enemies hit for a few seconds. Each stack reduces the target's ar and mr and increases ''Rell's' ar and mr by the same amount.*

**Innate:** ''Rell's** basic attacks deal **bonus** magic damage on-hit equal to the sum of 5% of her **total** armor and 5% of her **total'' magic resistance. **Innate:** ''Rell's* basic attacks and abilities against non-minions apply a stack of *Break The Mold' for 5 seconds, refreshing on subsequent hits and stacking up to 5 times. Each stack reduces the target's ar and mr by 3% for a maximum of 15% reduction. **Rell** gains *bonus armor and **bonus** magic resistance equal to the sum resistances reduced from all afflicted enemies. *Break the Mold* will reduce the target's armor and magic resistance by a minimum of 1.5 to 3.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Self, Enemies |
| **Damage Type** | magic |
| **Spell Shield** | False |
| **Spell Effects** | proc |

**Notes:**

- The attacks do not affect structures.
- The first stack of *Break The Mold* that is applied to a target will snapshot the targets current ar and mr. All subsequent stacks will reduce the targets resistances based on the snapshotted amount, even if the target's total resistances change. This snapshot lasts until the debuff expires.
  - For example, applying a stack of *Break The Mold* to a champion with 100 ar and mr will reduce their resistance by 3 and grant **Rell** the lost stats. If their resistances are then boosted to 500 before another stack is applied, the subsequent stack will only reduce their resistance by a further 3 despite 3% of 500 being 15.

---

### Q: Shattering Strike

**Active:** **Rell** thrusts her lance in the target direction that destroys damage shield of enemies hit before dealing magic damage and stun.

**Active:** **Rell** thrusts her lance in the target direction, lunge forward 100 units and destroying the damage-mitigating shield of all enemies hit (excluding the shields of monsters) before dealing them magic damage and stun them for $0.65$ seconds.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $11-9$ seconds |
| **Cast Time** | $0.4$ seconds |
| **Cost** | 50 mana |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | False |

**Scaling:**
- **Magic Damage:** $60-220$ (+ 60% AP)

**Notes:**

- **Rell** is lockout of moving, attacking, and casting any other ability for $0.35$ seconds after *Shattering Strike* is cast.

---

### W: Ferromancy: Crash Down

**Passive - Mounted Alacrity:** While **Rell** is **Mounted**, she gains **bonus** movement speed.

**Active:** **Rell** becomes **Dismounted** and dash to the target location, gaining a shield that lasts until destroyed or casting **Ferromancy: Mount Up**.

**Passive - Mounted Alacrity:** While **Rell** is **Mounted**, she gains **bonus** movement speed. **Active:** **Rell** becomes **Dismounted** and dash to the target location over the cast time, granting herself a shield that lasts until destroyed or casting **Ferromancy: Mount Up**. Upon arrival, she deals magic damage to nearby enemies, stun them for $0.8$ seconds, and airborne for $0.4$ seconds. She will continue dash forward another 320 units over $0.5$ seconds, though not through terrain, affecting further enemies along her path. While **Rell** is **Dismounted**, she can cast **Ferromancy: Mount Up**. Upon completing a Recall channel or respawning, **Rell** will automatically revert to **Mounted** form without casting the ability and reset **Ferromancy: Mount Up*’s* *cooldown*. *This ability can be cast only while Rell is **Mounted**. **Rell** can cast *Magnet Storm* during the dash, and is not considered to be dismounted until after the leap ends.*

| Attribute | Value |
|-----------|-------|
| **Range** | 400 / 100 units |
| **Cooldown** | 10 seconds |
| **Cast Time** | $0.625$ seconds |
| **Cost** | 40 Mana |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | 180 units |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |

**Scaling:**
- **Bonus Movement Speed:** $20-40$
- **Shield Strength:** $20-100$ (+ 11%
- **maximum** health)
- **Magic Damage:** $60-180$ (+ 60% AP)

**Notes:**

- **Rell** dash between 250 and 350 units in testing, depending on how far the spell was targeted. It is not known where this is intended.
- If **Rell** dashes before a wall on the map, the slide will cover a shorter distance to the terrain over the same time, moving slower.
  - Interaction with player-made walls.
- The following table refers for interactions while **Rell** is dashing/in cast time:

---

### W: Ferromancy: Mount Up

**Passive:** While **Rell** is **Dismounted**, she gains *ar *bonus armor* and *mr **bonus** magic resist*.

**Active:** **Rell** becomes **Mounted** and gains *ms **bonus** movement speed* for a short time, increased while moving towards enemy champions.

**Passive:** While **Rell** is **Dismounted**, she gains *15% *bonus armor*, *15% **bonus** magic resistance*, *20% *bonus attack speed*, and *75 **bonus** attack range*. **Active:** **Rell** becomes **Mounted**, gaining ms*bonus** movement speed* decaying over 2 seconds and empowering her next basic attack within $3.5$ seconds to have a $0.2$-second cast time, gain *100 **bonus** attack range* and cause her to dash at the target's location, during which she also gains 40% *bonus attack speed. Upon arrival or collision, she deals **bonus** magic damage, stun the target for $0.6$ seconds, and airborne them 150 units over herself, though not through terrain, over $0.4$ seconds. While **Rell** is **Mounted**, she can cast **Ferromancy: Crash Down**. *Ferromancy: Mount Up basic attack reset *'Rell's** basic attack timer. This ability can be cast only while **Rell'* is **Dismounted**.*

| Attribute | Value |
|-----------|-------|
| **Cooldown** | 10 seconds |
| **Cast Time** | $0.25$ seconds |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spell |

**Scaling:**
- **Bonus Magic Damage:** $10-70$ (+ 40% AP)

**Notes:**

- **Rell** will always respawn and start the game as mounted.
- The basic attack reset is not considered one for *Hail of Blades*.
- **Rell** can use the empowered attack even while ground or root.
- The dash can be knockdown but the attack's effects will still be applied.
- The movement speed reduction is a negative bonus, not a slow, and is thus not reduced by slow resist.
- Displacement immunity will not resist the application of the stun.
- If the target becomes untargetable, death, or is too far away or no longer in sight during the empowered attack's cast time, it is cancelled but not consumed.

---

### E: Full Tilt

**Active:** **Rell** empowers herself and an allied champion with **bonus** movement speed for a few seconds, which is increased when moving towards the empowered ally or an enemy champion.

*''Rell's' next basic attack or *Shattering Strike* within 5 seconds will also explode around the target, dealing magic damage to nearby enemies.*

**Active:** **Rell** powers up herself and the target allied champion for 3 seconds, both gaining 10% **bonus** movement speed, increased to 25% while facing the empowered ally or a sight enemy champion. Additionally, ''Rell's* next basic attack or **Shattering Strike*' within 5 seconds creates an explosion around the target that deals **bonus** magic damage. The damage based on the target's health is capped at 150 to 300 against monsters and structures. If cast without a valid target, or self-cast, *Full Tilt* will automatically target the closest allied champion in range.

| Attribute | Value |
|-----------|-------|
| **Range** | 1200 units |
| **Cooldown** | $14-10$ seconds |
| **Cast Time** | none |
| **Cost** | 40 Mana |
| **Targeting** | Unit / Auto |
| **Affects** | Self, Allies, Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | 300 / 2200 / 1600 units |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |

**Scaling:**
- **Bonus Magic Damage:** $5-7$%

**Notes:**

- If **Shattering Strike** hits more than one enemy, the *Full Tilt* explosion will be centered around the unit with the lowest Spawn ID.
  - "Spawn ID" is an unofficial abbreviation to describe the spawn order for all units at the beginning of games.

---

### R: Magnet Storm

**Active:** **Rell** erupts in magnetic fury, airborne nearby enemies inward. She then emits a Kinematics field that continually deals magic damage over a short time.

**Active:** **Rell** erupts with magnetic fury, airborne nearby enemies inward and creating a gravitational field around her for the next 2 seconds that deals magic damage every $0.25$ seconds to nearby enemies and kinematics them towards her.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $120-80$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | 100 Mana |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Speed** | 300 units/second |
| **Effect Radius** | 450 / er 375 units |
| **Spell Shield** | Special |
| **Spell Effects** | aoedot |

**Scaling:**
- **Magic Damage Per Tick:** $15-35$ (+ $13.75$% AP) Total Magic Damage $120-280$ (+ 110% AP)

**Notes:**

- *Magnet Storm* will **not** drag units that are:
  - Dash.
  - Being displaced (e.g. by airborne effects)
  - Attach.
  - Displacement immune.
  - cc-immune.
- Spell shield will block the initial airborne but not the dragging effect.
- *Magnet Storm* cannot be cast again while it is active.

---

## Patch History

### V25.17
- *Ferromancy: Crash Down*
  - **Bug Fixes:** Tooltip now notes the proper shield health ratio in accordance with the

### V25.16
- *Ferromancy: Crash Down*
  - Shield health ratio reduced to 11% **maximum** health from 13%.

### V25.15
- *Ferromancy: Crash Down*
  - Base shield reduced to $20-100$ from $25-125$.
- Stats
  - Base health reduced to 620 from 640.
  - Armor growth reduced to 4 from $4.3$.

### V14.24
- Stats
  - Base health increased to 640 from 610.
- *Break the Mold*
  - Resistances reduction per stack increased to 3% from 2%.
  - Minimum resistances reduction increased to 1.5 to 3 from 1 to 2.
- *Ferromancy: Crash Down*
  - Base shield increased to $25-125$ from $15-115$.
  - Shield health ratio increased to 13% **maximum** health from 12%.

### V14.23
- Stats
  - Base mana reduced to 320 from 350.
  - Mana growth reduced to 40 from 45.
  - Base mana regeneration increased to 7 from 6.
  - Mana regeneration growth increased to $0.7$ from $0.35$.
  - Base armor reduced to 30 from 36.
  - Armor growth increased to $4.3$ from $4.2$.
  - Base magic resistance reduced to 28 from 30.
  - Magic resistance growth reduced to $1.8$ from $2.05$.
  - Attack speed growth increased to 2% from $1.5$%.
  - Attack windup reduced to $100×0.3/1.6$% from $100×0.42/2$%.
  - Attack windup time is no longer reduced by 100-(0.42 to 0.39)/0.42×100.
  - Attack windup modifier increased to 1 (default) from $0.4$.
  - Base movement speed reduced to 315 from 330.
- *Break the Mold*
  - Minimum resistance steal increased to 1 to 2 from 0.8 to 2.
  - **New Effect:** Now deals on-hit magic damage equal to armor**total** armor* and mr**total** magic resistance*.
- *Shattering Strike*
  - Stun duration reduced to $0.65$ seconds from $0.75$.
- *Ferromancy: Crash Down*
  - Cooldown reduced to 10 seconds from 11.
  - Knock up duration reduced to $0.4$ seconds from 1.
  - **New Effect:** Now also stun for $0.8$ seconds at the start of the displacement.
  - ***New Effect - Mounted Alacrity:*** While **Rell** is mounted, she passively gains $20-40$ **bonus** movement speed.
  - Base shield increased to $15-115$ from $15/40/65/90/110$.
- *Ferromancy: Mount Up*
  - Cooldown reduced to 10 seconds from 11.
  - Stun duration reduced to $0.6$ seconds from 1.
  - **Removed:*** ''Rell's' movement speed is no longer reduced by 10% while dismounted.
  - Bonus resistances increased to 15% from 12%.
  - Bonus attack speed reduced to 20% from 30%.
- *Full Tilt*
  - Cooldown reduced to $14-10$ seconds from 15 at all ranks.
  - ***Removed - Mounted Alacrity:*** **Rell** no longer passively gains 5 to 20 for 6–50 **bonus** movement speed while **Mounted**, reduced by 50% while in combat.
    - *Effect name moved to *Ferromancy: Crash Down*.*
  - **Removed:*** Active movement speed no longer ramps up linearly from 75% of its maximum value over the first 2 seconds.
  - Base movement speed reduced to 10% from $12-16$%.
  - Rally movement speed modifier increased to 250% of base from 200%.
    - Rally movement speed changed to $10×2.5$% from $12×2-16×2$%.
  - **Removed:*** No longer has a base damage of $25-65$.
  - **Removed:*** Damage no longer scales with 50% AP.
  - Health ratio increased to $5-7$% of target's **maximum** health from 3% at all ranks.
  - **New Effect:** Health ratio now scales with 3% per 100 AP.
  - Monster damage cap increased to 150 to 300 from 150 at all levels.
  - **New Effect:** Damage based on the target's health now applies against structures.
  - **New Effect:** Damage based on the target's health is now capped at 150 to 300 against structures.

### V14.18
- *Full Tilt*
  - Bonus movement speed reduced to $12-16$% from $12-20$%.
    - Increased bonus movement speed reduced to $12×2-16×2$% from $12×2-20×2$%.

### V14.16
- *Full Tilt*
  - **Bug Fixes:** Bonus damage no longer improperly counts towards applying the resistances reduction from *Break the Mold* if applied by *Shattering Strike*.

### V14.14
- *Full Tilt*
  - **Bug Fixes:** Can no longer be used on allies without having vision of them.

### V14.7
- Stats
  - Magic resistance growth increased to $2.05$ from $1.85$.
- *Shattering Strike*
  - **Removed:*** No longer deals $170-470$ **bonus** damage against monsters.
- *Ferromancy: Crash Down*
  - **Removed:*** No longer deals $125-225$ **bonus** damage against monsters.
- *Ferromancy: Mount Up*
  - **Removed:*** No longer deals $55-275$ **bonus** damage against monsters.
- *Full Tilt*
  - **Removed:*** No longer deals $120-300$ **bonus** damage against monsters.

### V14.4
- *Shattering Strike*
  - **Removed:*** No longer destroys shields against monsters.

## Trivia

- Rell's lance is featured in the promotional art for Preseason 2021 Mythic Forge as a reveal teaser.

---
*This page was automatically generated from League of Legends Wiki data.*