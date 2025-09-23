# Smolder

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
| **Champion** | Smolder |
| **Title** | the Fiery Fledgling |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2024-01-31 |
| **Release Patch** | V14.2 |
| **Latest Changes** | V25.15 |
| **Roles** | Marksman |
| **Riot Positions** | Bottom |
| **External Positions** | Top, Middle, Bottom |
| **Blue Essence** | 3150 |
| **Riot Points** | 975 |
| **Difficulty** | 2 |
| **Hero Type** | Marksman |
| **Alt Type** | Mage |
| **Adaptive Type** | Physical |
| **Damage** | 3 |
| **Toughness** | 1 |
| **Control** | 1 |
| **Mobility** | 1 |
| **Utility** | 1 |
| **Style** | 10 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $575.0$ | $+100.0$ |
| **Mana** | $300.0$ | $+40.0$ |
| **Health Regen** | $3.75$ | $+0.6$ |
| **Mana Regen** | $8.5$ | $+0.7$ |
| **Armor** | $24.0$ | $+4.7$ |
| **Magic Resist** | $30.0$ | $+1.3$ |
| **Attack Damage** | $60.0$ | $+2.3$ |
| **Attack Speed** | $0.638$ | |
| **Movement Speed** | $330.0$ | $+0.0$ |
| **Attack Range** | $550.0$ | $+0.0$ |
| **Base Attack Speed** | $0.638$ | |
| **Attack Speed Ratio** | $0.638$ | |
| **Bonus AS per Level** | $4.0\%$ | |
| **Missile Speed** | $1800$ units/second | |
| **Acquisition Radius** | $900$ units | |
| **Pathing Radius** | $30$ units | |
| **Selection Radius** | $130$ units | |
| **Selection Height** | $150$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Taken** | $102.0\%$ |

## Abilities

### Passive: Dragon Practice

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Self/Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | proc |
| **Parry** | True |

**INNATE:** **Smolder** generates a stack of *Dragon Practice* each time he hits an enemy champion with an ability, or whenever *Super Scorcher Breath* kills a minion or monster. His basic abilities deal **bonus** magic damage based on stacks.
- *Q*: Deals **bonus** magic damage equal to key=% (+ Infinity Edge key=%*0.4 for 11)|critical damage (based on critical strike chance) of *Dragon Practice* stacks.
- *W*: Explosions deal **bonus** magic damage equal to 55% of *Dragon Practice* stacks.
- *E*: Attacks each deal **bonus** magic damage equal to 12% of *Dragon Practice* stacks.

**Notes:**

- Super Scorcher Breath still grants stacks if the target dies while the initial missile is in flight.
- Achooo! can only grant a stack of Dragon Practice with the first hit against each enemy champion.
- Flap, Flap, Flap can only grant one stack of Dragon Practice per cast.

---

### Q: Super Scorcher Breath

