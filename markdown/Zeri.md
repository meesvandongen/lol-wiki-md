# Zeri

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
| **Champion** | Zeri |
| **Title** | The Spark of Zaun |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2022-01-20 |
| **Release Patch** | V12.2 |
| **Latest Changes** | V25.12 |
| **Roles** | Marksman |
| **Riot Positions** | Bottom |
| **External Positions** | Bottom |
| **Blue Essence** | 2400 |
| **Riot Points** | 880 |
| **Difficulty** | 3 |
| **Hero Type** | Marksman |
| **Adaptive Type** | Physical |
| **Damage** | 2 |
| **Toughness** | 1 |
| **Control** | 2 |
| **Mobility** | 3 |
| **Utility** | 0 |
| **Style** | 35 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $600.0$ | $+110.0$ |
| **Mana** | $250.0$ | $+45.0$ |
| **Health Regen** | $3.25$ | $+0.7$ |
| **Mana Regen** | $6.0$ | $+0.8$ |
| **Armor** | $24.0$ | $+4.2$ |
| **Magic Resist** | $30.0$ | $+1.3$ |
| **Attack Damage** | $56.0$ | $+2.0$ |
| **Attack Speed** | $0.658$ | |
| **Movement Speed** | $330.0$ | $+0.0$ |
| **Attack Range** | $500.0$ | $+0.0$ |
| **Base Attack Speed** | $0.658$ | |
| **Attack Speed Ratio** | $0.625$ | |
| **Bonus AS per Level** | $2.0\%$ | |
| **Missile Speed** | $0$ units/second | |
| **Acquisition Radius** | $800$ units | |
| **Pathing Radius** | $40$ units | |
| **Selection Radius** | $100$ units | |
| **Selection Height** | $122.222$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $110.0\%$ |
| **Damage Taken** | $95.0\%$ |

## Abilities

### Passive: Living Battery

| Attribute | Value |
|-----------|------:|
| **Attack Range** | Zeri |
| **Speed** | N/A (Non-projectile) |
| **Targeting** | Unit |
| **Affects** | Self |
| **Damage Type** | magic |
| **Spell Shield** | Special |
| **Spell Effects** | Spell |
| **Projectile** | False |
| **Parry** | False |

**INNATE:** **Zeri** generates 1 charge for every 40 units she travels by any means and 10 charge every time she casts *Burst Fire*, up to a maximum of 100 charge. Her basic attacks consume charge to deal ***modified** damage.
**Zeri** gains maximum charge when the game starts and upon respawning.

**BASIC ATTACK:** **Zeri** zaps the target, applying spell effects as spell damage, and triggering on-cast effects. This cannot critically strike nor trigger on-hit and on-attack effects.

