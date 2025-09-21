# Karma

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
| **Champion** | Karma |
| **Title** | the Enlightened One |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2011-02-01 |
| **Release Patch** | V1.0.0.110 |
| **Roles** | Burst, Enchanter |
| **Riot Positions** | Middle, Support |
| **External Positions** | Top, Middle, Support |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $630.0$ | $+109.0$ | $2483.0$ |
| **Mana** | $374.0$ | $+40.0$ | $1054.0$ |
| **Health Regen** | $5.5$ | $+0.55$ | $14.9$ |
| **Mana Regen** | $13.0$ | $+0.8$ | $26.6$ |
| **Armor** | $28.0$ | $+5.0$ | $113.0$ |
| **Magic Resist** | $30.0$ | $+1.3$ | $52.1$ |
| **Attack Damage** | $51.0$ | $+3.3$ | $107.1$ |
| **Attack Speed** | $0.625$ | $+2.3\%$ | $0.869$ |
| **Movement Speed** | $335.0$ | $+0.0$ | $335.0$ |
| **Attack Range** | $525.0$ | $+0.0$ | $525.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.625$ |
| **Attack Speed Ratio** | $0.625$ |
| **Bonus AS per Level** | $2.3\%$ |
| **Missile Speed** | $1500 units/second$ |
| **Acquisition Radius** | $525 units$ |
| **Pathing Radius** | $35 units$ |
| **Selection Radius** | $120 units$ |
| **Selection Height** | $145 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Gathering Fire

**Innate:** **Mantra*’s* current cooldown is cdr by a few seconds for each enemy champion hit by **Karma**’s damaging abilities.

**Innate:** **Mantra*’s* **current cooldown** is reduced by 4 seconds for each enemy champion hit by ''Karma's' damaging abilities.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Self |

**Notes:**

- *Inner Flame* / *Soulflare* reduces *Mantra*’s cooldown per champion hit by either damage instance (up to a maximum total reduction of $4×10$ seconds if 5 enemy champions are hit by both instances of damage).
- *Focused Resolve* / *Renewal* reduces *Mantra*’s cooldown when initially cast and again after the duration for the tether has expired (for a total reduction of $4×2$ seconds).

---

### Q: Inner Flame

**Active:** **Karma** fires a bolt in the target direction that explodes on the first enemy hit, dealing magic damage to nearby enemies and briefly slow them.

**Active:** **Karma** fires a bolt in the target direction that explodes on the first enemy hit, dealing magic damage to nearby enemies and slow them by 40% for $1.5$ seconds.

| Attribute | Value |
|-----------|-------|
| **Range** | 950 / er 890 units |
| **Cooldown** | $9-5$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $40-80$ Mana |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Speed** | 1700 units/second |
| **Effect Radius** | 280 units |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |
| **Projectile** | True |

**Scaling:**
- **Magic Damage:** $60-260$ (+ 70% AP)

**Notes:**

- Spell shield will block the initial impact from *Inner Flame*. Effect at cast time end
- 'Inner Flame's effect radius is centered around the location of the missile as it collides.

---

### Q: Soulflare

***Mantra* *Inner Flame* deals increased damage, and fires a larger bolt that also explodes at max range.

*The explosion briefly creates a field that slow enemies, which then ruptures to deal magic damage.*

***Mantra* *Inner Flame* deals increased damage, and fires a larger bolt that also explodes at maximum range. The explosion creates a field for $1.5$ seconds that slow enemies within by 50%, which then ruptures to deal magic damage. *Soulflare* scales with **Mantra*’s* rank.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | 280 units |
| **Spell Shield** | Special |
| **Spell Effects** | spellaoe |
| **Projectile** | True |

**Scaling:**
- **Bonus Magic Damage:** *$40-220 4$ (+ 30% AP)*) (+ $70+30$% AP)
- **Magic Damage:** $40-310 4$ (+ 50% AP)

**Notes:**

- Spell shield will block the initial impact from either ability but will not block 'Soulflare's field damage.
- 'Soulflare's detonation radius is centered around the location of the missile as it collides, while the lingering field will always be created at the impacted enemy's center instead.
- *Soulflare* will cast from wherever **Karma** is at the end of the cast time.

---

### W: Focused Resolve

