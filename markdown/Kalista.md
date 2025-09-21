# Kalista

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
| **Champion** | Kalista |
| **Title** | the Spear of Vengeance |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2014-11-20 |
| **Release Patch** | V4.20 |
| **Roles** | Marksman |
| **Riot Positions** | Bottom |
| **External Positions** | Bottom |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $560.0$ | $+-20.0$ | $220.0$ |
| **Mana** | $300.0$ | $+45.0$ | $1065.0$ |
| **Health Regen** | $4.0$ | $+0.75$ | $16.8$ |
| **Mana Regen** | $6.3$ | $+0.8$ | $19.9$ |
| **Armor** | $24.0$ | $+5.2$ | $112.4$ |
| **Magic Resist** | $30.0$ | $+1.3$ | $52.1$ |
| **Attack Damage** | $57.0$ | $+4.25$ | $129.2$ |
| **Attack Speed** | $0.694$ | $+4.5\%$ | $1.225$ |
| **Movement Speed** | $330.0$ | $+0.0$ | $330.0$ |
| **Attack Range** | $525.0$ | $+0.0$ | $525.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.694$ |
| **Attack Speed Ratio** | $0.694$ |
| **Bonus AS per Level** | $4.5\%$ |
| **Windup Modifier** | $0.75$ |
| **Missile Speed** | $2600 units/second$ |
| **Acquisition Radius** | $900 units$ |
| **Pathing Radius** | $35 units$ |
| **Selection Radius** | $100 units$ |
| **Selection Height** | $135 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Martial Poise

**Innate:** During each basic attack or casting of **Pierce**, **Kalista** can optionally choose a target direction to quickly dash to. The range is based on dash angle and ''Kalista's** Boots tier. The dash speed scales with **bonus'' attack speed and certain movement speed effects.

**Innate - Oathsworn Bond:** **Kalista** begins the game with an exclusive *Black Spear*.

**Innate:** Whenever **Kalista** inputs a movement command during her attack windup or the cast time of **Pierce**, at the end she will dash a short distance in the direction of the inputted location. **Kalista cannot dash while immobilize or ground, and she will be knockdown by any immobilize or polymorph crowd control during the dash.** 'Martial Poise**s dash range and speed details: * Dash range of basic attacks is modified by the tier of **Kalista's* Boots and the specific angle of the dash. ** At a 0° angle towards the target, the minimum dash range is 140–175 units. ** At a 90° angle, the maximum dash range is 240–300 units. * Dash range of **Pierce** is 300 units when dashing anywhere in the direction of the point of cast. ** If the dash is inputted to a location that is *away* from **Pierce*’s* point of cast, the dash range is reduced based on the cast angle, with a minimum distance of 150 units if the inputted location is directly opposite to the point of cast. * Dash speed is increased based on the tier of *'Kalista's** Boots, and is affected by her ***bonus'' attack speed* as well as multiplicative modifiers she is under the effect of (including slow). **Innate - Oathsworn Bond:** **Kalista** begins the game with an exclusive **Black Spear** that she can activate on an allied champion to force them to swear an *oath* with her. Once the *oath* has been sworn, the ally becomes her *Oathsworn*, causing **Kalista** to form a tether between her and them. While she and the ally bound as the *Oathsworn* are within tether range of each other, they can both interact with **Soul-Mark** and '*Fate's Call*'. **Kalista can purchase the **Black Spear* from the shop at no cost to obtain it again and, before 3 minutes of game time, may activate the item again on a different target to rebind and swear an oath with a new ally. See *Black Spear* for more details.*

| Attribute | Value |
|-----------|-------|
| **Targeting** | Direction |
| **Affects** | Self |

**Notes:**

- Movement commands inputted during an attack windup or the cast time of **Pierce** while **Kalista** is immobilize or ground will fail to cast and thus prevent the dash from triggering.
  - The movement command will still be buffered to the end of the immobilization or the end of the attack windup or cast time of **Pierce** while grounded if it has not been overridden by other inputs.
- If multiple movement commands are inputted during the attack windup, the most recent one is used for the dash's targeting.
  - If the most recent input is not a movement command, the dash will not trigger. *** In this case, the buffer for the previous movement command was cancelled by a new non-movement command such as an input for an attack or ability cast.
