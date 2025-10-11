# Template checklist — expanded

Generated: 2025-10-10T15:54:19.219Z

Batch 20 of 33 — items 381..400

## Other free

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AOther%20free%2Fdoc.txt
- Decoded name: Template:Other free/doc
- Namespace: Template
- Remainder: Other free/doc
- Path parts: Template:Other free / doc

```
{{Documentation subpage}}
;Description
:This template is used to mark images with a free license not covered by other image templates.
;Syntax
:Type <code>{{t|Other free}}</code> on the image information page.

<includeonly>[[Category:Image wiki templates|{{PAGENAME}}]]</includeonly><noinclude></noinclude>
```

### Notes

_No notes specified._

---

## Out of scope

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AOut%20of%20scope%2Fdoc.txt
- Decoded name: Template:Out of scope/doc
- Namespace: Template
- Remainder: Out of scope/doc
- Path parts: Template:Out of scope / doc

```
#REDIRECT [[Template:Ambox documentation]][[Category:Template documentation]]
```

### Notes

_No notes specified._

---

## Outdated

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AOutdated%2Fdoc.txt
- Decoded name: Template:Outdated/doc
- Namespace: Template
- Remainder: Outdated/doc
- Path parts: Template:Outdated / doc

```
#REDIRECT [[Template:Ambox documentation]][[Category:Template documentation]]
```

### Notes

_No notes specified._

---

## Outdated if after

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AOutdated%20if%20after%2Fdoc.txt
- Decoded name: Template:Outdated if after/doc
- Namespace: Template
- Remainder: Outdated if after/doc
- Path parts: Template:Outdated if after / doc

```
#REDIRECT [[Template:Ambox documentation]][[Category:Template documentation]]
```

### Notes

_No notes specified._

---

## Outdated if new champion

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AOutdated%20if%20new%20champion%2Fdoc.txt
- Decoded name: Template:Outdated if new champion/doc
- Namespace: Template
- Remainder: Outdated if new champion/doc
- Path parts: Template:Outdated if new champion / doc

```
#REDIRECT [[Template:Ambox documentation]][[Category:Template documentation]]
```

### Notes

_No notes specified._

---

## Outdated if new patch

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AOutdated%20if%20new%20patch%2Fdoc.txt
- Decoded name: Template:Outdated if new patch/doc
- Namespace: Template
- Remainder: Outdated if new patch/doc
- Path parts: Template:Outdated if new patch / doc

```
#REDIRECT [[Template:Ambox documentation]][[Category:Template documentation]]
```

### Notes

_No notes specified._

---

## Overline

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AOverline%2Fdoc.txt
- Decoded name: Template:Overline/doc
- Namespace: Template
- Remainder: Overline/doc
- Path parts: Template:Overline / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: Adds an overline/overbar to the input text.

;Syntax
: {{t|Overline|text}}

;Usage
: Usually for numbers but can also be used for emphasis in place of underlines.

;Example
: <code><nowiki>{{Overline|Hi!}}</nowiki></code> gives {{Overline|Hi!}}

;See also
* [[Template:Recurring]]

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Formatting templates]]
```

### Notes

_No notes specified._

---

## Pagelist

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3APagelist%2Fdoc.txt
- Decoded name: Template:Pagelist/doc
- Namespace: Template
- Remainder: Pagelist/doc
- Path parts: Template:Pagelist / doc

```
{{Documentation subpage}}
==Usage==
This template creates a comma-separated list of pages formatted as links.  The last two pages in the list are separated by "and" instead of a comma.  From 1–20 pages may be included in the list.

By default, the namespace of the page including this template is used for each of the links.  An optional "'''nspace'''" parameter can be specified to provide a different namespace to use for the links.

Two other optional parameters are supported to provide formatting of the page links.  If specified, the "'''delim'''" parameter is included before and after each page name.  Optionally, the "'''edelim'''" parameter can specify the delimiter used at the end of the page name&mdash;in this case the "'''delim'''" tag is still used for the beginning of the page name.

This template is mostly useful from within other templates that accept a variable number of page names as arguments.  For example, the page arguments <tt><nowiki>{{{1}}}</nowiki></tt>–<tt><nowiki>{{{10}}}</nowiki></tt> can be passed to this template using <tt><nowiki>{{pagelist|{{{1|}}}|{{{2|}}}|{{{3|}}}|{{{4|}}}|{{{5|}}}|{{{6|}}}|{{{7|}}}|{{{8|}}}|{{{9|}}}|{{{10|}}}}}</nowiki></tt>.  Any parameters not specified to the original template will be passed as empty parameters and not listed by the pagelist template.

