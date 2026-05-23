# Ashe

## Overview

- **Title:** Ashe
- **Resource:** Mana

## Stats

| Stat | Base | Growth |
|------|------|--------|
| Armor | 26 | 4.6 |
| Attack Damage | 59 | 3.45 |
| Attack Speed | 0.658 | 3.33 |
| HP | 610 | 101 |
| HP Regen | 3.5 | 0.55 |
| MP | 280 | 35 |
| MP Regen | 7 | 0.65 |
| Magic Resist | 30 | 1.3 |
| Move Speed | 325 | 0 |
| Range | 600 | 0 |

## Advanced Stats

| Metric | Value |
|--------|-------|
| Acquisition Radius | 600 |
| Attack Delay Offset (s) | -0.0807 |
| Attack Speed Ratio | 0.658 |
| Crit Base | 100 |
| Missile Speed | 2500 |
| Pathing Radius | 35 |
| Selection Height | 155 |
| Selection Radius | 120 |

## Abilities

### Passive – Frost Shot

**INNATE:** **Ashe's** basic attacks deal **bonus** physical damage equal to 75% total critical damage critical strike chance. Critical strikes do not deal any additional damage.

**INNATE - FROST SHOT:** **Ashe's** basic attacks and ability hits apply _Frost_ to enemies for 2 seconds, which slows them by key= / 20 to 30 for the duration.

**INNATE - CRITICAL SLOW:** **Ashe's** critical strikes double _Frost's_ slow strength to key= / 20*2 to 30*2, decaying over the first second of the duration to its normal strength.

| Detail | Value |
|--------|------:|
| **Targeting** | Passive |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Parry** | True |

**Notes:**

- **Ashe's** critical strikes are still considered critical strike damage and thus will be reduced by Randuin's Omen Resilience.
- Runaan's Hurricane will not deal additional damage on critical strikes.
- Cheap Shot will trigger on a subsequent basic attack even when the target is no longer slowed. <!--Blurb-->

### Q – Ranger's Focus

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Cost** | 30 |
| **Cost Type** | Mana + 4 Focus |

**PASSIVE:** While _Ranger's Focus_ is inactive, **Ashe's** basic attacks on-attack generate a stack of _Focus_ for 4 seconds, refreshing on subsequent attacks and stacking up to 4 times. Stacks expire by one every second when the duration ends.

**ACTIVE:** For 6 seconds, **Ashe** gains **bonus** attack speed and empowers her basic attacks to fire a flurry of five arrows that deal **modified** physical damage. The flurries apply life steal and Ashe per arrow, but apply on-hit effects only once.

_Ranger's Focus resets **Ashe's** basic attack timer._

| Detail | Value |
|--------|------:|
| **Targeting** | Auto |
| **Affects** | Self |
| **Damage Type** | Physical |
| **Spell Effects** | special |
| **Spell Shield** | false |
| **Parry** | True |
| **Projectile** | True |

**Notes:**

- The flurries deal 1 instance of basic damage followed by 4 instances of a non-reactive type of damage (either default or proc).<!--One test to determine the difference is Teemo's W, but the test is hard to set-up since the first bolt removes Teemo's W.-->
  - The flurries are classified as a basic attack on a script level, but are considered an ability for other effects (e.g. Maokai).
- The first flurry of a _Ranger's Focus_ cast firesÃ£ÂÂ one additional arrow, dealing 20% increased total damage. Ã¢ÂÂ· six arrows, dealing a total of (+110*1.2 to 140*1.2% AP)% attack damage. Ã£ÂÂ
- _Ranger's Focus_ works against structures.
- Flat damage reductions (e.g. Amumu, Fizz, or Leona) apply per arrow, resulting in extremely increased effectiveness against _Ranger's Focus_.
  - The sole exception is Warden's Mail Rock Solid, as that applies per cast instance rather than per damage instance.
- A flurry can apply 5 stacks of Black Cleaver Carve to a single target as it deals 5 instances of physical damage.
- _Ranger's Focus_ also empowers Runaan's Hurricane Wind's Fury bolts to become a flurry of 5 bolts each as well.
  - The empowered bolts use **Ashe's** attack projectile, including her missile speed (2500 compared to their default 2000).
  - The bolts are evenly distributed among the targets, but a single target cannot be hit by more than 5 bolts.
  - These flurries also apply on-hit effects only once to each target hit.
- Because _Focus_ stacks are generated on-attack, Runaan's Hurricane Wind's Fury bolts will not generate any.

### W – Volley

| Attribute | Value |
|-----------|------:|
| **Cast Time** | 0.25 |
| **Effect Radius** | 1200 (Not including the offset, see notes) |
| **Angle** | 27.75ÃÂ°/ 37ÃÂ° / 46.25ÃÂ° (Full cone at ranks 1, 2/3 and 4/5, respectively) / 4.62ÃÂ° (Angle between individual missiles) |
| **Width** | 20 (Each missile) |
| **Speed** | 2000 |
| **Cost** | (+75 to 55% AP) |
| **Cost Type** | Mana |
| **Cooldown** | (+18 to 4% AP) |

**ACTIVE:** **Ashe** shoots a volley of arrows in a cone in the target direction, each dealing physical damage to the first enemy hit, and applying _Ashe_ to enemy champions hit.

Enemies can intercept multiple arrows but do not take damage from any beyond the first.

| Detail | Value |
|--------|------:|
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Effects** | aoe |
| **Spell Shield** | True |
| **Projectile** | True |

**Notes:**

