# Katarina

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
| **Champion** | Katarina |
| **Title** | the Sinister Blade |
| **Resource** | None |
| **Range Type** | Melee |
| **Release Date** | 2009-09-19 |
| **Release Patch** | V0.9.25.21 |
| **Latest Changes** | V25.13 |
| **Roles** | Assassin |
| **Riot Positions** | Middle |
| **External Positions** | Middle |
| **Blue Essence** | 1575 |
| **Riot Points** | 790 |
| **Difficulty** | 2 |
| **Hero Type** | Assassin |
| **Alt Type** | Mage |
| **Adaptive Type** | Magic |
| **Damage** | 3 |
| **Toughness** | 1 |
| **Control** | 1 |
| **Mobility** | 3 |
| **Utility** | 1 |
| **Style** | 100 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $672.0$ | $+108.0$ |
| **Mana** | $0.0$ | $+0.0$ |
| **Health Regen** | $7.5$ | $+0.7$ |
| **Armor** | $28.0$ | $+4.7$ |
| **Magic Resist** | $32.0$ | $+2.05$ |
| **Attack Damage** | $58.0$ | $+3.2$ |
| **Attack Speed** | $0.658$ | |
| **Movement Speed** | $335.0$ | $+0.0$ |
| **Attack Range** | $125.0$ | $+0.0$ |
| **Base Attack Speed** | $0.658$ | |
| **Attack Speed Ratio** | $0.658$ | |
| **Bonus AS per Level** | $2.7\%$ | |
| **Acquisition Radius** | $400$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $120$ units | |
| **Selection Height** | $140$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $100.0\%$ |
| **Damage Taken** | $100.0\%$ |

#### URF

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $110.0\%$ |
| **Damage Taken** | $95.0\%$ |

## Abilities

### Passive: Voracity

| Attribute | Value |
|-----------|------:|
| **Effect Radius** | cr 340 (Spin damage radius, around Katarina) |
| **Targeting** | Passive |
| **Affects** | Enemies, Self |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |
| **Parry** | Special |

**INNATE:** Scoring a champion takedown within 3 seconds of damaging them will reduce the **current** cooldowns of **Katarina**’s abilities by 15 seconds.

**INNATE - SINISTER STEEL:** Whenever **Katarina** retrieves a Dagger, she slashes around herself to deal 68 / 72 / 77 / 82 / 89 / 96 / 103 / 112 / 121 / 131 / 142 / 154 / 166 / 180 / 194 / 208 / 224 / 240 (+ 60% **bonus** AD) (+ 70%–100%@1–16 AP) magic damage to nearby enemies, apply on-hit effects to enemy champions hit, and reduce *Shunpo*’s **current** cooldown by 78%–96%@1–16 of its **total** cooldown.

Daggers grant sight of their radius and will disappear after being on the ground for 4 seconds.

**Notes:**

- *Voracity*’s cooldown reduction will still take effect even after **Katarina** dies.
- The Dagger's area of effect indicator is displayed even before it lands, although **Katarina** may not slash until it has. This can be targeted by Shunpo.
- Daggers will still deal damage but not apply on-hit effects if dodged. They cannot be blocked nor can they be missed while **Katarina** is blinded.
- Single-use on-hit effects, such as Spellblade, will apply to the closest target to **Katarina** when she uses a Dagger.

---

### Q: Bouncing Blade

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 625 units |
| **Effect Radius** | 450 (Bounce range) units |
| **Speed** | 1600 (Initial missile speed) / $0.15$ seconds (Fixed travel time between bounce targets) units/second |
| **Cooldown** | 11 / 10 / 9 / 8 / 7 seconds |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | True |
| **Call For Help** | True |

**ACTIVE:** **Katarina** throws a Dagger at the target enemy that deals magic damage and can bounce to up to two additional visible nearby enemies.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 80 / 115 / 150 / 185 / 220 (+ 40% AP) |

The Dagger lands after striking the first target, about 350 units (slightly random location) opposite from their position when it struck them.

**Notes:**

- *Bouncing Blade* will not be destroyed in-flight if the target dies before reaching them but it will on clones that expire.
- If the Dagger's intended landing position is inside terrain it will instead land at the closest spot that isn't inside terrain.
- Valid targets are determined after each bounce.

