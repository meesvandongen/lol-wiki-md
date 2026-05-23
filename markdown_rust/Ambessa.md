# Ambessa

## Overview

- **Title:** Ambessa
- **Resource:** Energy

## Stats

| Stat | Base | Growth |
|------|------|--------|
| Armor | 35 | 4.9 |
| Attack Damage | 63 | 3 |
| Attack Speed | 0.625 | 2.5 |
| HP | 630 | 110 |
| HP Regen | 8.5 | 0.75 |
| MP | 200 | 0 |
| MP Regen | 50 | 0 |
| Magic Resist | 32 | 2.05 |
| Move Speed | 335 | 0 |
| Range | 125 | 0 |

## Advanced Stats

| Metric | Value |
|--------|-------|
| Acquisition Radius | 600 |
| Attack Cast Time (s) | 0.25 |
| Attack Speed Ratio | 0.625 |
| Base Attack Time (s) | 1.6 |
| Pathing Radius | 35 |
| Selection Height | 100 |
| Selection Radius | 100 |
| Windup % | 15.6% |

## Abilities

### Passive – Drakehound's Step

| Attribute | Value |
|-----------|------:|
| **Target Range** | 175 (Minimum dash distance) / 350 (Maximum dash distance) |
| **Speed** | 770 to 950 for 4 / 1;6;11;16 / formula=770 at level 1, then +60 every 5 levels thereafter + 100% movement speed |

**INNATE:** During the lockout of **Ambessa's** abilities, inputting an attack or movement command (Default MB2/right click) causes her to dash to or towards (If inputted outside of the maximum dash range, she will dash the maximum distance towards the target/location) the target or location, respectively, after the lockout ends. This dash cannot pass through terrain.
**_Ambessa** cannot dash (See notes) while immobilized or grounded._

**INNATE:** Whenever **Ambessa** casts an ability, she generates a stack of _Medarda Maxim_ after the ability's respective lockout for 4 seconds, refreshing with subsequent casts and stacking up to 3 times. Basic attacks consume one stack each to become empowered.

**MEDARDA MAXIM:** **Ambessa's** next basic attack on-attack is empowered to have an uncancellable windup, gain 75 **bonus** range and 50% **bonus** attack speed, and deal 5 to 30 (+ 30% **bonus** AD) **bonus** physical damage and restore 40 to 70 / 1;7;13 energy.

| Detail | Value |
|--------|------:|
| **Targeting** | Passive |
| **Affects** | Self, Enemies |
| **Damage Type** | physical |
| **Spell Effects** | proc |
| **Spell Shield** | false |
| **Parry** | true |
| **Grounded** | true |
| **Knockdown** | true |

**Notes:**

- If an Attack command has been queued during an ability lockout and both:
- # the player issues a Hold (default **J**) or Stop (default **S**) command, **and**
- # the acquired target is within **Ambessa's** attack range after the end of the lockout
  - then **Ambessa's** dash will cancel.
- If **Ambessa** is immobilized or grounded during an ability's lockout, any attack or movement commands inputted during the lockout will not be buffered to cast at the end of the lockout, even if the crowd control has worn off by the time she is able to move (and thus dash).
  - This also applies vice versa where she has successfully buffered an inputted attack or movement command but is afterwards affected with the aforementioned crowd control types during the remaining lockout time. This causes the buffer to be cancelled and the dash to fail to trigger as a consequence.
  - Attack commands will not be buffered to cast at the end of the lockout even while grounded.
- If multiple attack and movement commands are inputted during the lockout of an ability, the most recent one is used for the dash's targeting.
  - If the most recent input is not an attack or movement command, the dash will not trigger.
    - In this case, the buffer for the previous attack/movement command was cancelled by a new non-attack/movement command such as an input for an ability cast.
- During _Drakehound's Step_, a visual indicator is visible to **Ambessa** displaying the eventual dash's direction and destination location.
- The number of available attacks empowered by _Medarda Maxim_ are visible as pips under **Ambessa's** [health bar](./health_bar.md), only visible to the player.
- The following table refers for interactions while **Ambessa** is dashing: (attack=false)

### W – Repudiation

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 325 |
| **Cost** | 70 |
| **Cost Type** | Energy |
| **Cooldown** | (+18 to 14% AP) |

**ACTIVE:** **Ambessa** braces herself for up to 0.50 seconds, during which in the first 0.23 seconds she is unable to act. After **Ambessa** finishes bracing, she smashes the ground beneath her, dealing physical damage to nearby enemies.