- The missiles are spawned in a straight horizontal line 75 units in front of **Ashe**, with a total width of 75/100/100/123/123 units.
  - Between each missile spawn location is a distance of 12.50 units, except for the two outermost ones at the last two ranks (which are 11.50 units from the closest other one), and the two centermost missiles at ranks 2 and 4 (which have 20 units between them and 15 units to the next missile, respectively).
  - This inconsistent behaviour of spawn locations means the angle between each missile can vary off of 4.62ÃÂ° slighty. <!--Blurb-->

### E – Hawkshot

| Attribute | Value |
|-----------|------:|
| **Cast Time** | 0.25 |
| **Target Range** | Global |
| **Effect Radius** | 1000 (Vision bubble radiuses) |
| **Speed** | 1400 |
| **Cost** | 1 |
| **Cost Type** | Charge |
| **Cooldown** | 5 |
| **Recharge** | (+90 to 50% AP) |

**ACTIVE:** **Ashe** sends a hawk spirit to a location, granting sight of the area along its pathÃ£ÂÂ repeatedly for 0.50 seconds after every 100 units traveled Ã¢ÂÂ· for effectively up-to 1.80 seconds at any location Ã£ÂÂand at its destination for 5 seconds.

**Ashe** periodically [stocks](./ammunition.md) a _Hawkshot_ charge, up to a maximum of 2.

| Detail | Value |
|--------|------:|
| **Targeting** | Location |
| **Affects** | Enemies |
| **Spell Shield** | false |
| **Projectile** | special |

**Notes:**

- _Hawkshot's_ recharge timer seems to start at the beginning of the game, even prior to ranking the ability, but upon actually ranking the ability, it starts with one charge and the recharge timer starts from 0.
- _Hawkshot_ will ping enemy champions it spots if allies didn't already have vision of them, and put a 'revealed' visual effect on them for 0.00 (Estimated), but does not actually reveal the units in particular, and is removed if the champion leaves the area.
- **Ashe** marks enemy champions who were previously unseen but were revealed _Hawkshot_ in order to gain [assist](./assist.md) credit, lasting for the standard credit timer.
- _Hawkshot_ will trigger upon colliding with Samira or Yasuo but not Braum.
- _Hawkshot_ will grant 0.33 points of [vision score](./vision_score.md) for each champion revealed. <!--Blurb-->

### R – Enchanted Crystal Arrow

| Attribute | Value |
|-----------|------:|
| **Range** | Global |
| **Cast Time** | 0.25 |
| **Effect Radius** | 400 (Radius of explosion) /  350 (Missile sight radius) |
| **Width** | 260 (Missile width) |
| **Speed** | 1500 to 2100 by 200 / type=Seconds traveled / showtype=false / changedisplay=true / formula=1500 base speed + 200 acceleration |
| **Cost** | 100 |
| **Cost Type** | Mana |
| **Cooldown** | (+100 to 60% AP) |

**ACTIVE:** **Ashe** fires a massive arrow of ice in the target direction, granting sight of the area (Cannot grant sight through terrain and can only grant sight into brush when the missile flies through that brush) it flies through each for 1 second. The arrow shatters upon hitting an enemy champion, dealing them magic damage, stunning them for type=distance traveled / 1 to 1 for 5;1 to 3.5 for 10 / 0 to 2800 / formula=1s for the first 800 units traveled; 1+(0.25 per 200 units)s beyound 800 units. _This is capped at 2800 units traveled._ seconds, and granting sight of the area around them for 1 second.

Enemies surrounding the primary target are dealt the same damage and afflicted with _Ashe_.

| Detail | Value |
|--------|------:|
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Effects** | aoe |
| **Spell Shield** | True |
| **Projectile** | True |

**Notes:**

- _Enchanted Crystal Arrow's_ projectile has an  icon on the mini-map while it is in flight. It can be seen by only **Ashe** and her allies.
- The effect will be centered on the first champion struck, not at the location the missile collided with them. <!--Blurb-->

## Trivia

- Ashe's dance references [https://www.youtube.com/watch?v=G4LIM9VYhB0 J'en Ai Marre!] by AlizÃÂ©e.
  - A side by side comparison can be seen [http://www.youtube.com/watch?v=v3iyKHLrv_I here.]
- Ashe shares similarities with  from Final Fantasy XII. Their names are similar and both became queens, use ranged weapons (Ashelia uses hand bombs in Final Fantasy XII: Revenant Wings) and were once hunted down to prevent them from leading their respective peoples.
- Ashe is the first (and so far the only) champion to possess two global-ranged abilities: Ashe and Ashe.
  - Ekko and Ekko ranges can be global, albeit situationally.
- Ashe was named for [Marc 'Tryndamere' Merrill](./Marc_'Tryndamere'_Merrill.md)'s wife Ashley.
- As of December 9, 2017 Ashe has featured on the [free champion rotation](./free_champion_rotation.md) more than any other champion (76 times).
- In [Summoner's Rift](./Summoner's_Rift.md), Thornmail was once recommended as a situational item for Ashe, this was most likely a reference to the old [Basic Tutorial](./Tutorial.md) where Ashe is pre-selected as the player's champion and Thornmail is the only purchasable item.
- In the now-removed official League of Legends [https://web.archive.org/web/20130726165848/http://forums.na.leagueoflegends.com/board/ forums], the  old icon of Ashe/LoL/History#Previous_Abilities was used to represent the "New Player Forum" section.

<!-- Raw excerpt (first 20 lines) -->

<details><summary>Raw excerpt</summary>

```wikitext

== Abilities ==

==Champion skins==
:_This article section only contains champion skins. For all associated collection items, see [Ashe (Collection)](./Ashe_Cosmetics.md)._

==Patch history==

==Trivia==

```
</details>
