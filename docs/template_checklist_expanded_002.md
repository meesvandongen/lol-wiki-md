# Template checklist — expanded

Generated: 2025-10-10T15:54:19.132Z

Batch 2 of 33 — items 21..40

## Item icon

- [x] Status: done
- [x] Include decision: include

### Source (first 20 lines)

- Source filename: Template%3AItem%20icon%2Fdoc.txt
- Decoded name: Template:Item icon/doc
- Namespace: Template
- Remainder: Item icon/doc
- Path parts: Template:Item icon / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

;Usage
:This template will make a link to an item out of the square and name.

;Syntax
:{{t|ii|Item|Custom Name}}

;Example
:{{tl|ii|Phage}}
::{{ii|Phage}}

:{{tl|ii|Zeal|Swords}}
::{{ii|Zeal|Swords}}

:{{tl|ii|Stalker's Blade|Runic Echoes}}
::{{ii|Stalker's Blade|Runic Echoes}}

:{{tl|ii|Phage|3=number=2}}
```

### Notes

Creates an icon-link for items (inline item icon + link); include to preserve visual links in Markdown and support readers that render item references. Priority: 2.

---

## Item info

[x] Status: done
[x] Include decision: include

### Source (first 20 lines)

- Source filename: Template%3AItem%20info%2Fdoc.txt
- Decoded name: Template:Item info/doc
- Namespace: Template
- Remainder: Item info/doc
- Path parts: Template:Item info / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: This templates generates, manages and structures item pages.
:Automatically generates:
:* Recipe (see special syntax for exceptions)
:* Item infobox
:* Old Icons
:* (most of the) cost analysis.
:* Ornn item tabber
:* Most categories
:* Map specific differences

;General Syntax
<tabber>For Mythics=
<pre>
{{Item info|item = |ornn =
|mgv =
|goldvalue =
|goldefficiency =
```

### Notes

Generates and structures item pages (infobox, recipe, cost analysis); include and implement readers that use Module:ItemData and Module:Gold value. Priority: 1.

---

## Item info/var

[x] Status: done
[x] Include decision: include

### Source (first 20 lines)

- Source filename: Template%3AItem%20info%2Fvar%2Fdoc.txt
- Decoded name: Template:Item info/var/doc
- Namespace: Template
- Remainder: Item info/var/doc
- Path parts: Template:Item info / var / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: Adds an individual item's infobox and cost analysis section for [[Template:Item info]]. See parent template for more information.
: Uses getItemPage from [[Module:ItemData]] and most functions from [[Module:Gold value]].

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Section formatting templates]]
</includeonly>
```

### Notes

Adds an individual item's infobox and cost analysis for `Item info`; include (reads Module:ItemData/Gold value). Priority: 1.

---

## Item limit

[x] Status: done
[x] Include decision: include

### Source (first 20 lines)

- Source filename: Template%3AItem%20limit%2Fdoc.txt
- Decoded name: Template:Item limit/doc
- Namespace: Template
- Remainder: Item limit/doc
- Path parts: Template:Item limit / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: Shortcut to write item group limit.

;Syntax
: <code>{{t|item limit|item group}}</code>

;Example
: <code>{{tl|item limit|Crit Modifier}}</code>
:: {{item limit|Crit Modifier}}

: <code>{{tl|item limit|Potion}}</code>
:: {{item limit|Potion}}

: <code>{{tl|item limit|legendary|Abyssal Mask}}</code>
:: {{item limit|legendary|Abyssal Mask}}

: <code>{{tl|item limit|Mythic}}</code>
:: {{item limit|Mythic}}
```

### Notes

Small helper that prints item-group limit text (e.g., "Mythic"); include as a formatting helper. Priority: 3.

---

## Item presentation

[x] Status: done
[x] Include decision: include

### Source (first 20 lines)

- Source filename: Template%3AItem%20presentation%2Fdoc.txt
- Decoded name: Template:Item presentation/doc
- Namespace: Template
- Remainder: Item presentation/doc
- Path parts: Template:Item presentation / doc

```
{{Documentation subpage}}

== Usage ==
Displays the item name, and the stat provided in argument(s) 2 to 11. If there isn't any argument 2, it will display every stat, effect, recipe and builds into of the item.

Input arguments are : pass to pass6, act, aura, spec, spec2, consume, all item stats, cost, combine costs, sell, recipe, builds, type, ID, req, limit & caption.

;Syntax
<code>{{t|ip|Item name|Stat1|Stat2|...|Stat10}}</code>

;Example
* <code>{{tl|ip|Trinity Force}}</code>
: {{ip|Trinity Force}}

* <code>{{tl|ip|Long Sword}}</code>
: {{ip|Long Sword}}

* <code>{{tl|ip|Spirit Visage|passive}}</code>
: {{ip|Spirit Visage|passive}}

```

### Notes

Renders item name and stats/builds for display on item pages; include to capture readable item content and support downstream readers. Priority: 1.

---

## Item presentation/var

[x] Status: done
[x] Include decision: include

### Source (first 20 lines)

- Source filename: Template%3AItem%20presentation%2Fvar%2Fdoc.txt
- Decoded name: Template:Item presentation/var/doc
- Namespace: Template
- Remainder: Item presentation/var/doc
- Path parts: Template:Item presentation / var / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

== Description ==
List of possible inputs for the "Item presentation" template.

<includeonly>
<!-- Categories and interwikis go here: -->

</includeonly>
```

### Notes

Lists possible input variables for `Item presentation`; include as a helper/reference for the presentation reader. Priority: 2.

---

## Item stat table

[x] Status: done
[x] Include decision: include

### Source (first 20 lines)

- Source filename: Template%3AItem%20stat%20table%2Fdoc.txt
- Decoded name: Template:Item stat table/doc
- Namespace: Template
- Remainder: Item stat table/doc
- Path parts: Template:Item stat table / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: Shows a table of all items (either from {{tip|League of Legends}} or {{tip|Wild Rift}} (with <code>wr=true</code>)) based on their stat (or based on a general function) input in the first parameter.

: Available {{{1}}} parameter:
* <code>offensive</code>
* <code>magical</code>
* <code>defensive</code>
* <code><item stat></code>
* <code>pykehealth</code>
* <code>baseefficiency</code> (''Arena items excluded'')

: Uses [[Template:Do_for_every_item]] (or [[Template:Do_for_every_WR item]]).

;Syntax
: <code>{{t|Item stat table|stat}}</code>

;Example
: <code>{{tl|Item stat table|crit}}</code>
```

### Notes

Generates stat-filtered tables/lists of items; include if automated stat-based exports are desired or to support generating lists programmatically. Priority: 2.

---

## Rune choice description

[x] Status: done
[x] Include decision: include

### Source (first 20 lines)

- Source filename: Template%3ARune%20choice%20description%2Fdoc.txt
- Decoded name: Template:Rune choice description/doc
- Namespace: Template
- Remainder: Rune choice description/doc
- Path parts: Template:Rune choice description / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: Show the description for rune paths.

;Syntax
: <code>{{t|Rune choice description|Rune path|Focus text}}</code>

;Example
: <code>{{tl|Rune choice description|Precision|marksman and sustained damage dealers}}</code>
{{Rune choice description|Precision|marksman and sustained damage dealers}}

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Section formatting templates]]
</includeonly>
```

### Notes

Provides the short descriptive text for rune choices (path focus); include to capture readable rune descriptions for Markdown. Priority: 2.

---

## Rune combinations table

[x] Status: done
[x] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3ARune%20combinations%20table%2Fdoc.txt
- Decoded name: Template:Rune combinations table/doc
- Namespace: Template
- Remainder: Rune combinations table/doc
- Path parts: Template:Rune combinations table / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
Calculates the total amount of possible combinations of rune pages.

;Syntax
:<code><nowiki>{{Rune combinations table}}</nowiki></code>
{{Rune combinations table}}

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Table formatting templates]]
[[de:Vorlage:Runenkombinationstabelle]]
</includeonly>
```

