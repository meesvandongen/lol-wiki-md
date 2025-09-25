# Hwei

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
| **Champion** | Hwei |
| **Title** | the Visionary |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2023-12-05 |
| **Release Patch** | V13.24 |
| **Latest Changes** | V25.18 |
| **Roles** | Artillery |
| **Riot Positions** | Middle, Support |
| **External Positions** | Middle, Support |
| **Blue Essence** | 2400 |
| **Riot Points** | 880 |
| **Difficulty** | 3 |
| **Hero Type** | Mage |
| **Alt Type** | Support |
| **Adaptive Type** | Magic |
| **Damage** | 3 |
| **Toughness** | 1 |
| **Control** | 3 |
| **Mobility** | 1 |
| **Utility** | 2 |
| **Style** | 90 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $580.0$ | $+109.0$ |
| **Mana** | $480.0$ | $+30.0$ |
| **Health Regen** | $5.5$ | $+0.55$ |
| **Mana Regen** | $7.5$ | $+0.75$ |
| **Armor** | $21.0$ | $+4.7$ |
| **Magic Resist** | $30.0$ | $+1.3$ |
| **Attack Damage** | $54.0$ | $+3.3$ |
| **Attack Speed** | $0.690$ | |
| **Movement Speed** | $330.0$ | $+0.0$ |
| **Attack Range** | $550.0$ | $+0.0$ |
| **Base Attack Speed** | $0.69$ | |
| **Attack Speed Ratio** | $0.658$ | |
| **Bonus AS per Level** | $2.5\%$ | |
| **Attack Windup** | $18.7\%$ | |
| **Missile Speed** | $2800$ units/second | |
| **Acquisition Radius** | $600$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $135$ units | |
| **Selection Height** | $180$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $95.0\%$ |

## Abilities

### Passive: Signature of the Visionary

| Attribute | Value |
|-----------|------:|
| **Effect Radius** | er 285 (Center of marked champion to edge of other units) |
| **Targeting** | Passive |
| **Affects** | Self, Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | aoe |

**INNATE:** **Hwei**’s damaging abilities mark enemies hit for 4 seconds. Subsequent damaging abilities against marked targets consume the mark to create an explosion beneath them, dealing 35 to 230 (+ 35% AP) **bonus** magic damage to enemies in the area after a $0.85$-second delay.

**Notes:**

- The ability that consumes the mark cannot apply it on the same cast instance.
- The mark **cannot** be triggered from the same cast instance of an ability, even if the triggering ability was empowered by *Stirring Lights*.
- The explosion occurs around the marked target from where they were when the ability damaged them to consume the mark.
  - In other words, the explosion is at the location of where they were hit, not where they are at the end of the delay.
    - The target who had their mark consumed is able to escape the area of the explosion within the delay period.
- Enemies can be damaged by multiple explosions at once.
- Spell shield will block both the mark and its consumption as well as the detonation.
- The indicator for the effect telegraphs an unusually smaller radius than it actually hits.

---

### Q: Devastating Fire

| Attribute | Value |
|-----------|------:|
| **Range** | cr 800 / er 1075 (Range + explosion at end that is located 100 units past the missile end position) |
| **Cast Time** | $0.25$ seconds |
| **Effect Radius** | 175 (Explosion at end) units |
| **Width** | 140 units |
| **Speed** | 2000 units/second |
| **Queue Time** | $0.25$ seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | True |

**ACTIVE - QQ:** **Hwei** slings a fireball in the target direction that explodes upon colliding with the first enemy or reaching maximum range, dealing magic damage to all nearby enemies. The damage based on the target's health ratio is capped at 250 against monsters.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 50 / 80 / 110 / 140 / 170 (+ 70% AP) (+ 3 / 4 / 5 / 6 / 7% of target's **maximum** health) |

**Notes:**

- This ability will cast from wherever the caster is at the end of the cast time.

---

### Q: Fleeting Current

| Attribute | Value |
|-----------|------:|
| **Range** | 1200 units |
| **Cast Time** | $0.25$ seconds |
| **Width** | 225 (Estimated) units |
| **Queue Time** | $0.25$ seconds |
| **Targeting** | Direction |
| **Affects** | Self, Allies |
| **Projectile** | False |

**ACTIVE - WQ:** **Hwei** forms a current of water in the target direction, creating a path for a duration that grants him and allied champions **bonus** movement speed and ghosting for $0.5$ seconds, with the bonus refreshing every $0.125$ seconds while they remain in the area.

| Attribute | Value |
|-----------|------:|
| **Path Duration** | 4 / 4.5 / 5 / 5.5 / 6 seconds |

