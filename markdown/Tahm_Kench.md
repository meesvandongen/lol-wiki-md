# Tahm_Kench

## Table of Contents
- [Basic Information](#basic-information)
- [Abilities](#abilities)
- [Patch History](#patch-history)
- [Trivia](#trivia)

## Basic Information

| Attribute | Value |
|-----------|------:|
| **Champion** | Tahm Kench |

## Abilities

### Passive: An Acquired Taste

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Self |
| **Damage Type** | Magic |
| **Spell Effects** | Proc |
| **Parry** | True |

**INNATE:** **Tahm Kench**’s basic attacks on-hit and *Tongue Lash* are empowered to deal 6 to 42 for 10 / 48 (+ $1.5$% AP per 100 **bonus** health) (+ 4% of his **bonus** health) **bonus** magic damage and apply a stack of *An Acquired Taste* against enemy champions for 5 seconds, refreshing on subsequent hits and stacking up to 3 times. Stacks expire by one every $0.67$ seconds when the duration ends.

*Tongue Lash* and *Devour* gain additional effects against enemies with 3 stacks, consuming them all.

**Notes:**

- The passive bonus damage applies to turrets.

---

### Q: Tongue Lash

| Attribute | Value |
|-----------|------:|
| **Range** | 900 (Increased by modifiers) units |
| **Cast Time** | $0.25$ seconds |
| **Width** | 140 units |
| **Speed** | 2800 (Increased by modifiers) units/second |
| **Cost** | 50 / 46 / 42 / 38 / 34 Mana |
| **Cooldown** | 7 / 6.5 / 6 / 5.5 / 5 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Spell |
| **Projectile** | True |

**ACTIVE:** **Tahm Kench** lashes his tongue in the target direction that deals magic damage to the first enemy hit and slows them by 50% for 2 seconds. If this hits an enemy champion, **Tahm Kench** also heals himself.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 75 / 120 / 165 / 210 / 255 (+ 100% AP) + [ An Acquired Taste 6 to 48 (+ $1.5$% AP per 100 **bonus** health) (+ 4% **bonus** health) ] |

| Attribute | Value |
|-----------|------:|
| **Heal** | 10 / 15 / 20 / 25 / 30 (+ 5 / 5.5 / 6 / 6.5 / 7% of **missing** health) |

** The target is stunned for $1.5$ seconds. *Devour* can be cast during *Tongue Lash* at no cost to pull the target to **Tahm Kench** before swallowing them.

***Tahm Kench** is unable to move, attack, or cast Abyssal Dive or Devour on a different target while his tongue is in flight. Tongue Lash's range and travel speed can be increased by **Tahm Kench**’s size.*

**Notes:**

- **Tongue Lash**’s range and speed scale 1:1 alongside any modifiers to **Tahm Kench**, but will not be reduced *below* their base values if his overall modifier is smaller than 100%.
  - As the missile range and speed scale simultaneously, **Tongue Lash** will always take to reach its maximum range (no matter the cast range).
  - Because of this, increasing **Tahm Kench**’s with items such as Elixir of Iron allows him to hit targets at even the same distance more quickly and giving them less time to dodge.
- Devour can be 'queued' at any point during *Tongue Lash* (including the wind-up animation) and will not go on cooldown if **Tahm Kench** fails to hit a valid target with *Tongue Lash*.
- All three An Acquired Taste stacks will be consumed even if *Tongue Lash*’s stun is negated.
- An Acquired Taste is applied in a separate damage instance from *Tongue Lash*.
  - This causes effects like Bone Plating and Bloodletter's Curse Vile Decay to be applied twice. Effect at cast time start

---

### W: Abyssal Dive

| Attribute | Value |
|-----------|------:|
| **Target Range** | 1000 / 1050 / 1100 / 1150 / 1200 units |
| **Effect Radius** | 275 (Emerging effects radius) / sight 200 (Sight radius on target location during channel) units |
| **Cost** | 60 / 75 / 90 / 105 / 120 Mana |
| **Cooldown** | 21 / 20 / 19 / 18 / 17 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Location |
| **Affects** | Self, Allies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Out of Range Behavior** | cast at max |
| **Grounded** | True |
| **Silence** | True |

**ACTIVE:** **Tahm Kench** channels for $1.35$ seconds as he dives into the waters, then blinks to the target location after a $0.15$-second delay and remains unable to act for $0.65$ seconds after the channel completes. *Abyssal Dive* grants sight of the area during the channel.

**Tahm Kench** emerges to deal magic damage to nearby enemies, as well as knock up and stun them for 1 second. If this hits at least one enemy champion, 40% of *Abyssal Dive*’s mana cost as well as a percentage of its cooldown are refunded.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 100 / 135 / 170 / 205 / 240 (+ 150% AP) |

| Attribute | Value |
|-----------|------:|
| **Cooldown Refund** | 40 / 42.5 / 45 / 47.5 / 50% |

*Enemies can see the indicator for **Tahm Kench**’s destination after he has channeled for $0.75$ seconds.*

**Boy, the world's one river, and I'm its king. Ain't no place I ain't been; ain't no place I can't go again.**

**Notes:**

- **Tahm Kench** is vanished (but does not become untargetable) during the $0.15$ seconds delay.
- The following table refers for interactions while **Tahm Kench** is channeling:
- The following table refers for interactions while **Tahm Kench** is unable to act:

---

### E: Thick Skin

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Cooldown** | 3 (Starts upon shield destruction or expiration) seconds |
| **Cooldown Start** | post-effect |
| **Targeting** | Auto |
| **Affects** | Self |

**PASSIVE:** **Tahm Kench** stores a portion of the post-mitigation damage (Damage calculated after modifiers) he takes as grey health on his health bar, increased while there are at least 2 nearby visible enemy champions either dead or alive. He can store up to 300% of his **maximum** health.

| Attribute | Value |
|-----------|------:|
| **Damage Stored into Grey Health** | 15 / 23 / 31 / 39 / 47% |

| Attribute | Value |
|-----------|------:|
| **Increased Damage Stored into Grey Health** | 42 / 44 / 46 / 48 / 50% |

While *Thick Skin* is not on cooldown, and after 4 seconds without taking damage, **Tahm Kench** rapidly consumes his grey health to restore key=% of the amount, healing for 10% of his **maximum** health every .

**ACTIVE:** **Tahm Kench** converts his current grey health into a shield that lasts for $2.5$ seconds.

**Notes:**

- *Thick Skin* shield value and *grey health* percentage heal are both boosted by effects that increase heal and shield power. This means **Tahm Kench** can technically heal more than the normal amount of *grey health* and get a shield bigger than his maximum health.
  - At level 18 **Tahm Kench** would need 53.8% heal power to heal for 100% of the damage he takes, reduced to 34.9% with Revitalize, 23.1% with Spirit Visage, and 6.9% with both.
- *Grey health* converts all post-mitigation damage, *Thick Skin* effectively grants *Tahm Kench* a form of true damage mitigation.
- *Grey health* can temporarily exceed 100% of his missing health when **Tahm Kench** heals through other means (Any other source of healing besides Thick Skin) with a *grey health* bar at 100%. In that case he will keep the higher *grey health* value for a few moments until it updates and removes the amount that he healed.

---

### R: Devour

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 250 units |
| **Cost** | 100 Mana |
| **Cooldown** | 120 / 110 / 100 / 90 / 80 (Starts post-effect, but time spent active will be reduced by current cooldown afterwards) seconds |
| **Cooldown Start** | Special |
| **Targeting** | Unit |
| **Affects** | Self, Enemies / Allies |
| **Spell Shield** | True |

**ACTIVE:** **Tahm Kench** abducts the target champion, pulling them (See notes) to him over the cast time and then swallowing them to carry them in his belly for up to 3 seconds. While inside, the target is attached to **Tahm Kench** and is rendered untargetable and displacement immune. *Devour* can be cast on enemies only with 3 stacks of *An Acquired Taste*.

If the target is an ally, they become unable to act and are granted a shield, which decays in strength by 50 every $0.25$ seconds after they are *Regurgitated*. **Tahm Kench** will also gain 40% **bonus** movement speed for 3 seconds and is unable to cast movement spells besides *Abyssal Dive* while an ally is inside of his belly.

| Attribute | Value |
|-----------|------:|
| **Shield Strength** | 650 / 725 / 800 / 875 / 950 (+ 100% AP) |

After 1 second, **Tahm Kench** can cast *Regurgitate* while a target is inside of his belly, and automatically does so after the duration. The swallowed ally can also input a movement command (RMB) to force **Tahm Kench** to cast *Regurgitate* in a target direction of their choice, unless they are immobilized.

** The target is suppressed during *Devour*’s cast time and while attached. **Tahm Kench** will also be grounded and slowed by 40% under those same conditions.

*If **Tahm Kench** is performing Abyssal Dive, Regurgitate will not automatically cast until after he finishes.*

**Notes:**

- *Devour*’s initial pull is a non-airborne displacement that displaces the target toward **Tahm Kench** over the ability's cast time.
  - This is also the same displacement that is used for when *Devour* is cast with *Tongue Lash*, though with a greater pull distance as well as faster pull speed to account for *Devour*’s short cast time.
  - The target will blink to **Tahm Kench**’s location at the end of this displacement, including if it is interrupted.
- Against enemies, *Devour*’s pull from abducting the target depends on the application of the suppression; if the suppression is not applied, neither is the pull. Similarly, if the suppression is removed, the displacement from the pull is stopped.
  - The attachment also depends upon the occurrence of the pull; if the target is not pulled, they are not attached to **Tahm Kench** and the swallow fails, cancelling the ability entirely.
  - If the target resists the suppression by being immune to crowd control, displacement immune, or having a spell shield, **Tahm Kench** will not pull the target towards him.
  - If the target removes the suppression by any means, including with an applicable cleanse effect or dispel, one of two of the following interactions will occur based on the timing of the removal:
    - If the suppression was removed during *Devour*’s cast time, the target will stop the displacement from the pull instantly and prevent themselves from being attached to **Tahm Kench** after the cast time.
    - If the suppression was removed while the target is attached to **Tahm Kench** (in other words, inside of his belly), the target will detach themselves from him instantly without **Tahm Kench** performing *Regurgitate*.
- The untargetability and displacement immunity is granted to the target during *Devour*’s cast time as well. Enemies are granted these effects while they are suppressed; they are removed when the suppression is.
- *Devour*’s suppression against enemies is removed if **Tahm Kench** dies, enters resurrection, or transitions in or out of the Realm of Death.
- Enemies will be dealt *Regurgitate’s* damage regardless of **Tahm Kench** casting the ability if *Devour*’s suppression ends by any means other than transitioning out of Realm of Death or dismounting (including during *Devour*’s cast time).
- If the target transitions into the Realm of Death or enters resurrection, **Tahm Kench** will *Regurgitate* them automatically.
  - This only occurs for the former if **Tahm Kench** is in a different dimension from the target.
- *Devour* cannot be used on allies that are channeling or using an ability that preloads UnstoppableForceMarker.
- The self-slow is affected by slow resist.
- **Tahm Kench** cannot use gates during *Devour*.
- Allies can see if this spell is ready next to **Tahm Kench**’s health bar.
- The swallowed target is not considered to be vanished. They are instead hidden below the visible map plane, to maintain ongoing effects such as Curse of the Black Mist.
- During *Devour*’s attachment and Regurgitate’s displacement, the swallowed target's camera is locked and centered on their champion.
  - This does not affect **Tahm Kench**.
- The following table refers for interactions while the target is unable to act:

---

### R: Regurgitate

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Targeting** | Direction |
| **Affects** | Enemies / Allies |
| **Damage Type** | Magic |
| **Spell Shield** | False |
| **Spell Effects** | spell |

**ACTIVE:** **Tahm Kench** detaches the swallowed champion from himself and spits them out in the target direction. If they are an enemy, he deals magic damage to them at the end of the displacement.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 100 / 175 / 250 / 325 / 400 (+ 15% (+ 7% per 100 AP) of the target's **maximum** health) |

*If Regurgitate was cast automatically, **Tahm Kench** spits the target out in his facing direction.*

**Notes:**

- The displacement from spitting the target out also uniquely counts as the enemy's dash.
- The target will face in the direction of *Regurgitate*’s cast for the duration of the displacement.
- The target will blink to **Tahm Kench**’s location at the start of *Regurgitate*’s cast time.
- *Regurgitate* will also automatically cast if the target is affected by Fate's Call or enters resurrection.
- **Tahm Kench** becomes unable to act for $0.25$ seconds upon casting *Regurgitate*.

---

## Patch History

### V25.S1.2
- Tongue Lash
  - Base damage reduced to 75 / 120 / 165 / 210 / 255 from 80 / 130 / 180 / 230 / 280.

### V14.24#December 18th Hotfix|V14.24
- Stats
  - Base armor reduced to 39 from 42.

### V14.23
- An Acquired Taste
  - AP ratio reduced to $1.5$% AP per 100 **bonus** health from 2% AP per 100 **bonus** health.
- Devour
  - Shield AP ratio reduced to 100% AP from 150% AP.

### V14.15
- Abyssal Dive
  - **Bug Fixes:** Aurora is no longer visible and in her default model pose while he is carrying her via Devour.
- Devour
  - **Bug Fixes:** Aurora is no longer visible and in her default model pose while he is carrying her via *Devour*.

### V14.2
- Devour
  - **Bug Fixes:** Targeting a champion with the ability no longer triggers Experimental Hexplate despite the ability not being cast yet.

### V13.22
- An Acquired Taste
  - Health ratio reduced to 4% **bonus** health from 5%.

### V13.21
- An Acquired Taste
  - Base damage changed to 6 to 42 for 10 / 48 from 8 to 60.
  - Health ratio increased to 5% **bonus** health from 3%.
- Abyssal Dive
  - Cooldown refund increased to 40 / 42.5 / 45 / 47.5 / 50% from 40% at all ranks.

### V13.12
- Tahm Kench and Tahm Kench
  - An Acquired Taste
    - **Bug Fixes:** Special animations when approaching enemies with three stacks of the effect have been restored.

### V13.5
- Devour
  - **Bug Fixes:** Cast now properly triggers Radiant Virtue Guiding Light.

### V13.4
- Devour
  - **Bug Fixes:** No longer causes an allied Goes Where He Pleases and Void Shift to be triggered by the ability.

## Trivia

- Tahm Kench is one of the few champions who can apply crowd control on themselves (the others being Blitzcrank, Caitlyn, Lissandra, Rumble, Sion, Varus, Vi, Vel'Koz, Xerath, and Ziggs).
- At first it was thought the gambler and his bride were Twisted Fate and Evelynn (Twisted Fate’s cards went blank and featured an unknown cursive script in his official champion page; however, Twisted Fate and the original *Tahm Kench*, whose identity the malevolent River King usurped, are not the same.
- Tahm Kench's dance has him play the tune he hums in 'The River King' with his tongue (Rift Scuttler will join him if he dances near her).
- Tahm Kench's design and playstyle may be loosely based on the Banderhobb, a toad-like creature from the Forgotten Realms universe which kidnaps unsuspecting victims by swallowing them alive and carrying them to its master's lair.
- In VALORANT on the map (Mid Bottom), there is a park bench crafted with his likeness. This is most likely a reference to the "Unbench the Kench" meme.

---
*This page was automatically generated from League of Legends Wiki data.*