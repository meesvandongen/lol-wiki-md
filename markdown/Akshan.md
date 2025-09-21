# Akshan

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
| **Champion** | Akshan |
| **Title** | the Rogue Sentinel |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2021-07-22 |
| **Release Patch** | V11.15 |
| **Roles** | Marksman, Assassin |
| **Riot Positions** | Middle |
| **External Positions** | Middle |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $630.0$ | $+107.0$ | $2449.0$ |
| **Mana** | $350.0$ | $+40.0$ | $1030.0$ |
| **Health Regen** | $3.75$ | $+0.65$ | $14.8$ |
| **Mana Regen** | $8.2$ | $+0.7$ | $20.1$ |
| **Armor** | $26.0$ | $+4.7$ | $105.9$ |
| **Magic Resist** | $30.0$ | $+1.3$ | $52.1$ |
| **Attack Damage** | $52.0$ | $+0.7$ | $63.9$ |
| **Attack Speed** | $0.638$ | $+4.0\%$ | $1.072$ |
| **Movement Speed** | $330.0$ | $+0.0$ | $330.0$ |
| **Attack Range** | $500.0$ | $+0.0$ | $500.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.638$ |
| **Attack Speed Ratio** | $0.4$ |
| **Bonus AS per Level** | $4.0\%$ |
| **Missile Speed** | $2000 units/second$ |
| **Acquisition Radius** | $750 units$ |
| **Pathing Radius** | $35 units$ |
| **Selection Radius** | $100 units$ |
| **Selection Height** | $100 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Dirty Fighting

**Innate:** **Akshan**’s basic attack causes him to fire a second attack that deals **reduced** physical damage. If **Akshan** cancels the second attack, he gains a brief burst of *move speed*, increased based on his *attack speed*.

**Innate:** ''Akshan's** attacks and abilities apply a stack to enemies hit. The third stack consumes them all to deal **bonus'' magic damage; if the target was a champion, he also gains a brief shield.

**Innate:** Whenever **Akshan** uses a basic attack, he fires an additional shot after a delay that deals 50% AD physical damage, increased to 100% AD against minions. Issuing an attack order on a different target before the additional shot has been launched causes **Akshan** to fire it at the new target. If the second shot is cancelled instead, he gains 20 to 75 × (1 + 100% *bonus attack speed) **bonus movement speed** decaying over 1 second. The additional shot applies on-hit effects, triggers on-attack effects, and can critically strike bonus damage. 100% base damage + 30% **bonus critical damage**. **Innate:** ''Akshan's* basic attacks on-hit and ability hits apply a stack of *Dirty Fighting' to enemies for 5 seconds, refreshing on subsequent applications and stacking up to 3 times. The third stack against a target consumes them all to deal them 15@1; 40@6; 80@11; 150@16 (+ 60% AP) **bonus** magic damage; if the target is a champion, **Akshan** will also gain a 40+(240/17)*(x-1)*(0.7025+0.0175*(x-1)) (+ 35% *bonus AD) shield for 2 seconds. The shield may be gained only once every few seconds.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical Magic |
| **Speed** | speed of basic attacks and crits / 5000 units/second |
| **Spell Shield** | False |
| **Spell Effects** | Special |
| **Projectile** | True |

**Notes:**

- If the first shot has killed its target, **Akshan** will automatically acquire another enemy within 200 units beyond his range, else he can do so by issuing an attack order to a different target.
- Applies basic damage for the second shot and proc damage for the bonus damage.
- The second shot:
  - Is treated as a basic attack.
  - critical strike independently from the first shot.
  - Can be cancelled by inputting a different command right after using the first shot.
  - Counts as a separate hit for effects such as *Electrocute*, *Muramana* Shock, and *Eclipse* Ever Rising Moon.
  - Starts the attack windup's cooldown after it is used, rather than when the first shot is.
