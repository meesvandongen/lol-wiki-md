# Taric

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
| **Champion** | Taric |
| **Title** | the Shield of Valoran |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2009-08-19 |
| **Release Patch** | V0.9.22.15 |
| **Latest Changes** | V25.15 |
| **Roles** | Enchanter, Warden |
| **Riot Positions** | Support |
| **External Positions** | Middle, Support |
| **Blue Essence** | 675 |
| **Riot Points** | 585 |
| **Difficulty** | 2 |
| **Hero Type** | Support |
| **Alt Type** | Tank |
| **Adaptive Type** | Physical |
| **Damage** | 1 |
| **Toughness** | 2 |
| **Control** | 2 |
| **Mobility** | 1 |
| **Utility** | 3 |
| **Style** | 75 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $645.0$ | $+99.0$ |
| **Mana** | $300.0$ | $+60.0$ |
| **Health Regen** | $6.0$ | $+0.5$ |
| **Mana Regen** | $8.5$ | $+0.8$ |
| **Armor** | $40.0$ | $+4.3$ |
| **Magic Resist** | $28.0$ | $+2.05$ |
| **Attack Damage** | $55.0$ | $+3.5$ |
| **Attack Speed** | $0.625$ | |
| **Movement Speed** | $340.0$ | $+0.0$ |
| **Attack Range** | $150.0$ | $+0.0$ |
| **Base Attack Speed** | $0.625$ | |
| **Attack Speed Ratio** | $0.625$ | |
| **Bonus AS per Level** | $2.0\%$ | |
| **Windup Modifier** | $0.25$ | |
| **Acquisition Radius** | $350$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $135$ units | |
| **Selection Height** | $165$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Taken** | $110.0\%$ |

#### URF

| Metric | Value |
|--------|------:|
| **Healing** | $85.0\%$ |

## Abilities

### Passive: Bravado

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Self |
| **Damage Type** | Magic |
| **Spell Effects** | proc |
| **Parry** | Special |

**INNATE:** After casting an ability, **Taric** empowers his next two basic attacks within 5 seconds to each gain (as) 100% **total** attack speed, deal 25 to 93 (+ 15% **bonus** armor) **bonus** magic damage, and reduce the **remaining** cooldowns of his basic abilities by 1–2@0–0 (@=ability haste)
 seconds.

**Notes:**

- The first attack refreshes *Bravado*’s duration.
- The bonus damage will not apply against structures.
- *Bravado* can be dodged and blocked but it cannot miss if **Taric** is blinded. The cooldown reduction still applies when blinded, however it does not apply on dodged or blocked *Bravado* attacks

---

### Q: Starlight's Touch

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Effect Radius** | 325 units |
| **Cost** | 60 Mana + all charges |
| **Cooldown** | 3 seconds |
| **Recharge** | 15 seconds |
| **Targeting** | Auto |
| **Affects** | Allies |

**ACTIVE:** **Taric** heals himself and nearby allied champions for 25 (+ 15% AP) (+ 1% of his **maximum** health) per charge of *Starlight's Touch* that he periodically stocks, up to a maximum amount. *Bravado’s* empowered attacks each grant one charge.

| Attribute | Value |
|-----------|------:|
| **Maximum Charges** | 1 / 2 / 3 / 4 / 5 |

*Starlight's Touch* can heal up to a maximum of 125 (+ 75% AP) (+ 5% of **Taric**’s **maximum** health) at 5 charges.

**Notes:**

- The healing will apply heal effects (such as Summon Aery) prioritizing targets in **Taric**’s circle, based on proximity to him, then in the Bastioned ally's circle, based on proximity to that ally (including it).

---

### W: Bastion

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 800 units |
| **Tether Radius** | 1300 units |
| **Cost** | 60 Mana |
| **Cooldown** | 15 seconds |
| **Targeting** | Unit |
| **Affects** | Self, Allies |

**PASSIVE:** **Taric** gains (armor) **bonus** armor and forms a tether between him and the ally bound by *Bastion*. While the tether persists, the ally also gains the **bonus** armor and **Taric**’s abilities are mimicked to them, though the effects do not stack.

| Attribute | Value |
|-----------|------:|
| **Bonus Armor** | 6 / 7 / 8 / 9 / 10% of **Taric**’s armor |

**ACTIVE:** **Taric** grants himself and the target allied champion a shield for $2.5$ seconds, binding them with *Bastion*.

| Attribute | Value |
|-----------|------:|
| **Shield Strength** | 7 / 8 / 9 / 10 / 11% of target's **maximum** health |

**Taric** can also self-cast to shield himself; if he self-casts and is untethered, he will become tethered to the nearest ally. Similarly, if he is tethered to an ally and self-casts, both he and the ally will be shielded. Only one ally can be bound at a time, and selecting a new ally overrides the previous bind.

**Notes:**

- The mimicked abilities will still complete even if **Taric** dies or leaves range.
- Abilities are not mimicked to the bound ally if they are vanished via Alpha Strike, Hallucinate, Rappel, or Void Rush.
  - Mimicked abilities will also cancel if they vanish as above.
- *Bastion* has a forgiveness radius of 175 units.

---

### E: Dazzle