**Active:** **Karma** deals magic damage and tether to the target enemy champion, monster or pet.

*If the tether is not broken after a short time, the target is dealt magic damage, briefly root, and true sight for a short time.*

**Active:** **Karma** deals magic damage to the target enemy champion, monster or pet and forms a tether between her and them for 2 seconds, during which they are true sight. If the tether is not broken by the end of its duration, the target is dealt the same magic damage again and is root for a duration, during which they are true sight.

| Attribute | Value |
|-----------|-------|
| **Range** | 675 units |
| **Cooldown** | 12 seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $50-70$ Mana |
| **Targeting** | Unit |
| **Affects** | Enemies / Self |
| **Damage Type** | Magic |
| **Spell Shield** | Special |
| **Spell Effects** | spell |
| **Projectile** | False |

**Scaling:**
- **Magic Damage:** $40-140$ (+ 45% AP)
- **Total Magic Damage:** $40×2-140×2$ (+ 90% AP)
- **Root Duration:** $1.6-2$ seconds

**Notes:**

- Spell shield will block the tether's application and damage but not the aftereffects of one already applied.

---

### W: Renewal

***Mantra* 'Focused Resolve's root is increased.

**Karma** heal herself once on-cast, and again once the target is *bound* or dies while tethered.

***Mantra* 'Focused Resolve's root duration is increased. **Karma** heal for health*missing** health* once on-cast, and again once the tether lasts its full duration or the target dies while tethered. *Renewal* scales with **Mantra*’s* rank.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Unit |
| **Affects** | Enemies / Self |
| **Damage Type** | Magic |
| **Spell Shield** | Special |
| **Spell Effects** | spell |
| **Projectile** | False |

**Scaling:**
- **Root Duration Increase:** *$0.5-1.25 4$ seconds*) seconds

**Notes:**

- Spell shield will block the tether's application and damage but not the aftereffects of one already applied.

---

### E: Defiance

***Mantra* 'Inspire's* shield is increased, and *Inspire' spreads to surrounding allied champions at reduced power.

***Mantra* *Inspire* grants a bonus amount of shielding. *Inspire* spreads to surrounding allied champions, granting them a shield for $2.5$ seconds and *ms for 2 seconds. *Defiance* scales with **Mantra*’s* rank.

| Attribute | Value |
|-----------|-------|
| **Cast Time** | none |
| **Targeting** | Unit |
| **Affects** | Allies |
| **Effect Radius** | 700 units |