- The attack speed scaling on the movement speed buff includes the bonus attack speed gained from ''Akshan's' innate attack speed growth.
  - At level 18, at minimum it grants $ decaying movement speed.
- Changing targets for the second shot will also acquire the new target.
- The second shot, if the first shot's target was killed, will prioritize sight enemy champions, then minion on health.
- This ability's damage is calculated based on the caster's current stats and changes dynamically.

---

### Q: Avengerang

**Active:** **Akshan** throws a boomerang that deals physical damage to enemies hit, extending the boomerang range and briefly standard sight them. If this hits an enemy champion, **Akshan** gains a brief burst of ms.

*Once the boomerang has passed its original range and has not recently hit an enemy, it homes back to **Akshan** and deals the same effects to enemies hit.*

**Active:** **Akshan** throws a boomerang in the target direction that briefly grants sight around its trajectory and deals physical damage to enemies hit, standard sight them for 1 second and extending its range each time it hits a target. If this hits an enemy champion, **Akshan** gains ms (+ 5% per 100 AP) **bonus** movement speed that decays over 1 second. Once the boomerang has passed its original range and has not hit a target in the last 500 units of travelling, it homes back to **Akshan** and applies the same effects to enemies hit. *Avengerang* deals reduced damage against non-champions. *Enemies can be hit only once per pass.*

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $8-5$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $60-80$ Mana |
| **Targeting** | Direction |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical |
| **Speed** | 1500 / 2400 units/second |
| **Effect Radius** | 400 units |
| **Spell Shield** | Special |
| **Spell Effects** | spellaoe |
| **Projectile** | True |

**Scaling:**
- **Physical Damage:** $5-85$ (+ 80% AD)
- **Total Physical Damage:** $5×2-85×2$ (+ $80×2$% AD)
- **Non-Champion Damage:** $40-80$%

**Notes:**

- Spell shield will only block a single hit. They do not prevent the initial throw from extending its range. Effect at cast time end
- This ability's damage is calculated based on the caster's current stats and changes dynamically.

---

### W: Going Rogue

**Passive:** Enemy champions that kill allied champions are marked as a *Scoundrel* for a long time. When **Akshan** scores a takedown on a *Scoundrel*, he gains **bonus** gold gold, all allies killed by the *Scoundrel* are resurrected, and *Scoundrel* status is removed from all other enemies.

**Active:** **Akshan** briefly gains camouflage, or indefinitely while near terrain and in brush. During this time, he can see trails leading toward *Scoundrels* and gains *move speed* and *mana regeneration* while moving toward them.

**Passive:** **Akshan** marks enemy champions that kill allied champions as *Scoundrels* for 60 seconds. If **Akshan** scores a takedown against a *Scoundrel* while alive and within 3 seconds of damaging them, he receives an additional 100 gold and resurrects all death allied champions that they have slain after 1 second. When **Akshan** claims a 'Scoundrel's* bounty he removes the marks of all other enemies. *Scoundrels* refresh their mark duration on subsequent kills, and will have their mark removed when they die by any means. **Akshan** cannot mark enemies as *Scoundrels' while they are dead. Allies are resurrected at their summoning platform. **Active:** **Akshan** enters camouflage, which lasts indefinitely while he is near terrain or inside brush, and for 2 seconds otherwise. During this time. he can see trails leading toward *Scoundrels*, and while facing them if they are within 5000 units, he gains **bonus mana regeneration** equal to 12% of his **missing** mana as well as **bonus movement speed**. *Going Rogue* can be recast after 1 second, and does so automatically after its duration. Attacking or casting abilities ends *Going Rogue* immediately. '**Akshan** can move during Going Rogue's cast time.'

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $18-2$ seconds |
| **Cast Time** | $0.5$ seconds |
| **Cost** | $40-0$ Mana |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |

**Scaling:**
- **Bonus Movement Speed:** $80-120$

**Notes:**

- The mark displays the duration and slay count.
  - Enemy champions that kill **Akshan** will not be marked as *Scoundrels* nor increase their slay count.
