# Vi

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
| **Champion** | Vi |
| **Title** | the Piltover Enforcer |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2012-12-19 |
| **Release Patch** | V1.0.0.153 |
| **Roles** | Diver |
| **Riot Positions** | Jungle |
| **External Positions** | Jungle |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $655.0$ | $+99.0$ | $2338.0$ |
| **Mana** | $295.0$ | $+65.0$ | $1400.0$ |
| **Health Regen** | $10.0$ | $+1.0$ | $27.0$ |
| **Mana Regen** | $8.0$ | $+0.65$ | $19.1$ |
| **Armor** | $30.0$ | $+4.7$ | $109.9$ |
| **Magic Resist** | $32.0$ | $+2.05$ | $66.8$ |
| **Attack Damage** | $63.0$ | $+3.0$ | $114.0$ |
| **Attack Speed** | $0.644$ | $+2.0\%$ | $0.863$ |
| **Movement Speed** | $340.0$ | $+0.0$ | $340.0$ |
| **Attack Range** | $125.0$ | $+0.0$ | $125.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.644$ |
| **Attack Speed Ratio** | $0.644$ |
| **Bonus AS per Level** | $2.0\%$ |
| **Attack Windup** | $22.5\%$ |
| **Acquisition Radius** | $600 units$ |
| **Pathing Radius** | $35 units$ |
| **Selection Radius** | $100 units$ |
| **Selection Height** | $100 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Blast Shield

**Innate:** Periodically, **Vi**’s next ability hit will generate a shield by a portion of her **maximum** health for a short time.

*The cooldown is cdr each time **Denting Blows** is triggered.*

**Innate:** Periodically, ''Vi's** next ability hit grants her a shield equal to 12% of her **maximum'' health for 3 seconds. 'Blast Shield's* *cooldown* is reduced by 4 seconds each time **Denting Blows*' is consumed.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Self |

**Notes:**

- **Denting Blows** will not activate *Blast Shield*.

---

### Q: Vault Breaker

**Active:** **Vi** charges up to a few seconds to increase the *range*, speed, and damage of her punch. *Vault Breaker* can be recast within the duration and cancels automatically afterwards.

**Recast:** **Vi** dashes in the target direction to deal physical damage to enemies hit based on charge time. Non-champions hit are airborne towards her.

**Active:** **Vi** channel while being slow by 15% for up to 4 seconds to increase 'Vault Breaker's *range*, speed, and damage over the first $1.25$ seconds of the channel. *Vault Breaker* can be recast within the duration. If the charge completes without reactivation, *Vault Breaker* is cancelled, mana is refunded, and it is placed on cd. If the charge is interrupt, mana is refunded and it is placed on a cd. **Recast:** **Vi** dashes in the target direction to deal physical damage to enemies within her path, increased by type=channel time, and airborne all non-champions hit towards her. She stops upon hitting an enemy champion, airborne over $0.75$ seconds. **Relentless Force* and *Cease and Desist* can be cast during the dash.*

| Attribute | Value |
|-----------|-------|
| **Range** | type=channel time |
| **Cooldown** | $12-6$ seconds |
| **Cast Time** | none |
| **Cost** | $50-90$ Mana |
| **Targeting** | Auto / Direction |
| **Affects** | Enemies, Self |
| **Damage Type** | physical |
| **Speed** | channel time |
| **Spell Shield** | True |
| **Spell Effects** | aoe |

**Scaling:**
- **Minimum Physical Damage:** $40-120$ (+ 60% bonus AD)2.5-120×2.5$ (+ $60×2.5$% bonus AD)

**Notes:**

- 'Vault Breaker's dash range extends up to 50 units when going through a wall.
- Targets will be hit as soon as they come into contact with **Vi**.
- The knockback direction will be toward ''Vi's' facing direction regardless of where she hit the target.
- Casting Flash during the dash will cause enemies at the destination to be hit by *Vault Breaker*.
  - As Flash updates ''Vi's' facing direction, affected champions will be knocked back in the direction the summoner spell was cast in.
- **Vi** may cast spells that change her facing direction during the dash, including certain item actives and her own *R*, to knock champions she collides with in a different direction.
  - Items that also initiate their own dash *immediately* without delay can only redirect targets struck at 'Vault Breaker's maximum range. These items include: *** *Hextech Rocketbelt*
- Stuck targets (including those that block *Vault Breaker* with a spell shield) are briefly immune to being affected again by *Vault Breaker* from this player, for about $1.25$ seconds.
- The following table refers for interactions while **Vi** is channel:

---

### W: Denting Blows

**Passive:** **Vi**’s basic attacks and **Vault Breaker** apply a stack of *Denting Blows*.

*The third stack consumes them all to deal **bonus** physical damage based on the target's **maximum** health and inflict armor for a few seconds. **Vi** will also gain *as *bonus attack speed* for a few seconds.*