| Attribute | Value |
|-----------|------:|
| **Bonus Movement Speed** | 30 / 32.5 / 35 / 37.5 / 40% (+ 3% per 100 AP) |

**Notes:**

- This ability will cast from wherever the caster is at the start of the cast time.

---

### Q: Grim Visage

| Attribute | Value |
|-----------|------:|
| **Range** | 1100 units |
| **Cast Time** | $0.25$ seconds |
| **Width** | 140 units |
| **Speed** | 1300 units/second |
| **Queue Time** | $0.25$ seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spell |
| **Projectile** | True |

**ACTIVE - EQ:** **Hwei** launches a terrifying grin in the target direction that deals magic damage to the first enemy hit, knocks them down, and fears them for a duration, as well as slows them by 60% for the same duration.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 70 / 110 / 150 / 190 / 230 (+ 65% AP) |

| Attribute | Value |
|-----------|------:|
| **Disable Duration** | 1 / 1.125 / 1.25 / 1.375 / 1.5 seconds |

**Notes:**

- This ability will cast from wherever the caster is at the end of the cast time.

---

### Q: Subject: Disaster

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Cost** | 80 / 90 / 100 / 110 / 120 Mana (Consumes upon using a basic ability) |
| **Cooldown** | 10 / 9 / 8 / 7 / 6 seconds |
| **Cooldown Start** | Starts upon using a basic ability |
| **Queue Time** | $0.25$ seconds |
| **Targeting** | Auto |

**ACTIVE:** **Hwei** enters a disastrous mood, gaining access to its abilities as well as *Wash Brush*. He will exit the mood upon casting any of them.

**Notes:**

- **Hwei** cannot rank up this ability while he is in a mood.

---

### W: Gaze of the Abyss

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 900 units |
| **Effect Radius** | 350 (Trigger radius) / 600 (Maximum launch range) / 450 (Vision radius) units |
| **Speed** | 1700 (Launch speed) units/second |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spell |
| **Projectile** | True |
| **Out of Range Behavior** | walk to location |

**ACTIVE - EW:** **Hwei** tosses an eyeball to the target location. Upon arrival, it expands over $0.2$ seconds into a dark gaze lasting 3 seconds, granting sight in a larger area (Can see into brush and terrain). After $0.7$ seconds of being placed, the eye locks onto the nearest visible enemy champion or otherwise remains there until an enemy champion is in range. Once locked on, the eye launches itself at the target after $0.3$ seconds and collides with the first enemy hit to deal magic damage, reveal them for $2.5$ seconds, and root them for a duration.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 70 / 110 / 150 / 190 / 230 (+ 65% AP) |

| Attribute | Value |
|-----------|------:|
| **Root Duration** | 1.2 / 1.4 / 1.6 / 1.8 / 2 seconds |

**Notes:**

- The target that the eye locks onto and launches towards is revealed during the missile's flight.
- The lock-on missile's path is indicated by a dashed line.
  - Its launch range is about as wide as its vision range.
  - This is not to be confused with the eye's border which is the trigger range.

---

### W: Pool of Reflection

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.3$ seconds |
| **Target Range** | 650 units |
| **Effect Radius** | 350 units |
| **Targeting** | Location |
| **Affects** | Self, Allies |
| **Out of Range Behavior** | walk to location |

**ACTIVE - WW:** **Hwei** summons a pool of water at the target location, creating a protective zone for 3 seconds that grants him and allied champions a shield at the start of the cast time and for $0.5$ seconds while within the area. The shield refreshes and increases in strength by an amount every over the duration while they remain in the area.

| Attribute | Value |
|-----------|------:|
| **Initial Shield Strength** | 50 / 62.5 / 75 / 87.5 / 100 (+ 30% AP) |

| Attribute | Value |
|-----------|------:|
| **Bonus Shield per Tick** | 8.33 / 10.42 / 12.5 / 14.58 / 16.67 (+ 5% AP) |
| **Total Maximum Shield** | 100 / 125 / 150 / 175 / 200 (+ 60% AP) |

*Pool of Reflection*’s shield strength is reduced to 75% for allies.

| Attribute | Value |
|-----------|------:|
| **Ally Initial Shield** | 37.5 / 46.875 / 56.25 / 65.625 / 75 (+ 22.5% AP) |

| Attribute | Value |
|-----------|------:|
| **Ally Bonus Shield per Tick** | 6.25 / 7.81 / 9.38 / 10.94 / 12.5 (+ 3.75% AP) |
| **Ally Total Maximum Shield** | 75 / 93.75 / 112.5 / 131.25 / 150 (+ 45% AP) |

**Notes:**

