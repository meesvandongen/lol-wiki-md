# Zeri

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
| **Champion** | Zeri |
| **Title** | The Spark of Zaun |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2022-01-20 |
| **Release Patch** | V12.2 |
| **Roles** | Marksman |
| **Riot Positions** | Bottom |
| **External Positions** | Bottom |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $600.0$ | $+110.0$ | $2470.0$ |
| **Mana** | $250.0$ | $+45.0$ | $1015.0$ |
| **Health Regen** | $3.25$ | $+0.7$ | $15.1$ |
| **Mana Regen** | $6.0$ | $+0.8$ | $19.6$ |
| **Armor** | $24.0$ | $+4.2$ | $95.4$ |
| **Magic Resist** | $30.0$ | $+1.3$ | $52.1$ |
| **Attack Damage** | $56.0$ | $+2.0$ | $90.0$ |
| **Attack Speed** | $0.658$ | $+2.0\%$ | $0.882$ |
| **Movement Speed** | $330.0$ | $+0.0$ | $330.0$ |
| **Attack Range** | $500.0$ | $+0.0$ | $500.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.658$ |
| **Attack Speed Ratio** | $0.625$ |
| **Bonus AS per Level** | $2.0\%$ |
| **Missile Speed** | $0 units/second$ |
| **Acquisition Radius** | $800 units$ |
| **Pathing Radius** | $40 units$ |
| **Selection Radius** | $100 units$ |
| **Selection Height** | $122.222 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Living Battery

**Innate:** **Zeri** generates charge when she moves and casts **Burst Fire**. Her next basic attack will consume charge to deal magic damage.

*At full charge, the attack consumes all charge to deal magic damage based on the target's **maximum** health.*