**Scaling:**
- **Bonus Primary Target Shield:** $ (+ $) (+ $60+
- **Secondary Target Shield:** *$ 4 (+ $% AP)

**Notes:**

- *Defiance* has a forgiveness radius of 175 units.

---

### E: Inspire

**Active:** **Karma** applies a shield on herself or the target allied champion for a short time, which briefly grants *ms **bonus** movement speed*.

**Active:** **Karma** grants a shield to herself or the target allied champion for $2.5$ seconds as well as ms*bonus** movement speed* for 2 seconds.

| Attribute | Value |
|-----------|-------|
| **Range** | 800 units |
| **Cooldown** | $10-8$ seconds |
| **Cast Time** | none |
| **Cost** | $50-70$ Mana |
| **Targeting** | Unit |
| **Affects** | Allies |

**Scaling:**
- **Shield Strength:** $80-280$ (+ 60% AP)

**Notes:**

- *Inspire* has a forgiveness radius of 175 units.

---

### R: Mantra

**Active:** **Karma** empowers her next ability within a period to apply an additional effect.

**Active:** **Karma** empowers her next basic ability within 8 seconds for an additional effect. *Mantra can be used while affected by cast-inhibiting crowd control.* **Karma** begins the game with one rank in *Mantra*. Her empowered abilities scale based on 'Mantra's rank.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $40-34 4$ seconds |
| **Cast Time** | none |
| **Targeting** | Auto |
| **Affects** | Self |

**Notes:**

No additional notes.

---

## Patch History

### V25.13
- General
  - **Bug Fixes:** The symbol on her cape piece above her now properly plays its animations after the player has reconnected.

### V25.06
- *Inner Flame*
  - Base damage reduced to $60-260$ from $70-270$.
- *Soulflare*
  - **Bug Fixes:** Now properly benefits from *Axiom Arcanist*.

### V14.24
- *Mantra*
  - **Bug Fixes:** No longer reveals the user if cast while the cursor is near an enemy champion.

### V14.20
- *Inspire*
  - **New Effect:** Cast now has a forgiveness radius of 175 units.
- *Defiance*
  - **New Effect:** Cast now has a forgiveness radius of 175 units.

### V14.13
- *Mantra*
  - **Bug Fixes:** Cooldown is now refunded if *Soulflare* kills a target.

### V14.12
- *Inspire*
  - Bonus movement speed duration increased to 2 seconds from $1.5$.
  - Base shield increased to $80-280$ from $80-260$.
- *Defiance*
  - Bonus base shield increased to $50-200 4$ from $50-170 4$.

### V14.9
- *Inner Flame*
  - Slow increased to 40% from 35%.
- *Defiance*
  - Secondary target bonus movement speed increased to 15% from 12%.
  - Secondary target shielding increased to 100% of bonus shield from 90%.
    - Secondary target base shield increased to $50-170 4$ from $50×0.9-170×0.9 4$.
    - Secondary target AP ratio increased to 45% AP from $45×0.9$% AP.

### V14.7
- *Focused Resolve*
  - Root duration increased to $1.6-2$ seconds from $1.4-2$.
- *Defiance*
  - Secondary target total shield strength changed to 90% of 'Defiance's* bonus shield from 30% of *Inspire*’s base shield plus 30% of *Defiance's bonus shield.
    - Secondary target base shield strength changed to $50×0.9-170×0.9 4$ from $80×0.3-260×0.3$ (+ *$50×0.3-170×0.3 4$*).
    - Secondary target AP ratio increased to $45×0.9$% AP from $(45+60)*0.3$% AP.

### V14.6
- Stats
  - Mana growth reduced to 40 from 50.
  - Mana regeneration growth increased to $0.8$ from $0.5$.
- *Gathering Fire*
  - Cooldown reduction changed to 4 seconds from 5.
- *Inner Flame*
  - Mana cost changed to $40-80$ from 45 at all ranks.
- *Defiance*
  - Bonus base shield changed to $50-170 4$ from $25-175 4$.
- *Mantra*
  - Cooldown increased to $40-34 4$ seconds from $40-31 4$.

### V14.4
- *Focused Resolve*
  - Tether now innately checks every $0.05$ seconds for valid range conditions, and no longer waits for every 8th server tick.
- *Renewal*
  - Tether now innately checks every $0.05$ seconds for valid range conditions, and no longer waits for every 8th server tick.

## Trivia

- Karma was named after Karma.
  - Her title, *The Enlightened One*, references Gautama Buddha & Enlightenment in Buddhism.
- *Mantra* was named after Mantra, from Proto-Indo-Iranian language **mántram* < Proto-Indo-European language *Tt "instrument of thought".
  - The 'translation' for *Mantra*’s Ancient Ionian language was provided by Player Support Escalation Specialist 'Marowe'.
- Karma was the first champion to feature an 'Art Spotlight' before release and the first to be fully relaunched.
  - She was the fifth to have more than four abilities (the others being **Elise**, **Gnar**, **Heimerdinger**, **Jayce**, **Lee Sin**, and **Nidalee**).
- Like **Jayce**, Karma used to have ranks on each basic ability and only one on her *ultimate*.
- *Renewal*’s root has the longest duration in-game ($3.25$ seconds with *Focused Resolve* and *Mantra* both fully-ranked).
- In the minigame Astro Teemo, the most expensive upgrade in the game that **Teemo**’s can purchase, Soul Shield, is a reference to one of Karma's pre-rework *skills* of the same name.
- In Karma's most current lore, Karma's spirit has lived countless lives only to be re-incarnated within new bodies, inspired by Reincarnation in Indian religions.
  - The name of her current incarnation, *Darha, comes from Sanskrit adjective < ", steadfast", sharing Proto-Indo-European language root *dʰer-* "to uphold" with **Darius**.
- Karma was the fourth champion to feature two 'Champion Spotlights' due to significant gameplay changes (the others being **Ezreal**, **Katarina**, **Lee Sin**, and **Sivir**)
- *Inspire*’s speed buff reads “Vroom Vroom!”.

---
*This page was automatically generated from League of Legends Wiki data.*