# Template checklist — expanded

Generated: 2025-10-10T15:54:19.165Z

Batch 9 of 33 — items 161..180

## Current champion rotation/data

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ACurrent%20champion%20rotation%2Fdata%2Fdoc.txt
- Decoded name: Template:Current champion rotation/data/doc
- Namespace: Template
- Remainder: Current champion rotation/data/doc
- Path parts: Template:Current champion rotation / data / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

== Usage ==

== Syntax ==

== See also ==

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Champion rotation templates]]
[[de:Vorlage:Aktuelle Championrotation/var]]
</includeonly>
```

### Notes

_No notes specified._

---

## Current item data

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ACurrent%20item%20data%2Fdoc.txt
- Decoded name: Template:Current item data/doc
- Namespace: Template
- Remainder: Current item data/doc
- Path parts: Template:Current item data / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
: Retrieves item data from [[Module:ItemData]]. 
;Description
: This template retrieves various item values. It is the interface for [[Module:ItemData/getter]].

;Syntax
:<code>{{t|Current item data|item|datatype|output|prepend|append|separator|t_name|index}}</code>
:<code>{{t|cid|item|datatype|output|prepend|append|separator|t_name|index}}}</code>

;Parameters
: '''1/item''' - Required
: '''2/datatype''' - Required
: '''3/output''' - Required if datatype is a table, otherwise not needed
For use with output=custom
: '''prepend''' - Optional
: '''append''' - Optional
: '''separator''' - Optional
: '''index''' - Optional
For use with output=template
```

### Notes

_No notes specified._

---

## Current maintenance data

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ACurrent%20maintenance%20data%2Fdoc.txt
- Decoded name: Template:Current maintenance data/doc
- Namespace: Template
- Remainder: Current maintenance data/doc
- Path parts: Template:Current maintenance data / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: This template retrieves various version maintenance values. It is the interface for [[Module:Maintenance data/data]] which needs to be updated on a regular basis; see its page for details on the format of the parameters this template handles.

;Usage
: To retrieve data, simply enter one of the parameters in the following syntax:<br>
:: <code>{{t|Current maintenance data|Parameter}}</code><br> OR
:: <code>{{t|cmd|Parameter}}</code> (for short)