### Notes

Table-generation/layout-only (computes and displays combination counts); exclude from Markdown output (structural). Priority: 3.

---

## Rune data

[x] Status: done
[x] Include decision: include

### Source (first 20 lines)

- Source filename: Template%3ARune%20data%2Fdoc.txt
- Decoded name: Template:Rune data/doc
- Namespace: Template
- Remainder: Rune data/doc
- Path parts: Template:Rune data / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
* {{t|Rune data}} defines the format of [[:Category:Rune data templates|Rune data templates]] for runes in {{tip|League of Legends}} and {{tip|Wild Rift}}, and generates their documentation.
* ''Data templates'' store data centrally, which can then be retrieved for any purpose on the Wiki. '''Rune data''' templates are most commonly used in {{t|ri}} (an icon-link template + a tooltip) and in infoboxes for runes.
* The documentation produces a table on the data template's page that lists all possible parameters and their given value. Instructions for retrieving and formatting values are then provided.

;Creation of Rune data templates
* Create new '''Rune data''' templates using the following title scheme. Rune names should be typed <u>as shown in-game</u>.
** <code><nowiki>Template:Rune data <Name of rune></nowiki></code> for League of Legends
** <code><nowiki>Template:Rune data <Name of rune> (Wild Rift)</nowiki></code> for Wild Rift
* All '''Rune data''' templates begin with:
<pre><nowiki>{{{{{1<noinclude>|Rune data</noinclude>}}}|
}}
</nowiki></pre>
: The <nowiki><noinclude> </noinclude></nowiki> tags are necessary to prevent transclusion of the entire documentation page whenever the data template is invoked.
* Data is then stored in the data template as parameter inputs. Inputs are separated from each other by a <code>|</code> vertical bar.
** The first input must be the rune's name.
** The second input must be <code><nowiki>{{{2|}}}</nowiki></code>
** All other inputs are parameters with custom names. To provide a value, specify the parameter, then an <code>=</code> equals sign, then add its value.
```

### Notes

Data template storing rune values and metadata — include and implement a reader to extract rune fields for Markdown. Priority: 2.

---

## Rune footer

[x] Status: done
[x] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3ARune%20footer%2Fdoc.txt
- Decoded name: Template:Rune footer/doc
- Namespace: Template
- Remainder: Rune footer/doc
- Path parts: Template:Rune footer / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
Adding Reference Tag to every rune page.

== Usage ==

== Syntax ==

== See also ==

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:General wiki templates]]
</includeonly>
```

