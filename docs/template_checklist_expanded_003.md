# Template checklist — expanded

Generated: 2025-10-10T15:54:19.137Z

Batch 3 of 33 — items 41..60

## (no name)

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3A(no%20name)%2Fdoc.txt
- Decoded name: Template:(no name)/doc
- Namespace: Template
- Remainder: (no name)/doc
- Path parts: Template:(no name) / doc

_No matching export file found or file empty._

### Notes

Clears floats (layout-only) — exclude from markdown.

---

## !

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3A!%2Fdoc.txt
- Decoded name: Template:!/doc
- Namespace: Template
- Remainder: !/doc
- Path parts: Template:! / doc

_No matching export file found or file empty._

### Notes

Table pipe helper — layout-only; exclude.

---

## !!

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3A!!%2Fdoc.txt
- Decoded name: Template:!!/doc
- Namespace: Template
- Remainder: !!/doc
- Path parts: Template:!! / doc

_No matching export file found or file empty._

### Notes

Double pipe helper for tables — exclude.

---

## ·

- [x] Status: done
- [x] Include decision: include

### Source (first 20 lines)

- Source filename: Template%3A%C2%B7%2Fdoc.txt
- Decoded name: Template:·/doc
- Namespace: Template
- Remainder: ·/doc
- Path parts: Template:· / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

== Usage ==
Creates a middot with a preceding &amp;nbsp;.

== Syntax ==
<code>{{t|·}}</code> alias <code>{{t|,}}</code>

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:General wiki templates|{{PAGENAME}}]]

[[de:Vorlage:·]]
</includeonly>
```

### Notes

Outputs a middot (non-breaking) — include as literal punctuation.

---

## (

- [x] Status: done
- [x] Include decision: include

### Source (first 20 lines)

- Source filename: Template%3A(%2Fdoc.txt
- Decoded name: Template:(/doc
- Namespace: Template
- Remainder: (/doc
- Path parts: Template:( / doc

_No matching export file found or file empty._

### Notes

Wraps text in parentheses / prevents wrapping — include.

---

## )

- [x] Status: done
- [x] Include decision: include

### Source (first 20 lines)

- Source filename: Template%3A)%2Fdoc.txt
- Decoded name: Template:)/doc
- Namespace: Template
- Remainder: )/doc
- Path parts: Template:) / doc

_No matching export file found or file empty._

### Notes

Outputs a closing brace/character — include as literal.

---

## 1x

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3A1x%2Fdoc.txt
- Decoded name: Template:1x/doc
- Namespace: Template
- Remainder: 1x/doc
- Path parts: Template:1x / doc

```
{{Documentation subpage}}
==Usage==
===As a meta-metatemplate===
This template is particularly useful as a meta-metatemplate.  A metatemplate can use, for example, <code><nowiki>{{1x|&lt;}}includeonly{{1x|&gt;}}</nowiki></code> to generate an <code>&lt;includeonly&gt;</code> tag.  Unlike such a tag made with XML/HTML entities (<code>&amp;lt;</code> and <code>&amp;gt;</code>), this tag will be evaluated when the resulting template is substituted.