- Dash distance decreases as the dash direction becomes closer to the minimum direction. This defines a forward distance for basic attacks and a backwards distance for *Pierce*. Empirical testing has given the following values below.
  - Basic attack dash range given the angle θ: *** \begin{cases}(175 + (125 \times \sin(\theta))) \times T \end{cases} **** Where T = 0.800, 0.875, 0.950, 1.000 depending on the tier of Boots (T0, T1, T2, T3) *** Minimum dash range of 225 units when dashing backwards
  - '*Pierce's*' dash range given the angle θ: *** \begin{cases} 150 + (150 \times \sin(\theta)) & \mathrm{Backwards} \\ 300 & \mathrm{Otherwise} \end{cases}
- There is a very brief grace period after completing a basic attack windup wherein **Kalista** can still input a movement command to trigger the dash.
- The dash speed is unaffected by additive movement speed modifiers.
- There are a total of 24 different dash ranges based on direction with and *Pierce* being cast.
- **Kalista** will automatically be issued an attack command on her attack target at the end of the dash, as long as they remain in her attack range by then.
- The attack move (default **A + MB1**) click feature checks for targets in brief intervals only if **Kalista** is not dashing.
- ''Kalista's' facing direction when dashing is considered to be in the direction of the dash, not her apparent facing direction based on her model's animation state.
- Sleep does not count for knocking the dash down.

---

### Q: Pierce

**Active:** **Kalista** throws a spear in the target direction that deals physical damage to the first enemy hit.

*If *Pierce* kills the target, the spear continues onward to transfer all of the target's **Rend** stacks to the next enemy it hits.*

**Active:** **Kalista** launches a spear in the target direction that deals physical damage to the first enemy hit. If *Pierce* kills the target, the spear continues onward to transfer all of the target's **Rend** stacks to the next enemy it hits. This can repeat indefinitely until the spear reaches its maximum range.

| Attribute | Value |
|-----------|-------|
| **Range** | 1200 units |
| **Cooldown** | 9 seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $60-80$ Mana |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Speed** | 2400 units/second |
| **Spell Shield** | True |
| **Spell Effects** | Single target |
| **Projectile** | True |

**Scaling:**
- **Physical Damage:** $10-270$ (+ 105% AD)

**Notes:**

No additional notes.

---

### W: Sentinel

**Passive - Soul-Marked:** While **Kalista** and her *The Black Spear* are tether, their basic attacks and **Pierce** apply a *Soul-Mark* to their targets for a few seconds. If both *Soul-Marks* are applied to the same target, they take **bonus** magic damage.

**Passive:** **Kalista** periodically stocks a *Sentinel* charge, up to a cap.

**Passive - Soul-Marked:** While **Kalista** and her **The Black Spear** are tether, their basic attacks and **Pierce** apply a *Soul-Mark* to the target hit for 4 seconds. If both ''Kalista's* and the *Oathsworn's* *Soul-Mark* are applied to the same enemy, the marks are consumed to deal magic damage to the target, capped against non-champions. *Soul-Mark' cannot affect an enemy more than once every few seconds. **Active:** **Kalista** summons a *Sentinel* that patrols back and forth on a path along the target direction, granting sight of its surroundings as it travels. **Kalista** periodically stocks a *Sentinel* charge, up to a maximum of 2. *See [Pets](#Pets) for more details about Sentinels.*

| Attribute | Value |
|-----------|-------|
| **Range** | 1400 / 5000 units |
| **Cooldown** | 30 seconds |
| **Recharge** | $90-50$ seconds |
| **Cast Time** | $0.5$ seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | magic |
| **Spell Effects** | proc |

**Scaling:**
- **Bonus Magic Damage:** $10-18$% of target's
- **maximum** health
- **Maximum Non-Champion Damage:** $100-200$

**Notes:**

- The damage is dealt on-attack of ''Kalista's* basic attack if she applies the second mark. If the *Oathsworn' does so instead, the damage is dealt on-hit of their basic attack.
- 'Soul-Marked's bonus damage is credited to **Kalista** and will benefit from both her magic penetration and spell effects.
  - If her *The Black Spear* ally scores a kill using 'Soul-Marked's bonus damage they will get a message stating 'Kill Secured' in place of the usual gold pop-up (the gold itself is credited to **Kalista**).