- Clone do not count for triggering 'Going Rogue's passive.
- 'Going Rogue's buff refreshes to last indefinitely upon moving near terrain or into brush, and will refresh to 2 seconds after leaving near terrain or brush.
- Allied champions within a zombie state are resurrected only after the state ends.
- **Akshan** can still trigger 'Going Rogue's passive while he is within a zombie state.
- Using a basic attack breaks the stealth at the start of the attack windup.

---

### E: Heroic Swing

**First Cast:** **Akshan** prepares to swing by firing a hook that embeds in the first terrain hit. 'Heroic Swing's second cast can be used while the hook is attached.

**Second Cast:** **Akshan** dash around the terrain in a direction, firing physical damage bullets at the nearest enemy. He may swing indefinitely, or until he collides with an enemy champion or terrain.

**Active:** **Akshan** can activate *Heroic Swing* three times before the ability goes on cooldown, and can use the third cast only after $0.5$ seconds of the second cast. **Akshan** may input an attack or movement command to use the second and third casts. **First Cast:** **Akshan** fires a hook in the target direction that embeds in the first terrain hit, and enters preparation for up to $2.125$ seconds. 'Heroic Swing's* second cast can be used while the hook is attached. If the hook fails to attach or **Akshan** is moved or becomes immobilize, ground, or polymorph within the duration, the preparation will end prematurely, cancelling *Heroic Swing' in the process. **Second Cast:** **Akshan* dash around the terrain in either a clockwise or counterclockwise direction based on the position of the cursor relative to his facing direction, stopping upon colliding with an enemy champion or terrain. While swinging, he fires at the nearest sight enemy every rutngt*Akshan will be knockdown by any immobilize or polymorph crowd control during the dash.** **Third Cast:** **Akshan** ends the swing by dash to the target location and fires one last shot at a nearby sight enemy. Scoring an enemy champion takedown reduces 'Heroic Swing's **current cooldown* to $0.5$ seconds. The shots can critical strike forand applies life steal at 100% effectiveness.

| Attribute | Value |
|-----------|-------|
| **Range** | 800 units |
| **Cooldown** | $18-12$ seconds |
| **Cast Time** | none |
| **Cost** | 70 Mana |
| **Targeting** | Auto / Location |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical |
| **Speed** | 2500 / 1200 / 3000 units/second |
| **Spell Shield** | Special |
| **Spell Effects** | basic |
| **Projectile** | True |

**Scaling:**
- **Physical Damage per Shot:** $15-75$ (+ 15% AD)

**Notes:**

- The second cast's dash will displace **Akshan** around the terrain in either a clockwise or counterclockwise direction, with the specific direction being determined by the position of the player's cursor relative to his facing direction.
  - The dash will move clockwise if the player's cursor is anywhere to the left of **Akshan** from his facing direction (or perspective), and counterclockwise if the cursor is anywhere to his right.
- **Akshan** will not stop swinging until his dash is stopped.
  - The swing will end prematurely if the terrain the hook was attached to no longer exists, such as player-generated terrain.