**Passive:** ''Vi's* basic attacks on-hit and **Vault Breaker** apply a stack of *Denting Blows' to enemies hit for 4 seconds, refreshing on subsequent applications and stacking up to 3 times. The third stack consumes them all to deal **bonus** physical damage, capped at 300 against non-champions, and inflict armor penetration for 4 seconds. After consuming *Denting Blows*, **Vi** gains *as *bonus attack speed* for 4 seconds, which refreshes on subsequent triggers.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | False |
| **Spell Effects** | Proc |

**Scaling:**
- **Bonus Physical Damage:** $4-8$%
- **Bonus Attack Speed:** $30-50$%

**Notes:**

- The armor penetration will apply after the triggering damage.
- The enhanced attack applies other on-hit effects and can both critical strike as normal as well as apply life steal (the bonus damage cannot do either).
- *Denting Blows* can be blocked but cannot be dodged nor missed while **Vi** is blind.
- The attacks do not affect structures nor wards.

---

### E: Relentless Force

**Active:** **Vi**’s next basic attack gains *range **bonus** range* and deals *modified physical damage to enemies hit in a cone.

**Vi** periodically stocks a charge of *Relentless Force*, up to a maximum of 2.

**Active:** **Vi** empowers her next basic attack within 6 seconds to have an uncancellable windup, gain range*bonus** range* and trigger a blast in the target's direction that deals *modified physical damage to enemies hit in a cone. *Relentless Force can critically strike for critical damage*bonus** physical damage against the primary target. Secondary targets take the same damage but do not count as being critically struck. **Vi** periodically stocks a *Relentless Force* charge, up to a maximum of 2. *Relentless Force basic attack reset *'Vi's* basic attack timer.*

| Attribute | Value |
|-----------|-------|
| **Recharge** | $12-8$ seconds |
| **Cast Time** | none |
| **Cost** | $26-50$ Mana + 1 charge |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | physical |
| **Effect Radius** | 600 units |
| **Spell Shield** | True |
| **Spell Effects** | Special |

**Scaling:**
- **Physical Damage:** $10-90$

**Notes:**

- Applies spell damage to the primary target, applies basic damage required effects on the primary target, and applies area damage to secondary targets.
- The empowered attack has the same windup time as ''Vi's' basic attacks.
- *Relentless Force* will only apply a stack of *Denting Blows* to the primary target.
- If the *Relentless Force* buff is about to run out as **Vi** starts the attack windup, the duration will be updated and extended to allow **Vi** to still sucessfully attack.

---

### R: Cease and Desist

**Active:** **Vi** true sight the target enemy champion and dashes to them with displacement immunity, dealing physical damage to enemies she passes through and briefly airborne and stun them.

*Upon reaching the target, she deals physical damage and briefly airborne.*

**Active:** **Vi** singles out the target enemy champion, dash with displacement immunity towards them and true sight them. Upon approaching within 300 units of the target, she dashes through and grabs them over $1.05$ seconds, airborne for $1.3$ seconds and dealing physical damage after $0.75$ seconds into the grab duration. Enemies she dashes through are dealt the same damage, airborne by 350 units over $0.25$ seconds, and stun for $0.75$ seconds.

| Attribute | Value |
|-----------|-------|
| **Range** | 800 units |
| **Cooldown** | $140-90$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | 100 Mana |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Speed** | 800 units/second |
| **Spell Shield** | True |
| **Spell Effects** | spell |

**Scaling:**
- **Physical Damage:** $150-350$ bonus AD)

**Notes:**

- **Vi** will track the target if they change locations.
  - She will stop dashing if the target is too far away or moves beyond 1500 units.
- **Vi** will land to the opposite of the target from the dash after the grab ends.
  - She will not dash through the target if she would not be able to land to the opposite of them, such as by terrain.
- Secondary targets will be affected as soon as they come into contact with **Vi**, and cannot be affected by this again for ~4 seconds.
- The airborne debuff cannot be overridden.
- The dash will not be overridden if **Vi** blink during it.
- ''Vi's' displacement immunity and the target's true sight both last until the grab ends.
  - If the dash ends from entering resurrection or the target being too far away, the displacement immunity will remain for 7 seconds, and will otherwise be lost.
- The dash speed increases with time travelled. ** This will also occur for when **Vi** enters resurrection during the cast time.
- If the target becomes untargetable during the dash, **Vi** ends the dash prematurely and moves towards the target briefly, ignoring any movement commands. If **Vi** enters grabbing range, she will apply the airborne effect but not deal any damage.
- *Cease and Desist* will cancel and go on full cooldown if **Vi** is affected by *Fear Beyond Death's* recast during the cast time.
- Displacement immunity will not resist the application of the stun.
- The following table refers for interactions while **Vi** is dash and has grabbed the target:
  - Flash cannot be used during the grab.
  - This lockout is removed as soon as the grab ends, after which the the target's remaining airborne time will be $0.25$ seconds.

---

## Patch History

### V25.13
- *Blast Shield*
  - Health ratio reduced to 12% **maximum** health from 14%.

