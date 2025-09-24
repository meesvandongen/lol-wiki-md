# Master_Yi

## Table of Contents
- [Basic Information](#basic-information)
- [Abilities](#abilities)
- [Patch History](#patch-history)
- [Trivia](#trivia)

## Basic Information

| Attribute | Value |
|-----------|------:|
| **Champion** | Master Yi |

## Abilities

### Passive: Double Strike

| Attribute | Value |
|-----------|------:|
| **Effect Radius** | 300 (Secondary target range) units |
| **Targeting** | Passive |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Effects** | attack |
| **Parry** | Special |
| **Call For Help** | True |

**INNATE:** **Master Yi**’s basic attacks on-hit generate a stack of *Double Strike* for 4 seconds, refreshing on subsequent hits and stacking up to 3 times. At 3 stacks, **Master Yi**’s next basic attack on-attack is empowered to consume the stacks on-hit and strike twice, the second strike dealing 50% AD physical damage.

The second strike applies on-hit effects, triggers on-attack effects, and is affected by critical strike modifiers.

If **Master Yi**’s primary target is killed before the second strike, he will automatically target another nearby enemy.

*The second strike can critically strike against structures.*

**Notes:**

- The second strike separately rolls a critical strike.
- *Double Strike* does not cause **Master Yi** to attack twice. Effects that trigger on completing the attack windup will only trigger once and thus do not interact with *Double Strike*. (e.g. Hail of Blades).
- Since *Double Strike* resets the counter with the first strike, and the second strike applies on-hit effects, the second strike is able to add a stack for the next *Double Strike* activation.
- If *Double Strike*’s target becomes untargetable shortly after the triggering attack, it will still occur and deal its effects as usual.
- Basic attacks that are dodged, blocked, or missed while **Master Yi** is blinded will not generate any stacks of *Double Strike* nor refresh the duration of active stacks.
- *Double Strike*’s interaction with parry effects at 3 stacks (the damage of the strikes is negated in all cases):
  - If the triggering basic attack is dodged, **Master Yi** will not perform the second strike nor consume the stacks.
  - If the triggering basic attack is blocked, **Master Yi** will perform the second strike but not consume the stacks.
  - If the triggering basic attack misses while **Master Yi** is blinded, he will consume the stacks.
    - The second strike will fail to hit the target at all if the triggering attack misses. On-attack effects do not occur, despite the attack animation playing.

---

### Q: Alpha Strike

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 600 units |
| **Effect Radius** | 600 units |
| **Cost** | 50 / 55 / 60 / 65 / 70 Mana |
| **Cooldown** | 20 / 19.5 / 19 / 18.5 / 18 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Unit |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | Special |
| **Spell Effects** | AoE |
| **Parry** | Special |
| **Grounded** | True |

**ACTIVE:** **Master Yi** vanishes and becomes unable to act. After , he marks the target enemy and then proceeds to mark the nearest visible un-marked enemy within 600 units, recurring every 「 up to 3 times. ⟷ for a total of 4 marks. 」If there are no other un-marked targets before then, **Master Yi** can mark the same enemies again. During *Alpha Strike*, **Master Yi** can select a direction from the primary target.

Upon finishing marking, **Master Yi** reappears 75 units in the targeted direction, or in front of the target otherwise, and then becomes able to act again「 after . ⟷ $1.087$ seconds total (Effective time due to game ticks) after the start of the cast with 4 bounces. 」If the primary target dies or is too far away during the delay, **Master Yi** will reappear at the initial cast location instead.

**Master Yi** then detonates the marks to deal physical damage and apply on-hit effects, with on-hit damage reduced to 65% effectiveness. Marks after the first on the same target instead detonate instantly upon application to deal $25$% damage and apply on-hit effects, with on-hit damage reduced to 16.25% effectiveness. *Alpha Strike* deals **bonus** physical damage to monsters per hit.

| Attribute | Value |
|-----------|------:|
| **Primary Physical Damage** | 20 / 40 / 60 / 80 / 100 (+ $70$% AD) |
| **Reduced Damage per hit** | 5 / 10 / 15 / 20 / 25 (+ 17.5% AD) |
| **Maximum Single-Target Damage** | 35 / 70 / 105 / 140 / 175 (+ 122.5% AD) |

| Attribute | Value |
|-----------|------:|
| **Primary Bonus Monster Damage** | 60 / 85 / 110 / 135 / 160 |
| **Combined Primary Monster Damage** | 80 / 125 / 170 / 215 / 260 (+ $70$% AD) |
| **Reduced Monster Damage per hit** | 20 / 31.25 / 42.5 / 53.75 / 65 (+ 17.5% AD) |
| **Max Single-Target Monster Damage** | 140 / 218.75 / 297.5 / 376.25 / 455 (+ 122.5% AD) |

*Alpha Strike*’s primary and lesser damage can critically strike for「 and damage respectively. ⟷ 100% + 100% of **bonus** critical damage, reduced to 25% for the lesser damage. 」

Basic attacks on-hit reduce *Alpha Strike*’s **current** cooldown by 1 second, affected by ability haste.

*Wuju Style and Highlander can be cast during Alpha Strike. Each time Alpha Strike hits a target, the **current** durations of Wuju Style and Highlander are refreshed. Alpha Strike does not trigger its cooldown reduction nor grant a stack of Double Strike when applying on-hit effects.*

**Notes:**

- *Alpha Strike*’s primary damage applies after **Master Yi** reappears.
- *Alpha Strike*’s cooldown reduction applies on structures.
- When **Master Yi** bounces from a unit it will grant vision in a 600 radius around it for up to after *Alpha Strike* ends, including across terrain, though not into brush.
  - It will always grant vision around the primary target, even if he doesn't bounce off it.
- *Alpha Strike*’s damage is calculated at the moment that each mark is placed. Because of this, it is possible to inflict differing amounts of damage if **Master Yi's **AD changes between marks.
- **Master Yi** will follow all his primary target's movements.
  - *Alpha Strike* can only follow up-to 2000 (From Yi's location at start of cast) units; If the target teleports a very long distance, it will not be followed.
- While vanished, the initial cast location of *Alpha Strike* counts as **Master Yi**’s position, which is considered for effects such as tethers.
- If the primary target becomes untargetable and there are no other nearby valid targets, *Alpha Strike* will end prematurely and will not deal the damage occurring after **Master Yi**’s reappearance.
- The choice of exit direction occurs through an indicator anchored to the target during the time that **Master Yi** is vanished. You are unable to choose the exit direction after the third mark. Selection commands (default: MB1/left click) and movement commands (default: MB2/right click) are both valid.
  - Multiple commands can be issued, but only the *final* command before exiting will determine the direction.
- If the primary target is a champion, **Master Yi** will be automatically ordered to basic attack them after *Alpha Strike* ends.
  - **Master Yi** will not be able to buffer other abilities.
- Only the first instance of damage counts as a hit for effects such as Conqueror, Electrocute and Eclipse Ever Rising Moon.
- *Alpha Strike* does not interact with Permafrost and Press the Attack.
- Spell shield will only block a single instance of damage.
- On-hit damage applied by *Alpha Strike* will be negated by dodge and block, but not while **Master Yi** is blinded.
  - *Alpha Strike*’s own damage will not be negated.
- If **Master Yi** dies during *Alpha Strike* and he is only focusing one target, the ability will only deal the reduced damage strikes and not the final tick of damage from reappearing.
  - If he is striking multiple targets the damage will be dealt immediately when *dying*, and if he *dies* before the bounce from the last target, *Alpha Strike* will deal reduced damage only to all units he already bounced from.
- If **Master Yi** enters resurrection during *Alpha Strike* and he is only focusing one target, the ability will deal the reduced damage strikes as normal and the final tick of damage after being *resurrected*.
  - If he is striking multiple targets, *Alpha Strike* will *sometimes* deal the reduced damage from repeated bounces and the main damage after being *resurrected*, and *sometimes* only the reduced damage, unrelated to the number of targets available.
- The following table refers for interactions while **Master Yi** is performing *Alpha Strike*:

---

### W: Meditate

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Cost** | 40 mana + 6% **maximum** mana per second |
| **Cooldown** | 10 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Auto |
| **Affects** | Self |
| **Silence** | True |

**ACTIVE:** **Master Yi** channels for up to 4 seconds, healing himself every $0.5$ seconds, increased by 0%–100%@0–100 (@=**missing** health).

| Attribute | Value |
|-----------|------:|
| **Minimum Heal Per Tick** | 15 / 25 / 35 / 45 / 55 (+ $12.5$% AP) |
| **Maximum Heal Per Tick** | 30 / 50 / 70 / 90 / 110 (+ 25% AP) |

| Attribute | Value |
|-----------|------:|
| **Minimum Total Heal** | 120 / 200 / 280 / 360 / 440 (+ 100% AP) |
| **Maximum Total Heal** | 240 / 400 / 560 / 720 / 880 (+ 200% AP) |

While channeling, **Master Yi** gains 70% damage reduction for the first $0.5$ seconds, which is then modified to a reduced amount for the remaining duration of the channel. *Meditate*’s damage reduction is halved against turrets and lingers for $0.5$ seconds after the channel ends.

| Attribute | Value |
|-----------|------:|
| **Modified Damage Reduction** | 45 / 47.5 / 50 / 52.5 / 55% |
| **Turret Modified Damage Reduction** | 22.5 / 23.75 / 25 / 26.25 / 27.5% |

*Meditate resets **Master Yi**’s basic attack timer, pauses Wuju Style’s and Highlander’s duration, and grants one stack of Double Strike per second during the channel.*

**Notes:**

- *Meditate*’s healing amount updates dynamically as he heals himself, making **Master Yi** heal for less as his **current** health increases.
- The first tick of healing may occur at any moment on the first $0.5$ seconds of the channel. This means immediately canceling the channel for the attack reset does not guarantee one tick of healing, though it may occur with some chance.
- The following table refers for interactions while **Master Yi** is channeling:

---

### E: Wuju Style

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Cooldown** | 14 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Auto |
| **Affects** | Self |
| **Damage Type** | True |
| **Spell Effects** | Proc |
| **Parry** | Special |

**ACTIVE:** **Master Yi** empowers his basic attacks within the next 5 seconds to deal **bonus** true damage on-hit.

| Attribute | Value |
|-----------|------:|
| **Bonus True Damage** | 20 / 25 / 30 / 35 / 40 (+ 35% **bonus** AD) |

**Notes:**

- *Wuju Style* can be cast during *Alpha Strike*.
- *Wuju Style* cannot be cast while it is already active.
- *Wuju Style*’s bonus true damage does not interact with critical strikes.
- *Wuju Style*’s bonus true damage cannot be blocked but can be dodged and/or missed if **Master Yi** is blinded.
- The attacks do not deal the **bonus** damage against structures.
- While *Wuju Style* is active, the damage will dynamically update its calculations.

---

### R: Highlander

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Cost** | 100 Mana |
| **Cooldown** | 85 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Auto |
| **Affects** | Self |

**PASSIVE:** Scoring a champion takedown reduces the **current** cooldowns of **Master Yi**’s basic abilities by 70%.

**ACTIVE:** **Master Yi** cleanses himself from all slows and cripples. For the next 7 seconds, he gains ghosting, **bonus** attack speed, (ms) **bonus** movement speed, slow immunity, and cripple immunity.

| Attribute | Value |
|-----------|------:|
| **Bonus Attack Speed** | 25 / 35 / 45 / 55 / 65% |

| Attribute | Value |
|-----------|------:|
| **Bonus Movement Speed** | 35 / 40 / 45 / 50 / 55% |

While active, scoring a champion takedown extends *Highlander*’s duration by 7 seconds.

**Notes:**

- *Highlander* can be cast during Alpha Strike.
- There are two situations that can happen if **Master Yi** uses *Highlander* while *Highlander*’s buff is active.
  - If the buff's duration is under 7 seconds, the buff refreshes to 7 seconds.
  - If the buff's duration is above 7 seconds, nothing happens.

---

## Patch History

### V25.17
- Stats
  - Base health reduced to 655 from 669.
- Alpha Strike
  - Primary on-hit damage modifier reduced to 65% from 75%.
    - Lesser on-hit damage modifier reduced to 16.25% from 18.75%.

### V25.16
- Alpha Strike
  - **Removed:*** No longer is special-cased to not generate stacks of Kraken Slayer Bring It Down and Terminus Juxtaposition.

### V25.15
- Alpha Strike
  - Base damage reduced to 20 / 40 / 60 / 80 / 100 from 30 / 60 / 90 / 120 / 150.
  - AD ratio increased to 70% AD from 50% AD.
  - Bonus monster damage reduced to 60 / 85 / 110 / 135 / 160 from 65 / 90 / 115 / 140 / 165.
  - **UNDOCUMENTED / BUG FIX:** Tooltip critical damage calculations have been corrected and now use the proper bonus critical damage modifier instead of using the value as a multiplier for the total critical damage.

### V25.06
- Stats
  - Armor growth reduced to $4.2$ from $4.7$.
- Alpha Strike
  - Bonus critical damage modifier increased to 100% (default) from 75%.
    - Primary critical damage increased to 175% (+ Infinity Edge) from 156.25% (+ Infinity Edge).
    - Lesser critical damage increased to 43.75% (+ Infinity Edge) from 39.0625% (+ Infinity Edge).

### V25.S1.2
- Stats
  - Attack damage growth increased to $2.8$ from $2.5$.
- Wuju Style
  - Bonus AD ratio increased to 35% **bonus** AD from 30%.

### V25.S1.1
- Highlander
  - **Bug Fixes:** Slow immunity no longer ignores Gravity Field.

### V14.24
- Alpha Strike
  - Bonus critical damage modifier increased to 75% from 35%.
    - Primary critical damage increased to 156.25% (+ Infinity Edge) from 126.25% (+ Infinity Edge).

### V14.23
- Highlander
  - Attack speed increased to 25 / 45 / 65% from 25 / 35 / 45%.

### V14.12
- Alpha Strike
  - Bonus monster damage reduced to 65 / 90 / 115 / 140 / 165 from 75 / 100 / 125 / 150 / 175.
- Meditate
  - Cooldown increased to 10 seconds from 9.
  - Initial damage reduction reduced to 70% from 90%.

### V14.11
- Stats
  - Health growth increased to 105 from 100.
  - Attack speed growth increased to $2.5$% from 2%.
  - Attack damage growth increased to $2.5$ from $2.2$.
  - Armor growth increased to $4.7$ from $4.2$.

## Trivia

- Master Yi's name is a play on the word "mastery".
- Master Yi's goggles and his sword can both be seen in the game's Mac Version trailer.
- Master Yi is the champion with the highest base movement speed in-game (355).
- Master Yi's Recall animation references victory pose in ** videogame.
  - A side-by-side comparison can be seen here.
- Master Yi's dance references the "Running Man" dance.
  - A side-by-side comparison can be seen here, which references the version done by the character from the *Dexter's Laboratory* cartoons, specifically from the episode .
  - The more total movement speed he has, the faster he dances.
    - Inversely, the less total movement speed he has, the slower he dances.
  - His pre-rework dance references Napoleon Dynamite, via one of its portions.
    - He shared this dance with Katarina, whose dance references the other portions.
- Master Yi's title in the Chinese localization also dubs him as a "Sword Sage", a title given to most master swordmen in history such as Miyamoto Musashi.
  - In the Chinese localization, it is instead named as "Wuji" (無極, lit. Extremeless/Apolar/Infinite). Wuji is also represented by an empty circle atop the Taijitu (太極圖 lit. Great Poles' Chart), which includes the well-known Yin and Yang. His Wuju Style ability in Chinese is also dubbed as "Wuji Jiandao" (無極劍道, lit. Extremeless Sword Path), "jiandao" is also the pinyin reading of Japanese kendou.
- Highlander was named after as well as references Highlander (franchise), with its cooldown reset mechanic possibly referencing the "quickening" effect.
- A functional real-life replica of Master Yi's Highlander' Ring Sword was crafted in an episode of YouTube series Man At Arms: Reforged.
  - This video can be viewed here.
    - There are also videos where the following are crafted:
      - Moonfall (functional)
      - Preparation (functional)
      - Zenith Blade (functional)
      - Hammer Shock (functional)
      - Steel Tempest (functional)
      - Short Fuse (prop)
- Master Yi's Series 1 Eternals make the following references:
  - *Zen Health* alludes to the Buddhism Meditation of the Zen that has a health benefit to the person.

---
*This page was automatically generated from League of Legends Wiki data.*