- The 'Sentinel's sight reveal on enemy champion is accredited for assists and potentially kills (if they die shortly after being spotted).
  - It does not reveal stealth targets.
- *Sentinel* will cast from wherever **Kalista** is at the end of the cast time.
- 'Soul Mark's* interaction with *parrying' effects (dodge, block, blind).
- If *Pierce* kills the target by 'Soul-Mark's additional damage, it will/will not continue.

---

### E: Rend

**Passive:** **Kalista**’s basic attacks on-hit and **Pierce** lodge a spear into their target, applying a stack of *Rend* for a few seconds.

**Active:** **Kalista** all lodged spears from nearby enemies, consuming all of their stacks to deal physical damage and slow them for a short time. Each additional spear on the target deals reduced damage.

**Passive:** ''Kalista's* basic attacks on-hit and **Pierce** apply a stack of *Rend' to enemies for 4 seconds, refreshing on subsequent hits and stacking up to 254 times. **Active:** **Kalista** all lodged spears from nearby enemies, consuming all of their stacks to deal them physical damage and slow them for 2 seconds. Each additional spear on the target deals modified damage. *Rend* deals 50% damage against epic monsters. If *Rend* kills at least one target, the *cooldown* is reset and **Kalista** restores *mana*. 'A nearby enemy with a Rend stack is required to cast this ability. Rend can be used during the dash of *Martial Poise* and the cast time of *Pierce*. In-flight spears and *Pierce* if in cast will be empowered to apply Rend's effects to their targets. These empowered spears can also trigger Rend's refund.'

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $10-8$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | 30 Mana |
| **Targeting** | Proximity |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Effect Radius** | 1100 units |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |

**Scaling:**
- **Physical Damage:* $ (+ (+ *Slow:** $10-42$% (+ 5% per 100 AP)
- **Mana Restored:** $10-30$

**Notes:**

- *Rend* cannot activate on untargetable enemies.
- *Rend* applies its effects instantly on cast.
- The cooldown will not reset if the target is protected by resurrection effects.
- If *Rend* is used against **Sion** under the effects of *Glory in Death*, the cooldown will reset even if it does not kill him.
- A stack is not applied if the attack is dodge, block or missed while **Kalista** is blind.
- *Pierce* does not apply a stack of *Rend* if blocked by spell shield.
- While *berserk*, ''Kalista's* attacks will also apply *Rend' stacks on allies.
  - **Kalista** will be unable to cast *Rend* if there are no stacks on an enemy in range. *** *Rend* cannot cast on allies, even if a valid enemy target exists. As such, any spears on allied units are purely cosmetic.
- *Rend* has a lower cast range than effect range.
- *Rend* at maximum stacks will deal a total of $( to )253253% AD) (+ % AP) physical damage.

---

### R: Fate's Call

**Active:** **Kalista** invokes her *The Black Spear* to retrieve her *Oathsworn*, vanish and cleanse them from all crowd control, and hold them for a few seconds.

*While held, the *Oathsworn* may dash to reappear at the target location. If they collide with an enemy champion, they airborne all nearby enemies.*

**Active:** **Kalista** invokes her *The Black Spear* to retrieve and hold her *Oathsworn* for 4 seconds. **Kalista** cleanse the *Oathsworn* from all crowd control and renders them invulnerable and untargetable for the duration, and dash them to her over 1 second. While held, the *Oathsworn* is vanish and may select a target location to dash to with displacement immunity, ending 'Fate's Call's* invocation and reappearing at the location. If the invocation ends without the *Oathsworn* selecting a target location, they will automatically do so at maximum range from *'Kalista's' facing direction. The 'Oathsworn's* dash stops upon colliding with an enemy champion. Upon the dash ending by collision or arriving to the targeted location, the *Oathsworn' airborne all nearby enemies and keeps them airborne for a set duration, while simultaneously lunge to their *attack range **base** attack range* from the closest target hit. 'The Oathsworn must be within tether range to cast this ability, and is also silence and unable to perform movement or attack commands while Fate's Call is in effect.'

| Attribute | Value |
|-----------|-------|
| **Range** | 1200 units |
| **Cooldown** | $160-120$ seconds |
| **Cast Time** | none |
| **Cost** | 100 mana |
| **Targeting** | Proximity |
| **Affects** | *Black Spear* Ally / Enemies |
| **Spell Shield** | True |

**Scaling:**
- **Airborne Duration:** $1-2$ seconds

**Notes:**

- The 'Oathsworn's landing location of the lunge is precisely to their base attack range from the closest target they hit and relative to where this target would be displaced to from the knock back.
  - In other words, the *Oathsworn* will land to a location where they are within their exact base attack range of the target by the end of the target's displacement.
  - The calculated landing spot does not evaluate if the target was successfully displaced or not, meaning that the *Oathsworn* will end up closer than their base attack range if the closest target hit was not knocked back.
- If the *Oathsworn* does not hit any targets with the knock up effect, the dash will simply end and they will reappear at the target location.
- The following will defer 'Fate's Call's* effects at the time of cast to the end of their duration if the *Oathsworn' is:
  - In a cast time.
  - Channel. *** 'Fate's Call' will not defer by the following channels: Recall, *Defiant Dance*, *Realm Warp*, and *The Culling*. *** 'Fate's Call' cannot be cast during the following channels: Teleport, *Gate*, *Grand Starfall*, *Hero's Entrance* and *Stand United*.
  - Using an ability that preloads unstoppableforcemarker. *** If the *Oathsworn* is still occupied after 6 seconds of being deferred from this time, 'Fate's Call' will cancel.
- 'Fate's Call* will not pull the *Oathsworn' if they are dash but they will be retrieved after 1 second regardless.
- 'Fate's Call* can be cast even if the *Oathsworn' is untargetable.
- 'Fate's Call* is disabled if the *Oathsworn* is resurrection or is being affected by another *Fate's Call'.
- 'Fate's Call* can be used even if the *Oathsworn' is in a zombie state.
- 'Fate's Call' does not render the target vanish while being held if they are creating *E*’s aura.
- The *Oathsworn* being held is represented by a light over ''Kalista's' head.
- ''Kalista's* death does not cancel *Fate's Call'.

---

### A

---

### Basic Attack

Basic Attack ''Kalista's* basic attack windup is uncancellable windup except by casting **Rend*', although she can input a new attack command to change her target during the windup. Additionally, ''Kalista's** attack windup is only reduced by $0.75$% per as*bonus'' attack speed*.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Speed** | Kalista |
| **Spell Effects** | Attack |
| **Projectile** | True |

**Notes:**

No additional notes.

---

## Patch History

### V25.16
- Stats
  - Attack damage growth increased to $4.25$ from 4.
- *Rend*
  - First spear AP ratio increased to 65% AP from 20% AP.
  - Subsequent spear AP ratio increased to 50% AP from 20% AP.

### V25.12
- Stats
  - Base health reduced to 560 from 580.
- *Pierce*
  - Cooldown increased to 9 seconds from 8.

### V25.09
- Stats
  - Base health reduced to 580 from 600.
  - Base attack damage reduced to 57 from 59.
  - Attack damage growth increased to 4 from $3.25$.
- *Rend*
  - Base damage reduced to $5-45$ from $10-50$.

### V25.04
- General
  - **Bug Fixes:** Basic attack VFX no longer sometimes fails to properly render, causing it to occasionally be invisible.
- *Martial Poise*
  - **Bug Fixes:** Now properly benefits from certain boots.
- *Sentinel*
  - Mark damage reduced to $10-18$% of target's **maximum** health from $14-18$%.
- Stats
  - Mana regeneration growth increased to $0.8$ from $0.4$.
- *Martial Poise*
  - *Dash is still affected by slow and cripple. Has a new speed floor.*
  - New calculations for dash speed.
  - Base attack speed is now properly referenced. *This is a buff of around 8%.*
  - Influence of attack speed slows has been halved.
    - Attack speed increases still have full influence.
  - Movement speed multiplier changed to ($0.2$% **total** movement speed + $0.25$) from (1 - sum of slows).
    - This is typically close to 100% but now benefits from bonus movement speed and is more resilient to slows. The minimum multiplier is 48%.
  - Restored the dash animation when basic attacking.
  - **Bug Fixes:** Using *Pierce* during her attack winddown animation no longer causes her attack and idle animations to constantly reset when repeatedly inputting movement and stop commands without allowing the winddown animation cycle to finish.
- *Pierce*
  - Mana cost increased to $60-80$ from $50-70$.
  - Base damage reduced to $10-270$ from $20-280$.
- *Rend*
  - Base damage per subsequent spear changed to $7-35$ from $8-24$.
  - AD ratio per subsequent spear reduced to $20-40$% AD from $25-45$% AD.

### V14.13
- Stats
  - Base attack damage reduced to 59 from 61.
- *Fate's Call*
  - Cooldown increased to $160-120 3$ seconds from $150-90 3$.

### V14.4
- *Martial Poise*
  - **Bug Fixes:** Tier 1 *Boots* now properly affect dash distance.
- *Rend*
  - Base damage reduced to $10-50$ from $20-60$.

### V13.20
- Kalista
  - Skin renamed to *Worlds 2015 Kalista* from *Championship Kalista*.

### V13.11
- Stats
  - Base health increased to 600 from 574.
  - Base health regeneration increased to 4 from $3.75$.
  - Health regeneration growth increased $0.75$ from $0.55$.
  - Base attack damage reduced to 61 from 66.
  - Attack damage growth reduced to $3.25$ from $3.75$.
- *Basic Attack*
  - **Removed:*** No longer misses if the target leaves vision before the attack hits.
  - AD ratio increased to 100% AD from 90% AD.
- *Pierce*
  - AD ratio increased to 105% AD from 100% AD.
- *Rend*
  - Base damage per spear reduced to $8-24$ from $10-34$.
  - AD ratio per subsequent spear increased to $25-45$% AD from $23.2-40.6$% AD.
  - Cooldown reduced to $10-8$ from $14-8$.

### V13.10
- *Rend*
  - **New Effect:** Damage now scales with 20% AP and 20% AP per subsequent spear.
  - **New Effect:** Slow now scales with 5% per 100 AP.

### V13.7
- Stats
  - Attack damage growth increased to $3.75$ from $3.5$.
- *Martial Poise*
  - Oathsworn Bond timer reduced to 9 seconds from 13.
  - **New Effect:** Is now forced to channel on the *Black Spear* while her Oathsworn is reviving.

## Trivia

- A fully-stacked *Rend* has the highest AD ratio in the game ($60+20×253-60+35×253$% AD).
- **Kalista** has the longest basic attack windup time of all champions, however this is easily offset by her high base attack speed and attack speed scaling.
- Kalista's dance references spear dance from Game of Thrones.
  - A side-by-side comparison can be seen here.
- Kalista's 'Champion Spotlight' was the first to feature the updated Summoner's Rift map.
- **Kalista** resembles **Varus**.
- *The Black Spear* is the third champion-unique item, the first being *Prototype Hex Core* (**Viktor**’s) and the second being *Bonetooth Necklace* (**Rengar**’s before it merged with *Unseen Predator*).
- Her way of referring to herself in the third person, as well as the echo in her voice, resembles from Mortal Kombat.
- Kalista targeted many betrayers for execution during Harrowing 2014.
- # **Cassiopeia** for backstabbing **Sivir**.
- # **Hecarim** for his treachery against her in the Blessed Isles.
- # **LeBlanc** for aiding the barbarian invaders against **Mordekaiser** as well as betraying countless others.
- # **Lissandra** for murdering Avarosa for her spearheading the rebellion against the Frozen Watchers.
- # **Twisted Fate** for abandoning **Graves** to be imprisoned in the Locker (Kalista is trying to convince the Outlaw to 'give in to his hate' despite being told by the Card Master he and their crew did try to get him out, reconciling their differences and settling the feud)
- # **Xerath** for plotting against **Azir** and laying waste to Ancient Shurima.
- # **Zed** for murdering **Shen**’s father Kusho, slaughtering the Kinkou Order, and founding the Order of Shadow over the remains of their monastery.
- Kalista's Series 1 Eternals make the following references:
  - *Rip and R.I.P.* is a reference to the phrase *"Rip and Tear"*, originating from the "Metal Militia" song from the album Kill 'Em All by the Metallica heavy metal band and popularized by the *Doom (franchise)* franchise.
- The asset for the *Sentinel* was re-used in the Featured Game Mode Hunt of the Blood Moon's *Spirits*.

---
*This page was automatically generated from League of Legends Wiki data.*