_Ambessa_ dash may be buffered during the lockout or initiated within 0.25 seconds of the lockout ending.

Additionally, **Ambessa** shields herself at the time of cast for 50 to 320 (+ 150% **bonus** AD) for 1.50 seconds. If the shield mitigates any amount of damage taken from champions, large monsters, or turrets before **Ambessa** smashes the ground, _Repudiation's_ damage is increased by 50%.

**_Ambessa** will smash the ground at the end of Ambessa dash if she would finish bracing during it, and otherwise always smashes the ground from wherever she is at the end of the bracing time._

| Detail | Value |
|--------|------:|
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Effects** | spellaoe |
| **Spell Shield** | true |

**Notes:**

- _Repudiation's_ bracing time is unaffected by the initiation of _Ambessa_ dash.
  - **Ambessa** will smash the ground from wherever she is at the end of the bracing time even if the dash ends or completes its travel before the brace is finished.
    - She will remain in place until the brace is finished in this case.
    - If the brace would however finish during the dash, due to the specific timing of the dash's initiation, **Ambessa** will smash the ground at the end of the dash instead.
      - This would apply in cases where the dash is initiated right before or shortly before the bracing is finished.
- _Repudiation's_ shield buff and shield health on the health bar UI are not granted to and displayed on the caster on-cast, but rather only after a very short, inconsistent delay from the time of cast.
- The following table refers for interactions while **Ambessa** is locked out:
  - **Ambessa** will smash the ground instantly upon entering resurrection. (attack=Registers input for _Ambessa_.) <!-- Blurb -->

### E – Ambessa Lacerate

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 325 |
| **Cost** | 70 |
| **Cost Type** | Energy |
| **Cooldown** | (+13 to 9% AP) |

**ACTIVE:** **Ambessa** enters a 0.23-second lockout and spins her twin drakehounds around her to deal physical damage to nearby enemies and slow them by 99% decaying over 1 second.

_Ambessa_ dash may be buffered during the lockout or initiated within 0.28 seconds of the lockout ending; in either case, she will spin a second time at the end of the dash to apply the same effects at no additional cost.

| Detail | Value |
|--------|------:|
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Effects** | spellaoe |
| **Spell Shield** | true |

**Notes:**

- **Ambessa** will cast the second spin of _Lacerate_ from wherever she is at the end of the dash.
  - She will spin a second time even if the dash is interrupted.
- The following table refers for interactions while **Ambessa** is locked out: (attack=Registers input for _Ambessa_.) <!-- Blurb -->

### R – false

| Attribute | Value |
|-----------|------:|
| **Range** | 1250 |
| **Cast Time** | 0.55 |
| **Width** | 65 |
| **Cooldown** | (+130 to 100% AP) |

**PASSIVE:** **Ambessa** gains armor penetration and heals herself for a percentage of the post-mitigation damage (Damage calculated after modifiers) she deals to enemies with her active abilities. The healing effectiveness is reduced to 25% against minions and 40% against monsters.

**ACTIVE:** **Ambessa** prepares a strike in a line in the target direction, then blinks behind the farthest enemy champion within the area and seizes them. If she successfully (See notes) seizes the target, **Ambessa** attaches them to herself and suppresses them for 0.75 seconds. While the target is suppressed, they are revealed and **Ambessa** picks them up off the ground before crashing them back down, afterwards dealing physical damage and stunning them for 0.40 seconds.

**_Ambessa** is displacement immune and unable to act during the cast time and while the target is suppressed, lingering for 0.10 seconds afterwards. Ambessa dash may be buffered during the cast time or lockout, as well as initiated within 0.28 seconds of the lockout ending._

| Detail | Value |
|--------|------:|
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Effects** | spell |
| **Spell Shield** | true |
| **Grounded** | true |

**Notes:**

- _Public Execution's_ attachment depends on the application of the suppression; if the suppression is not applied, neither is the attachment. Similarly, if the suppression is removed, so is the attachment.
  - If the target resists the suppression by being immune to crowd control, displacement immune, or having a spell shield, **Ambessa** will not attach them to herself.
    - She will be unable to apply the damage and stun in this case since the suppression was not applied in the first place.
    - She will still blink to the target in this case.
  - If the target removes the suppression by any means, including with an applicable cleanse effect or dispel, they will instantly detach themselves from **Ambessa**, which causes the ability to cancel immediately.
    - She will fail to apply the damage and stun to the target since the suppression was removed from them early in this case.
    - Her lockout and displacement immunity also ends prematurely in this case.
