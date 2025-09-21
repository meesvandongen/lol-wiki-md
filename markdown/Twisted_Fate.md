# Twisted_Fate

## Table of Contents
- [Basic Information](#basic-information)
- [Abilities](#abilities)
- [Patch History](#patch-history)
- [Trivia](#trivia)

## Basic Information

| Attribute | Value |
|-----------|-------|
| **Champion** | Twisted Fate |

## Abilities

### Passive: Loaded Dice

**Innate:** Whenever **Twisted Fate** kills an enemy, he generates a small amount of **bonus** gold gold based on chance.

**Innate:** Whenever **Twisted Fate** kills an enemy, he generates icononly=true goldtype=probability. He also has a chance equal to his critical strike chance to generate an additional type=probability gold. 'As you'd expect, **Twisted Fate** plays with loaded dice. His initial dice roll is weighted to have an increased chance of granting 6 gold, while his critical dice roll is weighted against him and has an increased chance of granting 1 gold.'

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Self |

**Notes:**

- The expected bonus gold gained per unit killed is 4 gold.
  - The expected additional gold due to critical strike chance is 3 gold.

---

### Q: Wild Cards

**Active:** **Twisted Fate** throws a fan of three cards in the target direction that each deal magic damage to enemies hit.

**Active:** **Twisted Fate** throws a fan of three cards in a cone in the target direction that each deal magic damage to enemies hit. *Enemies can be damaged only once per pass.*

| Attribute | Value |
|-----------|-------|
| **Range** | 1450 units |
| **Cooldown** | $6-5$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $60/70/80/90/100$ Mana |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Speed** | 1000 units/second |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |
| **Projectile** | True |

**Scaling:**
- **Magic Damage:** $60-240$
- *bonus AD) (+ 85% AP)

**Notes:**

No additional notes.

---

### W: Pick a Card

**Active:** **Twisted Fate** cycles through three cards over the next few seconds. *Pick a Card* can be recast within the duration, which selects the current card hovered.

**Recast:** ''Twisted Fate's' next basic attack becomes empowered to deal magic damage and apply an additional effect based on the card selected. * **Blue Card Bonus:** Deals significantly increased magic damage and restores *mana*. * **Red Card Bonus:** Deals increased magic damage to the target and surrounding enemies, slow them for a short time. * **Gold Card Bonus:** Deals magic damage and briefly stun the target.

**Active:** **Twisted Fate** cycles through three cards for 6 seconds, hovering each for $0.5$ seconds at a time. *Pick a Card* can be recast within the duration, which selects the current card hovered. **Recast:** **Twisted Fate** empowers his next basic attack within 6 seconds to have a $0.25$-second cast time, deal *modified magic damage, and apply an additional effect based on the card selected. This attack cannot critically strike but its damage is increased based on critical strike chance. **Blue Card Bonus:** Deals magic damage, increased by key=%, and restores *mana*. **Red Card Bonus:** Deals magic damage to the target and surrounding enemies, increased by key=%. All targets hit are slow for $2.5$ seconds. **Gold Card Bonus:** Deals magic damage, increased by key=%, and stun the target for a duration.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | 6 seconds |
| **Cast Time** | none |
| **Cost** | $50-70$ Mana |
| **Targeting** | Auto |
| **Affects** | Enemies, Self |
| **Damage Type** | Magic |
| **Speed** | Attack missile speed, same as basic attacks |
| **Effect Radius** | 325 units |
| **Spell Shield** | True |
| **Spell Effects** | spell |
| **Projectile** | True |

**Scaling:**
- **Magic Damage:** $40-120$ (+ 100% AD)
- **Mana Restored:** $70-150$

**Notes:**

- The initial cast counts as an ability activation for the purposes of on-cast effects such as Spellblade and triggering *Force Pulse*’s passive.
  - Recasting the ability does not.
- The first card shown is random but the order always remains the same (*Blue Card*, *Red Card*, *Gold Card*, repeat).
  - Upon activation, an indicator visible to allies and enemies appears above **Twisted Fate** to show him shuffling his deck of cards while remaining in the same order, the card he can choose at every moment glows over the others. The selected card is specified for the duration of the enhanced attack.
- The interval until the next card is hovered is every time, instead of some times $0.528$ and other times $0.495$.
- If the target becomes untargetable, death, or is too far away during the empowered attack's cast time, it is cancelled but not consumed.
- The empowered attack will trigger against structures and buildings but not wards and plants.

---

### E: Stacked Deck

**Passive:** **Twisted Fate** gains *as *bonus attack speed*. His basic attacks on-hit generate stacks of *Stacked Deck*, which stacks up to a cap.

*At max stacks, his next basic attack consumes them all to deal **bonus** magic damage.*

**Passive:** **Twisted Fate** gains *as *bonus attack speed*. His basic attacks on-hit generate a stack of *Stacked Deck*, stacking up to 3 times. At 3 stacks, his next basic attack is empowered to consume them all to deal **bonus** magic damage, reduced to 50% against structures. **Twisted Fate** gains maximum stacks of *Stacked Deck* upon death.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Enemies, Self |
| **Damage Type** | Magic |
| **Spell Shield** | False |
| **Spell Effects** | proc |
| **Projectile** | True |

**Scaling:**
- **Bonus Attack Speed:** $15-55$%
- **Bonus Magic Damage:** $65-165$ (+ 20%
- *bonus AD) (+ 40% AP)

**Notes:**

- *Stacked Deck* can be dodged and/or missed if **Twisted Fate** is blind (the on-hit effect is not consumed in either case) but cannot be blocked (the enhanced attack is consumed and the bonus damage is still applied).
- As of patch V14.2, **Twisted Fate** is intended to gain maximum stacks of *Stacked Deck* upon learning the ability, but currently does not.
- The empowered attack will trigger against structures but not wards.

---

### R: Destiny

**Active:** **Twisted Fate** marks all enemy champions, true sight them for a few seconds. *Destiny* can be recast within the duration.

**Recast - Gate:** **Twisted Fate** briefly channels before blink to the target location, homing projectile destruction all incoming projectiles.

**Active:** **Twisted Fate** marks all targetable enemy champions, true sight them for a duration. *Destiny* can be recast after $0.5$ seconds within the duration. **Recast - Gate:** **Twisted Fate** channels for $1.5$ seconds before blink to the target location, homing projectile destruction all projectiles targeting him in the process.

| Attribute | Value |
|-----------|-------|
| **Range** | 5500 units |
| **Cooldown** | $170-110$ seconds |
| **Cast Time** | none |
| **Cost** | 100 Mana |
| **Targeting** | Auto / Location |
| **Affects** | Enemies, Allies, Self |
| **Effect Radius** | Global |
| **Spell Shield** | True |

**Scaling:**
- **Effect Duration:** $6-10$ seconds

**Notes:**

- **Twisted Fate** cannot recast *Destiny* while ground or root.
- The nearsight from *Paranoia* will overpower the true sight from *Destiny* regardless of which one was cast first (enemy champions can still be revealed by *Destiny* after *Paranoia* ends if durations permit).
- Using the recast will inform allies with a ping.
  - Both allies and enemies can see an indicator at the targeted location of where **Twisted Fate** will appear.
- The following table refers for interactions while **Twisted Fate** is channel:
  - Item actives with cast times as well as *Titanic Hydra* are disabled during the channel. *** Trying to cast a disabled item active will buffer it to cast at the completion of the channel.

---

## Patch History

### V25.14
- *Wild Cards*
  - Base damage reduced to $60-240$ from $70-250$.

### V25.13
- *Destiny*
  - Cooldown increased to $170-110 3$ seconds from $160-100 3$.

### V25.12
- *Stacked Deck*
  - Bonus attack speed increased to $15-55$% from $10-50$%.
- *Destiny*
  - Cooldown reduced to $160-100 3$ seconds from $180-120 3$.

### V25.11
- *Wild Cards*
  - Base damage increased to $70-250$ from $60-240$.

### V25.06
- Stats
  - Base armor increased to 24 from 21.

### V14.21
- General
  - Recommended runes updated.
    - *Electrocute* to *Arcane Comet*.

### V14.17
- *Wild Cards*
  - Base damage increased to $60-240$ from $60-220$.

### V14.12
- Stats
  - Attack damage growth reduced to $2.5$ from $3.3$.
- *Wild Cards*
  - AP ratio reduced to 85% AP from 90% AP.
- *Pick a Card*
  - Mana cost increased to $50-70$ from $30-70$.
  - Blue card mana restore increased to $70-150$ from $50-150$.

### V14.6
- *Pick a Card*
  - **New Effect:** Can now be recast during cast times (Recall or Hex-gate).

### V14.5
- *Stacked Deck*
  - Bonus attack speed reduced to $10-50$% from $10-60$%.
  - Bonus AD ratio reduced to 20% *bonus AD from 75%.

## Trivia

- Twisted Fate's name is a play on the phrase twist of fate, befitting his theme of chances and probability.
  - The cinematic he stars in, A Twist of Fate, is directly named after this phrase.
- Many of Twisted Fate's abilities were named after real-life gambling or card-related terms.
  - *Loaded Dice* are Dice#Loaded_dice that have a higher probability of having a certain face up.
  - *Wild Cards* references a wild card, which is a card that can represent any value in various card games.
  - *Pick a Card* references the commonly used phrase that magicians say to their participants for card tricks.
  - *Stacked Deck* references card decks that are unfairly stacked in order to give someone an advantage.
  - *Gate* references the Gate (solitaire).
- Twisted Fate was one of the first six champions designed (the others being **Annie**, **Lee Sin** **Singed**, **Sion**, and **Sivir**).
- Twisted Fate's dance references Gangnam Style by Psy.
  - A side-by-side comparison can be seen here.
- Twisted Fate is the third champion to feature vocals while dancing (the others being **Draven** and **Rengar**).
- Twisted Fate's eyes glow cyan in-game (only Cutpurse has his pupils visible) but they do not in A Twist of Fate.
- The icon for *Loaded Dice* is reused for the Teamfight Tactics item Loaded Dice.
- The *Blue Card* is also used to depict *Gate*.

---
*This page was automatically generated from League of Legends Wiki data.*