Introduction
============

Building the project requires

- Android SDK API 18
- Android SDK Build-tools 18.1
- You might need to update Android SDK Tools to latest one too (>= 22.2.1)

This is due to android.support.v8 support library enabling RenderScript usage all
the way back to API 8. Now it remains to be seen does it really work on those devices.

Pretty much all the code is licensed under Apache 2.0 license.


Motivation
==========

Idea for this project came from what I saw happening on iOS7 and Blur Effect
for Android Design -project. But as an Android developer I wanted try to achieve
something more of a flexible framework, namely bundle the effect into existing layouts, in
order to make it easier for doing experiments on the technique.

Ultimately I wanted to make it all into two separate projects -- one example
application and a library project one can hook into own applications more easily.
Unfortunately I faced problems moving RenderScript code into library project
and due had to put all the code within one application project only.

Will do this re-factoring later on if Android SDK starts supporting RenderScript
from library projects.. Or I figure out how to do it with current SDK..

Until then :: http://youtu.be/weGYilwd1YI


Thank Yous
==========

- Stack Blur Algorithm by Mario Klingemann <mario@quasimondo.com>
- Background image is from Nicolas Pomepuy's Blur Effect for Android Design -project<br>
  https://github.com/PomepuyN/BlurEffectForAndroidDesign
- Application icon was found on Google Image Search but I wasn't able to find
  the creator for crediting the person properly


Google Play Love
================

https://play.google.com/store/apps/details?id=fi.harism.anndblur