**Innate:** **Zeri** generates 1 charge for every 40 units she travels by any means and 10 charge every time she casts **Burst Fire**, up to a maximum of 100 charge. Her basic attacks consume charge to deal *modified damage. **Zeri** gains maximum charge when the game starts and upon respawning. Basic Attack **Zeri** zaps the target, applying spell effects as spell damage, and triggering on-cast effects. This cannot critically strike nor trigger on-hit and on-attack effects. At full charge, ''Zeri's' next attack is empowered to consume all charge to deal 75 to 160 (+ 110% AP) (+ 1 to 11 of target's **maximum** health) magic damage. The damage based on the target's health ratio is capped at 300 against monsters. While not at full charge, ''Zeri's' attacks deal 10+(15/17)*(x-1)*(0.7025+0.0175*(x-1)) (+ 3% AP) magic damage, and execute targets below 6×10 to 6×25 (+ 18% AP) *health*. Each attack consumes 10 charge if she has enough already.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Unit |
| **Affects** | Self |
| **Damage Type** | magic |
| **Speed** | N/A |
| **Spell Shield** | Special |
| **Spell Effects** | Spell |
| **Projectile** | False |

**Notes:**

- Charged attacks only deal the **base** damage to structures.
- Uncharged attacks do not execute enemies that are shield or invulnerable while below the health threshold.
- Spell shield will only block a fully charged attack. Uncharged attacks are not blocked.
- The attack's range is **not** increased from attack range increases (*Rapid Firecannon*). Instead, *Burst Fire*’s reach is.
- Uncharged and charged attacks trigger *Tear of the Goddess* Mana Charge.
- The empowered attack will trigger but not be consumed against wards or jungle plants.

---

### Q: Burst Fire

**Active:** **Zeri** fires a burst of 7 rounds in the target direction that each deal physical damage to the first enemy hit. This is treated as a basic attack, can critically strike, and applies on-hit and on-attack effects to the first enemy hit.

*The *cooldown* scales down with *attack speed*, up to a cap. A portion of the excess attack speed is converted into **bonus attack damage**.*

**Active:** **Zeri** fires a burst of 7 rounds in the target direction that each deal physical damage to the first enemy hit. 'Burst Fire's projectile is treated as a basic attack: it hits any enemy unit a typical basic attack can; deals basic damage; can critically strike for critical damageBurst Fire's *cooldown* and cast time are reduced with *attack speed*, with the maximum of $1.5$ attacks per second. 70% of attack speed in excess of the cap is converted into **bonus attack damage**.

| Attribute | Value |
|-----------|-------|
| **Cast Time** | 100% of ''Zeri's* windup time ($(1/Zeri*base'' attack speed) |
| **Targeting** | Direction |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical |
| **Speed** | 2600 / 3400 units/second |
| **Spell Shield** | False |
| **Spell Effects** | basic |
| **Projectile** | True |

**Scaling:**
- **Total Physical Damage:** $15-23$ (+ $104-120$% AD) Physical Damage per Hit $15/7-23/7 round=2$ (+ $104/7-120/7

**Notes:**

- *Burst Fire* is automatically learned with her first skill point upon spawning.
- Any of 'Burst Fire's* rounds hitting a champion will draw minion aggro, regardless of whether or not other targets were hit first by *Burst Fire's rounds.
  - A lightning chain from a *Lightning Crash* empowered *Burst Fire* will not draw minion aggro.
- Casting *Burst Fire* does not cancel previous attack (*Living Battery*) and movement orders.
- *Burst Fire* does not trigger *Tear of the Goddess* Mana Charge.
- 'Burst Fire's cooldown is not reduced by *Navori Flickerblade* Transcendence.
- , , and flat damage reduction apply to **each** of the 7 instances of damage from *Burst Fire*.
  - *Warden's Mail* flat damage reduction and cap only apply to the first instance of damage.
  - *Guardian's Horn* flat damage reduction only applies to the first instance of damage, but handles it as damage-over-time (reduced to 25% effectiveness).
- *Burst Fire* still counts as an ability activation for the purposes of on-cast effects such as triggering *Force Pulse*’s passive.
  - It does **not** count as an ability activation for Spellblade.
- *Burst Fire* can hit **all** enemy units that a basic attack would be able to target. This does include jungle plants, structures, and wards.
  - Wards will only be hit once by *Burst Fire*. Stealthed wards and traps will not be hit.
  - Champion-summoned units behave differently depending on the unit. *** 'Gangplank's* *Powder Keg* and *Yorick’s* *Dark Procession* are hit only once by *Burst Fire', and any subsequent rounds not empowered by *Spark Surge* will stop upon reaching the same object. *** 'Kalista's* *Sentinel* is hit only once by *Burst Fire*, but any subsequent rounds will continue to travel through the *Sentinel'.
- If *Burst Fire* hits an enemy while **Zeri** is not visible to enemies, the area around her (400 units) will be revealed for $4.5$ seconds.
- *Burst Fire* rolls critical strike for all rounds as well as the additional physical damage dealt when **Zeri** is *Overcharged*.
- *Burst Fire* is parried by dodge and block.
  - Blind causes *Burst Fire* to cast in a random direction. The rounds will still deal damage to enemies hit.
- 'Burst Fire's rounds are each fired in the target direction from where **Zeri** is at the time.
- **Only** attack speed granted by being *Overcharged* can exceed the cap of $1.5$.
  - *Hail of Blades* does not allow **Zeri** to exceed her attack speed cap.
- Items and runes that trigger off of attacking [eg *Fleet Footwork*, *Kraken Slayer*] will only trigger if *Burst Fire* hits a unit.
- If Zeri would get full Energized stacks from a *Burst Fire*, the rest of the rounds will trigger Energized.
- 'Burst Fire's cast-indicator does not show *range* increases; modified range VFX are still visible around the champion model.
- *Burst Fire* can apply the effect of *Horizon Focus* when the enemy hit is within the last 50 units of this ability.
- *Burst Fire* uses a modified icon when empowered by *Spark Surge* () and *Lightning Crash* () as well as both ().
- Destroying a ward that is targetable but not visible to ''Zeri's* team (e.g. a *Control Ward* in Fog of War) via *Burst Fire' uniquely allows her to remove its accompanying ward timer (if it exists) without seeing the ward being destroyed.

---

### W: Ultrashock Laser

**Active:** **Zeri** fires an electric pulse in the target direction that deals physical damage to the first enemy hit and slows them for a short duration.

*If this hits terrain, it transforms into a laser that fires in a line, dealing physical damage to enemies hit and critical strike against champions and monsters.*

**Active:** **Zeri** fires an electric pulse in the target direction that deals physical damage to the first enemy hit and slows them for 2 seconds. If the pulse hits terrain, it transforms into a laser in a line that grants sight of the area for $1.75$ seconds and fires after $0.85$ seconds, applying the same effects to enemies hit and critical strike for critical damage against champions and monsters.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $12-8$ seconds |
| **Cast Time** | type=*bonus attack speed |
| **Cost** | $50-90$ Mana |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Speed** | 2500 units/second |
| **Spell Shield** | True |
| **Spell Effects** | Special |
| **Projectile** | Special |

**Scaling:**
- **Physical Damage:** $30-190$ (+ 130% AD)
- **Slow:** $30-50$%

**Notes:**

- Applies spell damage for the pulse and area damage for the laser.
- The pulse missile is blocked by projectile effects but not the laser.
- *Ultrashock Laser* interacts with player-generated terrain. Effect at cast time start

---

### E: Spark Surge

**Active:** **Zeri** dashes in the target direction, she will dash across any terrain she hits. Afterwards, she gains *Lightning Rounds* for a short time.

*'Spark Surge's* cooldown is cdr for every enemy champion hit by her **charged*' basic attacks and abilities, increased on critical strikes.*

**Active:** **Zeri** dashes in the target direction. She will dash farther across terrain if *Spark Surge* was cast within 50 units of any, gaining unobstructed vision of the surrounding 800 units and sight herself while there are enemy champions within 1500 units of her. Afterwards, she gains *Lightning Rounds* for 5 seconds, empowering **Burst Fire** to deal **bonus** magic damage to the first enemy hit, increased by 0–85, and pierce through enemies. Targets after the first take modified damage which does not apply on-hit effects or nor trigger on-attack effects, but is affected by critical strike modifiers. 'Spark Surge's* **current cooldown** is reduced by $0.5$ seconds for every champion **Zeri** hits with **charged** basic attacks or abilities, increased to $1.5$ seconds if she does so with a cast of **Burst Fire** or **Ultrashock Laser*' that critical strike. *Spark Surge basic attack reset *'Zeri's* basic attack timer and *Burst Fire*’s cooldown. *Ultrashock Laser* and *Lightning Crash* can be cast during the dash.*

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $22-18$ seconds |
| **Cast Time** | None |
| **Cost** | $90-70$ Mana |
| **Targeting** | Direction |
| **Affects** | Enemies, Self |
| **Damage Type** | Magic |
| **Speed** | 600 + 100% movement speed |
| **Spell Effects** | proc |

**Scaling:**
- ** **

**Notes:**

- 'Spark Surge's extended dash travels based on the thickness of the terrain, up to a maximum distance.
- The following table refers for interactions while **Zeri** is dashing:

---

### R: Lightning Crash

**Active:** **Zeri** explodes to deal magic damage to nearby enemies. If this hits at least one enemy champion, she enters *Overcharged* for a few seconds. Damaging an enemy will extend *Overcharged*.

**Overcharged:** **Zeri** gains and . **Burst Fire** is empowered to instead fire 3 rounds that can chain up to 4 surrounding enemies to deal physical damage.

**Active:** **Zeri** discharges an electric nova that deals magic damage to nearby enemies. If this hits at least one enemy champion, she enters *Overcharged* for 5 seconds. Hitting an enemy champion with an ability or **charged** attack will extend the duration by $1.5$ seconds, up to its original duration. **Overcharged:** **Zeri** gains and that is allowed to exceed her attack speed cap by the amount gained. **Burst Fire** is empowered to have a 20% shorter cast time and instead fire 3 rounds that travel with increased speed and chain to the nearest sight enemy within 650 units of the target, up to 4 subsequent targets, to deal 40% AD physical damage. This damage is affected by critical strike modifiers. During *Overcharged*, **Zeri** can generate stacks of *Hypercharged* from enemy champions that last $1.5$ seconds. She generates 1 stack for each one she hits with an ability or **charged** attack, increased to 3 on abilities that critically strike. Subsequent hits refresh the duration of *Hypercharged*. **Burst Fire** grants stacks only against the first target hit. **Hypercharged:** For each stack, **Zeri** gains .

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $80-70$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | 100 Mana |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical Magic |
| **Effect Radius** | 825 units |
| **Spell Shield** | Special |
| **Spell Effects** | Special |
| **Projectile** | False |

**Scaling:**
- **Magic Damage:** $200-400$
- *bonus AD) (+ 110% AP)

**Notes:**

- **Zeri** can move while casting *Lightning Crash*, and it does not interrupt her previous orders.
- **Zeri** can stack *Overcharge* up to 100,000 times.
- Applies area damage for the nova and proc damage for the chain lightning.
- Spell shield blocks 'Lightning Crash's* initial nova but does not prevent **Zeri** from becoming *Overcharged'.
  - It does not block **Burst Fire*’s* electricity chain.
- Clone count for granting *Overcharge*.
- 'Lightning Crash's* and **Spark Surge*’s* empowerments to **Burst Fire*' can combine together.
- **Burst Fire** electricity chain will not chain across structures.
- If **Zeri** enters resurrection during *Overcharge*, she will lose all stacks of the effect but *Overcharge* will not end prematurely.
  - She cannot gain stacks again after reviving while the buff is active. Effect at cast time end

---

## Patch History

### V25.12
- *Lightning Crash*
  - Nova base damage increased to $200-400 3$ from $175-375 3$.
  - Nova bonus AD ratio increased to 100% *bonus AD from 85%.
  - Chain lightning damage increased to 40% AD from 30% AD.

### V25.08
- *Lightning Crash*
  - Chain lightning damage increased to 30% AD from 25% AD.

### V14.18
- *Living Battery*
  - Charged attack base damage changed 75 to 160 from 90+(110/17)*(x-1)*(0.7025+0.0175*(x-1)). *Formula changed.*
  - Charged attack health ratio changed to 1 to 11 of target's **maximum** health from 1+(14/17)*(x-1)*(0.7025+0.0175*(x-1)). *Formula changed.*

### V14.8
- Stats
  - Base health reduced to 600 from 630.

### V14.3
- Stats
  - Base attack damage increased to 56 from 53.
- *Lightning Crash*
  - Cooldown reduced to $80-70 3$ seconds from $100-70 3$.

### V13.24
- Stats
  - Attack damage growth increased to 2 from $1.3$.
- *Ultrashock Laser*
  - Base damage increased to $30-190$ from $20-180$.

### V13.19
- Stats
  - Base movement speed reduced to 330 from 335.

### V13.14
- *Burst Fire*
  - **Bug Fixes:** Now deals the correct amount of damage against Jack in the Box.

### V13.12
- Stats
  - Base movement speed increased to 335 from 330.
- *Living Battery*
  - Tooltip now contains information about ''Zeri's' basic attacks.
  - **Removed Innate:** **Zeri** absorbs the energy of shield she damages, granting herself a shield equal to 45% of the post-mitigation damage dealt to the shield. Each instance of shield gained by *Living Battery* lasts for 3 seconds.
  - ***Removed Innate - Gotta Zip!:*** Whenever **Zeri** gains a shield, she gains *10% **bonus** movement speed* for 2 seconds.
- *Burst Fire*
  - **Removed:*** No longer triggers Spellblade.
- *Spark Surge*
  - Critical strike chance ratio increased to 0–85 from 0–65.

### V13.7
- Stats
  - Health growth reduced to 110 from 115.
- *image=Zeri Living Battery old.png*
  - Shield steal reduced to 45% from 60%.
- *Lightning Crash*
  - Bonus AD ratio reduced to 85% *bonus AD from 100%.

## Trivia

- When **Zeri** and **Ekko** are on the same team, they gain a buff named *Zaunite Ingenuity*, which reads "Kids from the fi
- **Zeri**'s splash was different upon release. After a few days it was edited to show a darker skintone, smaller and more detailed eyes, among other changes. Gif comparison.
- She is the only champion with 0 point at a skill by the client classification.
  - In this case, her Utility rating is 0.
- Her Q can destroy Control wards without having vision on them.

---
*This page was automatically generated from League of Legends Wiki data.*