| Attribute | Value |
|-----------|------:|
| **Range** | 575 units |
| **Cast Time** | none |
| **Width** | 140 units |
| **Cost** | 40 Mana |
| **Cooldown** | 16 / 15 / 14 / 13 / 12 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |
| **Projectile** | False |

**ACTIVE:** **Taric** winds up over 1 second, granting ghosting to nearby units, then projects a beam of starlight in the target direction that deals magic damage to enemies hit and stuns them for $1.5$ seconds.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 90 / 130 / 170 / 210 / 250 (+ 50% AP) (+ 50% **bonus** armor) |

***Taric** can move during Dazzle.*

**Notes:**

- *Dazzle* will towards the target direction on-cast when firing from both his and his Bastion's positions.

---

### R: Cosmic Radiance

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Effect Radius** | 400 units |
| **Cost** | 100 Mana |
| **Cooldown** | 180 / 150 / 120 seconds |
| **Targeting** | Auto |
| **Affects** | Allies |

**ACTIVE:** **Taric** calls down a star from above that descends to him over $2.5$ seconds. Afterwards, he and nearby allied champions become invulnerable for $2.5$ seconds.

**Notes:**

- *Cosmic Radiance* has no effect on untargetable allies.
  - It will affect the primary unit (the one the star is falling down on) even if they are untargetable.

---

## Patch History

### V25.15
- Bastion
  - **Bug Fixes:** Tether no longer persists and functions without a range limitation on a linked ally after **Taric** successfully survives and exits Realm of Death.

### V14.21
- Bastion
  - Armor ratio reduced to 6 / 7 / 8 / 9 / 10% of **Taric**’s armor from 9 / 10 / 11 / 12 / 13%.

### V14.20
- Bastion
  - **New Effect:** Cast now has a forgiveness radius of 175 units.

### V14.15
- Starlight's Touch
  - **Bug Fixes:** No longer fails to cast on an ally tethered by Bastion at max tether range.
  - **Bug Fixes:** Using Navori Flickerblade Transcendence no longer incorrectly increases its current cooldown.
- Bastion
  - **Bug Fixes:** No longer fails to cast on an ally tethered by *Bastion* at max tether range.
  - **Bug Fixes:** Self-casting after a previously tethered ally has died no longer seeks for ally corpse targets instead of only alive allies.
  - **Bug Fixes:** Tether link and unlink SFX now has a delay to prevent it from repeatedly playing when the tethered ally also repeatedly enters and exits the tether's max range.

### V14.2
- Dazzle
  - **New Effect:** Now grants ghosting to nearby units during the windup.

### V14.1#January 12th Hotfix|V14.1
- Stats
  - Armor growth reduced to $4.3$ from $4.6$.
- Bastion
  - Armor ratio reduced to 9 / 10 / 11 / 12 / 13% of **Taric**’s armor from 10 / 11 / 12 / 13 / 14%.

### V12.17
- Stats
  - Base magic resistance reduced to 28 from 32.
- Dazzle
  - Cooldown increased to 16 / 15 / 14 / 13 / 12 seconds from 15 / 14 / 13 / 12 / 11.

### V12.11
- Starlight's Touch
  - Heal health ratio per stack increased to 1% of his **maximum** health from $0.75$%.
  - Mana cost reduced to 60 at all ranks from 65 / 70 / 75 / 80 / 85.
- Dazzle
  - Stun duration increased to $1.5$ seconds from $1.25$.
  - Cooldown reduced to 15 / 14 / 13 / 12 / 11 seconds from 17 / 16 / 15 / 14 / 13.

### V12.10
- Stats
  - Base health increased to 645 from 575.
  - Health growth increased to 99 from 85.
  - Armor growth increased to $4.6$ from $3.4$.
  - Magic resistance growth increased to $2.05$ from $1.25$.
- Starlight's Touch
  - Base heal per stack reduced to 25 from 30.
  - Heal AP ratio per stack reduced to 15% AP from 20% AP.
  - Heal health ratio per stack reduced to $0.75$% of his **maximum** health from 1%.
- Bastion
  - Shield health ratio reduced to 7 / 8 / 9 / 10 / 11% **maximum** health from 8 / 9 / 10 / 11 / 12%.

### V11.14
- General
  - **UNDOCUMENTED/BUG FIX:** First move voice line now always triggers.
  - **UNDOCUMENTED/BUG FIX:** Generic item purchase lines have been restored.
  - **UNDOCUMENTED/BUG FIX:** Missing item specific voice interactions have been repurposed.
- Starlight's Touch
  - Mana cost reduced to 65 / 70 / 75 / 80 / 85 from 70 / 75 / 80 / 85 / 90.
- Dazzle
  - Mana cost reduced to 40 from 60.
- Cosmic Radiance
  - **UNDOCUMENTED/BUG FIX:** Voice lines have been made more frequent to trigger.

## Trivia

- *Taric* comes from طارق Arabic *Tariq* "striker" < Semitic root *ṭ-r-q* "to strike".
  - *Ṭariq* later underwent semantic shift from "striker" to "One knocking at the door; wayfarer, traveller by night" and thus "star".
- Singed carries a giant bottle of "Taric’s Tropical Tan" sunscreen.

---
*This page was automatically generated from League of Legends Wiki data.*