### V25.12
- *Denting Blows*
  - Base damage reduced to $4-8$% of the target's **maximum** health from $4-10$%.
  - Bonus AD ratio increased to $3.5$% per 100 *bonus AD from $2.857$%.
  - Bonus attack speed reduced to $30-50$% from $30-60$%.
- *Relentless Force*
  - Base damage increased to $10-90$ from $0-60$.
  - AD ratio reduced to 110% AD from 120% AD.

### V25.11
- *Cease and Desist*
  - Cast time is now cancelled if the caster enters resurrection during it.
    - **Note: Vi is no longer able to successfully cast the ability after her resurrection effect is triggered during the cast time, which allowed her to perform the ability as normal but under the revival state.**

### V25.10
- *Vault Breaker*
  - Minimum base damage reduced to $40-120$ from $45-145$.
  - Minimum bonus AD ratio reduced to 60% *bonus AD from 80%.
  - Maximum damage increased to 250% from 200%.
    - Maximum base damage increased to $40×2.5-120×2.5$ from $45×2.0-145×2.0$.
    - Maximum bonus AD ratio reduced to $60×2.5$% *bonus AD from $80×2.0$%.
  - **Bug Fixes:** Improved the responsiveness of the cooldown refund when the charge is interrupted by disables.
- *Relentless Force*
  - **Bug Fixes:** Critical **bonus** physical damage reduced to 75% AD from 100% AD.
- *Cease and Desist*
  - Base damage reduced to $150-350 3$ from $150-400 3$.
  - Mana cost reduced to 100 at all ranks from $100-150 3$.

### V25.09
- *Denting Blows*
  - **Bug Fixes:** Restored SFX for when the cooldown elapses on all skins.

### V14.24
- Vi
  - Renamed to *Arcane Undercity* from *Arcane*.

### V14.22
- *Blast Shield*
  - Shield health ratio increased to 14% of her **maximum** health from 12%.

### V14.18
- *Vault Breaker*
  - Minimum base damage reduced to $45-145$ from $50-150$.
    - Maximum base damage reduced to $45×2-145×2$ from $50×2-150×2$.

### V14.16
- *Vault Breaker*
  - Minimum base damage increased to $50-150$ from $45-145$.
    - Maximum base damage increased to $100-300$ from $90-290$.

### V14.10
- *Vault Breaker*
  - **Bug Fixes:** Charge VFX on her model no longer sometimes disappears.

## Trivia

- Vi stands for Violet.
- Vi performs the 'Ali Shuffle' while taunting.
- Vi's dance references the Dougie dance.
  - A side-by-side comparison can be seen here.
- Vi and **Caitlyn** both gain On the Case ('"Piltover's Finest"') when they find themselves on the same team.
- Having **Jinx* on the opposing team gives Vi or per Criminal (Jinx) Apprehended (Takedown).
- An enemy **Caitlyn** taunted by Vi will gain the Agitated cosmetic debuff (*"How agitating."*).
- Vi's theme was sung by Nicki Taylor from Running the Risk.
- Vi's name is the same as the Roman numerals, which are tattoed on her cheek.
- *Vi* is the shortest champion name at 2 characters long.
- Keeping with her criminal-turned-law enforcer theme, several of Vi's abilities are named after or reference various types of real-life criminal charges.
  - *Vault Breaker* references the act of breaking into a bank vault, a typical course of action for burglary.
  - *Cease and Desist* is named after the cease and desist which typically is a document sent to an individual or business to stop allegedly illegal activity.
- *Vault Breaker* and *Cease and Desist* apply all airborne variations combined (*knock back*, *knock up*, *knock aside*)
  - *Cease and Desist* was conceived from *Vault Breaker* when enhanced by using Vi's very first ultimate.
- Vi, **Blitzcrank**, **Caitlyn**, **Lissandra**, **Rumble**, **Sion**, **Varus**, **Viego**, **Xerath**, and **Ziggs** are the only champions who can apply crowd control on themselves.
- A light on Vi's back turns on when *Blast Shield* is off cooldown.
- Vi was the first champion revealed through a forum thread.
- Vi, **Zed**, **Sona**, and **Garen** were targeted by **Jhin**.
  - After Vi was shot, her champion icon on her League of Legends website page and champion list were updated to a gif with what seems to be energy floating up from her gauntlets.
- Vi - **Jinx** is one of seven pairs of sibling champions (the others being **Cassiopeia** - **Katarina**, **Kayle** - **Morgana**, **Nasus** - **Renekton**, **Garen** - **Lux**, **Yasuo** - **Yone**, and **Darius** - **Draven**).
  - Though not a pair, **Anivia**, **Ornn**, and **Volibear** are also siblings.
- Vi's Series 1 Eternals make the following references:
  - *Ora Ora Ora!* references Jotaro Kujo eponymous stand cry from Jojo's Bizarre Adventure.

---
*This page was automatically generated from League of Legends Wiki data.*