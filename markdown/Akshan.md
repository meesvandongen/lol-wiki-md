# Akshan

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
| **Champion** | Akshan |
| **Title** | the Rogue Sentinel |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2021-07-22 |
| **Release Patch** | V11.15 |
| **Latest Changes** | V14.22 |
| **Roles** | Marksman, Assassin |
| **Riot Positions** | Middle |
| **External Positions** | Middle |
| **Blue Essence** | 2400 |
| **Riot Points** | 880 |
| **Difficulty** | 3 |
| **Hero Type** | Marksman |
| **Alt Type** | Assassin |
| **Adaptive Type** | Physical |
| **Damage** | 3 |
| **Toughness** | 1 |
| **Control** | 1 |
| **Mobility** | 3 |
| **Utility** | 2 |
| **Style** | 1 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $630.0$ | $+107.0$ |
| **Mana** | $350.0$ | $+40.0$ |
| **Health Regen** | $3.75$ | $+0.65$ |
| **Mana Regen** | $8.2$ | $+0.7$ |
| **Armor** | $26.0$ | $+4.7$ |
| **Magic Resist** | $30.0$ | $+1.3$ |
| **Attack Damage** | $52.0$ | $+3.0$ |
| **Attack Speed** | $0.638$ | |
| **Movement Speed** | $330.0$ | $+0.0$ |
| **Attack Range** | $500.0$ | $+0.0$ |
| **Base Attack Speed** | $0.638$ | |
| **Attack Speed Ratio** | $0.4$ | |
| **Bonus AS per Level** | $4.0\%$ | |
| **Missile Speed** | $2000$ units/second | |
| **Acquisition Radius** | $750$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $100$ units | |
| **Selection Height** | $100$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $105.0\%$ |
| **Damage Taken** | $95.0\%$ |

#### URF

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $105.0\%$ |
| **Damage Taken** | $100.0\%$ |

## Abilities

### Passive: Dirty Fighting

| Attribute | Value |
|-----------|------:|
| **Speed** | / 5000 (Additional shot missile speed) |
| **Targeting** | Passive |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical Magic |
| **Spell Shield** | False |
| **Spell Effects** | Special |
| **Projectile** | True |
| **Parry** | True |
| **Static Cooldown (Unaffected by ability haste, starts upon the shield being destroyed or expiring)** | 16@1; 12@6; 8@11; 4@16 |

**INNATE:** Whenever **Akshan** uses a basic attack, he fires an additional shot after a delay that deals 50% AD physical damage, increased to 100% AD against minions. Issuing an attack order (Default right-click/MB2) on a different target before the additional shot has been launched causes **Akshan** to fire it at the new target. If the second shot is cancelled instead, he gains 20 to 75 × (1 + 100% **bonus** attack speed) **bonus** movement speed decaying over 1 second.

The additional shot applies on-hit effects, triggers on-attack effects, and can critically strike「 for bonus damage. ⟷ 100% base damage + 30% **bonus** critical damage. 」

**INNATE:** **Akshan**’s basic attacks on-hit and ability hits apply a stack of *Dirty Fighting* to enemies for 5 seconds, refreshing on subsequent applications and stacking up to 3 times. The third stack against a target consumes them all to deal them 15@1; 40@6; 80@11; 150@16 (+ 60% AP) **bonus** magic damage; if the target is a champion, **Akshan** will also gain a 40 / 280 (+ 35% **bonus** AD) shield for 2 seconds. The shield may be gained only once every few seconds.

**Notes:**

- If the first shot has killed its target, **Akshan** will automatically acquire another enemy within 200 units beyond his (range) basic attack range, else he can do so by issuing an attack order (Default right-click/MB2) to a different target.
- Applies basic damage for the second shot and proc damage for the bonus damage.
- The second shot:
  - Is treated as a basic attack.
  - (critical strike) Critically strikes independently from the first shot.
  - Can be cancelled by inputting a different command right after using the first shot.
  - Counts as a separate hit for effects such as Electrocute, Muramana Shock, and Eclipse Ever Rising Moon.
  - Starts the attack windup's cooldown after it is used, rather than when the first shot is.