### Notes

Adds reference/category footer tags to rune pages (layout-only); exclude from Markdown. Priority: 3.

---

## Rune header

[x] Status: done
[x] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3ARune%20header%2Fdoc.txt
- Decoded name: Template:Rune header/doc
- Namespace: Template
- Remainder: Rune header/doc
- Path parts: Template:Rune header / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: Adds the relevant text, infoboxes, and categories to articles for [[rune]]s in [[League of Legends]] and [[League of Legends: Wild Rift|Wild Rift]].

;Syntax
:<code>{{t|Rune header|name of the rune}}</code> for League of Legends.
:<code>{{t|Rune header|name of the rune (Wild Rift)}}</code> for Wild Rift.

;Examples
: League of Legends
::<code>{{tl|Rune header|Conqueror}}</code>
: Wild Rift
::<code>{{tl|Rune header|Conqueror (Wild Rift)}}</code>

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:General wiki templates]]
[[de:Vorlage:Rune Info]]
</includeonly>
```

### Notes

Page header that injects infoboxes/categories for rune articles — structural/layout-only; exclude. Priority: 3.

---

## Rune icon

- [x] Status: done
- [x] Include decision: include

### Source (first 20 lines)

- Source filename: Template%3ARune%20icon%2Fdoc.txt
- Decoded name: Template:Rune icon/doc
- Namespace: Template
- Remainder: Rune icon/doc
- Path parts: Template:Rune icon / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

;Usage
:This template will make a link to an rune out of the square and name.

;Syntax
:{{t|ri|Rune Name|Custom Name}}

;Example
:{{t|ri|Overheal}}
::{{ri|Overheal}}

:{{t|ri|Overheal|Superpower}}
::{{ri|Overheal|Superpower}}

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Icon templates]]
[[de:Vorlage:Rune icon]]
```

### Notes

Creates inline rune icon links and names; include to preserve in-text rune references and tooltips in Markdown. Priority: 2.

---

## Rune path infobox/Trait

[x] Status: done
[x] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3ARune%20path%20infobox%2FTrait%2Fdoc.txt
- Decoded name: Template:Rune path infobox/Trait/doc
- Namespace: Template
- Remainder: Rune path infobox/Trait/doc
- Path parts: Template:Rune path infobox / Trait / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: This template is used in the creation of one standard row that showcases a [[rune]] ''Trait'' combination under a rune path.

;Syntax
: {{t|Rune path infobox/Trait|Rune path|Trait title|Trait description}}

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Infobox templates]]
</includeonly>
```

### Notes

Infobox row used for rendering rune path traits (visual/structural); exclude from Markdown (infobox-only). Priority: 3.

---

## Rune presentation

[x] Status: done
[x] Include decision: include

### Source (first 20 lines)

- Source filename: Template%3ARune%20presentation%2Fdoc.txt
- Decoded name: Template:Rune presentation/doc
- Namespace: Template
- Remainder: Rune presentation/doc
- Path parts: Template:Rune presentation / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

==Usage==
Displays the rune name, and its description, including cooldown and range if there is any.

;Syntax
: <code>{{t|rp|Rune name}}</code>

;Example
: <code>{{tl|rp|Press the Attack}}</code>
:* {{rp|Press the Attack}}

: <code>{{tl|rp|Electrocute}}</code>
:* {{rp|Electrocute}}

: <code>{{tl|rp|Revitalize}}</code>
:* {{rp|Revitalize}}

: <code>{{tl|rp|Demolish}}</code>
```