---

### W: Preparation

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Cooldown** | 15 / 14 / 13 / 12 / 11 seconds |
| **Targeting** | Auto |
| **Affects** | Self |

**ACTIVE:** **Katarina** tosses a Dagger into the above her current location and gains (ms) **bonus** movement speed that decays until it lands after $1.25$ seconds, becoming ghosted for the same duration.

| Attribute | Value |
|-----------|------:|
| **Bonus Movement speed** | 50 / 60 / 70 / 80 / 90% |

**Notes:**

- No additional details.

---

### E: Shunpo

| Attribute | Value |
|-----------|------:|
| **Cast Time** | None |
| **Target Range** | 725 (Normal cast) / 775 (Cast on daggers) units |
| **Cooldown** | 12 / 11 / 10 / 9 / 8 seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Single target |
| **Out of Range Behavior** | walk to location |
| **Parry** | Special |
| **Call For Help** | True |
| **Grounded** | True |

**ACTIVE:** **Katarina** blinks to a target location near the unit or Dagger closest to the cursor, dealing magic damage to the nearest enemy in range and applying on-hit effects.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 20 / 30 / 40 / 50 / 60 (+ 40% AD) (+ 25% AP) |

**Katarina** prioritizes blinking to the location near Daggers, then enemy champions, then enemy non-champions, and then any other unit.

*Shunpo resets **Katarina**’s basic attack timer. Upon blinking, **Katarina** enters a $0.15$-second cast time.*

**Notes:**

- *Shunpo* does not reset **Katarina**’s basic attack timer if it killed the target.
- *Shunpo* can be used on any unit except structures and wards.
- *Shunpo* can be cast on Daggers that haven't landed yet.
- *Shunpo* prioritizes damaging enemy champions within range at the targeted location.
- After the cast time, **Katarina** will attempt to move towards the closest enemy in acquisition range and basic attack them, prioritizing champions, unless an attack command was already active on another target in range or any attack command was applied during the cast time. Movement commands applied during cast time will override the automatic attack, but they may not override an already active manually applied attack command. If no enemy is in range, pre-cast commands will remain active, and attack commands applied during the cast time will be ignored .
  - If a Dagger is picked during the cast time, pre-cast commands will be cancelled, even if no enemy is nearby. And if an enemy is nearby, **Katarina** may or may not start an automatic attack .
- If the target is very close to a wall and **Katarina** targets *Shunpo* behind the target, she can blink over the wall.
- *Shunpo* will still deal damage but not apply on-hit effects if dodged. It cannot be blocked nor can it be missed while **Katarina** is blinded.
- *Shunpo* does not require the enemy to be within targeting range to damage them. It only requires them to be within the max range around a unit that *Shunpo* allows them to blink to.

---

### R: Death Lotus

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 550 units |
| **Speed** | 2400 (Dagger missiles travel speed) units/second |
| **Cooldown** | 75 / 67.5 / 60 / 52.5 / 45 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Proximity |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | Special |
| **Spell Effects** | aoe |
| **Projectile** | True |
| **Parry** | False |
| **Silence** | True |

**ACTIVE:** **Katarina** channels for up to $2.5$ seconds, rapidly throwing a dagger every $0.166$ seconds to up to 3 of the closest nearby enemy champions, revealing herself in the process.

Each dagger deals physical damage and magic damage, applies on-hit effects, with on-hit damage reduced in effectiveness, triggers on-attack effects, and inflicts Grievous Wounds on the target for 3 seconds.

| Attribute | Value |
|-----------|------:|
| **Physical Damage Per Dagger** | 16% (+ 50% per 100% **bonus** attack speed) **bonus** AD |
| **Maximum Physical Damage** | 240% (+ 750% per 100% **bonus** attack speed) **bonus** AD |

| Attribute | Value |
|-----------|------:|
| **Magic Damage Per Dagger** | 25 / 31.25 / 37.5 / 43.75 / 50 (+ 19% AP) |
| **Maximum Magic Damage** | 375 / 468.75 / 562.5 / 656.25 / 750 (+ 285% AP) |

| Attribute | Value |
|-----------|------:|
| **On-Hit Damage Effectiveness** | 25 / 27.5 / 30 / 32.5 / 35% |