===Examples===
{| class="wikitable"
! Type this !! To get this
|-
| <tt><nowiki>{{Pagelist|Pagelist|clr|W}}</nowiki></tt>
| {{Pagelist|Pagelist|clr|W}}
|-
| <tt><nowiki>{{Pagelist|nspace=Template|Pagelist|clr|W}}</nowiki></tt>
| {{Pagelist|nspace=Template|Pagelist|clr|W}}
|-
```

### Notes

_No notes specified._

---

## Passive progression

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3APassive%20progression%2Fdoc.txt
- Decoded name: Template:Passive progression/doc
- Namespace: Template
- Remainder: Passive progression/doc
- Path parts: Template:Passive progression / doc

```
{{Documentation subpage}}

;Description
This template is used to display values or statistics that scale, usually based on level. It can also be applied to non-linear scaling, or values that scale based on a number of stacks.

There are two generated outputs: the visible text output, and the table embedded within the tooltip. The displayed tooltip is generated by [[Template:Tooltip/Pp]]. All values are applied [[Template:Format decimal|default decimal formatting]].

;Syntax
The abbreviated link {{t|pp|}} is used for brevity.

However, there are numerous optional parameters of variable usefulness:

{{t|pp|1|2|changedisplay|showtype|label1|type|label|formula|key|key1|round|round1|color}}

* '''1'''
** The first parameter specifies the progression's values, separated using semicolons and up to a maximum of 41. (bottom row of table)
* '''2'''
** The second parameter specifies the values' associated levels (or other basis of scaling), again separated using semicolons and capped at 41. (top row of table)
** If left blank, this will automatically be populated using counting numbers.
* '''changedisplay'''
```

### Notes

_No notes specified._

---

## PathOfChampions

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3APathOfChampions%2Fdoc.txt
- Decoded name: Template:PathOfChampions/doc
- Namespace: Template
- Remainder: PathOfChampions/doc
- Path parts: Template:PathOfChampions / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

;Description
:This template is used to add the three following navboxes for Path of Champions (Champions, Items, Relics, Powers)

;Syntax
:Add the following syntax to the end of a page.
<pre>
{{PathOfChampions}}
</pre>

;Output
{{PathOfChampions}}

;You may also transclude a specific navbox using the following syntax.
<pre>
{{LoRPvEModes}}
{{PathOfChampions/Champions}}
{{PathOfChampions/Items}}
```

### Notes

_No notes specified._

---

## PC

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3APC%2Fdoc.txt
- Decoded name: Template:PC/doc
- Namespace: Template
- Remainder: PC/doc
- Path parts: Template:PC / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

;Description
:This template will produce a [[WR:Poro Coin|Poro Coin]] icon and the link to its page. You can also adjust the size of the icon and add its cost.

;Syntax
:Type <code>{{t|PC|cost|3=size=icon size}}</code> at any part of the article.

;Example
:<code>{{tl|PC}}</code>
:* {{PC}}

:<code>{{tl|PC|100}}</code>
:* {{PC|100}}

:<code>{{tl|PC|100|3=size=30}}</code>
:* {{PC|100|size=30}}

<includeonly>
```

### Notes

_No notes specified._

---

## PD

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3APD%2Fdoc.txt
- Decoded name: Template:PD/doc
- Namespace: Template
- Remainder: PD/doc
- Path parts: Template:PD / doc

```
{{Documentation subpage}}
;Description
:This template is used to mark images as being in the public domain.
;Syntax
:Type <code>{{t|PD}}</code> on the image information page.

<includeonly>[[Category:Image wiki templates|{{PAGENAME}}]]</includeonly><noinclude></noinclude>
```

### Notes

_No notes specified._

---

## PE

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3APE%2Fdoc.txt
- Decoded name: Template:PE/doc
- Namespace: Template
- Remainder: PE/doc
- Path parts: Template:PE / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

;Description
:This template will produce a [[Poro Energy (Wild Rift)|Poro Energy]] icon and the link to its page. You can also adjust the size of the icon and add its cost.

;Syntax
:Type <code>{{t|PE|cost|3=size=icon size}}</code> at any part of the article.

;Example
:<code>{{tl|PE}}</code>
:* {{PE}}

:<code>{{tl|PE|100}}</code>
:* {{PE|100}}

:<code>{{tl|PE|100|3=size=30}}</code>
:* {{PE|100|size=30}}

<includeonly>
```

### Notes

_No notes specified._

---

## Pending for test

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3APending%20for%20test%2Fdoc.txt
- Decoded name: Template:Pending for test/doc
- Namespace: Template
- Remainder: Pending for test/doc
- Path parts: Template:Pending for test / doc

```
{{Documentation subpage}}
==Description==
Used for anything which is not confirmed and needs testing.
* Adds [[:Category:Pending for test]] to the article.

==Usage==
* <code>{{tl|Pending for test}}</code>
** {{Pending for test}}

<includeonly>[[Category:Article management templates|{{PAGENAME}}]]</includeonly>
```

### Notes

_No notes specified._

---

## Placeholder text

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3APlaceholder%20text%2Fdoc.txt
- Decoded name: Template:Placeholder text/doc
- Namespace: Template
- Remainder: Placeholder text/doc
- Path parts: Template:Placeholder text / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
This template formats text into the default placeholder design.

;Syntax
The template can be used anywhere, but is particularly useful for indicating template parameters.
<code><nowiki>{{Pht|text}}</nowiki></code> creates:<br>
{{pht|text}}

;Example
<code><nowiki><code>{{Pht|parameter description}}</code></nowiki></code> creates:<br>
<code>{{pht|parameter description}}</code>

;See also
*Redirecting template [[Template:Pht|Pht]]

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Formatting templates]]
```

### Notes

_No notes specified._

---

## Plus

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3APlus%2Fdoc.txt
- Decoded name: Template:Plus/doc
- Namespace: Template
- Remainder: Plus/doc
- Path parts: Template:Plus / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
Use in formulas to have proper non-breaking space placement; linebreaks will only occur after an operator, and not before.

{{Math symbols}}

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Character formatting templates]]
[[pt-br:Predefinição:Mais]]
</includeonly>
```

