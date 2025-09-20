# Hwei

## Table of Contents
- [Basic Information](#basic-information)
- [Statistics](#statistics)
- [Abilities](#abilities)
- [Patch History](#patch-history)
- [Trivia](#trivia)

## Basic Information

| Attribute | Value |
|-----------|-------|
| **Champion** | Hwei |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $550$ | $+85$ | $1995.0$ |
| **Mana** | $350$ | $+50$ | $1200.0$ |
| **Armor** | $22$ | $+3.5$ | $81.5$ |
| **Magic Resist** | $30$ | $+0.5$ | $38.5$ |
| **Attack Damage** | $56$ | $+3.1$ | $108.7$ |
| **Attack Speed** | $0.625$ | $+3.2\%$ | $0.965$ |

## Abilities

### Passive: Signature of the Visionary

**Innate:** damaging abilities mark enemies hit for a short time. Damaging them with a different ability consumes the mark to create an explosion at their location that deals magic damage in an area.

**Innate:** ''Hwei's** damaging [abilities](./champion_ability.md) mark enemies hit for 4 seconds. Subsequent damaging abilities against marked targets consume the mark to create an explosion beneath them, dealing $35 to 230$ (+ 35% AP) **bonus'' magic damage to enemies in the area after a $0.85$-second delay.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | targeting = Passive |

**Notes:**

- The ability that consumes the mark cannot apply it on the same cast instance.
- The mark **cannot** be triggered from the same cast instance of an ability, even if the triggering ability was empowered by **Stirring Lights**.
- The explosion occurs around the marked target from where they were when the ability damaged them to consume the mark.
  - In other words, the explosion is at the location of where they were hit, not where they are at the end of the delay. *** The target who had their mark consumed is able to escape the area of the explosion within the delay period.
- Enemies can be damaged by multiple explosions at once.
- Spell shield will block both the mark and its consumption as well as the detonation.
- The indicator for the effect telegraphs an unusually smaller radius than it actually hits.

---

### Q: Devastating Fire

**Active - QQ:** **Hwei** slings a fireball in the target direction, dealing magic damage to enemies hit.

**Active - QQ:** **Hwei** slings a fireball in the target direction that explodes upon colliding with the first enemy or reaching maximum range, dealing magic damage to all nearby enemies. The damage based on the target's health ratio is capped at 250 against monsters.

| Attribute | Value |
|-----------|-------|
| **Cast Time** | $0.25$ seconds |

**Scaling:**
- **Magic Damage:** $50-170$ (+ 70% AP) (+ $3-7$% of target's
- **maximum** health)

**Notes:**

No additional notes.

---

### Q: Fleeting Current

**Active - WQ:** **Hwei** forms a current of water in the target direction that grants him and allied champions **bonus movement speed** and ghosting while within it.

**Active - WQ:** **Hwei** forms a current of water in the target direction, creating a path for a duration that grants him and allied champions **bonus movement speed** and ghosting for $0.5$ seconds, with the bonus refreshing every $0.125$ seconds while they remain in the area.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | cdstart = |
| **Cast Time** | $0.25$ seconds |

**Scaling:**
- **Path Duration:** $4-6$ seconds
- **Bonus Movement Speed:** $30-40$% (+ 3% per 100 AP)

**Notes:**

No additional notes.

---

### Q: Grim Visage

**Active - EQ:** **Hwei** launches a terrifying grin in the target direction that deals magic damage to the first enemy hit, knockdown and fear and slow them.

**Active - EQ:** **Hwei** launches a terrifying grin in the target direction that deals magic damage to the first enemy hit, knockdown, and fear them for a duration, as well as slow them by 60% for the same duration.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | cdstart = |
| **Cast Time** | $0.25$ seconds |

**Scaling:**
- **Magic Damage:** $70-230$ (+ 65% AP)
- **Disable Duration:** $1-1.5$ seconds

**Notes:**

No additional notes.

---

### Q: Subject: Disaster

**Active:** **Hwei** enters a disastrous mood, gaining access to its abilities as well as **Wash Brush**. He will exit the mood upon casting any of them. * **Devastating Fire** (QQ) * **Severing Bolt** (QW) * **Molten Fissure** (QE)

**Active:** **Hwei** enters a disastrous mood, gaining access to its abilities as well as **Wash Brush**. He will exit the mood upon casting any of them.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $10-6$ seconds |
| **Cast Time** | none |
| **Cost** | $80-120$ Mana |

**Notes:**

- **Hwei** cannot rank up this ability while he is in a mood.

---

### W: Gaze of the Abyss

**Active - EW:** **Hwei** tosses an eyeball to the target location that expands into a dark gaze, granting sight in a larger area and locking onto the nearest visible enemy champion. The eye then launches itself at the locked-on target, dealing magic damage to the first enemy it collides with, root and true sight them.

**Active - EW:** **Hwei** tosses an eyeball to the target location. Upon arrival, it expands over $0.2$ seconds into a dark gaze lasting 3 seconds, granting sight in a larger area. After $0.7$ seconds of being placed, the eye locks onto the nearest visible enemy champion or otherwise remains there until an enemy champion is in range. Once locked on, the eye launches itself at the target after $0.3$ seconds and collides with the first enemy hit to deal magic damage, true sight them for $2.5$ seconds, and root them for a duration.

| Attribute | Value |
|-----------|-------|
| **Range** | 900 units |
| **Cooldown** | cdstart = |
| **Cast Time** | $0.25$ seconds |

**Scaling:**
- **Magic Damage:** $70-230$ (+ 65% AP)
- **Root Duration:** $1.2-2$ seconds

**Notes:**

- The target that the eye locks onto and launches towards is true sight during the missile's flight.
- The lock-on missile's path is indicated by a dashed line.
  - Its launch range is about as wide as its vision range.
  - This is not to be confused with the eye's border which is the trigger range.

---

### W: Pool of Reflection

**Active - WW:** **Hwei** summons a protective water pool at the target location that grants him and allied champions a shield for a few seconds that increases in strength if they remain within it.

**Active - WW:** **Hwei** summons a pool of water at the target location, creating a protective zone for 3 seconds that grants him and allied champions a shield at the start of the cast time and for $0.5$ seconds while within the area. The shield refreshes and increases in strength by an amount every over the duration while they remain in the area. 'Pool of Reflection's' shield strength is reduced to 75% for allies.

| Attribute | Value |
|-----------|-------|
| **Range** | 650 units |
| **Cooldown** | cdstart = |
| **Cast Time** | $0.3$ seconds |

**Scaling:**
- **Initial Shield Strength:** $50-100$ (+ 30% AP)
- **Bonus Shield per Tick:** $50/6-100/6 round=2$ (+ $30/6 Total Maximum Shield $100-200$ (+ 60% AP)
- **Ally Initial Shield:** $50*0.75-100*0.75$ (+ $30*0.75$% AP)
- **Ally Bonus Shield per Tick:** $50*0.75/6-1000.75/60.75/6 Ally Total Maximum Shield $100*0.75-200*0.75$ (+ $60*0.75$% AP)

**Notes:**

- The pool is summoned at the start of the cast time.
- The maximum shield defines the cap for the strength, and it takes approximately $1.5$ seconds to gain the full shield.
  - The shield will be regenerated back to its cap if it mitigates damage.
  - The total amount of shield that can be generated given damage mitigated is far greater than this cap.
- The initial shield amount is inconsistent due to one or two ticks of the bonus shield being granted on-cast.

---

### W: Severing Bolt

**Active - QW:** **Hwei* strikes the target area with a lightning bolt, dealing magic damage to enemies within the area. Isolated or immobilize enemies are dealt ccs*missing** health.

**Active - QW:** **Hwei** calls upon a lightning bolt to strike at the target location after 1 second, dealing magic damage to enemies within the area. If *Severing Bolt hits only one enemy or immobilize enemies, it deals ccs*bonus** damage is capped at 300 against monsters. *Severing Bolt* deals 50% damage to minions and non-epic monsters.

| Attribute | Value |
|-----------|-------|
| **Range** | 1900 units |
| **Cooldown** | cdstart = |
| **Cast Time** | $0.5$ seconds |

**Scaling:**
- **Magic Damage:** $ (+
- **Maximum Damage Increase:** $% (+(-)*(1/4))*(+(-)(1/4))(2/4))*(+(-)(2/4))(3/4))*(+(-)(3/4)) (+ $*% AP)

**Notes:**

- 'Severing Bolt's' damage against immobilized or isolated enemies is increased based on both its rank and a target's **missing** health. The list below shows the total damage multiplier at various thresholds as a percentage of the base/non-increased damage.
  - Rank 1: $key1=%$%*100 for 11 health
  - Rank 2: $key1=%$%+()*(1/4))*100 for 11/0 to 100 showtype=false health
  - Rank 3: $key1=%$%+()*(2/4))*100 for 11/0 to 100 showtype=false health
  - Rank 4: $key1=%$%+()*(3/4))*100 for 11/0 to 100 showtype=false health
  - Rank 5: $key1=%$%*100 for 11 health

---

### W: Subject: Serenity

**Active:** **Hwei** enters a serene mood, gaining access to its abilities as well as **Wash Brush**. He will exit the mood upon casting any of them. * **Fleeting Current** (WQ) * **Pool of Reflection** (WW) * **Stirring Lights** (WE)

**Active:** **Hwei** enters a serene mood, gaining access to its abilities as well as **Wash Brush**. He will exit the mood upon casting any of them.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $18-16$ seconds |
| **Cast Time** | none |
| **Cost** | $90-110$ Mana |

**Notes:**

- **Hwei** cannot rank up this ability while he is in a mood.

---

### E: Crushing Maw

**Active - EE:** **Hwei** summons a jaw at the target location that snaps, dealing magic damage to all enemies in the area and applying a decaying slow to them. Enemies that are not standing at the center of the jaw when it snaps are airborne there.

**Active - EE:** **Hwei** conjures a jaw at the target location that snaps after $0.6$ seconds, dealing magic damage to enemies in the area and slow them by an amount that decays over $1.25$ seconds. Enemies that are not standing at the center when the jaw snaps are airborne there.

| Attribute | Value |
|-----------|-------|
| **Range** | 800 units |
| **Cooldown** | cdstart = |
| **Cast Time** | $0.35$ seconds |

**Scaling:**
- **Magic Damage:** $70-230$ (+ 65% AP)
- **Slow:** $40-70$%

**Notes:**

- Enemies that are pulled will be slowed after the displacement ends.
- The jaws are made of two rectangles in an angle of each other.

---

### E: Molten Fissure

**Active - QE:** **Hwei** marks a blazing path in a direction that erupts after a delay into explosions. Each explosion deals magic damage and leaves behind a volcanic fissure that continually deals magic damage and slow enemies within.

**Active - QE:** **Hwei** marks a blazing path in the target direction. After $0.6$ seconds, the path erupts into volcanic explosions every $0.2$ seconds from the point of cast, for a total of 7 explosions. Each explosion creates a shockwave that deals magic damage to nearby enemies. Each explosion also leaves a lava fissure in its wake. A fissure lasts for $2.5$ seconds, dealing magic damage every $0.25$ seconds to enemies within the area and slow them by 30%. 'Molten Fissure's' damage is reduced to 60% against minions and increased to 135% against monsters.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | cdstart = |
| **Cast Time** | $0.35$ seconds |

**Scaling:**
- **Magic Damage:** $20-100$ (+ 30% AP)
- **Magic Damage per Tick:** $50/10-250/10$ (+ $50/10$% AP) Total Fissure Magic Damage $50-250$ (+ 50% AP) Total Magic Damage $50+20-250+100$ (+ $50+30$% AP)

**Notes:**

- Applies area damage for the explosions and deals persistent area damage for the lava.
- The shockwaves can only hit each enemy once.
- Enemies will take damage over time for each lava fissure they are standing in.
- Spell shield will block an explosion but not the persistent lava damage.

---

### E: Stirring Lights

**Active - WE:** **Hwei** surrounds himself in swirling flares that empower his next 3 basic attacks or abilities to deal **bonus** magic damage and restore *mana*.

**Active - WE:** **Hwei** surrounds himself in swirling flares that empower his next 3 [basic attacks](./basic_attacks.md) or [ability](./champion_ability.md) hits within 9 seconds to each deal **bonus** magic damage and restore *mana*. 'Stirring Lights' ' **bonus** damage is reduced to 50% against minions or monsters if applied by his area of effect abilities.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | cdstart = |
| **Cast Time** | none |

**Scaling:**
- **Bonus Magic Damage:* $20-60$ (+ 15% AP)3-60*3$ (+ $15*3$% AP) **Mana Restore:** $135/3-195/3$ Total Mana Restore $135-195$
- **Reduced Bonus Damage:** $20*0.5-60*0.5$ (+ $15*0.5$% AP)

**Notes:**

- 'Stirring Lights' * bonus damage to non-champions is reduced if applied by **Devastating Fire**, **Severing Bolt**, **Molten Fissure**, **Crushing Maw**, or **Spiraling Despair*'.
- Spell shield will not block the bonus damage even if it is applied by an ability.

---

### E: Subject: Torment

**Active:** **Hwei** enters a tumultuous mood, gaining access to its abilities as well as **Wash Brush**. He will exit the mood upon casting any of them. * **Grim Visage** (EQ) * **Gaze of the Abyss** (EW) * **Crushing Maw** (EE)

**Active:** **Hwei** enters a tumultuous mood, gaining access to its abilities as well as **Wash Brush**. He will exit the mood upon casting any of them.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $13-11$ seconds |
| **Cast Time** | none |
| **Cost** | $50-70$ Mana |

**Notes:**

- **Hwei** cannot rank up this ability while he is in a mood.

---

### R: Spiraling Despair

**Active:** **Hwei** launches a globule in the target direction that collides with the first enemy champion hit, afflicting them with an [aura](./aura.md) that grows over a few seconds. Enemies within are continually dealt magic damage and gradually slow.

*After the duration ends or when the target dies, the aura explodes to deal magic damage to all enemies within.*

**Active:** **Hwei** launches a globule of pure despair in the target direction that collides with the first enemy champion hit, afflicting them with an [aura](./aura.md) that grows over 3 seconds, true sight the target, and grants sight within its radius. Enemies within are both dealt magic damage and applied a stack of *Despair* every $0.25$ seconds. **Despair:** For each stack, the target is slow by 10% for $0.25$ seconds, stacking up to 12 times. At the end of the duration or when the target dies, the aura explodes to deal magic damage to enemies within and remove all *Despair* stacks from affected enemies. *Spiraling Despair can only be cast if **Hwei** has not entered a mood.*

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $140/115/80$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | 100 Mana |

**Scaling:**
- **Magic Damage per Tick:* $10/4-30/4$ (+ $5/4$% AP)3-30*3$ (+ $5*3$% AP)
- **Magic Damage:* $200-450$ (+ 80% AP)3-450+30*3$ (+ $80+5*3$% AP)

**Notes:**

- Applies persistent area damage for the aura and deals area damage for the explosion.
- Spell shield can block the aura, explosion, and the application of the first *Despair* stack.

---

### R: Wash Brush

**Active:** **Hwei** exits his current *mood* without incurring its ability cost or cooldown.

**Active - QR, WR, ER:** **Hwei** exits his current *mood* without incurring its ability cost or cooldown. *Wash Brush can only be cast if **Hwei** has entered a mood, and can be used while affected by cast-inhibiting crowd control.*

| Attribute | Value |
|-----------|-------|
| **Cast Time** | none |

**Notes:**

No additional notes.

---

## Patch History

### V25.18
- *Signature of the Visionary*
  - AP ratio increased to 35% AP from 30% AP.
- *Fleeting Current*
  - Base bonus movement speed increased to $30-40$% from $20-30$%.
  - Bonus movement speed AP ratio increased to 3% per 100 AP from 2% per 100 AP.
- *Grim Visage*
  - AP ratio increased to 65% AP from 60% AP.
- *Gaze of the Abyss*
  - AP ratio increased to 65% AP from 60% AP.
- *Crushing Maw*
  - AP ratio increased to 65% AP from 60% AP.

### V25.13
- *Signature of the Visionary*
  - ***Bug Fixes:*** After marking an enemy using *Gaze of the Abyss* and using another ability to trigger the explosion, the explosion no longer incorrectly casts at the location of a previously slain target who had been marked by *Signature of the Visionary* but its effect was not consumed to trigger an explosion, instead of at the location of the first.

### V25.10
- *Molten Fissure*
  - ***Bug Fixes:*** Spawned fissures no longer prevent minions' pathing.

### V25.08
- *Signature of the Visionary*
  - Base damage increased to $35 to 230$ from $35 to 180$.
- *Spiraling Despair*
  - Explosion base damage increased to $200-450 3$ from $200-400 3$.

### V25.04
- *Signature of the Visionary*
  - AP ratio reduced to 30% AP from 35% AP.
- *Severing Bolt*
  - Base damage reduced to $60-160$ from $80-160$.
- *Stirring Lights*
  - Base damage per hit reduced to $20-60$ from $25-65$.
    - Total base damage reduced to $20*3-60*3$ from $25*3-65*3$.
  - AP ratio per hit reduced to 15% AP from 20% AP.
    - Total AP ratio reduced to $15*3$% AP from $20*3$% AP.

### V14.22
- General
  - ***Bug Fixes:*** Basic attack missile VFX now spawns at the brush's tip instead of ''Hwei's'' center.

### V14.21
- *Devastating Fire*
  - Base damage reduced to $50-170$ from $60-180$.
  - AP ratio reduced to 70% AP from 75% AP.
- *Subject: Torment*
  - Cooldown increased to $13-11$ seconds from $12-10$.

### V14.18
- *Pool of Reflection*
  - Shield strength on allies increased to 75% from 50%.

### V14.11
- *Devastating Fire*
  - Health ratio reduced to $3-7$% of target's **maximum** health from $4-8$%.
- *Molten Fissure*
  - Fissure AP ratio per tick reduced to $50/10$% AP from $62.5/10$% AP.
    - Fissure total AP ratio reduced to 50% AP from $62.5$% AP.

### V14.10
- *Wash Brush*
  - ***Bug Fixes:*** After the **Hwei** player disconnects, no longer locks **Hwei** in a permanent state of having no abilities due to activating one of his subjects but not using an ability before disconnecting.

## Trivia

- **Hwei** is currently the champion with the largest number of abilities in League of Legends: 11 when excluding his subject selection spells, and 15 when including them.
- ''Hwei's'' kit obeys the Rule of thirds in multiple aspects:
  - He has 33 basic abilities.
  - His passive requires two damaging abilities in order to deal a third damaging ability.
  - He studies two negative subjects and one positive subject.
- *Subject: Serenity*, ''Hwei's'' positive mood, is placed between his other two negative moods, *Subject: Disaster* and *Subject: Torment*. This fact stems from lore implications, namely his inner turmoil, as detailed in his universe short stories: "Hwei faces the conflicting hues of Ionia" in [The Visionary](./The_Visionary.md) and "Art saves me, yet it can shatter me" in [Paintings Framed in Half-Light](./Paintings_Framed_in_Half-Light.md).
- When **Hwei** dies, his eyes, which reflect his current mood, fade into a colorless blankness, symbolizing the absence of any emotional presence in death.
- ''Hwei's** name comes from Chinese, being the character **彗'' (Huì), meaning "comet".

---
*This page was automatically generated from League of Legends Wiki data.*