===To have an option that uses the return value of the template as an input===
This template can be very useful if you want to have an option that uses the return value of the template as an input. For example, consider the following template:
<pre>{{ {{#if: {{{sbc|}}} | sbc | 1x }}
|1=The entire template comes here.}}</pre>
If the parameter ''sbc'' is defined, the result will be in small capitals using {{tl|sbc}}. To test this sample, suppose the parameter is ''yes'':
<pre>{{ {{#if: yes | sbc | 1x }}|1=The entire template comes here.}}</pre>
The result will be:
:{{ {{#if: yes | sbc | 1x }}|1=The entire template comes here.}}

Then suppose it is a null:
<pre>{{ {{#if:  | sbc | 1x }}|1=The entire template comes here.}}</pre>
The result will be:
:{{ {{#if:  | sbc | 1x }}|1=The entire template comes here.}}

<includeonly>[[Category:General wiki templates]]</includeonly><noinclude></noinclude>
```

### Notes

Meta helper for template generation; does not affect final text — exclude.

---

## 2xko

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3A2xko%2Fdoc.txt
- Decoded name: Template:2xko/doc
- Namespace: Template
- Remainder: 2xko/doc
- Path parts: Template:2xko / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: Link to the 2XKO wiki with an inline icon.

;Syntax
: Using wikitext, describe the template parameters

;Usage
: Briefly describe how the template is used

;Example
* {{2xko|Ahri}}
* {{2xko|Ahri|Dynasty}}
* {{2xko|Ahri|Dynasty|Catseye}}


;See also
*

```

### Notes

Link/icon to an external wiki — nav-only; exclude from main content.

---

## 2XKO only roster

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3A2XKO%20only%20roster%2Fdoc.txt
- Decoded name: Template:2XKO only roster/doc
- Namespace: Template
- Remainder: 2XKO only roster/doc
- Path parts: Template:2XKO only roster / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: Grid-only version of [[Template:2XKO roster]].

;Usage
{{t|2XKO only roster}}
{{2XKO only roster}}

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Navigation templates]]
</includeonly>
```

### Notes

Navigation box (grid) — layout-only; exclude.

---

## 2XKO roster

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3A2XKO%20roster%2Fdoc.txt
- Decoded name: Template:2XKO roster/doc
- Namespace: Template
- Remainder: 2XKO roster/doc
- Path parts: Template:2XKO roster / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
This template creates a flex-grid container all the available Fighters in 2XKO. The icon-template used is [[Template:2XKO roster/Do]]. Expand as necessary or desired.

;Usage
{{t|2XKO roster}}
{{2XKO roster}}

;Customization
* <code>|hide=true</code> collapses the box, which is otherwise expanded by default

;See also
* [[Template:2XKO only roster]], the non-navbox version of this template.

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Navigation templates]]
</includeonly>
```

### Notes

Navigation template — exclude.

---

## 2xko

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3A2xko%2Fdoc.txt
- Decoded name: Template:2xko/doc
- Namespace: Template
- Remainder: 2xko/doc
- Path parts: Template:2xko / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: Link to the 2XKO wiki with an inline icon.

;Syntax
: Using wikitext, describe the template parameters

;Usage
: Briefly describe how the template is used

;Example
* {{2xko|Ahri}}
* {{2xko|Ahri|Dynasty}}
* {{2xko|Ahri|Dynasty|Catseye}}


;See also
*

```

### Notes

(duplicate entry) exclude.

---

## Abilities

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3AAbilities%2Fdoc.txt
- Decoded name: Template:Abilities/doc
- Namespace: Template
- Remainder: Abilities/doc
- Path parts: Template:Abilities / doc

```
{{Documentation subpage}}
{{Removed|Superseded by [[Template:Ability]]}}

;Description
: This template is used to create the abilities.

;Syntax
: Type <code>{{t|Abilities|...}}</code> somewhere, with parameters as shown below.

;Sample Output
<pre>
{{Abilities
|name         = [Defaults to Page Name] Bob
|ver          = V1.0.0.108
|innatename   = Innate
|innateinfo   = '''(Innate)''': Description
|firstname    = First
|firstinfo    = '''(Passive/Active/Toggle/Stance)''': Description
|firstlevel   = {{level up|Damage|(+0.66 per ability power)|75|115|155|195|235}}
|secondname   = Second/1
```

### Notes

Deprecated/superseded by Template:Ability — exclude.

---

## Active item cooldown table

- [x] Status: done
- [x] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3AActive%20item%20cooldown%20table%2Fdoc.txt
- Decoded name: Template:Active item cooldown table/doc
- Namespace: Template
- Remainder: Active item cooldown table/doc
- Path parts: Template:Active item cooldown table / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: This template is used to show a list of all effects that grant '''active item cooldown'''.

;Syntax
: <code>{{t|Active item cooldown table}}</code>

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Data templates]]
[[de:Vorlage:Active item cooldown table]]
</includeonly>
```

### Notes

Lists effects that grant active-item cooldowns (data template); exclude from Markdown unless used to generate content lists. Priority: 2.

---

## Adaptive

- [x] Status: done
- [x] Include decision: include

### Source (first 20 lines)

- Source filename: Template%3AAdaptive%2Fdoc.txt
- Decoded name: Template:Adaptive/doc
- Namespace: Template
- Remainder: Adaptive/doc
- Path parts: Template:Adaptive / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
Displays a styled text for bonus adaptive stats (AD or AP). The number you input is the Ability Power value, the Attack Damage value is 60% of the AP amount.

;Syntax

:<code><nowiki>{{adaptive|</nowiki>{{pht|amount of adaptive stat}}<nowiki>}}</nowiki></code>
'''OR'''
:<code><nowiki>{{adaptive|</nowiki>{{pht|stat at level 1}} to {{pht|stat at level 18}}<nowiki>}}</nowiki></code>

;Examples
*<code><nowiki>{{adaptive|20}}</nowiki></code> creates: {{adaptive|20}}
*<code><nowiki>{{adaptive|2.5}}</nowiki></code> creates: {{adaptive|2.5}}
*<code><nowiki>{{adaptive|5 to 25}}</nowiki></code> creates: {{adaptive|5 to 25}}

==See also ==
*Template [[Template:Passive progression|Passive progression]]

<includeonly>
```

### Notes

Formats adaptive-stat text (AD/AP) inline; include as a small formatting helper to preserve readable stat output. Priority: 3.

---

## Administration category

- [x] Status: done
- [x] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3AAdministration%20category%2Fdoc.txt
- Decoded name: Template:Administration category/doc
- Namespace: Template
- Remainder: Administration category/doc
- Path parts: Template:Administration category / doc

```
#REDIRECT [[Template:Ambox documentation]][[Category:Template documentation]]
```

### Notes

Redirect to Ambox documentation (admin meta); exclude from article output. Priority: 3.

---

## Bureaucrat Promotion Box

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3ABureaucrat%20Promotion%20Box%2Fdoc.txt
- Decoded name: Template:Bureaucrat Promotion Box/doc
- Namespace: Template
- Remainder: Bureaucrat Promotion Box/doc
- Path parts: Template:Bureaucrat Promotion Box / doc

```
{{Documentation subpage}}
;Description
:This template is to reward users for being granted bureaucrat rights.
;Syntax
:Type <code><nowiki>{{bureaucrat promotion}}</nowiki></code> in a message on their talk page.

<includeonly>[[Category:User templates|{{PAGENAME}}]]</includeonly><noinclude></noinclude>
```

### Notes

User promotion badge — user-management; exclude from article output.

---

## BXP

- [x] Status: done
- [x] Include decision: include

### Source (first 20 lines)

- Source filename: Template%3ABXP%2Fdoc.txt
- Decoded name: Template:BXP/doc
- Namespace: Template
- Remainder: BXP/doc
- Path parts: Template:BXP / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

;Description
:This template will produce a [[Battle Pass]] XP icon and the link to that page.

;Syntax
:<code><nowiki>{{BXP|<text to display in the page>|size=<size in pixels>}}</nowiki></code>

;Example
:<code>{{tl|BXP}}</code>
:* {{BXP}}

:<code>{{tl|BXP|250}}</code>
:* {{BXP|250}}

:<code>{{tl|BXP|250|3=size=32}}</code>
:* {{BXP|250|size=32}}

<includeonly>
```

### Notes

Battle Pass XP icon helper — include as inline icon.

---

## Cai

- [x] Status: done
- [x] Include decision: include

### Source (first 20 lines)

- Source filename: Template%3ACai%2Fdoc.txt
- Decoded name: Template:Cai/doc
- Namespace: Template
- Remainder: Cai/doc
- Path parts: Template:Cai / doc

```
{{Documentation subpage}}
;Description
:This template makes a link to an ability and its respective champion out of their images and names.
:When using code (see syntax below), 1st parameter is the ability name, 2nd parameter is for the champion name.
:When using an inserted template, Parameter #1 is for the champion's name, Parameter #2 is for the ability name (flipped version of code syntax). Parameter #3 can be left blank.
;Syntax
:Type <code>{{tl|cai|''Ability''|''Champion''}}</code> at any part of the article.
;Examples
*<code>{{tl|cai|Noxious Trap|Teemo}}</code>
** {{cai|Noxious Trap|Teemo}}
*<code>{{tl|cai|Requiem|Karthus}}</code>
** {{cai|Requiem|Karthus}}

<includeonly>
[[Category:Icon templates]]

[[de:Vorlage:Cai]]
</includeonly>
```

### Notes

Champion ability icon-link (ci variant) — include (icon+tooltip).

---

## Cais

- [x] Status: done
- [x] Include decision: include

### Source (first 20 lines)

- Source filename: Template%3ACais%2Fdoc.txt
- Decoded name: Template:Cais/doc
- Namespace: Template
- Remainder: Cais/doc
- Path parts: Template:Cais / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

;Description
:This template shortcuts usage of {{t|cai}} when using possessive apostrophes.
== Usage ==
;Syntax
:Type <code>{{t|cais|Ability|Champion}}</code> instead of <code>{{t|cai|Ability|Champion|Ability's}}</code> at any part of the article.

:Correct:
:<code>{{tl|cais|Demacian Justice|Garen}}</code>
::{{cais|Demacian Justice|Garen}}
:<code>{{tl|cais|Concussive Blows|Braum}}</code>
::{{cais|Concussive Blows|Braum}}

:Wrong:
:<code>{{tl|cai|Demacian Justice|Garen}}'s</code>
::{{cai|Demacian Justice|Garen}}'s
:<code>{{tl|cai|Concussive Blows|Braum}}'</code>
::{{cai|Concussive Blows|Braum}}'
```

### Notes

Possessive shortcut for {{cai}} — include.

---

## Cancelled champion

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3ACancelled%20champion%2Fdoc.txt
- Decoded name: Template:Cancelled champion/doc
- Namespace: Template
- Remainder: Cancelled champion/doc
- Path parts: Template:Cancelled champion / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

;Description
This template was created for cancelled champions, rather than using the custom champion template.

;GalleryHelper information
: ''This template uses [[Template:GalleryHelper|GalleryHelper]].''
* Only uses 1 instance: for the champion's icon, render, or other identification images like primary concept art.
** Image parameter(s): <code>image</code> '''or''' <code>gallery</code>
** Additional caption parameter: (none)
** Placeholder image: [[:File:Champion Render.png]]

;Syntax & Example
 <nowiki>{{Cancelled champion
| name        = Averdrian
| title       = The Astral Guardian
| (GalleryHelper parameters)=
| herotype    = Mage
| alttype     =
```

### Notes

Infobox/navigation for cancelled champions — structural; exclude.

---
