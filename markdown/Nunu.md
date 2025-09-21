# Nunu

## Table of Contents
- [Basic Information](#basic-information)
- [Abilities](#abilities)
- [Patch History](#patch-history)
- [Trivia](#trivia)

## Basic Information

| Attribute | Value |
|-----------|-------|
| **Champion** | Nunu |

## Abilities

### Passive: Call of the Freljord

**Innate:** When **Nunu** and Willump deal damage against an enemy champion, large monster or turrets, they and a nearby allied champion gain *Call of the Freljord* for a few seconds, prioritizing the ally with **highest attack speed**.

*Call of the Freljord* cannot be gained from the same enemy again for some time.

**Innate:** When **Nunu** and **Willump** deal damage against an enemy champion, large monster or turrets, they and a nearby allied champion gain *Call of the Freljord* for 4 seconds, prioritizing the ally with **highest attack speed**. **Call of the Freljord:** Gain *20% *bonus attack speed* and ms*bonus** movement speed*. ''Willump's' basic attack additionally deal 30% AD physical damage to secondary targets in a cone in front of him. **Nunu** and **Willump** cannot trigger *Call of the Freljord* from the same enemy again for a time. Successive triggers of *Call of the Freljord* while it is active will extend the duration by 4 seconds.

| Attribute | Value |
|-----------|-------|
| **Range** | 1000 / 700 units |
| **Targeting** | Passive |
| **Affects** | Self, Allies |
| **Damage Type** | Physical |
| **Spell Effects** | periodic |

**Notes:**

- The splash damage will critically strike when the basic attack does, but will not deal any bonus damage.
- *Call of the Freljord* range to be shared with allies is based on **Nunu** and ''Willump's' location.
  - If no ally is prioritized based on their stats, it will be given to the one closest to them.
- Enemies that can trigger *Call of the Freljord* have a marker overhead.
  - This marker isn't present on inhibitors or the nexus (because they cannot hold buffs and debuffs including the one that hosts this VFX), but they still trigger *Call of the Freljord* when attacked.
- 'Call of the Freljord's maximum remaining duration is 44 seconds.
  - The buff has no cap for its maximum total duration.

---

### Q: Consume

**Active:** Willump takes a bite out of the target enemy, dealing damage and healing himself. The heal is increased when he is below half health.

*Against minions and monsters he deals *true damage*.*

**Active:** **Willump** takes a bite out of the target enemy, dealing damage and healing himself. The heal is increased by $(% while he is below 50% **maximum** health. Against minions and monsters, he deals *true damage*. Additionally, if *Consume* would kill the target minion or small or medium monster, **Willump** devours the target, stun and airborne them towards him over the cast time. Against champions, he deals magic damage and the heal is reduced to $%.

| Attribute | Value |
|-----------|-------|
| **Range** | 125 units |
| **Cooldown** | $12-8$ seconds |
| **Cast Time** | $0.3$ seconds |
| **Cost** | 60 Mana |
| **Targeting** | Unit |
| **Affects** | Enemies, Self |
| **Damage Type** | Magic True |
| **Spell Shield** | True |
| **Spell Effects** | spell |

**Scaling:**
- **Non-Champion True Damage:** $400-1200$
- **Base Non-Champion Heal:** $65-185$ (+ 90% AP) (+ 10% **bonus* health) (+ $90* (+ $10*
- **Champion Magic Damage:** $60-220$ (+ 65% AP) (+ 5%
- **bonus** health)
- **Base Champion Heal:** $65* (+ $90* (+ $10* Empowered Champion Heal $65* to 185** (+ $90*% AP) (+ $10*%
- **bonus** health)

**Notes:**

- Icon border glows when he is within the maximum healing threshold.
- Spell shield will block the damage and the heal.
- While not on *cooldown*, 'Consume's ability icon on the HUD can be pinged to announce its damage to monsters in the ally chat.

---

### W: Biggest Snowball Ever!

**Active:* Willump becomes slow-immune and ghosted, and channel for a long duration to create a ccib*Willump** can gradually steer the snowball during this time.

*When hitting minions and small monsters, the snowball rolls over them, dealing magic damage. When hitting an enemy champion, large monster or terrain, the snowball instead explodes, dealing magic damage to nearby enemies, and briefly airborne then stun them based on the snowball's size. *Biggest Snowball Ever!* can be recast during the channel, and does so automatically afterwards or if interrupt.*

**Active:** **Willump* cleanse himself from all slow and channel for up to 10 seconds, becoming ghosted and slow-immune and creating a rolling ccib*Willump** automatically navigates his movement in the direction of the cursor and can steer the snowball at an increasing speed over time, which resets upon changing direction. The duo have their *movement speed* reduced by 50% for the first 1 second, but after a delay they start gaining Estimated every $0.25$ seconds, up to a cap. The snowball increases in and power linearly over the duration. Its changes from *small* to *medium* after $2.5$ seconds, then to *large* after 5 seconds into the charge. *Biggest Snowball Ever!* can be recast after $0.5$ seconds during the channel, and does so automatically after the channel or if interrupt. The snowball explodes upon hitting an enemy champion, pet, medium or large monster, or terrain, dealing magic damage to nearby enemies, increased by type=charge time, airborne for type=charge time seconds and subsequently stun them for type=charge time seconds. The snowball rolls over enemy minion and small monsters hit, dealing $33.3$% of the damage as magic damage. **Recast:** **Willump** releases the snowball, sending it rolling forward up to a maximum distance during which it will not grow any larger and will explode upon hitting an enemy.

| Attribute | Value |
|-----------|-------|
| **Range** | type=charge time |
| **Cooldown** | 14 seconds |
| **Cast Time** | none |
| **Cost** | $50-70$ Mana |
| **Targeting** | Direction |
| **Affects** | Self, Enemies |
| **Damage Type** | Magic |
| **Speed** | 425 / 435 – 594 / 595 units/second |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | Special |

**Scaling:**
- **Minimum Magic Damage:* $36-72$ (+ 30% AP)5-72×5$ (+ $30×5$% AP)

**Notes:**

- The movement speed reduction the duo receive from initially casting *Biggest Snowball Ever!* is not considered a slow, thus it will not be resisted by their slow-immune.
- The time threshold to begin gaining bonus movement speed may be bugged, as Flat Bonus MS increases the time while % Bonus MS decreases the time.
  - The formula appears to be (X ÷ 100) - Y, where: ** X = Total*** Flat MS after caps *** Y = $0.1$ per 1% %MS
  - The delay is not modified from its initially calculated value on-cast, even if additional movement speed is gained.
- **Nunu and Willump** keep all increases in movement speed at time of cast for the duration of the charge.
- ''Nunu and Willump's** movement speed always increases in increments of ~14, reaching the greater value between
  - This takes 6 increments to reach the **Total** MS + 85, but may take more to reach the Uncapped Flat + 85. Due to flat bonuses increasing the time threshold, it is possible for the ability's effects to end before reaching the **total** uncapped movement speed.
  - The **bonus** movement speed ignores the movement speed cap.
- If **Nunu** turns continuously he will create a 600 unit circle, increased with **bonus** movement speed, striking the same point up to 3 times, for a maximum of $180 (+ 300% AP) magic damage. The damage can be further increased by detonating the snowball, for a **total** of $360 (+450% AP) magic damage.
- Realm Warp will only teleport **Nunu and Willump**, leaving the behind.
- Hovering the cursor on the HUD (e.g mini map.md) / champion portraits) does not affect the cast and steering of *Biggest Snowball Ever!*.
- **Nunu** gains a slightly larger field of view during *Biggest Snowball Ever!*.
- Projectile effects will destroy the snowball only after it has been released.
- *Biggest Snowball Ever!* is disabled for the first 15 seconds of the game.
- Displacement immunity will not resist the application of the stun.
- The following table refers for interactions while **Nunu & Willump** are channel:

---

### E: Snowball Barrage

**Active:** **Nunu** throws a volley of 3 snowballs in the target direction that each shatter upon hitting an enemy, dealing magic damage to enemies in a cone. Enemies hit 3 times are briefly slow. Champions and large monsters hit once are marked *Snowbound* for a few seconds. Shortly after activating *Snowball Barrage*, *Snowbound* enemies near Willump take magic damage and are briefly root.

*Snowball Barrage* can be recast twice within a short time.

**Active:** **Nunu** throws a volley of 3 snowballs in the target direction over $0.4$ seconds that each shatter upon hitting an enemy, dealing magic damage to enemies in a cone. Enemies hit 3 times are slow for 1 second. Enemy champions and large monsters hit once are marked *Snowbound* for 4 seconds, refreshing on subsequent hits. *Snowball Barrage* can be recast twice within 3 seconds at no additional cost and with a cooldown in between casts. **Recast:** **Nunu** hurls another volley of snowballs, mimicking the first cast's effects. An enemy may only be slow once for being hit by snowballs. After 3 seconds from the first cast, *Snowbound* enemies near **Willump** take magic damage and are root for 0.5 to 1.5 seconds.

| Attribute | Value |
|-----------|-------|
| **Range** | 700 units |
| **Cooldown** | $14-10$ seconds |
| **Cast Time** | None / None |
| **Cost** | $50-70$ Mana |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Speed** | 2000 units/second |
| **Spell Shield** | True |
| **Spell Effects** | aoe |
| **Projectile** | True |

**Scaling:**
- **Magic Damage Per Hit:* $16-48$ (+ 15% AP)3-48×3$ (+ $15×3$% AP)
- **Slow:** $30-50$%
- **Maximum Total Magic Damage:** $16×3×3-48×3×3$ (+ $15×3×3$% AP)

**Notes:**

- Each cast volley of snowballs counts as an ability activation for the purposes of on-cast effects such as Spellblade and triggering *Force Pulse*’s passive
  - Each volley of snowballs is also a separate cast instance, allowing *Snowball Barrage* to trigger *Phase Rush* on its own or trigger 6 stacks of *Conqueror*.
  - *Snowbound* consumption does not trigger on-cast effects, neither does it count as a separate cast instance (e.g. for the purposes of *Conqueror*).
- The three snowballs are thrown independently from one another over the $0.4$ seconds and their direction is determined from ''Nunu & Willump's' position at any given moment.

---

### R: Absolute Zero

**Active:** **Nunu** and Willump channel a blizzard for a short time, gaining a shield and gradually slow nearby enemies.

*Absolute Zero* can be recast during the channel, and does so automatically when the channel ends by any means.

**Active:** **Nunu** and **Willump** channel a blizzard for up to 3 seconds, granting themselves a shield and slow nearby enemies by type=channel time. *Absolute Zero* can be recast after $0.5$ seconds during the channel, and does so automatically when the channel ends by any means. **Recast:** **Nunu** and **Willump** cause the blizzard to explode, dealing magic damage to nearby enemies, modified to key=%. Affected enemies will remain slow for type=channel time seconds and any of the duo's remaining shield will decay over 3 seconds.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $110-90$ seconds |
| **Cast Time** | none |
| **Cost** | 100 Mana |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | 650 units |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |

**Scaling:**
- **Shield Strength:** $65-85$ (+ 150% AP) (+ $30-50$%
- **bonus** health)
- **Magic Damage:** $625-1275$ (+ 300% AP)

**Notes:**

- The affected area is hidden if the duo is not sight to the enemy team.
- While it's unlikely to happen normally, it is possible to deal zero damage with *Absolute Zero* if canceled as soon as it's started.
- The following table refers for interactions while **Nunu & Willump** are channel:

---

## Patch History

### V25.09
- *Consume*
  - **New Effect:** While the ability is not on cooldown, its icon can now be pinged to display the damage to monsters in chat, similarly to Smite.
- *Biggest Snowball Ever!*
  - **Undocumented / Bug Fix:** Flash can no longer be used while channeling if **Nunu** previously detonated a Blast Cone with *Hextech Flashtraption* equipped.

### V13.19
- *Snowball Barrage*
  - AP ratio per snowball increased to 15% AP from 10% AP.

### V13.16
- *Biggest Snowball Ever!*
  - **Bug Fixes:** Upon recast, the snowball no longer incorrectly checks for collision at a stored location from a previous cast if the previous cast's channel was cancelled by either being root or rolling the snowball into terrain with an enemy target nearby, leading to the effect disappearing in the caster's field of view and affecting targets in a completely different location.

### V13.5
- *Biggest Snowball Ever!*
  - **Bug Fixes:** Snowball no longer gets stuck in place if it was released at exactly the same time the caster left the Fog of War.

### V12.22
- *Consume*
  - Base damage increased to $400-1200$ from $340-980$.

### V12.19
- *Absolute Zero*
  - Maximum AP ratio increased to 300% AP from 250% AP.

### V12.11
- *Consume*
  - Heal AP ratio increased to 90% AP from 70% AP.
  - Heal health ratio increased to 10% from 6% **bonus** health.
- *Snowball Barrage*
  - Cooldown reduced to $14-10$ seconds from 14 at all ranks.

### V12.10
- Stats
  - Base health increased to 610 from 540.
  - Health growth increased to 90 from 76.
  - Armor growth increased to $4.2$ from 3.
  - Magic resistance growth increased to $2.05$ from $1.25$.
- *Consume*
  - Base heal reduced to $65-185$ from $75-215$.
  - Heal AP ratio reduced to 70% AP from 90% AP.
  - Heal health ratio reduced to 6% **bonus** health from 10%.

### V12.4
- Stats
  - Health growth reduced to 76 from 82.
- *Consume*
  - Healing against champions reduced to 60% from 80%.
    - Base heal against champions reduced to $45-129$ from $60-172$.
    - Heal AP ratio against champions reduced 54% AP from 72% AP.
    - Heal health ratio against champions reduced to 6% **bonus** health from 8%.

### V11.17
- *Biggest Snowball Ever!*
  - **Bug Fixes:** Fixed a bug where he would be knocked up to a very high height when hit by Unburrow while channeling.
- *Snowball Barrage*
  - **Bug Fixes:** Fixed a bug where it was missing its tooltip when hovering during mid-cast.
  - **Bug Fixes:** Fixed a bug where the tooltip was incorrectly displaying damage as "total damage" when it is actually "per snowball".

## Trivia

- Nunu was named after Steve 'Guinsoo' Feak's dog.
  - *Nunu* is also a nickname for his wife.
- The update in patch V7.16 that made it so the portrait of the selected skin would be used in the central portion of the in-game UI showed the portrait of Willump instead of Nunu for every skin except for his Nunu & Willump skin.
- *Absolute Zero* was named after Absolute Zero at which matter stops moving completely (referencing both slows the ability has) and has the highest single ability power ratio in-game (300%).
- In the V1.0.0.115 April Fools' Day patch, he was jokingly listed to receive a skin called **"Nunurf"**, referencing Urf.
- Nunu & Willump drop a *icononly=true**icononly=true* soul each.
- Their dance is an updated version of their old animation in which Willump dances and **Nunu** provides *flute* music.
- Willump was most likely inspired by several characters from different media, such as:
  - from the franchise,
  - Wild Thing from Where the Wild Things Are series,
  - from series.
- Nunu's Series 2 Eternals make the following references:
  - *The Fast and The Curious* could be a reference to the Fast & Furious franchise.

---
*This page was automatically generated from League of Legends Wiki data.*