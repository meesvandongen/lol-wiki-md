# Illaoi

## Table of Contents
- [Basic Information](#basic-information)
- [Statistics](#statistics)
  - [Advanced Stats](#advanced-stats)
- [Abilities](#abilities)
- [Patch History](#patch-history)
- [Trivia](#trivia)

## Basic Information

| Attribute | Value |
|-----------|-------|
| **Champion** | Illaoi |
| **Title** | the Kraken Priestess |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2015-11-24 |
| **Release Patch** | V5.23 |
| **Roles** | Juggernaut |
| **Riot Positions** | Top |
| **External Positions** | Top |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $656.0$ | $+115.0$ | $2611.0$ |
| **Mana** | $350.0$ | $+50.0$ | $1200.0$ |
| **Health Regen** | $9.5$ | $+0.8$ | $23.1$ |
| **Mana Regen** | $7.5$ | $+0.75$ | $20.2$ |
| **Armor** | $35.0$ | $+5.0$ | $120.0$ |
| **Magic Resist** | $32.0$ | $+2.05$ | $66.8$ |
| **Attack Damage** | $65.0$ | $+-1.0$ | $48.0$ |
| **Attack Speed** | $0.625$ | $+2.5\%$ | $0.891$ |
| **Movement Speed** | $350.0$ | $+0.0$ | $350.0$ |
| **Attack Range** | $125.0$ | $+0.0$ | $125.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.625$ |
| **Attack Speed Ratio** | $0.625$ |
| **Bonus AS per Level** | $2.5\%$ |
| **Attack Windup** | $21.4\%$ |
| **Acquisition Radius** | $600 units$ |
| **Pathing Radius** | $35 units$ |
| **Selection Radius** | $100 units$ |
| **Selection Height** | $100 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Prophet of an Elder God

**Innate:** Periodically, a Tentacle will spawn next to nearby terrain. It can be killed by enemy basic attacks and will become dormant if **Illaoi** leaves it alone.

*A Tentacle only attacks through ''Illaoi's** abilities and does so by slamming the ground once, dealing physical damage to enemies hit. Whenever it hits at least one enemy champion, **Illaoi** is heal for a portion of her **missing'' health.*

**Innate:** Periodically, **Illaoi** spawns a Tentacle on the nearest terrain perpendicular to her location, if no other Tentacles are already nearby it. Tentacles fully spawn after a 2 second delay and are untargetable in the meantime. Tentacles are commanded to attack by ''Illaoi's' abilities, dealing 9 to 162 to all enemies struck, increased by 0–10 to 30. Each enemy individually reduces the slam damage by 50% for every slam they were hit by in the last $0.66$ seconds, capped at 75% less damage. Each Tentacle also heal **Illaoi** for 5% of her **missing** health if it hits at least one enemy champion. *See [Pets](#Pets) for more details about Tentacles.*

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Effect Radius** | 1000 units |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |
| **Projectile** | False |

**Notes:**

- **Illaoi** cannot spawn Tentacles on ally invulnerable turrets [https://www.youtube.com/watch?v=D8r_7zpD3tY] , in addition to neither her nor the Vessel being able to spawn them in Nexus Obelisk's attack range.
  - Even after *Nexus turrets* have lost their invulnerability, **Illaoi** still has issues spawning Tentacles on them (probably because of the nearby invulnerable *Nexus*). [https://www.youtube.com/watch?v=xJ4Jwm7jhZ8]
- Tentacles will prioritize spawning on map terrain before player-made ones.
- If multiple Tentacles hit simultaneously, the received healing is calculated iteratively based on the new amount of **missing** health, as per this formula: (**maximum** health×(10.95n), with 'n' the number of Tentacle hits.

---

### Q: Tentacle Smash

**Passive:** *Tentacle* damage is increased.

**Active:** **Illaoi** slams down a Tentacle in the target direction, dealing physical damage.

**Passive:** Tentacle damage is increased. **Active:** **Illaoi** slams down a Tentacle in the target direction. **Illaoi sight herself during the cast if there is an enemy champion nearby**.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $10/9/8/7/6$ seconds |
| **Cast Time** | $0.75$ seconds |
| **Cost** | $40/45/50/55/60$ Mana |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |
| **Projectile** | False |

**Scaling:**
- **Damage Increase:** $10/15/20/25/30$%

**Notes:**

  - 'Tentacle Smash's hitbox is fixed in front of **Illaoi**, while the indicator on the ground is fixed to the targeted direction.
- *Conqueror* will pre-emptively amplify 'Tentacle Smash's* damage by the generated *Conqueror' stack.
  - Only *Tentacle Smash* or also *I* attacks?
- The self-reveal has not been fully tested for complete confirmation. Is the ability invisible when hitting minions and there is no enemy champion nearby? Is the tip of the ability enough to reveal her? Whether it can hit a minion or not? Just if she's around the exit of the fog of war, and even if she points towards the inside of the fog of war? [https://youtu.be/JRizCjxyc5g]

---

### W: Harsh Lesson

**Active:** **Illaoi**’s next basic attack gains **bonus attack range** and causes her to dash to her target, dealing **bonus** physical damage and commanding nearby Tentacles to attack the target.

**Active:** ''Illaoi's** next basic attack within 6 seconds gains range*bonus'' range* and causes her to dash to the target's location if they're beyond her normal basic attack range. The attack deals **bonus** physical damage, with a minimum threshold, and capped at 300 against non-champions, and commands all Tentacles in range to attack the target. The minimum **bonus** damage can also be applied against turrets. *Harsh Lesson* basic attack reset ''Illaoi's' basic attack timer. *Harsh Lesson can be activated during *'Illaoi's* other abilities.*

| Attribute | Value |
|-----------|-------|
| **Cooldown** | 4 seconds |
| **Cast Time** | none |
| **Cost** | 30 Mana |
| **Targeting** | Auto |
| **Affects** | Self |
| **Damage Type** | Physical |
| **Spell Shield** | False |
| **Spell Effects** | spell |

**Scaling:**
- **Bonus Physical Damage:** $3-5$% as of target's **maximum* health*Minimum Physical Damage:** $20-60$

**Notes:**

- *Harsh Lesson* cannot be cast while *pulling* a Spirit.
- The **bonus** damage doesn't apply but it does apply spell vamp and Omnivamp.
- *Harsh Lesson* is made of two different abilities which are decided by ''Illaoi's' range to the target, being either melee or within dash range.
- The close range ability:
  - Has an uncancellable windup.
  - Can be used while ground or root. If **Illaoi** attack commands and then casts *Harsh Lesson* while *rooted*, she is able to use the close range version within its dash range.
  - Starting the attack before triggering *Guardian Angel* passive and hitting while she's in it will not cause the ***Tentacles*** to slam. [https://www.youtube.com/watch?v=3pqtZtUZMqI]
  - The attack will still hit if the target becomes untargetable [https://www.youtube.com/watch?v=cUlojKtZGF8], but will not if it death.
  - ***Tentacles*** will slam if the attack is block [https://www.youtube.com/watch?v=rTgSMOuszLU], but not if it's dodge [https://www.youtube.com/watch?v=r02dsIN6uBs] or if it blind [https://www.youtube.com/watch?v=AbeVcwWZdDs]. ** In all cases the bonus*** damage is prevented.
  - Does trigger *Hail of Blades*.
  - If the empowered attack duration ends during the attack animation, the attack won't be empowered. [https://www.youtube.com/watch?v=LrBXpAhSR_M]
  - After hitting the target, **Illaoi** can quickly cast other abilities.
  - Abilities cannot be buffered during the windup.
  - The bonus damage and Tentacle commands is prevented by spell shield. [https://www.youtube.com/watch?v=MmVGVrsNWv0]
- The dash range ability:
  - Does not have an uncancellable windup, meaning effects that can interrupt the dash will cancel the attack and put the ability on cooldown. [https://www.youtube.com/watch?v=TdO6v3SCzTw] *** That includes self effects, such as Flash. [https://www.youtube.com/watch?v=d0MDFoWYI-U]
  - Can be cast but cannot be used while ground or root because **Illaoi** loses the **bonus attack range**.
  - The attack will not hit if the target becomes untargetable [https://www.youtube.com/watch?v=JOcA7oSdlqM] , but does hit if it death. [https://www.youtube.com/watch?v=2WgdlXm07cs]
  - ***Tentacles*** will slam if the attack is block, dodge or if it blind. ** The bonus*** damage is dealt when the attack is block or dodge, but not if blind.
  - Does not trigger *Hail of Blades*. [https://www.youtube.com/watch?v=NjDAm14y1HI]
  - If the empowered attack duration ends during the dash, the buff will linger until the attack has finished.
  - After hitting the target, **Illaoi** has a short delay before she can cast other abilities. [https://www.youtube.com/watch?v=P4kviRALbtk]
  - Abilities can be buffered during the windup.
  - The bonus damage and Tentacle commands is prevented by spell shield. [https://www.youtube.com/watch?v=4z1QU2DWIvw]
  - Attack speed will desync the animation from the dash. [https://www.youtube.com/watch?v=B4TNumlko-c]
- If the target dies at the instant *Harsh Lesson* is about to hit the target, for both the close and dash version of the ability, the attack will hit, be consumed, but will not make the ***Tentacles*** slam. [https://www.youtube.com/watch?v=XMGBEXrMSio]
- The ***Tentacle*** attacking because of *Harsh Lesson* is considered to be of the same cast instance, and thus will not grant additional stacks of *Conqueror*, *Electrocute*, *Phase Rush* and Tribute.
- *Harsh Lesson* triggers *Bone Plating* and one of its plates with the same attack.
  - It also triggers *Arcane Comet* and reduces its cooldown twice.
- While the ability is active, ***Tentacles*** are commanded to attack on-hit, interacting with *Guinsoo's Rageblade* without consuming the empowered attack. [https://www.youtube.com/watch?v=PnBIVSfg9w8]

---

### E: Test of Spirit

**Active:** **Illaoi** reaches with a tendril in the target direction that stops upon the first enemy hit. If the target is a champion, she their Spirit out from them to stand before her.

*The Spirit tether to the target for a duration, dealing a portion of any damage it takes. If the Spirit is killed or the target leaves its range, the target will be slow and marked as a Vessel for a duration or until they takedown **Illaoi**, spawning Tentacles while marked.*

**Active:** **Illaoi** launches a tendril in the target direction that stops at the first enemy hit, pulling their Spirit out in front of **Illaoi** over $0.75$ seconds if they are a champion. The target is tether to the Spirit for 7 seconds, true sight them in addition to granting sight of the area around them while the tether remains. The Spirit spawns with the target's **current** health, *armor* and mr, and redirects a portion of the pre-mitigation damage received to the tethered champion. When the Spirit is killed or the target leaves tether range, the tether is severed, marking the target as a Vessel for 4 seconds and slow them by 80% for $1.5$ seconds once the tether returns back to them. Each Tentacle autonomously attacks the closest Vessel or Spirit, prioritizing the latter, once every 4.5–3.5@1–13 seconds. Enemies made a Vessel continuously spawn a Tentacle near themselves, when possible. Tentacles can spawn 300 units closer to each other than when spawned by **Illaoi**. '**Illaoi** is lockout while firing the projectile and while the target's Spirit is being pulled. She also sight herself while firing the projectile if there is an enemy champion nearby. Vessels can dispel their mark by scoring a takedown against **Illaoi**. Test of Spirit cannot be cast again until the Spirit dies.'

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $16-12$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $35-55$ Mana |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Physical damage / Magic damage / True damage (Redirected damage) |
| **Speed** | 1900 units/second |
| **Spell Shield** | Special |
| **Projectile** | Special |

**Scaling:**
- **Damage Transmission:** $25-45$% (+ 8% per 100 AD)

**Notes:**

- The self-reveal has not been fully tested for complete confirmation. Is the ability invisible when passing nearby minions and there is no enemy champion nearby? Is the tip of the ability enough to reveal her? Whether it can collide with a minion or not? Just if she's around the exit of the fog of war, and even if she points towards the inside of the fog of war? [https://youtu.be/Fq2KuW55oVs]
- Spirits are classified as clone:
  - Spirits can only be fully targeted by ''Illaoi's' team and neutral monsters. The Spirit's allies cannot manually select it as a valid target for spells and abilities, but they can still do so with automatically targeted effects.
  - Spirits take 50% damage from turrets.
  - Spirits cannot be heal nor shield (*Summon Aery* excluded).
  - The Spirit levels up when the Vessel does, gaining the target's stats per level. If the target has a resource that also increases per level (i.e mana), the Spirit's lifetime will increase instead.
- The Spirit's destination is decided based on ''Illaoi's' position and facing direction when the target is hit.
- The Spirit is untargetable while it is being pulled, and spawns with the tethered champion's default model and size. [https://www.youtube.com/watch?v=xWkLcM6Xs9I]
  - The Spirit's lifetime is sometimes $0.25$ seconds longer, while its *untargetability* is sometimes shorter, potentially causing Tentacles to hit an inconsistent amount of times, as was easily the case pre V25.15. [https://www.youtube.com/watch?v=lVAg2zjX-kY]
- Redirected damage is calculated by taking the pre-mitigation damage against the Spirit, applying 'Test of Spirit's damage modifier, and then capping it to the Spirit's **current** health. This damage will then be affected by damage modifiers (resistances and other effects) of the Vessel.
  - This order of operations allows *Test of Spirit* to deal more or less percentage of the Vessel's health than would be expected: **# Because the Spirit's modifiers can be different from the Vessel's, allowing the Spirit to have higher or lower effective health, respectively increasing or decreasing the maximum pre-mitigated damage from killing it. **# Because, while it would be expected to block excess pre-mitigated damage over what kills the Spirit, since the cap does not consider the Spirit's damage modifier, it may cause it to block damage before the Spirit is killed [https://www.youtube.com/watch?v=y2ASLYcuTXE], or in corner cases not block excess damage even if it is killed. On the other hand, 'Test of Spirit's damage modifier is unexpectedly applied before the cap instead of after, with the same effect of potentially causing the cap to not block excess damage even if the Spirit is killed, or in corner cases block damage before it is killed. **#* The addition of the two, causes an increasing difference of expected damage when the pre-mitigated damage is between Spirit current healthSpiritmod and Spirit current healthEmod.
  - **Illaoi** needs $(100-(25-45))/8×100$ **total** AD to reach 100% redirected damage. As per above, the damage is still dependent on the Spirit's and Vessel's damage modifiers and may not always be able to deal 100% of the target's **current** health as total redirected damage.
- Spell shield may only prevent the initial spirit grab and the *slow* upon becoming a Vessel. *Spell shields* will not prevent becoming a Vessel or Tentacles from attacking.
- The tendril can be blocked by projectile-destroying effects (e.g. W).
  - It will only be destroyed and not replicated against Rebuttal.
- The application of the slow missile cannot be blocked by projectile- destroying effects (e.g. W).
  - Effects that homing projectile destruction (i.e certain untargetability) will prevent the application of the slow missile.
  - The slowing missile will not be fired if the target is already untargetability when the Spirit dies.
- The spirit pull is not considered a crowd control effect and will not be blocked by cc-immune, with the exception of E.
- The projectile tentacle model is linked to ''Illaoi's' model animation and size. However gameplay wise it is not, thus the actual hitbox being different than the position, width and length of the visible model. [https://www.youtube.com/watch?v=uH8C8ByFH0Q]
  - The model will stop mid-air / vanish if **Illaoi** goes into stasis, sleep or airborne, however the actual hitbox will continue its trajectory unseen. [https://www.youtube.com/watch?v=PoLVkhvhY-c] Effect at cast time end
  - And will fire towards the same pointed location.
- If the target's health is reduced to 0 while their Spirit is being pulled out, and they do not death (i.e because of resurrection), then the Spirit will still spawn with 0 health, making it invulnerable. The target will successively become a Vessel when the Spirit's lifetime expires. [https://www.youtube.com/watch?v=cWhlnIZgSwQ]
  - If some effect regenerates the champion's health while they are *resurrecting*, and before the Spirit spawns, then the same will not apply. [https://www.youtube.com/watch?v=h5555ttplNY] [https://www.youtube.com/watch?v=oV7pBhM88Eg]
- Grabbing the Spirit of **Viego** will make the Spirit's lifetime (the white bar below the lifebar) disappear, similar to other cases in the past. [https://www.youtube.com/watch?v=N91Qshu-XgU]
- Post V12.8, does grabbing the Spirit of an *Oathsworn* make the Spirit invisible? [https://www.youtube.com/watch?v=cApbCTz_lIQ]
- **Illaoi** is the only one able to see who's been Vesseled, by adding a layer of watery effects on their model. The Vessel will also be let known it's debuffed, by having a layer of watery effects on its screen instead.
  - If the target becomes a Vessel while vanish, the layer on their model will fail to be added. [https://www.youtube.com/watch?v=udxjd11eUs4]
- The Vessel status effect is a debuff that doesn't *persist through death*.
- If the target is made a Vessel while resurrection, some *resurrections* may cleanse the Vessel debuff, while others will not. [https://www.youtube.com/watch?v=Sl1Iil8Ed70] [https://www.youtube.com/watch?v=h5555ttplNY]
- Runes consider the Spirit as a champion for their effects and stat-tracking.
  - *First Strike* may cause Tentacle damage on the *Spirit* to redirect close to 0 damage [https://www.youtube.com/watch?v=Xfr76zWezZg], similar to past cases [https://www.youtube.com/watch?v=lNxHbtbebdY].
- Transferred damage will apply on-damage effects, such as *Black Cleaver*, *Morellonomicon*, *Executioner's Calling*, *Riftmaker*, Aspect of the Dragon burn damage, and *The Darkin Scythe*.
  - Redirected damage always counts as single target for the purpose of Omnivamp.
  - Transferred damage will not doubly apply on-hit or on-spell effects versus the target (e.g. *Liandry's Torment*, *Phase Rush* stacks). These effects are only applied versus the Spirit.
  - Attacking the Vessel and its Spirit simultaneously will only apply one stack of *Black Cleaver* to the Vessel.
- When the tether is broken (whether the Spirit's lifetime ran out or the target moved out of range), the Spirit will vanish and move to ''Illaoi's' location.
  - An uncancellable windup attack may still kill the Spirit after it *vanished* to grant the gold, but without making the target a Vessel. [https://www.youtube.com/watch?v=Ph8_kGqAlOQ&t=670s]
- When the Spirit is killed, it will remain in the same position for less than a second before moving to an unknown location. This will cancel any remaining Tentacle queued attacks, for the target being out of range. [https://youtu.be/2svtm_PHEug] [https://youtu.be/dqvf_ID0UFo] [https://youtu.be/x7tXVYPXJ48]
- The Spirit is immune to stasis (i.e. Tempered Fate).
- Riposte can be used to prevent becoming a Vessel if used before the Spirit is killed [https://www.youtube.com/watch?v=2jcsdMpvZRc], or while leaving the tether's range. [https://www.youtube.com/watch?v=yI121hbf3jA]
- **Sion**’s Spirit cannot be spawned during *Glory in Death*. Spirits created before 'Sion's zombie state will remain active.
- If the target that had its Spirit pulled from them is taken into the *Realm of Death*, the tether will break, but they will not be made into a Vessel.
- The following table refers for interactions while **Illaoi** is firing the projectile and while the target's Spirit is being pulled:

---

### R: Leap of Faith

**Active:** **Illaoi** becomes displacement immunity before dealing physical damage to nearby enemies. For each enemy champion hit, a Tentacle is summoned around her for a period.

*For the duration, **Harsh Lesson*’s* cooldown is ah and nearby Tentacles attack faster.*

**Active:** **Illaoi** gains displacement immunity and leaps into the for the cast time. She then slams her idol into the ground, dealing physical damage to nearby enemies. For each enemy champion hit, a Tentacle is summoned for 8 seconds (up to a maximum of 6 summons). For the duration, **Harsh Lesson*’s* *cooldown* is halved, all Tentacles awakened by **Illaoi** are untargetable, will not show their hitboxes, nor go dormant, and will take $0.5$ seconds to make an attack; however they will then incur a lockout of $0.5$ seconds before being able to start a new attack. *This does not affect the Tentacle attack from *Tentacle Smash*.*

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $120-70$ seconds |
| **Cast Time** | $0.5$ seconds |
| **Cost** | 100 Mana |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Effect Radius** | 500 units |
| **Spell Shield** | Special |
| **Spell Effects** | spellaoe |

**Scaling:**
- **Physical Damage:** $150-350$ bonus AD)

**Notes:**

- **Illaoi** spawns the **Tentacle**** 700 range away from her, unless there's a natural wall blocking the way making the ***Tentacle*** spawn on it, with an angle relative to the direction she's facing.
  - 1+ champions: one *Tentacle* at 0 degrees. (towards ''Illaoi's' facing direction)
  - 2+ champions: one *Tentacle* at 60 degrees.
  - 3+ champions: one *Tentacle* at 300 degrees.
  - 4+ champions: one *Tentacle* at 120 degrees.
  - 5+ champions: one *Tentacle* at 240 degrees.
  - 6+ champions: one *Tentacle* at 180 degrees.
  - ***Tentacles** may spawn on base barriers. If Illaoi*** is in the center of a base barrier, all ***Tentacles*** will spawn in the same position and stack over each other, making it appear as if there was only one ***Tentacle***. [https://www.youtube.com/watch?v=Ym0dCT7RmsE]
  - ''Illaoi's' facing direction is decided by the position of the cursor on cast, but can also be changed mid-cast through other effects, such as Flash. [https://www.youtube.com/watch?v=5g08ajBml2U]
  - If there are no walls nearby, spawning the last *Tentacle* will form a perfect hexagon.
- Spell shield will block the ability's damage but will not prevent ***Tentacles*** from spawning.
- If **Illaoi** death within $0.25$ seconds of dropping down, she will spawn buffed ***Tentacles***, but she won't manage in time to buff the already spawned ones. [https://www.youtube.com/watch?v=rq3eihP2wdE]
- If **Illaoi** is near a ***Tentacle*** that is about to spawn, it will spawn buffed by *Leap of Faith* even if **Illaoi** is no longer nearby.
- Using **Harsh Lesson** after **Illaoi** drops down will command ***Tentacles*** to instantly attack, regardless if they were already attacking. Their old attack will keep on going and will hit while the second attack is on its animation attack. [https://www.youtube.com/watch?v=L6bpl9fngHA]
  - This makes it possible for the same tentacle to reduce its own damage by having the second attack hit too fast, which is not otherwise possible. [https://www.youtube.com/watch?v=hBBIQuzM8Xs]

---

## Patch History

### V25.18
- *Prophet of an Elder God*
  - **New Effect:** Tentacles will no longer spawn during the channel or dash of Teleport / Unleashed Teleport.
    - Tentacle spawning is now deferred until their cast ends.
- *Harsh Lesson*
  - AD ratio reduced to $3.5$% per 100 AD from 4%.
- *Test of Spirit*
  - *Vessel* debuff duration increased to 4 seconds from 3.
  - **Undocumented:** The *Vessel* no longer has a 5–3@1–13 second lockout for spawning successive *Tentacles*.
    - The only condition for a new Tentacle to spawn is now range between them.

### V25.16
- *Prophet of an Elder God*
  - Tentacle AD ratio increased to 110% AD from 105% AD.
  - Spawn cooldown reduced to 18 to 7 seconds from 20 to 7.25.

### V25.15
- General
  - All tooltip locations where values related to *Tentacles* are noted will now properly update with changes to their actual values.
- *Prophet of an Elder God*
  - Tentacle AD ratio increased to 105% AD from 100% AD.
- *Tentacle Smash*
  - **Bug Fixes:** Corrected AD ratio in the tooltip.
- *Test of Spirit*
  - *Vessel* debuff duration reduced to 3 seconds from 10.
  - Automatic Tentacle slam cooldown against *Spirits* and *Vessels* reduced to 4.5–3.5@1–13 from 5.5–3.5@1–13.
  - **Removed:*** There no longer exists a lockout of $1.1$ seconds before the *Vessel* can start spawning the first Tentacle.
  - **Undocumented:** Tentacles now stay awake if there is a *Spirit* in range, even if **Illaoi** or a *Vessel* is not around, allowing them to continue attacking targets.

### V25.09
- *Prophet of an Elder God*
  - **Bug Fixes:** Tentacle slams no longer apply their damage through different dimensions (e.g. Realm of Death).
- *Test of Spirit*
  - **Bug Fixes:** VO for when an enemy's spirit is active no longer unintentionally plays for allies as well.

### V14.22
- Stats
  - Base attack damage reduced to 65 from 68.
  - Mana growth reduced to 50 from 60.

### V14.4
- *Prophet of an Elder God*
  - Tentacle AD ratio reduced to 100% AD from 115% AD.
- *Test of Spirit*
  - Tether now innately checks every $0.05$ seconds for valid range conditions, and no longer waits for every 8th server tick.
  - **Bug Fixes:** Tentacle attack call range has been increased to match *Harsh Lesson*’s.

### V14.3
- *Prophet of an Elder God*
  - Tentacle range increased to 925 from 800.
  - Tentacle base damage reduced to 9 to 162 from 10 to 180.
  - AD ratio reduced to 115% AD from 120% AD.
- *Harsh Lesson*
  - **Undocumented:** Tentacle call range increased to 950 from 800.
  - **Undocumented:** The dash version of the ability now properly causes the basic attack to critical strike with *Sundered Sky*.
- *Test of Spirit*
  - **Undocumented:** Tentacle call range increased to 900 from 800.

### V14.2
- Stats
  - Health growth increased to 115 from 109.
  - Base mana increased to 350 from 300.
  - Mana growth increased to 60 from 50.
- *Prophet of an Elder God*
  - Heal per tentacle hit increased to 5% **missing** health from $4.5$%.
- *Test of Spirit*
  - **Bug Fixes:** Spirits of champions without a resource now have the proper duration.

### V13.20
- *Test of Spirit*
  - **Bug Fixes:** The spirit out of an enemy **Briar** no longer instantly reaches the end of its life duration if the clone was created while she was not under the effect of her *Blood Frenzy*.
    - Now bugged in this same way with resourceless champions such as **Riven** and **Garen** instead.
  - ***Undocumented - Estimated:*** Grabbing the spirit of champions with special resource bars (i.e Flow, Ferocity) no longer makes the spirit not show its lifetime.

### V12.15
- *Test of Spirit*
  - **Undocumented:** Spirits now properly cause turret to *Ohmwrecker (Turret Item)* their attacks.

## Trivia

- Illaoi has the 2nd highest (153), behind **Mega Gnar**.
- She is the only champion who can summon an enemy unit (*Vessel*).

---
*This page was automatically generated from League of Legends Wiki data.*