- **Akshan** will prioritize firing at enemies he damaged with *any* unit-targeted ability or spell within the last 4 seconds, such as *R* or Ignite.
- *Heroic Swing* grants a buff to **Akshan** for 2 seconds that indicates and determines the first cast's duration.
  - This buff starts as soon as the ability is cast, and lingers for $0.125$ seconds after it expires. *** This means the hook can be attached to terrain for longer if it hits it earlier in the duration, and vice versa.
  - If **Akshan** dash (excluding second cast's dash) or blink or becomes affected by immobilize, ground, or polymorph in the duration, the buff is removed *immediately*, causing the first cast to be lost and cancelling *Heroic Swing* entirely.
- **Akshan** immediately fires one shot at the beginning of his swing and a final shot while dismounting from the swing.
- **Akshan** can fire at any targetable enemy unit excluding structures and jungle plants.
- Only 'Heroic Swing's first cast is disabled while ground or root. The third cast is still usable during those effects.
- 'Heroic Swing's second and third cast can both be used while silence.
- Takedown against clone do not count for resetting 'Heroic Swing's cooldown.
- If a takedown is scored while *Heroic Swing* is active, the cooldown afterwards will be $0.5$ seconds.
- **Akshan** will attempt to basic attack the target he fired at with the last shot after the third cast's dash ends, if there is no other input given.
- Each shot generates a stack of *Conqueror*.
- Spell shield will only block a single shot.
- The attack speed scaling on the damage of the attacks includes the bonus attack speed gained from ''Akshan's' innate attack speed growth.
  - At level 18, each shot at minimum deals $ (+ $% AD) physical damage.
- Despite this ability not applying on-attack effects, it does apply *Navori Flickerblade* Transcendence.
- Under certain circumstances, upon attaching the hook to terrain, **Akshan** will create a particle on the location permanently.
  - This particle can be attacked by minions.
- This ability's damage is calculated based on the caster's current stats and changes dynamically.
- Hook range including lollipop against walls, automated attacks range while swinging, final dash distance and speed.
- The following table refers for interactions while the hook is attached or in flight:
- The following table refers for interactions while **Akshan** is swinging:

---

### R: Comeuppance

**Active:** **Akshan** locks onto the target enemy champion and begins channel power into his gun to store bullets, true sight them as well as sight himself. *Comeuppance* will recast after the channel, or can recast early.

**Recast:** Akshan fires the stored bullets at the target, each dealing physical damage to the first enemy hit, increased by their **missing** health.

**Active:** **Akshan** locks onto the target enemy champion and begins channel for $2.5$ seconds, true sight them as well as sight himself. He gradually stores bullets into his weapon over the duration. *Comeuppance* can be recast after $0.5$ seconds during the channel, and does so automatically afterwards. *Comeuppance* is placed on a cd cooldown if the channel is cancelled. **Recast:** **Akshan** fires all stored bullets at the target, each briefly granting sight around their trajectory and dealing physical damage to the first enemy hit, increased by % of *critical chance* and *critical damage* bonuses as well as by key=%. The shots can hit structures. Each bullet's damage applies life steal at 100% effectiveness and execute minions. **Akshan can move while channeling Comeuppance.**

| Attribute | Value |
|-----------|-------|
| **Range** | 2500 units |
| **Cooldown** | $100-70$ seconds |
| **Cast Time** | none |
| **Cost** | 100 Mana |
| **Targeting** | Unit |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical |
| **Speed** | 3200 units/second |
| **Spell Shield** | Special |
| **Spell Effects** | spell |
| **Projectile** | True |

**Scaling:**
- **Maximum Bullets Stored:** $5-7$
- **Bullet Storing Interval Time:** $2.5/4-2.5/6 round=4$ seconds
- **Minimum Physical Damage per Bullet:* [$25-45$ (+ 15% AD)]25/63545$ (+ $15×5-157$% AD)]5256357×45$ (+ $3×5×15-3×7×15$% AD)]

**Notes:**

- *Comeuppance* refreshes the duration of **Dirty Fighting** stacks on the target every $0.25$ seconds.
- A kill threshold will appear on the health bar of the target locked onto during the channel, which increases each time **Akshan** stores a bullet. If the target's **current** health falls below the threshold, their health bar will be framed in red.
  - The indicator factors the bonus damage applied by *Dirty Fighting* as well as damage modifiers and the target's resistances.
  - It also factors the projected increase in the target's **missing** health, from which each consecutive bullet scales more.
  - It updates dynamically over the channel's duration, reacting to fluctuations of the target's health and damage mitigations.