At full charge, **Zeri**’s next attack is empowered to consume all charge to deal 75 to 160 (+ 110% AP) (+ 1 to 11 of target's **maximum** health) magic damage. The damage based on the target's health ratio is capped at 300 against monsters.

While not at full charge, **Zeri**’s attacks deal 10 / 25 (+ 3% AP) magic damage, and execute targets below 6×10 to 6×25 (+ 18% AP) health. Each attack consumes 10 charge if she has enough already.

**Notes:**

- Charged attacks only deal the **base** damage to structures.
- Uncharged attacks do not execute enemies that are shielded or invulnerable while below the health threshold.
- Spell shield will only block a fully charged attack. Uncharged attacks are not blocked.
- The attack's range is **not** increased from attack range increases (Rapid Firecannon). Instead, Burst Fire’s reach is.
- Uncharged and charged attacks trigger Tear of the Goddess Mana Charge.
- The empowered attack will trigger but not be consumed against wards or jungle plants.

---

### Q: Burst Fire

| Attribute | Value |
|-----------|------:|
| **Range** | cr 750 (Active spray missile reaches) + 100% **bonus** attack range |
| **Cast Time** | 100% of **Zeri**’s windup time ((1/ at **base** attack speed) |
| **Angle** | 5° / 2° |
| **Width** | 80 (Active spray missiles) units |
| **Speed** | 2600 (Active spray missile speed) / 3400 (Empowered spray missile speed while Overcharged) units/second |
| **Static Cooldown** | 1**Zeri**’s attack speed |
| **Queue Time** | $0.25$ (Regular cast) / $0.05$ (Empowered cast while Overcharged) seconds |
| **Targeting** | Direction |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | False |
| **Spell Effects** | basic |
| **Projectile** | True |
| **Parry** | Special |
| **Call For Help** | True |

**ACTIVE:** **Zeri** fires a burst of 7 rounds in the target direction that each deal physical damage to the first enemy hit.

| Attribute | Value |
|-----------|------:|
| **Total Physical Damage** | 15 / 17 / 19 / 21 / 23 (+ 104 / 108 / 112 / 116 / 120% AD) |
| **Physical Damage per Hit** | 2.14 / 2.43 / 2.71 / 3 / 3.29 (+ 14.86 / 15.43 / 16 / 16.57 / 17.14% AD) |

*Burst Fire*’s projectile is treated as a basic attack: it hits any enemy unit a typical basic attack can; deals basic damage; can critically strike for damage; applies on-hit effects to the first enemy hit; and triggers on-attack effects once. *Burst Fire*’s cooldown and cast time are reduced with attack speed, with the maximum of $1.5$ attacks per second. 70% of attack speed in excess of the cap is converted into **bonus** attack damage.

**Notes:**

- *Burst Fire* is automatically learned with her first skill point upon spawning.
- Any of *Burst Fire*’s rounds hitting a champion will draw minion aggro, regardless of whether or not other targets were hit first by *Burst Fire*’s rounds.
  - A lightning chain from a Lightning Crash empowered *Burst Fire* will not draw minion aggro.
- Casting *Burst Fire* does not cancel previous attack (Living Battery) and movement orders.
- *Burst Fire* does not trigger Tear of the Goddess Mana Charge.
- *Burst Fire*’s cooldown is not reduced by Navori Flickerblade Transcendence.
- Tantrum’s, Nimble Fighter’s, and Eclipse’s flat damage reduction apply to **each** of the 7 instances of damage from *Burst Fire*.
  - Warden's Mail flat damage reduction and cap only apply to the first instance of damage.
  - Guardian's Horn flat damage reduction only applies to the first instance of damage, but handles it as damage-over-time (reduced to 25% effectiveness).
- *Burst Fire* still counts as an ability activation for the purposes of on-cast effects such as triggering Force Pulse’s passive.
  - It does **not** count as an ability activation for Spellblade.
- *Burst Fire* can hit **all** enemy units that a basic attack would be able to target. This does include jungle plants, structures, and wards.
  - Wards will only be hit once by *Burst Fire*. Stealthed wards and traps will not be hit.
  - Champion-summoned units behave differently depending on the unit.
    - *Gangplank*’s Powder Keg and *Yorick’s* Dark Procession are hit only once by *Burst Fire*, and any subsequent rounds not empowered by Spark Surge will stop upon reaching the same object.
    - *Kalista*’s Sentinel is hit only once by *Burst Fire*, but any subsequent rounds will continue to travel through the *Sentinel*.
- If *Burst Fire* hits an enemy while **Zeri** is not visible to enemies, the area around her (400 units) will be revealed for $4.5$ seconds.
- *Burst Fire* rolls critical strike for all rounds as well as the additional physical damage dealt when **Zeri** is Overcharged.
- *Burst Fire* is parried by dodge and block.
  - Blind causes *Burst Fire* to cast in a random direction. The rounds will still deal damage to enemies hit.
- *Burst Fire*’s rounds are each fired in the target direction from where **Zeri** is at the time.
- **Only** attack speed granted by being Overcharged can exceed the cap of $1.5$.
  - Hail of Blades does not allow **Zeri** to exceed her attack speed cap.
- Items and runes that trigger off of attacking [eg Fleet Footwork, Kraken Slayer] will only trigger if *Burst Fire* hits a unit.
- If Zeri would get full Energized stacks from a *Burst Fire*, the rest of the rounds will trigger Energized.
- *Burst Fire*’s cast-indicator does not show range increases; modified range VFX are still visible around the champion model.
- *Burst Fire* can apply the effect of Horizon Focus when the enemy hit is within the last 50 units of this ability.
- *Burst Fire* uses a modified icon when empowered by Spark Surge () and Lightning Crash () as well as both ().
- Destroying a ward that is targetable but not visible to **Zeri**’s team (e.g. a Control Ward in Fog of War) via *Burst Fire* uniquely allows her to remove its accompanying ward timer (if it exists) without seeing the ward being destroyed.

---

### W: Ultrashock Laser

| Attribute | Value |
|-----------|------:|
| **Range** | cr 1200 (Initial missile) / er -50 – 1550 (1600 units total empowered length) |
| **Cast Time** | 0.55–0.3@0–0 (@=**bonus** attack speed) seconds |
| **Width** | 80 (Initial missile) / er 200 (Laser beam width) units |
| **Speed** | 2500 (Initial missile) units/second |
| **Cost** | 50 / 60 / 70 / 80 / 90 Mana |
| **Cooldown** | 12 / 11 / 10 / 9 / 8 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Special |
| **Projectile** | Special |

**ACTIVE:** **Zeri** fires an electric pulse in the target direction that deals physical damage to the first enemy hit and slows them for 2 seconds.

If the pulse hits terrain, it transforms into a laser in a line that grants sight of the area for $1.75$ seconds and fires after $0.85$ seconds, applying the same effects to enemies hit and critically striking for damage against champions and monsters.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 30 / 70 / 110 / 150 / 190 (+ 130% AD) (+ 25% AP) |

| Attribute | Value |
|-----------|------:|
| **Slow** | 30 / 35 / 40 / 45 / 50% |

**Notes:**

- Applies spell damage for the pulse and area damage for the laser.
- The pulse missile is blocked by projectile-interception effects but not the laser.
- *Ultrashock Laser* interacts with player-generated terrain.
- This ability will cast from wherever the caster is at the start of the cast time.

---

### E: Spark Surge

| Attribute | Value |
|-----------|------:|
| **Range** | 300 (Standard dash distance) units |
| **Cast Time** | None |
| **Speed** | 600 + 100% movement speed |
| **Cost** | 90 / 85 / 80 / 75 / 70 Mana |
| **Cooldown** | 22 / 21 / 20 / 19 / 18 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Direction |
| **Affects** | Enemies, Self |
| **Damage Type** | Magic |
| **Spell Effects** | proc |
| **Parry** | True |
| **Grounded** | True |
| **Knockdown** | True |

**ACTIVE:** **Zeri** dashes in the target direction. She will dash farther across terrain if *Spark Surge* was cast within 50 units of any, gaining unobstructed vision of the surrounding 800 units and revealing herself while there are enemy champions within 1500 units of her.

Afterwards, she gains *Lightning Rounds* for 5 seconds, empowering *Burst Fire* to deal **bonus** magic damage to the first enemy hit, increased by 0%–85%@0–100 (@=critical strike chance), and pierce through enemies. Targets after the first take modified damage which does not apply on-hit effects or life steal nor trigger on-attack effects, but is affected by critical strike modifiers.

| Attribute | Value |
|-----------|------:|
| **Burst Fire Bonus Magic Damage** | 20 / 22 / 24 / 26 / 28 (+ 12% **bonus** AD) (+ 20% AP) |

| Attribute | Value |
|-----------|------:|
| **Burst Fire Secondary Target Damage** | 80 / 85 / 90 / 95 / 100% |

*Spark Surge*’s **current** cooldown is reduced by $0.5$ seconds for every champion **Zeri** hits with *charged* basic attacks or abilities, increased to $1.5$ seconds if she does so with a cast of *Burst Fire* or *Ultrashock Laser* that critically strikes.

*Spark Surge resets **Zeri**’s basic attack timer and Burst Fire’s cooldown. Ultrashock Laser and Lightning Crash can be cast during the dash.*

**Notes:**

- *Spark Surge*’s extended dash travels based on the thickness of the terrain, up to a maximum distance.
- The following table refers for interactions while **Zeri** is dashing:

| Aspect | State / Notes |
|--------|---------------|
| **Attacking** | Disabled |
| **Movement** | Disabled |
| **Abilities** | Q is disabled. W and R are usable. |
| **Items** | Interrupted by: Shurelya's Battlesong, Youmuu's Ghostblade, Randuin's Omen; Other items: Usable |
| **Summoner Spells** | Usable: Barrier, Clarity, Cleanse, Exhaust, Ghost, Heal, Ignite, Smite, Teleport, Recall; Interrupted by: Flash |
| **Consumables** | Usable |
| **Interrupted by** | Death |

---

### R: Lightning Crash

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Effect Radius** | 825 (Nova radius) units |
| **Cost** | 100 Mana |
| **Cooldown** | 80 / 77.5 / 75 / 72.5 / 70 seconds |
| **Cooldown Start** | post-effect |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical Magic |
| **Spell Shield** | Special |
| **Spell Effects** | Special |
| **Projectile** | False |

**ACTIVE:** **Zeri** discharges an electric nova that deals magic damage to nearby enemies. If this hits at least one enemy champion, she enters *Overcharged* for 5 seconds. Hitting an enemy champion with an ability or *charged* attack will extend the duration by $1.5$ seconds, up to its original duration.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 200 / 250 / 300 / 350 / 400 (+ 100% **bonus** AD) (+ 110% AP) |

**OVERCHARGED:** **Zeri** gains 10% **bonus** movement speed and 30% **bonus** attack speed that is allowed to exceed her attack speed cap (normally 1.5 attacks per second) by the amount gained. *Burst Fire* is empowered to have a 20% shorter cast time and instead fire 3 rounds that travel with increased speed and chain to the nearest visible enemy within 650 units of the target, up to 4 subsequent targets, to deal 40% AD physical damage. This damage is affected by critical strike modifiers.

During *Overcharged*, **Zeri** can generate stacks of *Hypercharged* from enemy champions that last $1.5$ seconds. She generates 1 stack for each one she hits with an ability or *charged* attack, increased to 3 on abilities that critically strike. Subsequent hits refresh the duration of *Hypercharged*. *Burst Fire* grants stacks only against the first target hit.

**HYPERCHARGED:** For each stack, **Zeri** gains $0.5$% **bonus** movement speed.

**Notes:**

- **Zeri** can move while casting *Lightning Crash*, and it does not interrupt her previous orders.
- **Zeri** can stack *Overcharge* up to 100,000 times.
- Applies area damage for the nova and proc damage for the chain lightning.
- Spell shield blocks *Lightning Crash*’s initial nova but does not prevent **Zeri** from becoming *Overcharged*.
  - It does not block *Burst Fire’s* electricity chain.
- Clones count for granting *Overcharge*.
- *Lightning Crash*’s and *Spark Surge’s* empowerments to *Burst Fire* can combine together.
- *Burst Fire* electricity chain will not chain across structures.
- If **Zeri** enters resurrection during *Overcharge*, she will lose all stacks of the effect but *Overcharge* will not end prematurely.
  - She cannot gain stacks again after reviving while the buff is active.

- This ability will cast from wherever the caster is at the end of the cast time.

---

## Patch History

### V25.12
- Lightning Crash
  - Nova base damage increased to 200 / 300 / 400 from 175 / 275 / 375.
  - Nova bonus AD ratio increased to 100% **bonus** AD from 85%.
  - Chain lightning damage increased to 40% AD from 30% AD.

### V25.08
- Lightning Crash
  - Chain lightning damage increased to 30% AD from 25% AD.

### V14.18
- Living Battery
  - Charged attack base damage changed 75 to 160 from 90 / 200. *Formula changed.*
  - Charged attack health ratio changed to 1 to 11 of target's **maximum** health from 1% / 15%. *Formula changed.*

### V14.8
- Stats
  - Base health reduced to 600 from 630.

### V14.3
- Stats
  - Base attack damage increased to 56 from 53.
- Lightning Crash
  - Cooldown reduced to 80 / 75 / 70 seconds from 100 / 85 / 70.

### V13.24
- Stats
  - Attack damage growth increased to 2 from $1.3$.
- Ultrashock Laser
  - Base damage increased to 30 / 70 / 110 / 150 / 190 from 20 / 60 / 100 / 140 / 180.

### V13.19
- Stats
  - Base movement speed reduced to 330 from 335.

### V13.14
- Burst Fire
  - **Bug Fixes:** Now deals the correct amount of damage against Shaco’s Jack in the Box.

### V13.12
- Stats
  - Base movement speed increased to 335 from 330.
- Living Battery
  - Tooltip now contains information about **Zeri**’s basic attacks.
  - **REMOVED INNATE:** **Zeri** absorbs the energy of shields she damages, granting herself a shield equal to 45% of the post-mitigation damage (Damage calculated after modifiers) dealt to the shield. Each instance of shield gained by *Living Battery* lasts for 3 seconds.
  - ***REMOVED INNATE - GOTTA ZIP!:*** Whenever **Zeri** gains a shield, she gains 10% **bonus** movement speed for 2 seconds.
- Burst Fire
  - **Removed:*** No longer triggers Spellblade.
- Spark Surge
  - Critical strike chance ratio increased to 0%–85%@0–100 (@=critical strike chance) from 0%–65%@0–100 (@=critical strike chance).

### V13.7
- Stats
  - Health growth reduced to 110 from 115.
- Living Battery
  - Shield steal reduced to 45% from 60%.
- Lightning Crash
  - Bonus AD ratio reduced to 85% **bonus** AD from 100%.

## Trivia

- When Zeri and Ekko are on the same team, they gain a buff named *Zaunite Ingenuity*, which reads *"Kids from the Zaun stick together."*
- Zeri's splash was different upon release. After a few days it was edited to show a darker skintone, smaller and more detailed eyes, among other changes. Gif comparison.
- She is the only champion with 0 point at a skill by the client classification.
  - In this case, her Utility rating is 0.
- Her Q can destroy Control wards without having vision on them.

---
*This page was automatically generated from League of Legends Wiki data.*