== List of all parameters ==
* ''Note: If the total number of champions number seems wrong, check the [{{fullurl:Special:CategoryTree|target=League_of_Legends_champion&mode=all&dotree=Show+tree}} CategoryTree] to find any stray pages.''
{| class="article-table"
!Syntax
!Current value
|-
|style="text-align:right"|<code><nowiki>{{cmd|Patch}}</nowiki></code>
|style="text-align:left"| '''{{cmd|Patch}}'''
|-
|style="text-align:right"|<code><nowiki>{{cmd|PatchId}}</nowiki></code>
```

### Notes

_No notes specified._

---

## Current Unofficial champion data

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ACurrent%20Unofficial%20champion%20data%2Fdoc.txt
- Decoded name: Template:Current Unofficial champion data/doc
- Namespace: Template
- Remainder: Current Unofficial champion data/doc
- Path parts: Template:Current Unofficial champion data / doc

```
{{Documentation subpage}}
{{Construction}}
<!-- Categories and interwikis go at the bottom of this page. -->

;Description
: This template retrieves various '''unofficial''' champion data values. It is the interface for [[Module:UnofficialChampionData/data]].

;Syntax
: ''The template's title can be shortened to "cucd"''
: <code>{{t|cucd|champion|datatype|output|prepend|append|separator|t_name|index}}</code>

;Parameters
: '''1/champname''' - Required
: '''2/datatype''' - Required
: '''3/output''' - Required if datatype in {stats, role, skill_i, skill_q, skill_w, skill_e, skill_r}, otherwise not needed
For use with output=custom
: '''prepend''' - Optional
: '''append''' - Optional
: '''separator''' - Optional
: '''index''' - Optional
```

### Notes

_No notes specified._

---

## Custom Ability bar

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ACustom%20Ability%20bar%2Fdoc.txt
- Decoded name: Template:Custom Ability bar/doc
- Namespace: Template
- Remainder: Custom Ability bar/doc
- Path parts: Template:Custom Ability bar / doc

```
{{Documentation subpage}}
;Usage
:This template is a more customizable version of [[Template:Ability bar]]. It allows you to use your own icons, stats, abilities, summoners and items.

;Syntax
Parameters:
* resource = Type of resource, e.g. mana, energy, courage, etc. Allows for custom resource with custom tooltip via:
* resourcetooltip = Custom tooltip for custom resource.
* bar = Custom resource bar color if using a custom resource ("Mana", "White", "Red", "Energy" and "*none*" available)
* maxhealth = Max health to be displayed
* maxresource = Max resource to be displayed
* currenthealth = Either a flat number or percentage of maxhealth
* currentresource = Either a flat number or percentage of maxresource
* maxqrank = Maximum number of ranks in q ("5" and "6" allowed)
* maxwrank = Maximum number of ranks in w ("5" and "6" allowed)
* maxerank = Maximum number of ranks in e ("5" and "6" allowed)
* maxrrank = Maximum number of ranks in r ("1", "2", "3", "4" and "5" allowed)
* qrank = Number of ranks in q (Number between 1 and maxqrank)
* wrank = Number of ranks in w (Number between 1 and maxwrank)
* erank = Number of ranks in e (Number between 1 and maxerank)
```

### Notes

_No notes specified._

---

## Custom ability details

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ACustom%20ability%20details%2Fdoc.txt
- Decoded name: Template:Custom ability details/doc
- Namespace: Template
- Remainder: Custom ability details/doc
- Path parts: Template:Custom ability details / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

== Usage ==
This is a template to be used on all Ability Details pages to provide consistent format. See [[Template:Ability]] for practical usage.

<pre width=50%>
{{Ability details
|targeting=
|damagetype=
|projectile=
|name=
|spelleffects=
|spelleffects-single=
|spelleffects-aoe=
|spelleffects-dot=
|spelleffects-false=
|onhiteffects=
|spellshield=
|additional=
```

### Notes

_No notes specified._

---

## Custom champion info

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ACustom%20champion%20info%2Fdoc.txt
- Decoded name: Template:Custom champion info/doc
- Namespace: Template
- Remainder: Custom champion info/doc
- Path parts: Template:Custom champion info / doc

```
{{Documentation subpage}}
;Description

This is an empty champion infobox designed for custom champions with little or no artwork. It will automatically add the page it is used on to [[:Category:Custom champions]] - a convenience feature to reduce workload on blog writers wishing to publish a champion of their own design and the moderators who oversee them.

See the [[Special:PrefixIndex/Template:Custom_champion_info|subpages]] for other infobox designs.

<pre>
 {{custom champion info <!-- or "custom champion info/Client" for the new layout -->
 | name        = Champion
 | title       = The Champion Title
 | author      = You
 | latest      = <!--optional: data of latest changes-->
 | image       = ChampionSquare.png
 | role        = <!-- a comma list, e.g. Tank,Mage -->
 <!--Rating-->
 | damage      = 1
 | toughness   = 1
 | control     = 1
 | mobility    = 1
```

### Notes

_No notes specified._

---

## Custom champion info/Client

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ACustom%20champion%20info%2FClient%2Fdoc.txt
- Decoded name: Template:Custom champion info/Client/doc
- Namespace: Template
- Remainder: Custom champion info/Client/doc
- Path parts: Template:Custom champion info / Client / doc

```
{{Documentation subpage}}
<pre> 
 {{custom champion info/Client
 | name        = Champion
 | title       = The Champion Title
 | author      = You
 | latest      = <!--optional: data of latest changes-->
 | image       = ChampionSquare.png
 | role        = <!-- a comma list, e.g. Tank,Mage -->
 <!--Rating-->
 | damage      = 1
 | toughness   = 1
 | control     = 1
 | mobility    = 1
 | utility     = 1
 | style       = 50
 | difficulty  = 1
 <!--Basics-->
 | resource    = Mana <!-- can be changed to any existing or custom resource -->
 <!--| regen_name  = only use if using a custom resource that has a custom regeneration name-->
```

### Notes

_No notes specified._

---

## Custom content icon

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ACustom%20content%20icon%2Fdoc.txt
- Decoded name: Template:Custom content icon/doc
- Namespace: Template
- Remainder: Custom content icon/doc
- Path parts: Template:Custom content icon / doc

```
{{Documentation subpage}}__NOTOC__
==Usage==
This template creates a custom content icon, using any image and customizable text. It is a shorthand alternative to the Mediawiki syntax that may be ideal so long as the resources are established.

* Icons are hyperlinked to the page specified instead of the actual file.
===Syntax===
{| class="article-table"
! Name
! Accepted parameters
! Notes
|-
| image<br/>file<br/>{{{1}}}
| custom string (filename)
| Indicates the icon to render in prefix to the text.
|-
| image<code>#</code>
| custom string (filename)
| Extra icons that follow the first. Currently supports slots 2–3.
|-
| icononly
```

### Notes

_No notes specified._

---

## Custom grouped ability

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ACustom%20grouped%20ability%2Fdoc.txt
- Decoded name: Template:Custom grouped ability/doc
- Namespace: Template
- Remainder: Custom grouped ability/doc
- Path parts: Template:Custom grouped ability / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
Displays two abilities grouped under the same ability frame.

;Syntax
:<code>{{t|Custom grouped ability|LETTER|ABILITY-1|ABILITY-2|ABILITY-1-NAME|ABILITY-1-DETAILS|ABILITY-2-NAME|ABILITY-2-DETAILS}}</code>

;Example
:<code><nowiki>{{Custom grouped ability|LETTER=Q|ABILITY-1=

{{Ability|Q
|name= Razor Strike
|icon= Divine Ascent 2.png
|description='''Adalantia''' slashes in front of her with conjured sword strikes. Can be cast 4 times within 4 seconds.
|description2={{sbc|First cast:}} '''Adalantia''' slashes to the left, dealing {{sti|{{as|physical damage}}}} to all enemies struck.
|description3={{sbc|Second cast:}} '''Adalantia''' slashes to the right, dealing {{sti|{{as|physical damage}}}} to all enemies struck.
|description4={{sbc|Third cast:}} '''Adalantia''' slashes in front of her in an X shape, dealing {{sti|{{as|physical damage}}}} to all enemies struck.
|description5={{sbc|Fourth cast:}} '''Adalantia''' slashes in front of her, sending forth a shockwave in a straight line that deals {{sti|{{as|magic damage}}}} to all enemies struck.
|leveling2= {{st|Physical damage on first cast|{{ap|15|25|35|45|55}} + {{as|(30% of AP)}}}}
```

### Notes

_No notes specified._

---

## Custom item info

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ACustom%20item%20info%2Fdoc.txt
- Decoded name: Template:Custom item info/doc
- Namespace: Template
- Remainder: Custom item info/doc
- Path parts: Template:Custom item info / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

;Description
: Infobox for showcasing custom item creations, meant to be used in userspace.
: Based on [[Template:Infobox item]].

{{Custom item info
|name    = Bob's Petal
|tier    = Basic
|ap      = 10
|cdr     = 10
|buy     = 350g
|sell    = 175g
|code    = 6847
}}

<pre>
{{Custom item info
|name    = Bob's Petal
```

### Notes

_No notes specified._

---

## Custom Loading screen

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ACustom%20Loading%20screen%2Fdoc.txt
- Decoded name: Template:Custom Loading screen/doc
- Namespace: Template
- Remainder: Custom Loading screen/doc
- Path parts: Template:Custom Loading screen / doc

```
{{Documentation subpage}}
;Usage
:Similar to [[Template:Loading screen]], but allows for custom content by taking full image links as arguments, and allows for custom tooltips.

;Syntax
{{t|Custom Loading screen|Champion name}}
:Additional parameters
*border = Displayed border, one of:
** blue (default)
** red
** silver
** gold
** platinum
** diamond
** master
** challenger
* division = Division for jewels on border (Does not affect blue, red, master or challenger).
* loading = Loading splash image file.
* championlink = Page the champion name should link to when clicked.
* summoner = Summoner name shown.
```

### Notes

_No notes specified._

---

## Dagger

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ADagger%2Fdoc.txt
- Decoded name: Template:Dagger/doc
- Namespace: Template
- Remainder: Dagger/doc
- Path parts: Template:Dagger / doc

```
{{Documentation subpage}}
;Description
Inserts a dagger icon, often used to denote deceased characters. 

;Syntax
{{t|dagger}} => {{dagger}}

;See also
{{Math symbols}}

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Character formatting templates]]
</includeonly>
```

### Notes

_No notes specified._

---

## Dead

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ADead%2Fdoc.txt
- Decoded name: Template:Dead/doc
- Namespace: Template
- Remainder: Dead/doc
- Path parts: Template:Dead / doc

```
{{Documentation subpage}}
;Description
Inserts a dagger icon, often used to denote deceased characters. Alternative of {{tl|dagger}}.

;Syntax
{{t|dead}} => {{dead}}

;See also
{{Math symbols}}

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Character formatting templates]]
</includeonly>
```

### Notes

_No notes specified._

---

## Degree

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ADegree%2Fdoc.txt
- Decoded name: Template:Degree/doc
- Namespace: Template
- Remainder: Degree/doc
- Path parts: Template:Degree / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
Use in formulas to have proper non-breaking space placement; linebreaks will only occur after an operator, and not before.

{{Math symbols}}

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Character formatting templates]]
[[de:Vorlage:Grad]]
[[pt-br:Predefinição:Grau]]
</includeonly>
```

### Notes

_No notes specified._

---

## Delete

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ADelete%2Fdoc.txt
- Decoded name: Template:Delete/doc
- Namespace: Template
- Remainder: Delete/doc
- Path parts: Template:Delete / doc

```
#REDIRECT [[Template:Ambox documentation]][[Category:Template documentation]]
```

### Notes

_No notes specified._

---

## Diminishing gold info

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ADiminishing%20gold%20info%2Fdoc.txt
- Decoded name: Template:Diminishing gold info/doc
- Namespace: Template
- Remainder: Diminishing gold info/doc
- Path parts: Template:Diminishing gold info / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: Used as a means to display a note about diminishing gold returns on the articles of Support items.

;Syntax
: <code>{{t|Diminishing gold info}}</code>
{{Diminishing gold info}}

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Data templates]]
</includeonly>
```

### Notes

_No notes specified._

---

## Disambig

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ADisambig%2Fdoc.txt
- Decoded name: Template:Disambig/doc
- Namespace: Template
- Remainder: Disambig/doc
- Path parts: Template:Disambig / doc

```
#REDIRECT [[Template:Ambox documentation]][[Category:Template documentation]]
```

### Notes

_No notes specified._

---

## Disambiguation

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ADisambiguation%2Fdoc.txt
- Decoded name: Template:Disambiguation/doc
- Namespace: Template
- Remainder: Disambiguation/doc
- Path parts: Template:Disambiguation / doc

```
#REDIRECT[[Template:Ambox documentation]][[Category:Template documentation]]
```

### Notes

_No notes specified._

---

## Divided by

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ADivided%20by%2Fdoc.txt
- Decoded name: Template:Divided by/doc
- Namespace: Template
- Remainder: Divided by/doc
- Path parts: Template:Divided by / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
Use in formulas to have proper non-breaking space placement; linebreaks will only occur after an operator, and not before.

{{Math symbols}}

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Character formatting templates]]
[[pt-br:Predefinição:Dividido por]]
</includeonly>
```

### Notes

_No notes specified._

---