- The pool is summoned at the start of the cast time.
- The maximum shield defines the cap for the strength, and it takes approximately $1.5$ seconds to gain the full shield.
  - The shield will be regenerated back to its cap if it mitigates damage.
  - The total amount of shield that can be generated given damage mitigated is far greater than this cap.
- The initial shield amount is inconsistent due to one or two ticks of the bonus shield being granted on-cast.

---

### W: Severing Bolt

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.5$ seconds |
| **Target Range** | 1900 units |
| **Effect Radius** | 225 units |
| **Queue Time** | $0.25$ seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Out of Range Behavior** | walk to location |

**ACTIVE - QW:** **Hwei** calls upon a lightning bolt to strike at the target location after 1 second (From start of cast time), dealing magic damage to enemies within the area.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 60 / 85 / 110 / 135 / 160 (+ 25% AP) |

If *Severing Bolt* hits only one enemy or immobilized enemies, it deals increased damage based on the . This **bonus** damage is capped at 300 against monsters.
|leveling2 = | Attribute | Value |
|-----------|------:|
| **Maximum Damage Increase** | 200 / 237.5 / 275 / 312.5 / 350% |
| **Maximum Damage** | 60×2.0 / 201.875 / 302.5 / 421.875 / 560 (+ 50 / 59.375 / 68.75 / 78.125 / 87.5% AP) |

*Severing Bolt* deals 50% damage to minions and non-epic monsters.

**Notes:**

- *Severing Bolt*’s damage against immobilized or isolated enemies is increased based on both its rank and a target's **missing** health. The list below shows the total damage multiplier at various thresholds as a percentage of the base/non-increased damage.
  - Rank 1: 1×100 to 2.0×100 for 11
  - Rank 2: 1×100 to (2.0+(3.5-2.0)*(1/4))*100 for 11
  - Rank 3: 1×100 to (2.0+(3.5-2.0)*(2/4))*100 for 11
  - Rank 4: 1×100 to (2.0+(3.5-2.0)*(3/4))*100 for 11
  - Rank 5: 1×100 to 3.5×100 for 11

---

### W: Subject: Serenity

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Cost** | 90 / 95 / 100 / 105 / 110 Mana (Consumes upon using a basic ability) |
| **Cooldown** | 18 / 17.5 / 17 / 16.5 / 16 seconds |
| **Cooldown Start** | Starts upon using a basic ability |
| **Queue Time** | $0.25$ seconds |
| **Targeting** | Auto |

**ACTIVE:** **Hwei** enters a serene mood, gaining access to its abilities as well as *Wash Brush*. He will exit the mood upon casting any of them.

**Notes:**

- **Hwei** cannot rank up this ability while he is in a mood.

---

### E: Crushing Maw

| Attribute | Value |
|-----------|------:|
| **Range** | 680 (Single rectangle length, see details) units |
| **Cast Time** | $0.35$ seconds |
| **Target Range** | 800 units |
| **Width** | 340 (Single rectangle width, see details) units |
| **Queue Time** | $0.25$ seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Out of Range Behavior** | walk to location |

**ACTIVE - EE:** **Hwei** conjures a jaw at the target location that snaps after $0.6$ seconds (From start of cast time), dealing magic damage to enemies in the area and slowing them by an amount that decays over $1.25$ seconds. Enemies that are not standing at the center when the jaw snaps are pulled there.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 70 / 110 / 150 / 190 / 230 (+ 65% AP) |

| Attribute | Value |
|-----------|------:|
| **Slow** | 40 / 47.5 / 55 / 62.5 / 70% |

**Notes:**

- Enemies that are pulled will be slowed after the displacement ends.
- The jaws are made of two rectangles in an angle of each other.

---

### E: Molten Fissure

| Attribute | Value |
|-----------|------:|
| **Range** | 1200 units |
| **Cast Time** | $0.35$ seconds |
| **Effect Radius** | 225 units |
| **Width** | 225 units |
| **Queue Time** | $0.25$ seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | Special |
| **Spell Effects** | Special |
| **Projectile** | True |

**ACTIVE - QE:** **Hwei** marks a blazing path in the target direction. After $0.6$ seconds (From start of cast time), the path erupts into volcanic explosions every $0.2$ seconds (Estimated) from the point of cast, for a total of 7 explosions. Each explosion creates a shockwave that deals magic damage to nearby enemies.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 20 / 40 / 60 / 80 / 100 (+ 30% AP) |

Each explosion also leaves a lava fissure in its wake. A fissure lasts for $2.5$ seconds, dealing magic damage every $0.25$ seconds to enemies within the area and slowing them by 30%.