- The true sight will linger for 2 seconds after the channel has ended.
- Spell shield will not prevent the lock-on and will only block a single bullet.
- *Comeuppance* will not go on a reduced cooldown if **Akshan** death during the channel.
- *Comeuppance* will cancel if the target becomes untargetable or death during the channel.
- Damage to structures **does** scale with their **missing** health.
- This ability's damage is calculated based on the caster's current stats and changes dynamically.
- The following table refers for interactions while **Akshan** is channel:

---

## Patch History

### V14.22
- *Heroic Swing*
  - **Bug Fixes:** Empowered shots now follow the same logic as regular shots.

### V14.20
- *Dirty Fighting*
  - **Bug Fixes:** Additional shot now correctly applies *Yun Tal Wildarrows* Serrated Edge.

### V14.14
- General
  - **Bug Fixes:** Issuing a regular movement or attack order no longer incorrectly plays his *Going Rogue* VO.
    - *Note: these voicelines are audible to all players.*

### V14.12
- *Dirty Fighting*
  - Base damage changed to 15@1; 40@6; 80@11; 150@16 from 10 to 45 for 8–120 to 165 for 4.
- *Heroic Swing*
  - Base damage per shot reduced to $15-75$ from $25-85$.
  - AD ratio per shot changed to 15% **total** AD from $17.5%$ *bonus AD.
  - **Bug Fixes:** Now properly prioritizes targets affected by *Dirty Fighting* if the ability is learned after they were marked.

### V14.9
- *Avengerang*
  - Non-champion damage reduced to $40-80$% from $40-90$%.
  - Bonus movement speed reduced to 20% at all ranks from $20-40$%.
- *Comeuppance*
  - Minimum base damage per bullet increased to $25-45 3$ from $20-30 3$.
  - Minimum AD ratio per bullet increased to 15% AD from 10% AD.
  - Bonus damage reduced to key=% from key=%.
    - Maximum base damage per bullet changed to $25×3-45×3 3$ from $20×4-30×4 3$.
    - Maximum AD ratio per bullet increased to $15×3$% AD from $10×4$% AD.

### V13.20
- *Avengerang*
  - Base bonus movement speed reduced to $20-40$% from 40% at all ranks.
- *Heroic Swing*
  - Base damage per shot reduced to $25-85$ from $30-90$.

### V13.17
- Stats
  - Health growth increased to 107 from 104.
  - Armor growth increased to $4.7$ from $4.2$.
  - Attack damage growth reduced to 3 from $3.5$.

### V13.15
- *Dirty Fighting*
  - **Bug Fixes:** Now properly triggers the second shot when attacking W and I.

### V13.13
- *Comeuppance*
  - **Bug Fixes:** Now properly grants true sight of the target.

### V13.10
- *Dirty Fighting*
  - **New Effect:** Bonus damage now scales with 60% AP.
  - Base shield reduced to 40+(240/17)*(x-1)*(0.7025+0.0175*(x-1)) from 40 to 280. *Uses level growth instead of linear interpolation*.
- *Avengerang*
  - **New Effect:** Bonus movement speed now scales with 5% per 100 AP.
- *Heroic Swing*
  - **Bug Fixes:** *Avengerang* and *Going Rogue* are now properly greyed out in the HUD interface during its cast.
  - **Bug Fixes:** Can no longer swing in place inside the pits for Baron Nashor and Dragon at specific locations.
  - **Bug Fixes:** VFX no longer sometimes indicates it is going in a different direction than intended.

## Trivia

- His dance references Indian musical video Malhari from album Bajirao Mastani).
  - A side-by-side comparison can be seen here.
- Akshan is the first champion to be revealed for League of Legends, Wild Rift and Legends of Runeterra around the same time, and with the next one being **Ambessa**.
- Akshan is the second of three champions released in 2021 tied to **Viego**’s return and a continuation of the Ruined King's story.
- Akshan's Series 2 Eternals make the following references:
  - *Plot Armor* is a reference to the Plot armor where the character remains alive despite adversity in order to sustain the story.

---
*This page was automatically generated from League of Legends Wiki data.*