- The attack speed scaling on the movement speed buff includes the bonus attack speed gained from **Akshan**’s innate attack speed growth.
  - At level 18, at minimum it grants $ decaying movement speed.
- Changing targets for the second shot will also acquire the new target.
- The second shot, if the first shot's target was killed, will prioritize visible enemy champions, then minions on (health) low health.
- This ability's damage is calculated based on the caster's current stats and changes dynamically.

---

### Q: Avengerang

| Attribute | Value |
|-----------|------:|
| **Range** | 850 + 500 per enemy hit units |
| **Cast Time** | $0.25$ seconds |
| **Effect Radius** | 400 (Missiles' own sight range) units |
| **Width** | 120 units |
| **Speed** | 1500 / 2400 units/second |
| **Cost** | 60 / 65 / 70 / 75 / 80 Mana |
| **Cooldown** | 8 / 7.25 / 6.5 / 5.75 / 5 (Starts after the boomerang returns) seconds |
| **Cooldown Start** | post-effect |
| **Targeting** | Direction |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | Special |
| **Spell Effects** | spellaoe |
| **Projectile** | True |

**ACTIVE:** **Akshan** throws a boomerang in the target direction that briefly grants sight around its trajectory and deals physical damage to enemies hit, revealing them for 1 second and extending its range each time it hits a target. If this hits an enemy champion, **Akshan** gains (ms) 20% (+ 5% per 100 AP) **bonus** movement speed that decays over 1 second.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 5 / 25 / 45 / 65 / 85 (+ 80% AD) |

Once the boomerang has passed its original range and has not hit a target in the last 500 units of travelling, it homes back to **Akshan** and applies the same effects to enemies hit.

| Attribute | Value |
|-----------|------:|
| **Total Physical Damage** | 10 / 50 / 90 / 130 / 170 (+ 160% AD) |

*Avengerang* deals reduced damage against non-champions.

| Attribute | Value |
|-----------|------:|
| **Non-Champion Damage** | 40 / 50 / 60 / 70 / 80% |

*Enemies can be hit only once per pass.*

**Notes:**

- Spell shield will only block a single hit. They do not prevent the initial throw from extending its range.
- This ability will cast from wherever the caster is at the end of the cast time.
- This ability's damage is calculated based on the caster's current stats and changes dynamically.

---

### W: Going Rogue

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.5$ seconds |
| **Cost** | 40 / 30 / 20 / 10 / 0 Mana |
| **Cooldown** | 18 / 14 / 10 / 6 / 2 seconds |
| **Cooldown Start** | post-effect |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Detection Radius** | 800 |

**PASSIVE:** **Akshan** marks enemy champions that kill allied champions as *Scoundrels* for 60 seconds. If **Akshan** scores a takedown against a *Scoundrel* while alive and within 3 seconds of damaging them, he receives an additional 100 gold and resurrects all dead allied champions that they have slain after 1 second.

When **Akshan** claims a *Scoundrel*’s bounty he removes the marks of all other enemies. *Scoundrels* refresh their mark duration on subsequent kills, and will have their mark removed when they die by any means. **Akshan** cannot mark enemies as *Scoundrels* while they are dead. Allies are resurrected at their summoning platform.

**ACTIVE:** **Akshan** enters camouflage, which lasts indefinitely while he is near terrain or inside brush, and for 2 seconds otherwise. During this time. he can see trails leading toward *Scoundrels*, and while facing them if they are within 5000 (Pending for test) units, he gains **bonus** mana regeneration equal to 12% of his **missing** mana as well as **bonus** movement speed.

| Attribute | Value |
|-----------|------:|
| **Bonus Movement Speed** | 80 / 90 / 100 / 110 / 120 |

*Going Rogue* can be recast after 1 second, and does so automatically after its duration. Attacking or casting abilities ends *Going Rogue* immediately.

***Akshan** can move during Going Rogue's cast time.*

**RECAST:** **Akshan** ends *Going Rogue*.

**Notes:**

- The mark displays the duration and slay count.
  - Enemy champions that kill **Akshan** will not be marked as *Scoundrels* nor increase their slay count.
- Clones do not count for triggering *Going Rogue*’s passive.
- *Going Rogue*’s buff refreshes to last indefinitely upon moving near terrain or into brush, and will refresh to 2 seconds after leaving near terrain or brush.
- Allied champions within a zombie state are resurrected only after the state ends.
- **Akshan** can still trigger *Going Rogue*’s passive while he is within a zombie state.
- Using a basic attack breaks the stealth at the start of the attack windup.

---

### E: Heroic Swing

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 800 units |
| **Width** | 80 (Hook missile width) units |
| **Speed** | 2500 (Hook missile speed) / 1200 (Dash orbital speed) / 3000 (Automatic attacks missile speed) units/second |
| **Cost** | 70 Mana |
| **Cooldown** | 18 / 16.5 / 15 / 13.5 / 12 seconds |
| **Cooldown Start** | post-effect |
| **Targeting** | Auto / Location |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | Special |
| **Spell Effects** | basic |
| **Projectile** | True |
| **Parry** | True |
| **Call For Help** | True |
| **Grounded** | Special |
| **Knockdown** | True |

**ACTIVE:** **Akshan** can activate *Heroic Swing* three times before the ability goes on cooldown, and can use the third cast only after $0.5$ seconds of the second cast. **Akshan** may input an attack or movement command to use the second and third casts.

**FIRST CAST:** **Akshan** fires a hook in the target direction that embeds in the first terrain hit, and enters preparation (See notes) for up to $2.125$ seconds. *Heroic Swing*’s second cast can be used while the hook is attached. If the hook fails to attach or **Akshan** is moved (See notes) or becomes immobilized, grounded, or polymorphed within the duration, the preparation will end prematurely, cancelling *Heroic Swing* in the process.

**SECOND CAST:** **Akshan** swings around the terrain in either a clockwise or counterclockwise direction based on the position of the cursor relative to his facing direction (See notes), stopping upon colliding with an enemy champion or terrain. While swinging, he fires at the nearest visible enemy every to deal them physical damage and apply on-hit effects for each shot, with on-hit damage reduced to 25% effectiveness.

***Akshan** will be knocked down by any immobilizing or polymorphing crowd control during the dash.*

| Attribute | Value |
|-----------|------:|
| **Physical Damage per Shot** | 15 / 30 / 45 / 60 / 75 (+ 15% AD) × (1 + $0.3$ per 100% **bonus** attack speed) |

**THIRD CAST:** **Akshan** ends the swing by jumping to the target location and fires one last shot at a nearby visible enemy.

Scoring an enemy champion takedown reduces *Heroic Swing*’s **current** cooldown to $0.5$ seconds. The shots can critically strike for「 damage ⟷ 90% **total** critical damage 」and apply life steal at 100% effectiveness.

*Avengerang and Going Rogue can be cast during the third cast's dash. Comeuppance can be cast at all points during Heroic Swing, though **Akshan** cannot fire during the swing while it is active. **Akshan** prioritizes firing at enemy champions with stacks of Dirty Fighting, then those damaged by his targeted spells in the last 4 seconds, then the nearest enemy. Heroic Swing can be cast during **Akshan**’s other abilities.*

**Notes:**

- The second cast's dash will displace **Akshan** around the terrain in either a clockwise or counterclockwise direction, with the specific direction being determined by the position of the player's cursor relative to his facing direction.
  - The dash will move clockwise if the player's cursor is anywhere to the left of **Akshan** from his facing direction (or perspective), and counterclockwise if the cursor is anywhere to his right.
- **Akshan** will not stop swinging until his dash is stopped.
  - The swing will end prematurely if the terrain the hook was attached to no longer exists, such as player-generated terrain.
- **Akshan** will prioritize firing at enemies he damaged with *any* unit-targeted ability or spell within the last 4 seconds, such as R or Ignite.
- *Heroic Swing* grants a buff to **Akshan** for 2 seconds that indicates and determines the first cast's duration.
  - This buff starts as soon as the ability is cast, and lingers for $0.125$ seconds after it expires.
    - This means the hook can be attached to terrain for longer if it hits it earlier in the duration, and vice versa.
  - If **Akshan** dashes (excluding second cast's dash) or blinks or becomes affected by immobilization, ground, or polymorph in the duration, the buff is removed *immediately*, causing the first cast to be lost and cancelling *Heroic Swing* entirely.
- **Akshan** immediately fires one shot at the beginning of his swing and a final shot while dismounting from the swing.
- **Akshan** can fire at any targetable enemy unit excluding structures and jungle plants.
- Only *Heroic Swing*’s first cast is disabled while grounded or rooted. The third cast is still usable during those effects.
- *Heroic Swing*’s second and third cast can both be used while silenced.
- Takedowns against clones do not count for resetting *Heroic Swing*’s cooldown.
- If a takedown is scored while *Heroic Swing* is active, the cooldown afterwards will be $0.5$ seconds.
- **Akshan** will attempt to basic attack the target he fired at with the last shot after the third cast's dash ends, if there is no other input given.
- Each shot generates a stack of Conqueror.
- Spell shield will only block a single shot.
- The attack speed scaling on the damage of the attacks includes the bonus attack speed gained from **Akshan**’s innate attack speed growth.
  - At level 18, each shot at minimum deals $ (+ $% AD) physical damage.
- Despite this ability not applying on-attack effects, it does apply Navori Flickerblade Transcendence.
- Under certain circumstances, upon attaching the hook to terrain, **Akshan** will create a particle on the location permanently.
  - This particle can be attacked by minions.
- This ability's damage is calculated based on the caster's current stats and changes dynamically.
- Hook range including lollipop against walls, automated attacks range while swinging, final dash distance and speed.
- The following table refers for interactions while the hook is attached or in flight:

| Aspect | State / Notes |
|--------|---------------|
| **Attacking** | Automatically initiates the second cast. |
| **Movement** | Automatically initiates the second cast. |
| **Abilities** | Avengerang and Going Rogue are disabled. Comeuppance is usable. |
| **Items** | Disabled: All items |
| **Summoner Spells** | Disabled: Barrier, Clarity, Cleanse, Exhaust, Ghost, Heal, Ignite, Smite, Teleport, Recall, Hexflash; Interrupted by: Flash |
| **Consumables** | Disabled |
| **Interrupted by** | Death, Immobilizing effects, Grounding effects |
- The following table refers for interactions while **Akshan** is swinging:

| Aspect | State / Notes |
|--------|---------------|
| **Attacking** | Automatically initiates the third cast. |
| **Movement** | Automatically initiates the third cast. |
| **Abilities** | Avengerang and Going Rogue are disabled. Comeuppance is usable. |
| **Items** | Usable: Shurelya's Battlesong, Youmuu's Ghostblade, Randuin's Omen, Stridebreaker; Interrupted by: Zhonya's Hourglass, Hextech Rocketbelt; Other items: Usable |
| **Summoner Spells** | Usable: Barrier, Clarity, Cleanse, Exhaust, Ghost, Heal, Ignite, Smite, Hexflash; Disabled: Teleport, Recall; Interrupted by: Flash |
| **Consumables** | Usable |
| **Interrupted by** | Death, Immobilizing effects |

---

### R: Comeuppance

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 2500 units |
| **Width** | 120 units |
| **Speed** | 3200 units/second |
| **Cost** | 100 Mana |
| **Cooldown** | 100 / 92.5 / 85 / 77.5 / 70 seconds |
| **Cooldown Start** | post-effect |
| **Targeting** | Unit |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | Special |
| **Spell Effects** | spell |
| **Projectile** | True |
| **Call For Help** | False |
| **Silence** | True |

**ACTIVE:** **Akshan** locks onto the target enemy champion and begins channeling for $2.5$ seconds, revealing them as well as revealing himself. He gradually stores bullets into his weapon over the duration.

| Attribute | Value |
|-----------|------:|
| **Maximum Bullets Stored** | 5 / 5.5 / 6 / 6.5 / 7 |

| Attribute | Value |
|-----------|------:|
| **Bullet Storing Interval Time** | 0.625 / 0.5729 / 0.5208 / 0.4688 / 0.4167 seconds |

*Comeuppance* can be recast after $0.5$ seconds during the channel, and does so automatically afterwards. *Comeuppance* is placed on a (cd) 5-second cooldown if the channel is cancelled.

**RECAST:** **Akshan** fires all stored bullets at the target, each briefly granting sight around their trajectory and dealing physical damage to the first enemy hit, increased by 50% of critical chance and critical damage bonuses as well as by 0%–200%@0–100 (@=target's **missing** health). The shots can hit structures.

| Attribute | Value |
|-----------|------:|
| **Minimum Physical Damage per Bullet** | [25 / 30 / 35 / 40 / 45 (+ 15% AD)] |

Each bullet's damage applies life steal at 100% effectiveness and executes minions.

***Akshan** can move while channeling Comeuppance.*

**Scaling:**
| Attribute | Value |
|-----------|------:|
| **Minimum Physical Damage per Bullet** | [25 / 30 / 35 / 40 / 45 (+ 15% AD)]
- *Comeuppance* refreshes the duration of ** stacks on the target every $0.25$ seconds.
- A kill threshold will appear on the health bar of the target locked onto during the channel, which increases each time **Akshan** stores a bullet. If the target's **current** health falls below the threshold, their health bar will be framed in red.
  - The indicator factors the bonus damage applied by as well as damage modifiers and the target's resistances.
  - It also factors the projected increase in the target's **missing** health, from which each consecutive bullet scales more.
  - It updates dynamically over the channel's duration, reacting to fluctuations of the target's health and damage mitigations.
- The true sight will linger for 2 seconds after the channel has ended.
- Spell shield will not prevent the lock-on and will only block a single bullet.
- *Comeuppance* will not go on a reduced cooldown if **Akshan** death during the channel.
- *Comeuppance* will cancel if the target becomes untargetable or death during the channel.
- Damage to structures **does** scale with their **missing** health.
- This ability's damage is calculated based on the caster's current stats and changes dynamically.
- The following table refers for interactions while **Akshan** is channel:


| Aspect | State / Notes |
|--------|---------------|
| **Attacking** | Disabled |
| **Items** | Disabled: All items |
| **Summoner Spells** | Usable: Barrier, Clarity, Cleanse, Exhaust, Ghost, Heal, Ignite, Smite, Flash; Disabled: Teleport, Recall; Interrupted by: Hexflash |
| **Consumables** | Disabled |
| **Interrupted by** | Death, Cast-inhibiting effects |


**Notes:**

- *Comeuppance* refreshes the duration of *Dirty Fighting* stacks on the target every $0.25$ seconds.
- A kill threshold will appear on the health bar of the target locked onto during the channel, which increases each time **Akshan** stores a bullet. If the target's **current** health falls below the threshold, their health bar will be framed in red.
  - The indicator factors the bonus damage applied by Dirty Fighting as well as damage modifiers and the target's resistances.
  - It also factors the projected increase in the target's **missing** health, from which each consecutive bullet scales more.
  - It updates dynamically over the channel's duration, reacting to fluctuations of the target's health and damage mitigations.
- The reveal will linger for 2 seconds after the channel has ended.
- Spell shield will not prevent the lock-on and will only block a single bullet.
- *Comeuppance* will not go on a reduced cooldown if **Akshan** dies during the channel.
- *Comeuppance* will cancel if the target becomes untargetable or dies during the channel.
- Damage to structures **does** scale with their **missing** health.
- This ability's damage is calculated based on the caster's current stats and changes dynamically.
- The following table refers for interactions while **Akshan** is channeling:

| Aspect | State / Notes |
|--------|---------------|
| **Attacking** | Disabled |
| **Movement** | Allowed |
| **Abilities** | Q and W are disabled. E is usable. |
| **Items** | Disabled: All items |
| **Summoner Spells** | Usable: Barrier, Clarity, Cleanse, Exhaust, Ghost, Heal, Ignite, Smite, Flash; Disabled: Teleport, Recall; Interrupted by: Hexflash |
| **Consumables** | Disabled |
| **Interrupted by** | Death, Cast-inhibiting effects |

---

## Patch History

### V14.22
- Heroic Swing
  - **Bug Fixes:** Empowered shots now follow the same logic as regular shots.

### V14.20
- Dirty Fighting
  - **Bug Fixes:** Additional shot now correctly applies Yun Tal Wildarrows Serrated Edge.

### V14.14
- General
  - **Bug Fixes:** Issuing a regular movement or attack order no longer incorrectly plays his Going Rogue VO.
    - *Note: these voicelines are audible to all players.*

### V14.12
- Dirty Fighting
  - Base damage changed to 15@1; 40@6; 80@11; 150@16 from 10 to 45 for 8 / 55 to 105 for 6 / 120 to 165 for 4.
- Heroic Swing
  - Base damage per shot reduced to 15 / 30 / 45 / 60 / 75 from 25 / 40 / 55 / 70 / 85.
  - AD ratio per shot changed to 15% **total** AD from $17.5%$ **bonus** AD.
  - **Bug Fixes:** Now properly prioritizes targets affected by Dirty Fighting if the ability is learned after they were marked.

### V14.9
- Avengerang
  - Non-champion damage reduced to 40 / 50 / 60 / 70 / 80% from 40 / 52.5 / 65 / 77.5 / 90%.
  - Bonus movement speed reduced to 20% at all ranks from 20 / 25 / 30 / 35 / 40%.
- Comeuppance
  - Minimum base damage per bullet increased to 25 / 35 / 45 from 20 / 25 / 30.
  - Minimum AD ratio per bullet increased to 15% AD from 10% AD.
  - Bonus damage reduced to 0%–200%@0–100 (@=target's **missing** health) from 0%–300%@0–100 (@=target's **missing** health).
    - Maximum base damage per bullet changed to 75 / 105 / 135 from 80 / 100 / 120.
    - Maximum AD ratio per bullet increased to 45% AD from 40% AD.

### V13.20
- Avengerang
  - Base bonus movement speed reduced to 20 / 25 / 30 / 35 / 40% from 40% at all ranks.
- Heroic Swing
  - Base damage per shot reduced to 25 / 40 / 55 / 70 / 85 from 30 / 45 / 60 / 75 / 90.

### V13.17
- Stats
  - Health growth increased to 107 from 104.
  - Armor growth increased to $4.7$ from $4.2$.
  - Attack damage growth reduced to 3 from $3.5$.

### V13.15
- Dirty Fighting
  - **Bug Fixes:** Now properly triggers the second shot when attacking Malzahar’s Voidlings and Yorick’s Mist Walkers.

### V13.13
- Comeuppance
  - **Bug Fixes:** Now properly grants true sight of the target.

### V13.10
- Dirty Fighting
  - **New Effect:** Bonus damage now scales with 60% AP.
  - Base shield reduced to 40 / 280 from 40 to 280. *Uses level growth instead of linear interpolation*.
- Avengerang
  - **New Effect:** Bonus movement speed now scales with 5% per 100 AP.
- Heroic Swing
  - **Bug Fixes:** Avengerang and Going Rogue are now properly greyed out in the HUD interface during its cast.
  - **Bug Fixes:** Can no longer swing in place inside the pits for Baron Nashor and Dragon at specific locations.
  - **Bug Fixes:** VFX no longer sometimes indicates it is going in a different direction than intended.

## Trivia

- His dance references Indian musical video Malhari from album Bajirao Mastani).
  - A side-by-side comparison can be seen here.
- Akshan is the first champion to be revealed for League of Legends, Wild Rift and Legends of Runeterra around the same time, and with the next one being Ambessa.
- Akshan is the second of three champions released in 2021 tied to Viego’s return and a continuation of the Ruined King's story.
- Akshan's Series 2 Eternals make the following references:
  - *Plot Armor* is a reference to the Plot armor where the character remains alive despite adversity in order to sustain the story.

---
*This page was automatically generated from League of Legends Wiki data.*