| Attribute | Value |
|-----------|------:|
| **Magic Damage per Tick** | 5 / 10 / 15 / 20 / 25 (+ 5% AP) |
| **Total Fissure Magic Damage** | 50 / 100 / 150 / 200 / 250 (+ 50% AP) |
| **Total Magic Damage** | 70 / 140 / 210 / 280 / 350 (+ 80% AP) |

*Molten Fissure*’s damage is reduced to 60% against minions and increased to 135% against monsters.

**Notes:**

- Applies area damage for the explosions and deals persistent area damage for the lava.
- The shockwaves can only hit each enemy once.
- Enemies will take damage over time for each lava fissure they are standing in.
- Spell shield will block an explosion but not the persistent lava damage.
- This ability will cast from wherever the caster is at the start of the cast time.

---

### E: Stirring Lights

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Queue Time** | $0.25$ seconds |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | False |
| **Spell Effects** | proc |

**ACTIVE - WE:** **Hwei** surrounds himself in swirling flares that empower his next 3 basic attacks or ability hits within 9 seconds to each deal **bonus** magic damage and restore mana.

| Attribute | Value |
|-----------|------:|
| **Bonus Magic Damage** | 20 / 30 / 40 / 50 / 60 (+ 15% AP) |
| **Maximum Magic Damage** | 60 / 90 / 120 / 150 / 180 (+ 45% AP) |

| Attribute | Value |
|-----------|------:|
| **Mana Restore** | 45 / 50 / 55 / 60 / 65 |
| **Total Mana Restore** | 135 / 150 / 165 / 180 / 195 |

*Stirring Lights' * **bonus** damage is reduced to 50% against minions or monsters if applied by his area of effect abilities.

| Attribute | Value |
|-----------|------:|
| **Reduced Bonus Damage** | 10 / 15 / 20 / 25 / 30 (+ 7.5% AP) |

**Notes:**

- *Stirring Lights' * bonus damage to non-champions is reduced if applied by *Devastating Fire*, *Severing Bolt*, *Molten Fissure*, *Crushing Maw*, or *Spiraling Despair*.
- Spell shield will not block the bonus damage even if it is applied by an ability.

---

### E: Subject: Torment

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Cost** | 50 / 55 / 60 / 65 / 70 Mana (Consumes upon using a basic ability) |
| **Cooldown** | 13 / 12.5 / 12 / 11.5 / 11 seconds |
| **Cooldown Start** | Starts upon using a basic ability |
| **Queue Time** | $0.25$ seconds |
| **Targeting** | Auto |

**ACTIVE:** **Hwei** enters a tumultuous mood, gaining access to its abilities as well as *Wash Brush*. He will exit the mood upon casting any of them.

**Notes:**

- **Hwei** cannot rank up this ability while he is in a mood.

---

### R: Spiraling Despair

| Attribute | Value |
|-----------|------:|
| **Range** | 1340 (1300 missile travel distance plus 40 to-edge radius check if nothing has been hit yet) units |
| **Cast Time** | $0.25$ seconds |
| **Effect Radius** | cr 500 (Detonation radius) |
| **Width** | 180 (Missile full width) units |
| **Speed** | 1400 units/second |
| **Cost** | 100 Mana |
| **Cooldown** | 140 / 115 / 80 seconds |
| **Queue Time** | $0.25$ seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | Special |
| **Spell Effects** | Special |
| **Projectile** | True |

**ACTIVE:** **Hwei** launches a globule of pure despair in the target direction that collides with the first enemy champion hit, afflicting them with an aura that grows over 3 seconds, reveals the target, and grants sight within its radius. Enemies within are both dealt magic damage and applied a stack of *Despair* every $0.25$ seconds.

| Attribute | Value |
|-----------|------:|
| **Magic Damage per Tick** | 2.5 / 3.75 / 5 / 6.25 / 7.5 (+ 1.25% AP) |
| **Total Magic Damage** | 30 / 45 / 60 / 75 / 90 (+ 15% AP) |

**DESPAIR:** For each stack, the target is slowed by 10% for $0.25$ seconds, stacking up to 12 times.

At the end of the duration or when the target dies, the aura explodes to deal magic damage to enemies within and remove all *Despair* stacks from affected enemies.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 200 / 262.5 / 325 / 387.5 / 450 (+ 80% AP) |
| **Maximum Total Damage** | 230 / 307.5 / 385 / 462.5 / 540 (+ 95% AP) |

*Spiraling Despair can only be cast if **Hwei** has not entered a mood.*

**Notes:**

- Applies persistent area damage for the aura and deals area damage for the explosion.
- Spell shield can block the aura, explosion, and the application of the first *Despair* stack.
- This ability will cast from wherever the caster is at the end of the cast time.