- If **Ambessa** would blink inside the attack range of an enemy turret, including the [Nexus Obelisk](./Nexus_Obelisk.md), _Public Execution_ will instead attempt to position her outside of it.
- **Ambessa** will reveal herself during the cast time if there is an enemy champion nearby.
- **Ambessa** will search for enemies in the direction she is facing at the end of cast time.
  - This direction can be changed by Seraphine.
- If **Ambessa** successfully seizes a target, any regular movement commands issued before doing so are discarded upon the lockout ending.
  - This does not affect movement orders issued during the lockout.
- _Public Execution_ will trigger a stack of Electrocute, Phase Rush, and Eclipse Ever Rising Moon for the suppression application and the damage dealt.
- The following table refers for interactions while **Ambessa** is in cast time and during the lockout: (attack=Registers input for _Ambessa_.) <!-- Blurb -->

## Trivia

- Ambessa is the second champion to be revealed for League of Legends, Wild Rift, and Legends of Runeterra around the same time, after Akshan.
  - To date, Ambessa is the only champion that has been revealed to be announced in all games within the League of Legends IP (Intellectual Property), including Teamfight Tactics.
- Ambessa is the first non-Ionian champion to use energy.
- With Mel's release, Ambessa is the first female champion to be a mother of another champion.
  - There are playable champions which are mothers, but not of other playable champions, such as Rek'Sai.
  - There are also champions which have known mothers, such as Smolder and Nunu, but their mothers are not playable champions themselves.
- Her buff flavortext reads:
  - _Medarda Maxim_ (Ambessa stacks): _"Don't pity the meek" Ã¢ÂÂ Medarda Family Motto_
  - _D I S R E S P E C T_ (Ambessa ready): _"Find out what it means to me." Ã¢ÂÂ Ambessa Medarda_
  - _AnticiÃ¢ÂÂ_ (Ambessa steadying): _Ã¢ÂÂpation_
    - This is a reference to Dr. Frank-N-Furter's [https://youtu.be/OqOEnbXmnGc?si=1ObjKNtdWMbag42k&t=7 iconic line] in _The Rocky Horror Picture Show_ comedy horror movie.
  - _Unfazed_ (Ambessa shield): _"Some say the best defense is a good offense. Why pick when I have both." Ã¢ÂÂ Ambessa Medarda_
  - _Pathetic_ (Ambessa slow): _"You're a third rate fighter with fourth rate technique." Ã¢ÂÂ Ambessa Medarda_ [sic]
    - This is a reference to [https://yugipedia.com/wiki/Seto_Kaiba Seto Kaiba's] insult to [https://yugipedia.com/wiki/Joey_Wheeler Joey Wheeler] in the _Yu-Gi-Oh_ anime.
  - _Always Win Your Battles_ (Ambessa successful cast): _"Defeat earns you nothing, I simply win." Ã¢ÂÂ Ambessa Medarda_
  - _Some Advice_ (Ambessa suppression): _"Don't be sorry, be better." Ã¢ÂÂ Ambessa Medarda_
    - This is similar to Kratos' [https://www.youtube.com/watch?v=Mwf3EPnusVQ quote] in _God of War (2018)_ video game. (_Do not be sorry, be better._)
  - Killing an enemy champion grants her a cosmetic buff that reads: _"For you, the day the Medardas graced your village was the most important day of your life. But for me, it was Tuesday."_
    - This [https://www.youtube.com/watch?v=GlhOUyy4wbs quotes] M. Bison in _Street Fighter_, referring to himself instead.
  - Being killed by an enemy Mel grants her the _Took Long Enough_ cosmetic buff that reads: _Mel has killed Ambessa. 'This is a new feeling. Pride in someone else. Finally.' - Ambessa Medarda_
    - This is a reference to DragonBall Z Abridged's [https://www.youtube.com/watch?v=dSJlqrnC6bE Vegeta's Unyielding Rage].

<!-- Raw excerpt (first 20 lines) -->

<details><summary>Raw excerpt</summary>

```wikitext

== Abilities ==

==Champion skins==
:_This article section only contains champion skins. For all associated collection items, see [Ambessa (Collection)](./Ambessa_Cosmetics.md)._

==Patch history==

==Trivia==
```
</details>
