# Sona

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
| **Champion** | Sona |
| **Title** | Maven of the Strings |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2010-09-21 |
| **Release Patch** | V1.0.0.101 |
| **Latest Changes** | V25.15 |
| **Roles** | Enchanter |
| **Riot Positions** | Support |
| **External Positions** | Support |
| **Blue Essence** | 225 |
| **Riot Points** | 260 |
| **Difficulty** | 1 |
| **Hero Type** | Support |
| **Alt Type** | Mage |
| **Adaptive Type** | Magic |
| **Damage** | 2 |
| **Toughness** | 1 |
| **Control** | 2 |
| **Mobility** | 1 |
| **Utility** | 2 |
| **Style** | 90 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $550.0$ | $+91.0$ |
| **Mana** | $340.0$ | $+45.0$ |
| **Health Regen** | $5.5$ | $+0.55$ |
| **Mana Regen** | $11.5$ | $+0.4$ |
| **Armor** | $26.0$ | $+4.2$ |
| **Magic Resist** | $30.0$ | $+1.3$ |
| **Attack Damage** | $49.0$ | $+3.0$ |
| **Attack Speed** | $0.644$ | |
| **Movement Speed** | $325.0$ | $+0.0$ |
| **Attack Range** | $550.0$ | $+0.0$ |
| **Base Attack Speed** | $0.644$ | |
| **Attack Speed Ratio** | $0.644$ | |
| **Bonus AS per Level** | $2.3\%$ | |
| **Missile Speed** | $1500$ units/second | |
| **Acquisition Radius** | $800$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $110$ units | |
| **Selection Height** | $145$ units | |
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
| **Damage Dealt** | $90.0\%$ |
| **Damage Taken** | $100.0\%$ |
| **Healing** | $70.0\%$ |

## Abilities

### Passive: Power Chord

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Self |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | proc |
| **Parry** | True |

**INNATE - ACCELERANDO:** **Sona** generates a stack of *Accelerando* each time she hits an enemy champion with *Hymn of Valor* and each time she mitigates sufficient damage or heals damaged allies with *Aria of Perseverance*, stacking up to 120 times. At maximum stacks, she instead reduces the **current** cooldown of *Crescendo* by $1.5$ seconds each time.


 **ACCELERANDO:** For each stack, **Sona** gains $0.5$ basic ability haste, up to 60 at maximum stacks.

**INNATE - MELODY:** Whenever **Sona** casts a basic ability, her other basic abilities incur a (cd) $0.5$-second global cooldown and she generates a unique aura for 3 seconds that empowers herself and nearby allied champions.

**INNATE - POWER CHORD:** **Sona**’s basic abilities generate a stack of *Power Chord*, stacking up to 3 times. At 3 stacks, her next basic attack is empowered to consume them all to have an uncancellable windup, deal 20 to 90 for 8 / 90 to 240 (+ 20% AP) **bonus** magic damage, and apply an additional effect based on the last basic ability she cast:
- *Hymn of Valor* - **STACCATO:** **Bonus** damage is modified to 20×1.5 to 90×1.5 for 8 / 90×1.5 to 240×1.5 (+ 30% AP).
- *Aria of Perseverance* - **DIMINUENDO:** Reduces the target's by 8% and damage dealt by 25% (+ 4% per 100 AP) for 3 seconds.
- *Song of Celerity* - **TEMPO:** Slows the target by 50% (+ 4% per 100 AP) for 2 seconds, capped at 99%.

*Gaining Power Chord's empowered attack resets *’Sona's** basic attack timer.*

**Notes:**

- *Power Chord* stacks are represented by a counter under **Sona**’s health bar, visible to the player only. It will light up when the empowered effect is available.
  - When *Power Chord* is ready, **Sona** gains a around her. This will have a different color depending on the last ability used (*blue, green or purple*).
  - The effect does not change if abilities are used while the projectile is in motion.
- The triggering attack will apply other on-hit effects and can critically strike as normal.
- **Sona** retains *Power Chord*’s stacks when entering resurrection.
- The empowered attack will trigger but not be consumed nor apply its effects against wards and jungle plants.

---

### Q: Hymn of Valor

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 825 (Sound bolts search range) / er 400 (Aura radius, center-to-edge) units |
| **Speed** | 1300 (Sound bolts missile speed) units/second |
| **Cost** | 50 / 55 / 60 / 65 / 70 mana |
| **Cooldown** | 8 seconds |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | aoe |
| **Projectile** | True |

**ACTIVE:** **Sona** sends out bolts of sound to the two nearest visible enemies, prioritizing champions. Each bolt deals magic damage and grants sight of the area around the target for 1 second.

**Sona** gains a stack of *Accelerando* for each bolt that hits an enemy champion.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 50 / 85 / 120 / 155 / 190 (+ 40% AP) |

**MELODY BONUS:** **Sona** and tagged allied champions deal **bonus** magic damage on their next basic attack within 5 seconds.

| Attribute | Value |
|-----------|------:|
| **Bonus Magic Damage** | 10 / 15 / 20 / 25 / 30 (+ 10% of **Sona**’s AP) |

**Notes:**

- Sight of the targets' area is granted on-cast.
- The **MELODY** empowerment given to tagged allies respects enchantment redirection.
- The empowered attack will trigger but not be consumed against wards.

---

### W: Aria of Perseverance

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 1000 (Heal search range) / er 400 (Aura radius, center-to-edge) units |
| **Speed** | 1300 (Heal missile speed) units/second |
| **Cost** | 80 / 85 / 90 / 95 / 100 mana |
| **Cooldown** | 10 seconds |
| **Targeting** | Auto |
| **Affects** | Self, Allies |
| **Projectile** | True |

