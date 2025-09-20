# Yasuo

## Table of Contents
- [Basic Information](#basic-information)
- [Statistics](#statistics)
- [Abilities](#abilities)
- [Patch History](#patch-history)
- [Trivia](#trivia)

## Basic Information

| Attribute | Value |
|-----------|-------|
| **Champion** | Yasuo |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $550$ | $+85$ | $1995.0$ |
| **Mana** | $350$ | $+50$ | $1200.0$ |
| **Armor** | $22$ | $+3.5$ | $81.5$ |
| **Magic Resist** | $30$ | $+0.5$ | $38.5$ |
| **Attack Damage** | $56$ | $+3.1$ | $108.7$ |
| **Attack Speed** | $0.625$ | $+3.2\%$ | $0.965$ |

## Abilities

### Passive: Way of the Wanderer

**Innate - Intent:** **Yasuo** gains increased .

*He also gains from excess .*

**Innate - Intent:** ''Yasuo's* is doubled from all other sources. Additionally, **Innate - Resolve:** **Yasuo* generates a stack of Flow for every $ for 3100 to 100 for 3*Yasuo** consumes all Flow to grant himself a shield for $-)/17*(x-1)*(0.7025+0.0175*(x-1))) that lasts for 1 second.

**Notes:**

- **Yasuo** gains the shield before the damage instance; the shield will mitigate the damage that triggered it.
- **Yasuo** is able to reach 100% critical strike chance with 'Way of the Wanderer's' multiplier after obtaining a total of *50% critical strike chance*.
- ''Yasuo's'' secondary resource bar will display the amount of shield stored as an equivalent percentage to his Flow stacks. (Each Flow stack will add $-)/100/17*(x-1)*(0.7025+0.0175*(x-1))) to the bar)
  - Upon taking damage, the bar will display his current shield strength.

---

### Q: Steel Tempest

**Active:** **Yasuo** thrusts his sword in a line in the target direction that deals physical damage to enemies hit and applies on-hit and on-attack effects to the first enemy hit. This can critically strike.

*If this hits an enemy, **Yasuo** generates a stack of *Gathering Storm* for a few seconds. At 2 stacks, the next *Steel Tempest* consumes them all to become empowered with a new effect.*

**Active:** **Yasuo** thrusts his sword in a line in the target direction that deals physical damage to enemies hit, applies on-hit effects to the first enemy hit, and triggers on-attack effects once. 'Steel Tempest's' damage based on its AD ratio can critically strike for critical damage. **Steel Wind Rising:** If this hits at least one enemy, **Yasuo** generates a stack of *Gathering Storm* for 6 seconds, stacking up to 2 times and refreshing on subsequent hits. At 2 stacks, the next *Steel Tempest* cast consumes them all to become empowered with a new effect. **Gathering Storm Bonus:** **Yasuo** unleashes a whirlwind in the target direction that applies and triggers the same effects and additionally airborne enemies hit for $0.9$ seconds. 'Steel Tempest's' thrust will fail to hit targets after the cast time if **Yasuo** was affected by disarming crowd control during it, but the *cooldown* of the ability is reset to $0.1$ seconds. If *Steel Tempest* is cast during **Sweeping Blade**, it will instead affect enemies around **Yasuo** at the end of the dash, or doing so immediately at his landing location after Blink while ending the dash prematurely. 'Steel Tempest's' cast time is not incurred in this case.

| Attribute | Value |
|-----------|-------|
| **Cast Time** | Thrust cast time / Whirlwind cast time |

**Scaling:**
- **Physical Damage:** $20-120$ (+ 105% AD)

**Notes:**

- Casting *Sweeping Blade* when 'Steel Tempest's* remaining cooldown is less than $0.5$ seconds resets this remaining cooldown to allow casting it in combination, but adds the refunded time to *Steel Tempest's' cooldown.
- If **Yasuo** becomes unable to cast abilities during the dash after buffering, *Steel Tempest* will not cast, but will still go on cooldown.
- The whirlwind from consuming stacks of *Gathering Storm* can strike targets whose center is behind **Yasuo**, unlike most Projectile.
- The whirlwind is not released when cast during *Sweeping Blade*, but instead airborne all enemies hit around **Yasuo** at the end of the dash.
- *Steel Tempest* applies basic damage to the first (closest) enemy hit and area damage to secondary enemies:
  - *Steel Tempest* will apply on-hit effects to the first target hit, but will not do so to the secondary ones.
  - *Steel Tempest* will not apply spell effects to the first target hit, but will do so to the secondary ones.
  - Spell vamp will only grant heal from the damage dealt to secondary targets, and healing is reduced to 33% effectiveness, accordingly.
  - will heal based on the damage dealt to the first target hit.
- The whirlwind is a missile and can therefore be projectile.
- *Steel Tempest* will only draw minion aggro if the first target is a champion.
- Each parry has different interactions with this ability, whether it's the first target of *Steel Tempest* or the secondary one. In either case **Yasuo** still gains a stack of *Gathering Storm* and his whirlwind knock-up cannot be negated by parries:
  - Dodge: both first target and secondary target **do not** take damage.
  - Block: first target **does not** take damage, secondary target **does** take damage.
  - Blind: both first target and secondary target **do** take damage.
- *Steel Tempest* rolls its critical strike individually against each enemy hit.
- Casting *Last Breath* during *Steel Tempest* will buffer it to cast after the cast time has completed.
  - However, if *Last Breath* is used on a target affected by an airborne source that is not **Yasuo**, it is cast immediately due to 'Steel Tempest's* cast time ending prematurely. *Steel Tempest' will still cast in this case. <!--14.11 fixed
- If *Steel Tempest* is buffered during *Sweeping Blade* and **Yasuo* uses siHextech Rocketbelt*, *Steel Tempest* will go on cooldown without casting. In the case of *Hextech Rocketbelt* **Yasuo** will also need to cast a movement command after it, else *Steel Tempest* will still trigger.
  - If, after this effect, **Yasuo** inputs a basic attack command quickly followed by a movement command, he will cast *Steel Tempest* around him and basic attack the target at the same time. *** *Steel Tempest* will not go on cooldown.-->
- Spell shield does not prevent a stack of *Gathering Storm* from being gained.
- While at two stacks, a range indicator will be shown for the effective range of the whirlwind.

---

### W: Wind Wall

**Active:** **Yasuo** raises a wind wall that drifts in the target direction for a few seconds, blocking all enemy projectile that hit it.

**Active:** **Yasuo** raises a wall of wind that travels in the target direction over $0.6$ seconds. It then drifts another 50 units over $3.4$ seconds, blocking all hostile non-turret projectile that hit it and granting sight of its surroundings.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $25-17$ seconds |
| **Cast Time** | $0.013$ seconds |

**Scaling:**
- **Wall Width:** $320-600$

**Notes:**

- *Wind Wall* grants sight in a small static area in front of the cast location.
- *Wind Wall* starts blocking projectiles on-cast (despite not being fully formed).
- *Wind Wall* travels via a missile, and thus stops upon colliding with an enemy *Wind Wall*.

---

### E: Sweeping Blade

**Active:** **Yasuo** dashes in the target enemy's direction, dealing magic damage, briefly becoming ghosted, and generating a stack of *Ride the Wind* for a few seconds, stacking up to a cap.

*Sweeping Blade* cannot be cast on the same target for a period.

**Active:** **Yasuo** dashes a fixed distance in the direction of the target enemy and becomes ghosted for 2 seconds, refreshing on subsequent casts. Upon impact, he deals magic damage to the target and generates a stack of *Ride the Wind* for 5 seconds, which refreshes on subsequent hits and stacks up to 4 times. **Ride the Wind:** 'Sweeping Blade's' damage is increased by 25% per stack, up to $25*4$% at maximum stacks. *Sweeping Blade* can be cast on the same target only once every few seconds. **Yasuo will be knockdown by any immobilize or polymorph crowd control during the dash.** **Last Breath* can be cast during the dash.*

| Attribute | Value |
|-----------|-------|
| **Range** | 475 units |
| **Cooldown** | $0.5-0.1$ seconds |
| **Cast Time** | none |

**Scaling:**
- **Magic Damage:** $70-130$ (+ 20%
- *bonus AD) (+ 60% AP)
- **Bonus Damage per Stack:** $70*0.25-130*0.25$ (+ $20*0.25$%
- *bonus AD) (+ $600.25$% AP)*bonus* AD) (+ 60% AP)2-130*2$ (+ $20*2$%
- *bonus AD) (+ $60*2$% AP)

**Notes:**

- If *Steel Tempest* has been buffered, *Wind Wall* may also be cast during the dash.
- *Sweeping Blade* will generate $7.5$ Flow stacks per cast that travels the maximum distance.
- **Yasuo** can use 'Sweeping Blade's' fixed dash distance to cross walls both before and after hitting his target (provided his proximity to the wall and/or target allows it).
- After *Sweeping Blade*, **Yasuo** becomes ghosted for a moment to prevent situations where he would face extreme cases of body block.
- Sleep does not count for knocking the dash down.

---

### R: Last Breath

**Active:** **Yasuo** blinks to the target airborne enemy champion and instantly generates max Flow. He then briefly airborne, true sight, and deals physical damage to all nearby airborne enemies.

*He becomes empowered for some time, causing his critical strikes to gain *lethality **bonus**-armor penetration*.*

**Active:** **Yasuo** blinks to the opposite of a sight airborne enemy champion nearest to the cursor, instantly generating maximum Flow while resetting **Steel Tempest** stacks. Upon arrival, he airborne all nearby airborne enemy champions for 1 second, true sight them, becoming lockout himself, and slashing them with his sword over the duration to deal physical damage thereafter. Surrounding enemy champions that become airborne during this time will also be affected by *Last Breath* through the remaining duration. For the next 15 seconds, the damage dealt by ''Yasuo's'' critical strikes ignores lethalitybonus armor*. *A nearby airborne enemy champion is required to cast this ability. If **Yasuo** would blink inside the *attack range* of an enemy turret (excluding the Nexus Obelisk), Last Breath will instead attempt to position him outside of it.*

| Attribute | Value |
|-----------|-------|
| **Range** | 1400 units |
| **Cooldown** | $70-30$ seconds |
| **Cast Time** | none |

**Scaling:**
- **Physical Damage:* $200bonus AD)

**Notes:**

- *Last Breath* can be cast on targets suspension in mid-air by stun.
  - The wiki has adopted the term *suspension* for this unique property due to a statement by ''Yasuo's'' designer.
- The percentage bonus armor penetration stacks additively with other sources of percentage armor penetration.
- **Yasuo** gains the bonus armor penetration on-cast.
- *Last Breath* requires the target to be affected by airborne from an enemy source, and simultaneously that the target is not considered cc-immune, displacement immune, or untargetable during it.
  - *Last Breath* cannot be cast if there are no nearby enemy champions that meet the conditions. *** Attempting to cast the ability on an enemy champion that is not airborne will indicate that they "*Must Be Airborne!*". *** Spell shield will not prevent *Last Breath* from being cast and the target will be knocked up and dealt the damage. **** The spell shield is still consumed in this case.
  - Self-applied or allied-applied airborne do not allow *Last Breath* to target the champion. Those applied by the *neutral team (e.g. uis*can** be targeted.
  - An [indicator](./spell_indicator.md) will be placed towards enemy champions that meet the cast conditions within range of *Last Breath* to signify the ability can be cast.
  - *The Hextech Ultimatum* will prevent **Yasuo** from casting *Last Breath* on an airborne enemy champion far outside its borders. ** Yasuo*** can only do so if the airborne enemy champion is positioned slightly outside of the borders.
- Upon 'Last Breath's' cast, the targets' facing directions snap to **Yasuo**. Afterwards, their facing directions shift in all other directions every $0.25$ seconds over the duration of the airborne.
  - The last shift snaps them to the facing direction of the second one.
  - On the skin, these shifts in facing direction do not occur, but the targets' facing directions will still snap to **Yasuo** upon 'Last Breath's' cast.
- **Yasuo** does not need sight of enemy champions near his target to affect them with the ability.
- *Last Breath* applies its damage and removes its forced movement on the target when it ends, including if it is removed early by being cleanse.
  - This is in contrast to most other effects with airborne, which have their forced movement linger if the disabling debuff is removed and have to be overridden by another movement spell (such as or ).
  - It behaves similar to in this but is not considered suspension.
- Terrain displacing abilities (e.g. *Weaver's Wall*, *Pillar of Ice*, *Volcanic Rupture*) that affect enemy champions are considered airborne, and will therefore interact with *Last Breath*.
- While *blinking*, **Yasuo** creates an untargetable *clone* of himself that cues a dashing animation (similar to ) towards the target. This unit's rules do not follow those of a clone. As such, it is more alike a champion, unless special cased.
- While performing *Last Breath*, **Yasuo** is lockout of performing actions.
  - The lock out will end prematurely if all targets: *** Cleanse the airborne. ** Move far away from Yasuo***. *** Leave ''Yasuo's'' sight (e.g. by nearsight). *** Become untargetable. *** Death. *** Enter the *Realm of Death*.
- The following table refers for interactions while **Yasuo** is locked out:

---

## Patch History

### V25.14
- Stats
  - Attack damage growth reduced to $2.5$ from 3.

### V25.11
- *Last Breath*
  - ***Bug Fixes:*** While he has gained the effects of the bonus-armor penetration, no longer incorrectly deals excessive damage to **K'Sante** who is *All Out* and has equipped *Aftershock*.
  - ***Undocumented:*** Percentage bonus-armor penetration now stacks additively with other sources of percentage armor penetration.

### V25.05
- *Steel Tempest*
  - ***Bug Fixes:*** While Gathering Storm stacks and aisSteel Tempest* no longer fails to cast.

### V25.04
- *Way of the Wanderer*
  - ***Removed:**** No longer grants Yasuo a critical damage penalty of 10%.
    - This penalty would also apply to *Steel Tempest*.
- *Wind Wall*
  - ***Bug Fixes:*** no longer sometimes bypasses Wind Wall.

### V14.24
- *Wind Wall*
  - ***Bug Fixes:*** SFX is no longer audible through the Fog of War.

### V14.20
- *Steel Tempest*
  - Total critical damage penalty changed to -10% from -20%. No longer applies an *additional* critical damage penalty; only applies penalty.
    - Critical strike AD ratio increased to $175*0.9*1.05$%Way of the Wanderer's penalty of -10% AD from $0.9*0.9333*175Way of the Wanderer's penalty of -10% AD.
- *Last Breath*
  - Bonus armor penetration increased to 60% from 50%.

### V14.15
- Stats
  - Base armor increased to 32 from 30.
- *Sweeping Blade*
  - Base damage increased to $70-130$ from $60-100$.
  - Bonus damage per stack increased to 25% at all levels from $15 to 25$.

### V14.11
- *Steel Tempest*
  - ***Bug Fixes:*** Using siSweeping Blade* + *Steel Tempest* combo no longer incorrectly delays the knock-up until 'Recall's' channel ends or is interrupted.

### V14.10
- *Way of the Wanderer*
  - Critical strike chance multiplier reduced to 2 from $2.5$.
  - Bonus attack damage per excess critical strike chance increased to $0.5$ from $0.4$.

### V14.9
- Stats
  - Selection radius increased to 120 units from 75.

## Trivia

- Yasuo - **Yone** is one of seven pairs of sibling champions (the others being **Jinx** - **Vi**, **Cassiopeia** - **Katarina**, **Kayle** - **Morgana**, **Garen** - **Lux**, **Nasus** - **Renekton**, and **Darius** - **Draven**).
  - Though not a pair, **Anivia**, **Ornn**, and **Volibear** are also siblings.
- The flute Yasuo plays in his dance emote is most likely a Shakuhachi (harmonica for ; wind controller for ).
  - His Classic 'dance' references a [performance](https://www.youtube.com/watch?v=f7s-wXZWT5o) of this instrument.
    - A by comparison can be seen [here](https://www.youtube.com/watch?v=szArIi-HAUc).
    - Yasuo is the first champion to have an emote that will orient itself the same (dancing emote, facing the bottom-right of the screen), no matter what direction Yasuo was facing. Akali is the second.
  - The melody of his Classic dance complements **Yone** when the two use the dance emote simultaneously. A side-by-side comparison can be seen [here](https://youtu.be/OSx8KfnqSRw?t=20).
- Yasuo's design is largely based on the Japanese Rōnin (lit. a wanderer), a samurai whose masters had died or stripped of titles & lands. Many rōnin became mercenaries, Yojimbo, or even criminals.
- The length of Yasuo's sword is about that of a tachi or even an ōdachi. However, he wears it cutting edge up like the shorter katana.
- *Yasuo* [jasɯ̟ᵝo] is a Japanese masculine Yasuo, rendered as 康夫 (roughly "peaceful man") in the Japanese localization.
  - From Old Japanese **yasu-* "cheap, easy > peaceful" & masculine morpheme **wo*.
- modified critical strike chance references Samurai gameplay portrayals in video games.
- For April Fools' Day [2017](./Season_2017.md), a video showcasing a joke [rework](./Champion_updates.md) for Yasuo was [posted](https://twitter.com/leagueoflegends/status/848208117208764416?lang=en) by the official [Riot Games](./Riot_Games.md) Twitter account.
  - *Way of the Wanderer* was shown to have **Resolve's** shield also provide cc-immune, similar to .
  - *Steel Tempest 3* was shown to split into three additional whirlwinds in a cone after hitting a champion, similar to .
  - *Wind Wall* was shown to form a pentagon of walls, similar to .
    - Notably, this 'protection from projectile from all angles' mechanic was later implemented by .
  - *Sweeping Blade* was shown to be able to target ward and [champion summoned units](./champion_summoned_units.md) (e.g. *Demacian Standard*).
  - *Last Breath* was shown to be able to be recast for a seemingly infinite amount of times while the affected champion is still airborne.
- A functional real-life replica of Yasuo's *Steel Tempest* was crafted in an episode of YouTube series [Man At Arms: Reforged](https://www.youtube.com/playlist?list=PLUUGFk1wE5OFOpfPz3ggXQrCSdQdFEslx).
  - This video can be viewed [here](https://www.youtube.com/watch?v=W_7vdfoJApQ).
    - There are also videos where the following are crafted:
      - ([functional](https://www.youtube.com/watch?v=kz-0nfkVegg))
      - ([functional](https://www.youtube.com/watch?v=Lq-h3KkF_bY))
      - ([functional](https://www.youtube.com/watch?v=yEwne-IEG9s))
      - ([functional](https://www.youtube.com/watch?v=-EqnHVY1uEo))
      - ([functional, but impractically wieldy](https://www.youtube.com/watch?v=eqCmrn-80sQ))
      - ([prop](https://www.youtube.com/watch?v=3I6MK1SV1Lk))
- On 25 August 2018, some results of a survey regarding champion gameplay was published by Riot Blaustoise. On the statement "One of my favorites", players ranked Yasuo 2nd in CN and 56th in NA. On "Fair to play against" they ranked him 23rd in CN and 137th in NA. Overall on "Gameplay, Visuals, Voice, etc.." they ranked him 1st in CN and 38th in NA.
- As an [easter egg](./easter_egg.md), in Set 4 of [Teamfight Tactics](./Teamfight_Tactics.md), Yasuo would flash a [Mastery 7](./Champion_Mastery.md) emote if he single-handedly defeats the enemy team as the only remaining unit. This references the community stereotype of Yasuo players loving to flash their Mastery in League of Legends matches.
  - A video demonstration of this can be seen [here](https://www.youtube.com/watch?v=hj-SUl-mc0U).

---
*This page was automatically generated from League of Legends Wiki data.*