### Notes

Displays rune name and description (including cooldown/range) — include to capture readable rune descriptions. Priority: 2.

---

## Rune table row

[x] Status: done
[x] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3ARune%20table%20row%2Fdoc.txt
- Decoded name: Template:Rune table row/doc
- Namespace: Template
- Remainder: Rune table row/doc
- Path parts: Template:Rune table row / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
Gives an automated table of the requested [[Runes Reforged|rune]] path.

;Syntax
<pre>
{| border="0" cellpadding="1" cellspacing="1" class="article-table rune-table" width="100%" style="font-size:14px; text-align:center; background:transparent;"
! width="100px" style="text-align:center;" | Paths
! width="20%" style="text-align:center;" | Keystone
! width="20%" style="text-align:center;" | Slot 1
! width="20%" style="text-align:center;" | Slot 2
! width="20%" style="text-align:center;" | Slot 3
{{Rune table row|Precision}}
|}
</pre>

{| border="0" cellpadding="1" cellspacing="1" class="article-table rune-table" width="100%" style="font-size:14px; text-align:center; background:transparent;"
! width="100px" style="text-align:center;" | Paths
! width="20%" style="text-align:center;" | Keystone
```

### Notes

Generates a visual rune-path table row (layout/infobox table); exclude from Markdown output. Priority: 3.

---

## Sbc

- [x] Status: done
- [x] Include decision: include

### Source (first 20 lines)

- Source filename: Template%3ASbc%2Fdoc.txt
- Decoded name: Template:Sbc/doc
- Namespace: Template
- Remainder: Sbc/doc
- Path parts: Template:Sbc / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
"Small bold capitals."

== Usage ==
*<code>{{tl|sbc|Foo}} Bar</code>
**{{sbc|Foo}} Bar

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Formatting templates]]
[[de:Vorlage:Sbc]]
</includeonly>
```

### Notes

Formatting helper producing small bold capitals (inline styling); include because it affects rendered inline text, but low priority for content extraction. Priority: 3.

---

## Times

- [x] Status: done
- [x] Include decision: include

### Source (first 20 lines)

- Source filename: Template%3ATimes%2Fdoc.txt
- Decoded name: Template:Times/doc
- Namespace: Template
- Remainder: Times/doc
- Path parts: Template:Times / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
Use in formulas to have proper non-breaking space placement; linebreaks will only occur after an operator, and not before.

{{Math symbols}}

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Character formatting templates]]
[[pt-br:Predefinição:Vezes]]
</includeonly>
```

### Notes

Formatting helper for non-breaking spaces in formulas and math symbols; include to preserve correct inline spacing in technical content. Priority: 3.

---

## TT

- [x] Status: done
- [x] Include decision: include

### Source (first 20 lines)

- Source filename: Template%3ATT%2Fdoc.txt
- Decoded name: Template:TT/doc
- Namespace: Template
- Remainder: TT/doc
- Path parts: Template:TT / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

;Description
:This template will produce an icon for [[TFT:Treasure Tokens|Treasure Tokens]] in Teamfight Tactics and link to its page. You can also adjust the size of the icon and add its cost.

;Syntax
:Type <code>{{t|TT|cost|3=size=icon size}}</code> at any part of the article.

;Example
:<code>{{tl|TT}}</code>
:* {{TT}}

:<code>{{tl|TT|100}}</code>
:* {{TT|100}}

:<code>{{tl|TT|100|3=size=30}}</code>
:* {{TT|100|size=30}}

<includeonly>
```

### Notes

Renders Treasure Token icon and optional cost; include to preserve inline TFT token icons and costs where used. Priority: 2.

---

## (no name)

[x] Status: done
[x] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3A(no%20name)%2Fdoc.txt
- Decoded name: Template:(no name)/doc
- Namespace: Template
- Remainder: (no name)/doc
- Path parts: Template:(no name) / doc

_No matching export file found or file empty._

### Notes

No export/source found; treat as orphan/structural and exclude. Priority: 3.

---