| Attribute | Value |
|-----------|------:|
| **Range** | 500 (Tier 2 missiles) units |
| **Cast Time** | 100% of **Smolder**’s windup time ((1/ at **base** attack speed) |
| **Effect Radius** | 285 (Tier 1 explosion) / 150 (Tier 2 missile explosions at max range) units |
| **Angle** | er 15 (Angle between tier 2 missiles)° |
| **Speed** | 1800 (Fireball) / 900 (Tier 2 missiles) units/second |
| **Cost** | 25 mana |
| **Cooldown** | 5.5 / 5 / 4.5 / 4 / 3.5 seconds |
| **Queue Time** | $0.05$ seconds |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Physical True |
| **Spell Shield** | True |
| **Spell Effects** | Special |
| **Projectile** | True |
| **Parry** | Special |

**ACTIVE:** **Smolder** spits a fireball at the target enemy that deals physical damage, increased by key=% (+ Infinity Edge key=%) (based on critical strike chance); applies on-hit effects; applies life steal at 50% effectiveness; and triggers on-attack effects. If *Super Scorcher Breath* kills at least one target, **Smolder** restores 15 mana.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 65 / 80 / 95 / 110 / 125 (+ 130% **bonus** AD) |

| Attribute | Value |
|-----------|------:|
| **Maximum Physical Damage** | 113.75 / 140 / 166.25 / 192.5 / 218.75 (+ 227.5% **bonus** AD) |
| **Maximum Damage with Infinity Edge Infinity Edge** | 139.75 / 172 / 204.25 / 236.5 / 268.75 (+ 279.5% **bonus** AD) |

*Super Scorcher Breath* becomes empowered with new effects based on the number of *Dragon Practice stacks*:

**TIER 1 - 25 STACKS:** The fireball explodes upon collision to deal the same physical damage to nearby enemies. The explosion benefits from life steal at 50% effectiveness and applies on-hit effects.

**TIER 2 - 125 STACKS:** The fireball sends forth 2 (+ $0.8$ per 100 *Dragon Practice* stacks) bolts of fire in an arc upon collision that explode at maximum range, dealing 50% of the same damage to enemies hit. The bolts benefit from life steal at 50% effectiveness and apply on-hit effects.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 32.5 / 40 / 47.5 / 55 / 62.5 (+ 65% **bonus** AD) |

| Attribute | Value |
|-----------|------:|
| **Maximum Physical Damage** | 56.875 / 70 / 83.125 / 96.25 / 109.375 (+ 113.75% **bonus** AD) |
| **Maximum Physical Damage with Infinity Edge Infinity Edge** | 69.875 / 86 / 102.125 / 118.25 / 134.375 (+ 139.75% **bonus** AD) |

**TIER 3 - 225 STACKS:** Enemies hit are set on fire for 3 seconds, causing them to take **bonus** true damage equal to「 $2.5$% per 100 **bonus** AD (+ $0.5$% per 100 *Dragon Practice* stacks) of their **maximum** health over the duration, capped at 300 against monsters. ⟷ 0.83% per 100 **bonus** AD (+ 0.17% per 100 *Dragon Practice* stacks) of their **maximum** health each second, capped at 100 per tick against monsters. 」Subsequent applications of the burn stack, though not refreshing the duration of the previous burn. Enemy champions afflicted with the burn are also executed once **Smolder** deals damage to them that would leave them below $6.5$% of their **maximum** health.

**Notes:**

- Any applied *Dragon Practice* damage is affected by the same damage modifiers as *Super Scorcher Breath* and does not benefit from the ability's life steal.
- Applies default damage for the burn, spell damage and procs basic damage required effects for the fireball, and area damage for the explosions.
- Only **Smolder**’s damage counts for triggering the execute.
  - If the target is below the threshold while afflicted with the burn, they will be executed on the next instance of damage dealt by **Smolder**, which includes the burn tick.
- If *Super Scorcher Breath* would damage the target to below the execution threshold, they are executed on the same damage instance.
- The number of additional bolts launched based on Dragon Practice stacks is rounded up.
  - At 125 Dragon Practice stacks, the fireball will launch 3 bolts.
- At 225 Dragon Practice stacks, the burn's damage based on the target's health ratio will increase by at least $1.125$%.
- *Super Scorcher Breath* can only hit enemies once per cast.
- The minion and monster damage modifier applies only to the physical damage.
- *Super Scorcher Breath*’s initial fireball, explosion and extra bolts can all be blocked and dodged.
  - The application of the burn is negated in all cases.

---

### W: Achooo!

| Attribute | Value |
|-----------|------:|
| **Range** | 1500 units |
| **Cast Time** | $0.35$ seconds |
| **Effect Radius** | 385 (Explosion) units |
| **Width** | 115 (Sneeze) units |
| **Speed** | 2000 (Initial speed) / 400 (End speed) units/second |
| **Cost** | 50 / 55 / 60 / 65 / 70 mana |
| **Cooldown** | 14 / 13 / 12 / 11 / 10 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | physical |
| **Spell Shield** | Special |
| **Spell Effects** | spellaoe |
| **Projectile** | True |

**ACTIVE:** **Smolder** sneezes a fiery glob in the target direction that deals physical damage to enemies hit and slows them by 35% for $1.5$ seconds, slowing down in missile speed after travelling 1200 (estimated) units. Hitting an enemy champion creates an explosion that deals physical damage to nearby enemies, with subsequent explosions against the same target dealing 75% damage of the previous explosion's damage.

| Attribute | Value |
|-----------|------:|
| **Glob Physical Damage** | 60 / 70 / 80 / 90 / 100 (+ 60% **bonus** AD) |

| Attribute | Value |
|-----------|------:|
| **Explosion Physical Damage** | 10 / 35 / 60 / 85 / 110 (+ 65% **bonus** AD) (+ 80% AP) |

| Attribute | Value |
|-----------|------:|
| **Total Physical Damage On Champion Hit** | 70 / 105 / 140 / 175 / 210 (+ 125% **bonus** AD) (+ 80% AP) |

**Notes:**

- **Smolder** can't declare basic attacks for a brief moment after the cast time.
- Spell shield can block either the glob or the explosion, but not both.

---

### E: Flap, Flap, Flap

| Attribute | Value |
|-----------|------:|
| **Range** | 700 units |
| **Cast Time** | none |
| **Speed** | 1800 (Bolt speed) units/second |
| **Cost** | 65 mana |
| **Cooldown** | 24 / 22 / 20 / 18 / 16 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | physical |
| **Spell Shield** | Special |
| **Spell Effects** | spell |
| **Parry** | False |
| **Grounded** | True |

**ACTIVE:** **Smolder** takes flight for $1.25$ seconds, becoming ghosted and gaining 75% **bonus** movement speed, unobstructed vision, and the ability to ignore terrain collision. While in flight, he automatically fires up to 5 (+ 1 per 100 *Dragon Practice* stacks) bolts at the nearest and most wounded (Lowest health percent) visible enemy, dealing physical damage with each hit and prioritizing enemy champions.

| Attribute | Value |
|-----------|------:|
| **Physical Damage per Hit** | 10 / 15 / 20 / 25 / 30 (+ 30% AD) |

| Attribute | Value |
|-----------|------:|
| **Minimum Total Physical Damage** | 50 / 75 / 100 / 125 / 150 (+ 150% AD) |

*Flap, Flap, Flap* ends immediately if **Smolder** casts one of his abilities or becomes immobilized.

**Notes:**

- **Smolder**’s attack range is reduced to 0 during *Flap, Flap, Flap*.
- **Smolder** can't declare basic attacks for a brief moment after the end of the effect.
  - This is due to his attack range being reduced to 0, it doesn't apply when **Smolder** is very close to his target.
- The number of additional bolts based on Dragon Practice stacks is rounded down.
- **Smolder** reveals himself while attacking enemies, even inside terrain.
- Spell shield will block only one bolt.
- Self immobilizations such as Zhonya's Hourglass also count for ending *Flap, Flap, Flap*.
- Recall is disabled while **Smolder** is inside terrain.
- *Flap, Flap, Flap* can interact with player-generated terrain.
- If **Smolder** is inside terrain when the effect ends, he will be placed correspondingly to the nearest valid space.
- The following table refers for interactions while **Smolder** is flying:

---

### R: MMOOOMMMM!

| Attribute | Value |
|-----------|------:|
| **Range** | 4250 / -600 (Backwards range) units |
| **Cast Time** | $0.75$ seconds |
| **Effect Radius** | 1000 (Vision radius) units |
| **Width** | 125 (Sweetspot) units |
| **Speed** | 1700 units/second |
| **Cost** | 100 mana |
| **Cooldown** | 120 / 115 / 110 / 105 / 100 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies, Self |
| **Damage Type** | physical |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | True |

**ACTIVE:** **Smolder** shouts for his mother to attack from above, causing her to exhale a wave of fire from behind him and towards the target direction, granting sight of its surroundings (Cannot grant sight through terrain and can only grant sight into brush when the center part goes through that brush) as it travels. The wave heals **Smolder** and deals physical damage to enemies hit, with those in the center taking 50% increased damage and becoming slowed by 40% for 2 seconds.

| Attribute | Value |
|-----------|------:|
| **Self Heal** | 100 / 117.5 / 135 / 152.5 / 170 (+ 50% **bonus** AD) (+ 75% AP) |

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 200 / 250 / 300 / 350 / 400 (+ 110% **bonus** AD) (+ 100% AP) |
| **Increased Physical Damage** | 300 / 375 / 450 / 525 / 600 (+ 165% **bonus** AD) (+ 150% AP) |

*MMOOOMMMM!* deals 50% damage against minions and monsters.

**Notes:**

No additional notes.

---

## Patch History

### V25.15
- Super Scorcher Breath
  - **Bug Fixes:** No longer causes a short lockout on the next attack after the ability is cast or canceled.
- Flap, Flap, Flap
  - **Bug Fixes:** Spell shields are no longer consumed by a bolt without blocking its damage.

### V25.11
- Stats
  - Base health regeneration reduced to $3.75$ from $4.5$.
  - Base armor reduced to 24 from 26.

### V25.10
- Dragon Practice
  - Super Scorcher Breath stack ratio increased to key=% from key=%.
  - Flap, Flap, Flap stack ratio increased to 12% from 10%.
- Super Scorcher Breath
  - Tier 3 burn Dragon Practice stack ratio increased to $0.5$% per 100 Dragon Practice stacks from $0.4$%.

### V25.06
- Achooo!
  - Glob base damage changed to 60 / 70 / 80 / 90 / 100 from 30 / 50 / 70 / 90 / 110.
  - Explosion base damage reduced to 10 / 35 / 60 / 85 / 110 from 30 / 50 / 70 / 90 / 110.
  - Explosion bonus AD ratio increased to 65% **bonus** AD from 60%.
- Flap, Flap, Flap
  - Base damage per hit increased to 10 / 15 / 20 / 25 / 30 from 5 / 10 / 15 / 20 / 25.
  - AD ratio per hit increased to 30% AD from 25% AD.
- MMOOOMMMM!
  - Cooldown reduced to 120 / 110 / 100 seconds from 120 at all ranks.

### V25.S1.2
- Dragon Practice
  - **New Effect:** Super Scorcher Breath bonus damage based on critical chance is now affected by critical damage bonuses.
- Super Scorcher Breath
  - **New Effect:** Damage based on critical chance is now affected by critical damage bonuses.

### V14.23
- Dragon Practice
  - Super Scorcher Breath Dragon Practice stack ratio reduced to key=% from key=%.
  - Flap, Flap, Flap Dragon Practice stack ratio reduced to 10% from 20%.
- Super Scorcher Breath
  - **New Effect:** Now refunds 15 mana if this ability kills at least one target.
  - Base damage increased to 65 / 80 / 95 / 110 / 125 from 15 / 25 / 35 / 45 / 55.
  - AD ratio changed to 130% **bonus** AD from 100% **total** AD.
  - **Removed:*** Damage to non-champions reduced to 100% from 110%.
  - Tier 2 base missiles increased to 2 from 1.
  - Tier 2 bonus missiles per 100 Dragon Practice stacks reduced to $0.8$ from $1.5$.
  - Tier 2 missile explosion damage reduced to 50% from 75%.
    - Explosion base damage changed to 32.5 / 40 / 47.5 / 55 / 62.5 from 11.25 / 18.75 / 26.25 / 33.75 / 41.25.
    - Explosion AD ratio changed to 65% **bonus** AD from 75% **total** AD.
  - **Undocumented:** Tier 2 missile on-hit damage modifier increased to 100% from 75%.
  - Tier 3 burn AD ratio increased to $2.5$% per 100 **bonus** AD from 2% per 100 **bonus** AD.
  - **Removed:*** Tier 3 burn no longer scales with 1% per 100 AP.
  - Tier 3 burn Dragon Practice stack ratio reduced to $0.4$% per 100 Dragon Practice stacks from $0.8$% per 100 Dragon Practice stacks.
- Achooo!
  - Mana cost changed to 50 / 55 / 60 / 65 / 70 from 60 at all ranks.
  - Glob base damage reduced to 30 / 50 / 70 / 90 / 110 from 45 / 75 / 105 / 135 / 165.
  - Glob bonus AD ratio increased to 60% **bonus** AD from 25%.
  - **Removed:*** Glob damage no longer scales with 20% AP.
  - Explosion base damage increased to 30 / 50 / 70 / 90 / 110 from 25 / 40 / 55 / 70 / 85.
  - Explosion bonus AD ratio increased to 60% **bonus** AD from 25%.
  - Non-champion damage reduced to 100% from 140%.
- Flap, Flap, Flap
  - Base damage per bolt reduced to 5 / 10 / 15 / 20 / 25 from 15 / 20 / 25 / 30 / 35.
  - AD ratio per bolt increased to 25% AD from 10% AD.
  - Number of bolts per 100 Dragon Practice stacks increased to 1 from $0.75$.
- MMOOOMMMM!
  - Cooldown reduced to 120 seconds at all ranks from 140 / 130 / 120.
  - **New Effect:** Now deals 50% damage against non-champions.

### V14.18
- Stats
  - Base health reduced to 575 from 605.
- Super Scorcher Breath
  - Base damage reduced to 15 / 25 / 35 / 45 / 55 from 20 / 30 / 40 / 50 / 60.
  - **Removed:*** Damage no longer scales with 15% AP.
- Achooo!
  - Base damage reduced to 45 / 75 / 105 / 135 / 165 from 50 / 80 / 110 / 140 / 170.

### V14.14
- Super Scorcher Breath
  - Base damage increased to 20 / 30 / 40 / 50 / 60 from 15 / 25 / 35 / 45 / 55.

### V14.12
- Super Scorcher Breath
  - **Bug Fixes:** Fixed a bug that caused attacks that tag multiple targets to not receive the damage amp from Press the Attack.

### V14.11
- Stats
  - Base attack damage increased to 60 from 57.
- Dragon Practice
  - Super Scorcher Breath bonus damage increased to key=% of Dragon Practice stacks from key=%.
- Super Scorcher Breath
  - Mana cost changed to 25 at all ranks from 23 / 26 / 29 / 32 / 35.
  - Critical strike chance ratio increased to key=% from key=%.

## Trivia

- 
  - In Smolder's case, Dragon Practice stacks gained through I permanently increase the bonus damage of his basic abilities, E’s bolt amount, and Q’s bolt amount and burn damage as it upgrades.
- Smolder's joke animation references a popular meme of the character Toothless from *How to Train Your Dragon* series being depicted dancing.
- Smolder's dance animation references the 1988 movie Oliver and Company. Striking a similarity to Oliver's dance.

---
*This page was automatically generated from League of Legends Wiki data.*