*An enemy champion within cr 500 units is required to cast this ability. The target does not have to be visible to be hit by this ability.*

**Notes:**

- An alternate form of writing the formula for the **bonus AD** ratio (and which the game data is using internally) is (16%×(1 + $312.5$% per 100% **bonus** attack speed) **bonus** AD).
  - At level 6, **Katarina** has a minimum of $10.82$% **bonus** attack speed.
    - The minimum physical damage per dagger is 21.41% **bonus** AD.
- In *addition* to already revealing herself, **Katarina** will reveal her location (400 radius) for $4.5$ seconds every time she throws a dagger at an enemy while the enemy team does not otherwise have vision of her.
  - Because of this, the vision on **Katarina**’s location will persist for up-to $4.5$ seconds after the channel ends.
- *Death Lotus* will not end if no enemies remain in range.
- Spell shield will block and be consumed by only one dagger.
- Each dagger counts as a separate hit for effects such as Conqueror, Electrocute, and Eclipse Ever Rising Moon.
- The following table refers for interactions while **Katarina** is channeling:
  - Trying to cast a disabled active (excluding Hextech Rocketbelt) will buffer it to cast at the completion of the channel.

| Aspect | State / Notes |
|--------|---------------|
| **Type** | Charge channel |
| **Attacking** | Interrupts |
| **Movement** | Interrupts |
| **Abilities** | Interrupts |
| **Items** | Usable: Shurelya's Battlesong, Youmuu's Ghostblade, Randuin's Omen; Disabled: Zhonya's Hourglass, Hextech Rocketbelt, Stridebreaker; Other items: Interrupt |
| **Summoner Spells** | Usable: Barrier, Clarity, Cleanse, Exhaust, Ghost, Heal, Ignite, Smite; Disabled: Recall, Hexflash; Interrupted by: Flash, Teleport |

---

## Patch History

### V25.13
- Death Lotus
  - **Bug Fixes:** Channel no longer cancels upon using *Shunpo* on no target and outside of *Shunpo*’s range.

### V25.S1.3
- Death Lotus
  - **Bug Fixes:** If used/buffered during Bouncing Blade’s cast or travel time or Shunpo’s cast time, Axiom Arcanist no longer fails to amplify Death Lotus.

### V14.22
- Shunpo
  - Cooldown reduced to 12 / 11 / 10 / 9 / 8 seconds from 14 / 12.5 / 11 / 9.5 / 8.
- Death Lotus
  - Cooldown reduced to 75 / 60 / 45 seconds from 90 / 60 / 45.

### V14.17
- Bouncing Blade
  - Base damage reduced to 80 / 115 / 150 / 185 / 220 from 80 / 120 / 160 / 200 / 240
  - AP ratio reduced to 40% AP from 45% AP.

### V14.16
- Bouncing Blade
  - Base damage increased to 80 / 120 / 160 / 200 / 240 from 80 / 110 / 140 / 170 / 200.
  - AP ratio increased to 45% AP from 35% AP.
- Shunpo
  - Base damage reduced to 20 / 30 / 40 / 50 / 60 from 20 / 35 / 50 / 65 / 80.
- Death Lotus
  - On-hit modifier reduced to 25 / 30 / 35% from 30 / 35 / 40%.

### V14.10
- Death Lotus
  - **Bug Fixes:** No longer goes on cooldown without effect after being used, performing the intended channel and animation and throwing missiles but dealing no damage, if used after becoming affected by an immobilizing debuff without inputting a movement command first or within the first frame of the debuff expiring.

### V13.15
- Shunpo
  - **Bug Fixes:** Casting the ability now properly breaks stealth.

### V13.7
- Voracity
  - AP ratio increased to 70%–100%@1–16 AP from 65%–95%@1–16 AP.
- Death Lotus
  - Physical damage changed to 16% (+ 50% per 100% **bonus** attack speed) **bonus** AD from 18% (+ 30% per 100% **bonus** attack speed) **bonus** AD.
    - *Note:* The reduction of the base ratio evens out at 10% **bonus** attack speed, and **Katarina** has $10.82$% **bonus** attack speed at level 6 from level growth alone, hence this is always an increase.
    - The formula which the game data is using internally is changed to (16%×(1 + $312.5$% per 100% **bonus** attack speed) **bonus** AD) from (18%×(1 + $166.67$% per 100% **bonus** attack speed) **bonus** AD).