**ACTIVE:** **Sona** heals herself and sends out a tone to heal the most wounded (Based on greatest % missing health) allied champion nearby.

| Attribute | Value |
|-----------|------:|
| **Heal** | 30 / 45 / 60 / 75 / 90 (+ 30% AP) |

**MELODY BONUS:** **Sona** and tagged allied champions are granted a shield for $1.5$ seconds.

| Attribute | Value |
|-----------|------:|
| **Shield Strength** | 25 / 45 / 65 / 85 / 105 (+ 25% of **Sona**’s AP) |

**Sona** gains a stack of *Accelerando* whenever she heals a wounded ally or shields a minimum amount of damage for an ally with *Aria of Perseverance*.

| Attribute | Value |
|-----------|------:|
| **Minimum Damage Mitigated** | 25 / 45 / 65 / 85 / 105 |

**Notes:**

- If no one is injured, *Aria of Perseverance* will target the closest allied champion.
- The projectile of *Aria of Perseverance* will follow an ally in stealth but will not reveal them.

---

### E: Song of Celerity

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 400 (Aura radius, center-to-edge) units |
| **Cost** | 65 mana |
| **Cooldown** | 14 seconds |
| **Targeting** | Auto |
| **Affects** | Self, Allies |

**ACTIVE:** **Sona** gains (ms) 20% (+ 2% per 100 AP) **bonus** movement speed for 7 seconds. If she takes damage during this time, the duration ends prematurely once or if 3 seconds have elapsed.

**MELODY BONUS:** Tagged allied champions gain **bonus** movement speed for 3 seconds.

| Attribute | Value |
|-----------|------:|
| **Bonus Movement Speed** | 10 / 12 / 14 / 16 / 18% (+ 2% per 100 of **Sona**’s AP) |

**Notes:**

- To achieve a 99% slow with the empowered attack via Power Chord, it would require at least 1475 AP.

---

### R: Crescendo

| Attribute | Value |
|-----------|------:|
| **Range** | 1000 units |
| **Cast Time** | $0.25$ seconds |
| **Width** | 280 units |
| **Speed** | 2400 units/second |
| **Cost** | 100 mana |
| **Cooldown** | 140 / 120 / 100 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | True |

**ACTIVE:** **Sona** strikes an irresistible chord in the target direction that deals magic damage to enemies hit and stuns them for $1.5$ seconds.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 150 / 250 / 350 (+ 50% AP) |

**Notes:**

Effect at cast time start
- While stunned by *Crescendo*, enemies will perform their Dance emote.

---

## Patch History

### V25.15
- Sona
  - **Bug Fixes:** When the background music is enabled, it now properly overrides the Summoner's Rift music instead of the opposite.
  - **Bug Fixes:** Reinstated various missing audio layers from the music tracks.

### V25.08
- Hymn of Valor
  - Bolt base damage increased to 50 / 85 / 120 / 155 / 190 from 50 / 80 / 110 / 140 / 170.
- Power Chord
  - **UNDOCUMENTED / BUG FIX:** Tempo no longer slows cc-immune targets.

### V14.7
- Power Chord
  - Staccato damage modifier increased to 150% from 140%.
    - Staccato base damage increased to 20×1.5 to 90×1.5 for 8 / 90×1.5 to 240×1.5 from 20×1.4 to 90×1.4 for 8 / 90×1.4 to 240×1.4.
    - Stacatto AP ratio increased to 30% AP from 28% AP.
- Hymn of Valor
  - On-hit AP ratio reduced to 10% AP from 20% AP.
- Aria of Perseverance
  - Heal AP ratio increased to 30% AP from 15% AP.
- Song of Celerity
  - Ally bonus movement speed increased to 10 / 12 / 14 / 16 / 18% from 10 / 11 / 12 / 13 / 14%.

### V13.21
- Hymn of Valor
  - Damage now triggers **Sona**’s own damage effects but still grants kill credit to the ally applying it.

### V13.17
- Sona
  - Power Chord
    - **Bug Fixes:** VFX overlay for when her passive was ready no longer behaves unintentionally.
- Sona
  - **Bug Fixes:** HUD now uses the correct splash art for her icon instead of using the base Sona.

### V13.16
- Sona
  - **Bug Fixes:** Death animation no longer causes the feet on her character model to bend backwards.

### V13.14
- Aria of Perseverance
  - **Bug Fixes:** Cast no longer grants a stack of Accelerando while having Moonstone Renewer.

### V13.13
- Aria of Perseverance
  - **Bug Fixes:** Casting the ability no longer causes Moonstone Renewer chain shield to last for 25000 seconds.

### V13.8
- Power Chord
  - **Bug Fixes:** Passive effect is no longer duplicated if specific cast inputs were made at the same time.

### V12.19
- Hymn of Valor
  - Base damage increased to 50 / 80 / 110 / 140 / 170 from 40 / 70 / 100 / 130 / 160.
- Song of Celerity
  - Base slow increased to 50% from 40%.

## Trivia

- A Power Chord consists of a double root and the fifth, often heard in rock music.
- A Hymn (Greek: 'song of praises') is Hymn usually associated to religion.
  - While Hymn of Valor’s aura is active, the game's theme can be heard.
- An Aria is Aria usually associated with operas.
- Crescendo (Italian: 'growing') refers to Crescendo
- Sona was deemed overpowered/obnoxious for Ultra Rapid Fire in April 2014, and was ultimately disabled in non-custom games.
- At one time, Sona held the title of lowest base Armor (9).
- Sona's Deutsch/German title, *Die Virtuosin*, is the feminine derivative of Jhin’s title, *Der Virtuose*.
- Enemies hit by Crescendo play their dance emote while they are stunned.

---
*This page was automatically generated from League of Legends Wiki data.*