### Notes

_No notes specified._

---

## PlusMinus

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3APlusMinus%2Fdoc.txt
- Decoded name: Template:PlusMinus/doc
- Namespace: Template
- Remainder: PlusMinus/doc
- Path parts: Template:PlusMinus / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
Use in formulas to have proper non-breaking space placement; linebreaks will only occur after an operator, and not before.

{{Math symbols}}

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Character formatting templates]]
[[pt-br:Predefinição:MaisOuMenos]]
</includeonly>
```

### Notes

_No notes specified._

---

## Policy

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3APolicy%2Fdoc.txt
- Decoded name: Template:Policy/doc
- Namespace: Template
- Remainder: Policy/doc
- Path parts: Template:Policy / doc

```
#REDIRECT [[Template:Ambox documentation]][[Category:Template documentation]]
```

### Notes

_No notes specified._

---

## Portal

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3APortal%2Fdoc.txt
- Decoded name: Template:Portal/doc
- Namespace: Template
- Remainder: Portal/doc
- Path parts: Template:Portal / doc

```
{{Documentation subpage}}
__TOC__
<!-- Categories and interwikis go at the bottom of this page. -->
==Template Documentation==
This template is used to format segments of the homepage, such as News and Roster. The template can be used directly on the homepage or via a nested template. The latter might be preferred if the homepage is locked/protected, since the nested templates would still be editable.

;Basic usage
{{t|portal|content}}

;With a header
{{t|portal|header|content}}

;Extra customization
{| class="article-table"
! parameter
! example
! description
|-
| class || ''FullWidthImage'' || Adds custom classes to the portal container. Format as if HTML - spaces denote separate classes.
|-
```

### Notes

_No notes specified._

---

## Portal/League/Events

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3APortal%2FLeague%2FEvents%2Fdoc.txt
- Decoded name: Template:Portal/League/Events/doc
- Namespace: Template
- Remainder: Portal/League/Events/doc
- Path parts: Template:Portal / League / Events / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Preview
<tabber>
desktop=
<div class="mainpage-body" style="max-width:935px;">
{{Portal/League/Events}}
</div>
|-|
mobile=
<div class="mainpage-body" style="max-width:360px;">
{{Portal/League/Events}}
</div>
</tabber>

;Description
: This template lists recent events on the [[Main Page]].
: Updates are displayed with the help of {{t|Portal/Updates/Tile2}}.

;Syntax
```

### Notes

_No notes specified._

---
