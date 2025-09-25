# Twisted_Fate

## Table of Contents
- [Basic Information](#basic-information)
- [Abilities](#abilities)
- [Patch History](#patch-history)
- [Trivia](#trivia)

## Basic Information

| Attribute | Value |
|-----------|------:|
| **Champion** | Twisted Fate |

## Abilities

### Passive: Loaded Dice

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Self |

**INNATE:** Whenever **Twisted Fate** kills an enemy, he generates icononly=true gold1 / 2 / 3 / 4 / 5 / 6. He also has a chance equal to his critical strike chance to generate an additional 1 / 2 / 3 / 4 / 5 / 6 gold.

*As you'd expect, **Twisted Fate** plays with loaded dice. His initial dice roll is weighted to have an increased chance of granting 6 gold, while his critical dice roll is weighted against him and has an increased chance of granting 1 gold.*

**Notes:**

- The expected bonus gold gained per unit killed is 4 gold.
  - The expected additional gold due to critical strike chance is 3 gold.

---

### Q: Wild Cards

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 1450 units |
| **Angle** | 56°, 0°, and 28° from the cast direction |
| **Width** | 80 units |
| **Speed** | 1000 units/second |
| **Cost** | 60 / 70 / 80 / 90 / 100 Mana |
| **Cooldown** | 6 / 5.75 / 5.5 / 5.25 / 5 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |
| **Projectile** | True |

**ACTIVE:** **Twisted Fate** throws a fan of three cards in a cone in the target direction that each deal magic damage to enemies hit.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 60 / 105 / 150 / 195 / 240 (+ 50% **bonus** AD) (+ 85% AP) |

*Enemies can be damaged only once per pass.*

**Notes:**

Effect at cast time end

---

### W: Pick a Card

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 325 (Red card explosion) units |
| **Cost** | 50 / 55 / 60 / 65 / 70 Mana |
| **Cooldown** | 6 seconds |
| **Cooldown Start** | post-effect |
| **Targeting** | Auto |
| **Affects** | Enemies, Self |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spell |
| **Projectile** | True |
| **Parry** | True |

**ACTIVE:** **Twisted Fate** cycles through three cards for 6 seconds, hovering each for $0.5$ seconds at a time. *Pick a Card* can be recast within the duration, which selects the current card hovered.

**RECAST:** **Twisted Fate** empowers his next basic attack within 6 seconds to have a $0.25$-second cast time, deal ***modified** magic damage, and apply an additional effect based on the card selected. This attack cannot critically strike but its damage is increased based on critical strike chance.

**BLUE CARD BONUS:** Deals magic damage, increased by 0%–57.5%@0–100 (@=critical strike chance), and restores mana.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 40 / 60 / 80 / 100 / 120 (+ 100% AD) (+ 100% AP) |

| Attribute | Value |
|-----------|------:|
| **Mana Restored** | 70 / 90 / 110 / 130 / 150 |

**RED CARD BONUS:** Deals magic damage to the target and surrounding enemies, increased by 0%–35%@0–100 (@=critical strike chance). All targets hit are slowed for $2.5$ seconds.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 30 / 45 / 60 / 75 / 90 (+ 100% AD) (+ 70% AP) |

| Attribute | Value |
|-----------|------:|
| **Slow** | 30 / 35 / 40 / 45 / 50% |

**GOLD CARD BONUS:** Deals magic damage, increased by 0%–25%@0–100 (@=critical strike chance), and stuns the target for a duration.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 15 / 22.5 / 30 / 37.5 / 45 (+ 100% AD) (+ 50% AP) |

| Attribute | Value |
|-----------|------:|
| **Stun Duration** | 1 / 1.25 / 1.5 / 1.75 / 2 seconds |

*Pick a Card's recast resets **Twisted Fate**’s basic attack timer. The recast can be used while affected by cast-inhibiting crowd control.*

**Notes:**

- The initial cast counts as an ability activation for the purposes of on-cast effects such as Spellblade and triggering Force Pulse’s passive.
  - Recasting the ability does not.
- The first card shown is random but the order always remains the same (Blue Card, Red Card, Gold Card, repeat).
  - Upon activation, an indicator visible to allies and enemies appears above **Twisted Fate** to show him shuffling his deck of cards while remaining in the same order, the card he can choose at every moment glows over the others. The selected card is specified for the duration of the enhanced attack.
- The interval until the next card is hovered is every time, instead of some times $0.528$ and other times $0.495$.
- If the target becomes untargetable, dies, or is too far away during the empowered attack's cast time, it is cancelled but not consumed.
- The empowered attack will trigger against structures and buildings but not wards and plants.

---

### E: Stacked Deck

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Enemies, Self |
| **Damage Type** | Magic |
| **Spell Shield** | False |
| **Spell Effects** | proc |
| **Projectile** | True |
| **Parry** | Special |

**PASSIVE:** **Twisted Fate** gains (as) **bonus** attack speed. His basic attacks on-hit generate a stack of *Stacked Deck*, stacking up to 3 times. At 3 stacks, his next basic attack is empowered to consume them all to deal **bonus** magic damage, reduced to 50% against structures.

**Twisted Fate** gains maximum stacks of *Stacked Deck* upon respawning.

| Attribute | Value |
|-----------|------:|
| **Bonus Attack Speed** | 15 / 25 / 35 / 45 / 55% |

| Attribute | Value |
|-----------|------:|
| **Bonus Magic Damage** | 65 / 90 / 115 / 140 / 165 (+ 20% **bonus** AD) (+ 40% AP) |

**Notes:**

- *Stacked Deck* can be dodged and/or missed if **Twisted Fate** is blinded (the on-hit effect is not consumed in either case) but cannot be blocked (the enhanced attack is consumed and the bonus damage is still applied).
- As of patch V14.2, **Twisted Fate** is intended to gain maximum stacks of *Stacked Deck* upon learning the ability, but currently does not.
- The empowered attack will trigger against structures but not wards.

---

### R: Destiny

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 5500 units |
| **Effect Radius** | Global |
| **Cost** | 100 Mana |
| **Cooldown** | 170 / 155 / 140 / 125 / 110 (Starts after recast times out or is used) seconds |
| **Cooldown Start** | post-effect |
| **Targeting** | Auto / Location |
| **Affects** | Enemies, Allies, Self |
| **Spell Shield** | True |
| **Grounded** | Special |
| **Silence** | True |

**ACTIVE:** **Twisted Fate** marks all targetable enemy champions, revealing them for a duration. *Destiny* can be recast after $0.5$ seconds within the duration.

| Attribute | Value |
|-----------|------:|
| **Effect Duration** | 6 / 7 / 8 / 9 / 10 seconds |

**RECAST - GATE:** **Twisted Fate** channels for $1.5$ seconds before blinking to the target location, destroying all projectiles targeting him in the process.

**Notes:**

*Both casts count as ability activations for the purposes of on-cast effects such as Spellblade and triggering Force Pulse’s passive.
- **Twisted Fate** cannot recast *Destiny* while grounded or rooted.
- The nearsight from Paranoia will overpower the reveal from *Destiny* regardless of which one was cast first (enemy champions can still be revealed by *Destiny* after *Paranoia* ends if durations permit).
- Using the recast will inform allies with a ping.
  - Both allies and enemies can see an indicator at the targeted location of where **Twisted Fate** will appear.
- The following table refers for interactions while **Twisted Fate** is channeling:
  - Item actives with cast times as well as Titanic Hydra are disabled during the channel.
    - Trying to cast a disabled item active will buffer it to cast at the completion of the channel.

#### Channel Behavior (channel)

| Aspect | State / Notes |
|--------|---------------|
| **Attacking** | Disabled |
| **Movement** | Disabled |
| **Abilities** | Pick a Card is usable. Wild Cards is disabled. |
| **Items** | Allowed / Interrupts |
| **Summoner Spells** | Disabled |
| **Interrupted by** | death, ground, root, silence |

---

## Patch History

### V25.14
- Wild Cards
  - Base damage reduced to 60 / 105 / 150 / 195 / 240 from 70 / 115 / 160 / 205 / 250.

### V25.13
- Destiny
  - Cooldown increased to 170 / 140 / 110 seconds from 160 / 130 / 100.

### V25.12
- Stacked Deck
  - Bonus attack speed increased to 15 / 25 / 35 / 45 / 55% from 10 / 20 / 30 / 40 / 50%.
- Destiny
  - Cooldown reduced to 160 / 130 / 100 seconds from 180 / 150 / 120.

### V25.11
- Wild Cards
  - Base damage increased to 70 / 115 / 160 / 205 / 250 from 60 / 105 / 150 / 195 / 240.

### V25.06
- Stats
  - Base armor increased to 24 from 21.

### V14.21
- General
  - Recommended runes updated.
    - Electrocute to Arcane Comet.

### V14.17
- Wild Cards
  - Base damage increased to 60 / 105 / 150 / 195 / 240 from 60 / 100 / 140 / 180 / 220.

### V14.12
- Stats
  - Attack damage growth reduced to $2.5$ from $3.3$.
- Wild Cards
  - AP ratio reduced to 85% AP from 90% AP.
- Pick a Card
  - Mana cost increased to 50 / 55 / 60 / 65 / 70 from 30 / 40 / 50 / 60 / 70.
  - Blue card mana restore increased to 70 / 90 / 110 / 130 / 150 from 50 / 75 / 100 / 125 / 150.

### V14.6
- Pick a Card
  - **New Effect:** Can now be recast during cast times (Recall or Hex-gate).

### V14.5
- Stacked Deck
  - Bonus attack speed reduced to 10 / 20 / 30 / 40 / 50% from 10 / 22.5 / 35 / 47.5 / 60%.
  - Bonus AD ratio reduced to 20% **bonus** AD from 75%.

## Trivia

- Twisted Fate's name is a play on the phrase twist of fate, befitting his theme of chances and probability.
  - The cinematic he stars in, A Twist of Fate, is directly named after this phrase.
- Many of Twisted Fate's abilities were named after real-life gambling or card-related terms.
  - Loaded Dice are Dice#Loaded_dice that have a higher probability of having a certain face up.
  - Wild Cards references a wild card, which is a card that can represent any value in various card games.
  - Pick a Card references the commonly used phrase that magicians say to their participants for card tricks.
  - Stacked Deck references card decks that are unfairly stacked in order to give someone an advantage.
  - Gate references the Gate (solitaire).
- Twisted Fate was one of the first six champions designed (the others being Annie, Lee Sin Singed, Sion, and Sivir).
- Twisted Fate's dance references Gangnam Style by Psy.
  - A side-by-side comparison can be seen here.
- Twisted Fate is the third champion to feature vocals while dancing (the others being Draven and Rengar).
- Twisted Fate's eyes glow cyan in-game (only Cutpurse has his pupils visible) but they do not in A Twist of Fate.
- The icon for Loaded Dice is reused for the Teamfight Tactics item Loaded Dice.
- The Blue Card is also used to depict Gate.

---
*This page was automatically generated from League of Legends Wiki data.*