### V12.14
- General
  - **Bug Fixes:** Knocked up/stunned animation is now correctly played.

### V12.12#June_30th_Hotfix|V12.12
- Voracity
  - AP ratio increased to 65%–95%@1–16 AP from 55%–88%@1–16 AP.
- Bouncing Blade
  - Base damage increased to 80 / 110 / 140 / 170 / 200 from 75 / 105 / 135 / 165 / 195.
  - AP ratio increased to 35% AP from 30% AP.
- Death Lotus
  - On-hit modifier increased to 30 / 35 / 40% from 28 / 33 / 38%.
  - Physical damage per dagger increased to (18% × (1 + $166.67$% per 100% **bonus** attack speed) **bonus** AD) from (16% × (1 + $142.85$% per 100% **bonus** attack speed) **bonus** AD).
    - Physical damage per dagger formula rewritten in dependency on **bonus** attack speed:
      - **New:*** 18% (+ 30% per 100% **bonus** attack speed) **bonus** AD
      - **OLD:** 16% (+ $22.856$% per 100% **bonus** attack speed) **bonus** AD.
    - Maximum total damage over $2.5$ seconds, for comparison:
      - **New:*** 375 / 562.5 / 750 (+ 285% AP) magic damage plus 270% (+ $450.009$% per 100% **bonus** attack speed) **bonus** AD physical damage plus 450 / 525 / 600% On-hit damage
      - **OLD:** 375 / 562.5 / 750 (+ 285% AP) magic damage plus 240% (+ $342.84$% per 100% **bonus** attack speed) **bonus** AD physical damage plus 420 / 495 / 570% On-hit damage

## Trivia

- Katarina's name comes from Greek Αἰκατερίνα/η (Ekaterine).
  - Du Couteau (her family name) is French for "of the knife".
  - She shared her namesake with Caitlyn and Kai'Sa.
  - She may have been inspired by Katherina from The Taming of the Shrew by William Shakespeare.
- Shunpo (pronounced: [ɕʉ͍̃mpo] shoom-poh) comes from Sino-Japanese 瞬歩 "blink step".
- A framed portrait of Katarina's Katarina can be seen in the game's Mac Version trailer.
- Katarina - Cassiopeia is one of seven pairs of sibling champions (the others being Kayle - Morgana, Garen - Lux, Nasus - Renekton, Yasuo - Yone, Darius - Draven, and Vi - Jinx).
  - Though not a pair, Anivia, Ornn, and Volibear are also siblings.
- Katarina's dance references Napoleon Dynamite, via one of its portions.
  - She used to share this dance with Master Yi up until his rework, which referenced the other portions.
  - A side-by-side comparison can be seen here.
- Voracity’s old icon was directly edited from her second Katarina.
- Katarina is one of few champions to feature two 'Champion Spotlights' due to significant gameplay changes (the others being Ezreal, Karma, Lee Sin, and Sivir).
- Katarina is one of the first champions to have one skin for each of the three big annual events of the year: Lunar Revel, Harrowing, Snowdown Showdown. She received her Katarina skin alongside Nidalee’s in Warring Kingdoms 2015.md).
  - The other one is Jinx when she received Jinx in 2017.
  - Annie almost became another champion that fulfilled the criteria after Annie was released in 2013. However, Annie was not released during the Snowdown Showdown event.
- A functional real-life replica of Katarina's Daggers was crafted in an episode of YouTube series Man At Arms: Reforged.
  - This video can be viewed here.
    - There are also videos where the following are crafted:
      - Diana’s Crescent Moonblade (functional)
      - Leona’s Zenith Blade (functional)
      - Master Yi’s Highlander' Ring Sword (functional)
      - Poppy’s Hammer of Orlon (functional)
      - Yasuo’s Last Breath' Sword (functional)
      - Ziggs’ Hexplosive Bomb (prop)
- Katarina's Series 2 Eternals make the following references:
  - *9 Lives* is a reference to the myth that cats have 9 lives.

---
*This page was automatically generated from League of Legends Wiki data.*