---

### R: Wash Brush

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Targeting** | Auto |
| **Affects** | Self |

**ACTIVE - QR, WR, ER:** **Hwei** exits his current *mood* without incurring its ability cost or cooldown.

*Wash Brush can only be cast if **Hwei** has entered a mood, and can be used while affected by cast-inhibiting crowd control.*

**Notes:**

No additional notes.

---

## Patch History

### V25.18
- Signature of the Visionary
  - AP ratio increased to 35% AP from 30% AP.
- Fleeting Current
  - Base bonus movement speed increased to 30 / 32.5 / 35 / 37.5 / 40% from 20 / 22.5 / 25 / 27.5 / 30%.
  - Bonus movement speed AP ratio increased to 3% per 100 AP from 2% per 100 AP.
- Grim Visage
  - AP ratio increased to 65% AP from 60% AP.
- Gaze of the Abyss
  - AP ratio increased to 65% AP from 60% AP.
- Crushing Maw
  - AP ratio increased to 65% AP from 60% AP.

### V25.13
- Signature of the Visionary
  - **Bug Fixes:** After marking an enemy using Gaze of the Abyss and using another ability to trigger the explosion, the explosion no longer incorrectly casts at the location of a previously slain target who had been marked by *Signature of the Visionary* but its effect was not consumed to trigger an explosion, instead of at the location of the first.

### V25.10
- Molten Fissure
  - **Bug Fixes:** Spawned fissures no longer prevent minions' pathing.

### V25.08
- Signature of the Visionary
  - Base damage increased to 35 to 230 from 35 to 180.
- Spiraling Despair
  - Explosion base damage increased to 200 / 325 / 450 from 200 / 300 / 400.

### V25.04
- Signature of the Visionary
  - AP ratio reduced to 30% AP from 35% AP.
- Severing Bolt
  - Base damage reduced to 60 / 85 / 110 / 135 / 160 from 80 / 100 / 120 / 140 / 160.
- Stirring Lights
  - Base damage per hit reduced to 20 / 30 / 40 / 50 / 60 from 25 / 35 / 45 / 55 / 65.
    - Total base damage reduced to 60 / 90 / 120 / 150 / 180 from 75 / 105 / 135 / 165 / 195.
  - AP ratio per hit reduced to 15% AP from 20% AP.
    - Total AP ratio reduced to 45% AP from 60% AP.

### V14.22
- General
  - **Bug Fixes:** Basic attack missile VFX now spawns at the brush's tip instead of **Hwei**’s center.

### V14.21
- Devastating Fire
  - Base damage reduced to 50 / 80 / 110 / 140 / 170 from 60 / 90 / 120 / 150 / 180.
  - AP ratio reduced to 70% AP from 75% AP.
- Subject: Torment
  - Cooldown increased to 13 / 12.5 / 12 / 11.5 / 11 seconds from 12 / 11.5 / 11 / 10.5 / 10.

### V14.18
- Pool of Reflection
  - Shield strength on allies increased to 75% from 50%.

### V14.11
- Devastating Fire
  - Health ratio reduced to 3 / 4 / 5 / 6 / 7% of target's **maximum** health from 4 / 5 / 6 / 7 / 8%.
- Molten Fissure
  - Fissure AP ratio per tick reduced to 5% AP from 6.25% AP.
    - Fissure total AP ratio reduced to 50% AP from $62.5$% AP.

### V14.10
- Wash Brush
  - **Bug Fixes:** After the **Hwei** player disconnects, no longer locks **Hwei** in a permanent state of having no abilities due to activating one of his subjects but not using an ability before disconnecting.

## Trivia

- **Hwei** is currently the champion with the largest number of abilities in League of Legends: 11 when excluding his subject selection spells, and 15 when including them.
- **Hwei**’s kit obeys the Rule of thirds in multiple aspects:
  - He has 3×3 basic abilities.
  - His passive requires two damaging abilities in order to deal a third damaging ability.
  - He studies two negative subjects and one positive subject.
- Serenity, **Hwei**’s positive mood, is placed between his other two negative moods, anger and anguish. This fact stems from lore implications, namely his inner turmoil, as detailed in his universe short stories: "Hwei faces the conflicting hues of Ionia" in The Visionary and "Art saves me, yet it can shatter me" in Paintings Framed in Half-Light.
- When **Hwei** dies, his eyes, which reflect his current mood, fade into a colorless blankness, symbolizing the absence of any emotional presence in death.
- **Hwei**’s name comes from Chinese, being the character **彗** (Huì), meaning "comet".

---
*This page was automatically generated from League of Legends Wiki data.*