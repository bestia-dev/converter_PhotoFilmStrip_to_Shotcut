[//]: # (auto_md_to_doc_comments segment start A)

# converter_PhotoFilmStrip_to_Shotcut  

[//]: # (auto_cargo_toml_to_md start)

**converts PhotoFilmStrip projects to Shotcut projects**  
***version: 1.0.14 date: 2024-01-28 author: [bestia.dev](https://bestia.dev) repository: [Github](https://github.com/bestia-dev/converter_PhotoFilmStrip_to_Shotcut)***  

[//]: # (auto_cargo_toml_to_md end)

  ![work_in_progress](https://img.shields.io/badge/work_in_progress-yellow)



[//]: # (auto_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-107-green.svg)](https://github.com/bestia-dev/converter_PhotoFilmStrip_to_Shotcut/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-262-blue.svg)](https://github.com/bestia-dev/converter_PhotoFilmStrip_to_Shotcut/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-38-purple.svg)](https://github.com/bestia-dev/converter_PhotoFilmStrip_to_Shotcut/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-19-yellow.svg)](https://github.com/bestia-dev/converter_PhotoFilmStrip_to_Shotcut/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-30-orange.svg)](https://github.com/bestia-dev/converter_PhotoFilmStrip_to_Shotcut/)

[//]: # (auto_lines_of_code end)

 [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/converter_PhotoFilmStrip_to_Shotcut/blob/master/LICENSE)
  ![converter_PhotoFilmStrip_to_Shotcut](https://bestia.dev/webpage_hit_counter/get_svg_image/1287912941.svg)

Hashtags: #rustlang #tutorial #sqlite #xml #cli  
My projects on GitHub are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).

## Motivation

[PhotoFilmStrip](https://www.photofilmstrip.org/en/features/) has a great simple UI to make slideshows. I use it to make videos of my travels.  
First I choose a song as a background. That defines the duration of the video.  
Then I add images. PhotoFilmStrip adds some random animations to every image. That's ok.  
The duration of the image is automatically calculated to fill the song duration. Great.  
I then add subtitles to every image. Eventually, I can modify the animation and change the order of the images.  
The UI is so easy, that even my wife likes to use it.  
Finally, PhotoFilmStrip renders the slideshow as a mp4 file and the subtitles as a srt file. That is fine, but...

I prefer to have the subtitles embedded in the video, so every video player will show them exactly how I want it. I encountered a lot of odd problems with separated `srt` files in different video players online and offline.  

There are some technics to embed the subtitles into mp4 using `ffmpeg` or VLC. But I am a programmer and every problem is an excuse to start a new project. In Rust, of course.

For my video editing, I use [Shotcut](https://shotcut.org/). It is simple and works for me. I would like to export the PhotoFilmStrip project to Shotcut.  
Shotcut can embed "subtitles" into the video and maybe later I could make some video editing that is not possible in PhotoFilmStrip.

So the problem is simple. I need a simple tool to make a one-time conversion from PhotoFilmStrip to Shotcut.  
After that, I can continue to work in Shotcut to modify the video at my will without the limitations of PhotoFilmStrip.

PhotoFilmStrip saves the project in an SQLite database in a `pfs` file. I could read that with Rust.  
Shotcut saves the projects in XML format in an `mlt` file. I could write that in Rust.

It is time to discover the elements of these 2 formats and how to transform them.

## PhotoFilmStrip format

I need to read SQLite file `pfs`. 
https://github.com/rusqlite/rusqlite
VSCode extension: https://marketplace.visualstudio.com/items?itemName=qwtel.sqlite-viewer

table `picture` columns:
filename, 
width height, 
start_left, start_top, start_width, start_height,
target_left, target_top, target_width, target_height, 
duration, comment, 

what means duration=7 ? not seconds? I will have all same duration, so no problem.

This is the rectangle inside the image of the original size.

First image is 4:3 and not 16:9:
C:\Users\luciano\Downloads\photofilmstrip_format\LF2023-12-11 11-07-09 Minja Luciano.jpg
4000   3000
0   181   4000  2250
79  744   3920  2205
7  ¡Hola México!

Second image is portrait:
C:\Users\luciano\Downloads\photofilmstrip_format\LF2023-12-15 10-46-45 minja es alta.jpg
2268   4032
0   772   2268  1275
0  1793   2268  1275
7  Minja Alto

Third image 16:9:
C:\Users\luciano\Downloads\photofilmstrip_format\LF2023-12-13 13-42-18.JPG
6000  3376
0  0    6000 3375
973  802  4500 2531

Fourth image 16:9
C:\Users\luciano\Downloads\photofilmstrip_format\LF2023-12-14 15-18-14.JPG
6000  3376
1500  0  4500  2531.25
0  0  6000 3375

## Shotcut format

In the `Example_3_images.mlt` there are:

- 1 song on the V1 playlist
- 3 images with animation and transition on the V2 playlist and
- 3 subtitles on the V3 playlist

The music is defined with these XML elements:

```xml
  <chain id="chain0" out="00:03:26.763">
    <property name="length">00:03:26.796</property>
    <property name="eof">pause</property>
    <property name="resource">Cielito lindo_2.mp3</property>
    <property name="mlt_service">avformat-novalidate</property>
    <property name="meta.media.nb_streams">1</property>
    <property name="meta.media.0.stream.type">audio</property>
    <property name="meta.media.0.codec.sample_fmt">fltp</property>
    <property name="meta.media.0.codec.sample_rate">48000</property>
    <property name="meta.media.0.codec.channels">2</property>
    <property name="meta.media.0.codec.name">mp3float</property>
    <property name="meta.media.0.codec.long_name">MP3 (MPEG audio layer 3)</property>
    <property name="meta.media.0.codec.bit_rate">173608</property>
    <property name="meta.attr.0.stream.encoder.markup">LAME3.100</property>
    <property name="seekable">1</property>
    <property name="audio_index">0</property>
    <property name="video_index">-1</property>
    <property name="creation_time">2024-01-09T03:42:17</property>
    <property name="astream">0</property>
    <property name="shotcut:skipConvert">1</property>
    <property name="shotcut:hash">93c2fac0ff95295326d7e61a9113c409</property>
    <property name="xml">was here</property>
  </chain>
  <playlist id="playlist0">
    <property name="shotcut:video">1</property>
    <property name="shotcut:name">V1</property>
    <entry producer="chain0" in="00:00:00.000" out="00:00:11.418"/>
  </playlist>
```

The resource defines the file's name in the same folder as `mlt`.

The 3 images are defined like this:

```xml
 <producer id="producer0" in="00:00:00.000" out="04:00:00.000">
    <property name="length">04:00:00.033</property>
    <property name="eof">pause</property>
    <property name="resource">LF2023-12-11 11-07-09 Minja Luciano.jpg</property>
    <property name="ttl">1</property>
    <property name="aspect_ratio">1</property>
    <property name="meta.media.progressive">1</property>
    <property name="seekable">1</property>
    <property name="format">1</property>
    <property name="meta.media.width">4000</property>
    <property name="meta.media.height">3000</property>
    <property name="mlt_service">qimage</property>
    <property name="creation_time">2023-12-11T17:07:14</property>
    <property name="shotcut:skipConvert">1</property>
    <property name="shotcut:hash">49b86054f8a507f8007afebf1204ec95</property>
    <property name="shotcut:caption">LF2023-12-11 11-07-09 Minja Luciano.jpg</property>
    <property name="meta.shotcut.vui">1</property>
    <filter id="filter0" out="00:00:02.970">
      <property name="background">color:#00000000</property>
      <property name="mlt_service">affine</property>
      <property name="shotcut:filter">affineSizePosition</property>
      <property name="transition.fill">1</property>
      <property name="transition.distort">0</property>
      <property name="transition.rect">00:00:00.033=-9.15305 -65.6166 1919.29 1439.47 1;00:00:02.871=-3.44725 -299.554 1919.29 1439.47 1</property>
      <property name="transition.valign">middle</property>
      <property name="transition.halign">center</property>
      <property name="shotcut:animIn">00:00:00.000</property>
      <property name="shotcut:animOut">00:00:00.000</property>
      <property name="transition.threads">0</property>
      <property name="transition.fix_rotate_x">0</property>
    </filter>
  </producer>
  <producer id="producer1" in="00:00:00.000" out="04:00:00.000">
    <property name="length">04:00:00.033</property>
    <property name="eof">pause</property>
    <property name="resource">LF2023-12-15 10-46-45 minja es alta.jpg</property>
    <property name="ttl">1</property>
    <property name="aspect_ratio">1</property>
    <property name="meta.media.progressive">1</property>
    <property name="seekable">1</property>
    <property name="format">1</property>
    <property name="meta.media.width">2268</property>
    <property name="meta.media.height">4032</property>
    <property name="mlt_service">qimage</property>
    <property name="creation_time">2023-12-15T16:46:47</property>
    <property name="shotcut:skipConvert">1</property>
    <property name="shotcut:hash">eb4241e90288fa46456eedb129e23c6d</property>
    <property name="shotcut:caption">LF2023-12-15 10-46-45 minja es alta.jpg</property>
    <property name="meta.shotcut.vui">1</property>
    <filter id="filter1" out="00:00:03.267">
      <property name="background">color:#00000000</property>
      <property name="mlt_service">affine</property>
      <property name="shotcut:filter">affineSizePosition</property>
      <property name="transition.fill">1</property>
      <property name="transition.distort">0</property>
      <property name="transition.rect">00:00:00.330=-331.909 -800 2363.06 4201 1;00:00:02.772=-331.909 -1924.05 2363.06 4201 1</property>
      <property name="transition.valign">middle</property>
      <property name="transition.halign">center</property>
      <property name="shotcut:animIn">00:00:00.000</property>
      <property name="shotcut:animOut">00:00:00.000</property>
      <property name="transition.threads">0</property>
      <property name="transition.fix_rotate_x">0</property>
    </filter>
  </producer>
  <tractor id="tractor0" in="00:00:00.000" out="00:00:00.330">
    <property name="shotcut:transition">lumaMix</property>
    <track producer="producer0" in="00:00:02.640" out="00:00:02.970"/>
    <track producer="producer1" in="00:00:00.000" out="00:00:00.330"/>
    <transition id="transition0" out="00:00:00.330">
      <property name="a_track">0</property>
      <property name="b_track">1</property>
      <property name="factory">loader</property>
      <property name="mlt_service">luma</property>
      <property name="alpha_over">1</property>
      <property name="fix_background_alpha">1</property>
    </transition>
    <transition id="transition1" out="00:00:00.330">
      <property name="a_track">0</property>
      <property name="b_track">1</property>
      <property name="mlt_service">mix</property>
      <property name="start">-1</property>
      <property name="accepts_blanks">1</property>
    </transition>
  </tractor>
  <producer id="producer2" in="00:00:00.000" out="04:00:00.000">
    <property name="length">04:00:00.033</property>
    <property name="eof">pause</property>
    <property name="resource">LF2023-12-13 13-42-18.JPG</property>
    <property name="ttl">1</property>
    <property name="aspect_ratio">1</property>
    <property name="meta.media.progressive">1</property>
    <property name="seekable">1</property>
    <property name="format">1</property>
    <property name="meta.media.width">6000</property>
    <property name="meta.media.height">3376</property>
    <property name="mlt_service">qimage</property>
    <property name="creation_time">2023-12-13T20:42:18</property>
    <property name="shotcut:skipConvert">1</property>
    <property name="shotcut:hash">7c879a105b45925f2bfc779a3b8d0b52</property>
    <property name="shotcut:caption">LF2023-12-13 13-42-18.JPG</property>
    <property name="meta.shotcut.vui">1</property>
    <filter id="filter2" out="00:00:03.267">
      <property name="background">color:#00000000</property>
      <property name="mlt_service">affine</property>
      <property name="shotcut:filter">affineSizePosition</property>
      <property name="transition.fill">1</property>
      <property name="transition.distort">0</property>
      <property name="transition.rect">00:00:00.396=0 0 1919.43 1080 1;00:00:02.970=-479.858 -270 2879.15 1620 1</property>
      <property name="transition.valign">middle</property>
      <property name="transition.halign">center</property>
      <property name="shotcut:animIn">00:00:00.000</property>
      <property name="shotcut:animOut">00:00:00.000</property>
      <property name="transition.threads">0</property>
    </filter>
  </producer>
  <tractor id="tractor1" in="00:00:00.000" out="00:00:00.330">
    <property name="shotcut:transition">lumaMix</property>
    <track producer="producer1" in="00:00:02.937" out="00:00:03.267"/>
    <track producer="producer2" in="00:00:00.000" out="00:00:00.330"/>
    <transition id="transition2" out="00:00:00.330">
      <property name="a_track">0</property>
      <property name="b_track">1</property>
      <property name="factory">loader</property>
      <property name="mlt_service">luma</property>
      <property name="alpha_over">1</property>
      <property name="fix_background_alpha">1</property>
    </transition>
    <transition id="transition3" out="00:00:00.330">
      <property name="a_track">0</property>
      <property name="b_track">1</property>
      <property name="mlt_service">mix</property>
      <property name="start">-1</property>
      <property name="accepts_blanks">1</property>
    </transition>
  </tractor>
  <producer id="producer3" in="00:00:00.000" out="04:00:00.000">
    <property name="length">04:00:00.033</property>
    <property name="eof">pause</property>
    <property name="resource">LF2023-12-14 15-18-14.JPG</property>
    <property name="ttl">1</property>
    <property name="aspect_ratio">1</property>
    <property name="meta.media.progressive">1</property>
    <property name="seekable">1</property>
    <property name="format">1</property>
    <property name="meta.media.width">6000</property>
    <property name="meta.media.height">3376</property>
    <property name="mlt_service">qimage</property>
    <property name="creation_time">2023-12-14T22:18:14</property>
    <property name="shotcut:skipConvert">1</property>
    <property name="shotcut:hash">40f4246ad1988e820f60bddc2ca0713c</property>
    <property name="shotcut:caption">LF2023-12-14 15-18-14.JPG</property>
    <property name="xml">was here</property>
    <property name="meta.shotcut.vui">1</property>
    <filter id="filter3" out="00:00:02.871">
      <property name="background">color:#00000000</property>
      <property name="mlt_service">affine</property>
      <property name="shotcut:filter">affineSizePosition</property>
      <property name="transition.fill">1</property>
      <property name="transition.distort">0</property>
      <property name="transition.rect">00:00:00.363=-479.858 -270 2879.15 1620 1;00:00:02.739=0 0 1919.43 1080 1</property>
      <property name="transition.valign">middle</property>
      <property name="transition.halign">center</property>
      <property name="shotcut:animIn">00:00:00.000</property>
      <property name="shotcut:animOut">00:00:00.000</property>
      <property name="transition.threads">0</property>
    </filter>
  </producer>
  <tractor id="tractor2" in="00:00:00.000" out="00:00:00.297">
    <property name="shotcut:transition">lumaMix</property>
    <track producer="producer2" in="00:00:02.970" out="00:00:03.267"/>
    <track producer="producer3" in="00:00:00.000" out="00:00:00.297"/>
    <transition id="transition4" out="00:00:00.297">
      <property name="a_track">0</property>
      <property name="b_track">1</property>
      <property name="factory">loader</property>
      <property name="mlt_service">luma</property>
      <property name="alpha_over">1</property>
      <property name="fix_background_alpha">1</property>
    </transition>
    <transition id="transition5" out="00:00:00.297">
      <property name="a_track">0</property>
      <property name="b_track">1</property>
      <property name="mlt_service">mix</property>
      <property name="start">-1</property>
      <property name="accepts_blanks">1</property>
    </transition>
  </tractor>
  <playlist id="playlist1">
    <property name="shotcut:video">1</property>
    <property name="shotcut:name">V2</property>
    <property name="shotcut:lock">0</property>
    <entry producer="producer0" in="00:00:00.000" out="00:00:02.607"/>
    <entry producer="tractor0" in="00:00:00.000" out="00:00:00.330"/>
    <entry producer="producer1" in="00:00:00.363" out="00:00:02.904"/>
    <entry producer="tractor1" in="00:00:00.000" out="00:00:00.330"/>
    <entry producer="producer2" in="00:00:00.363" out="00:00:02.937"/>
    <entry producer="tractor2" in="00:00:00.000" out="00:00:00.297"/>
    <entry producer="producer3" in="00:00:00.330" out="00:00:02.871"/>
  </playlist>
```

The `producer` is the video, the `tractor` is the transition. They must be defined before the playlist contains a reference to them.  
The id is changing sequentially like producer0, producer1, producer2,... or tractor0, tractor1, tractor2,...  
This looks like a serialization of internal structures in the code. 
This looks like an alias to "max" `out="04:00:00.000"` or `name="length">04:00:00.033`.  
There are 2 points in time for animation.

First change the size of the image. From the original to the new. 
Then use the position to move the new image size.


First image:
<property name="transition.rect">00:00:00.033=-9.15305 -65.6166 1919.29 1439.47 1;00:00:02.871=-3.44725 -299.554 1919.29 1439.47 1</property>

Start:  
Position -9  -66  
Size 1919    1439  

End:  
Position -3   -300  
Size 1919   1439  

Second image
<property name="transition.rect">00:00:00.330=-331.909 -800 2363.06 4201 1;00:00:02.772=-331.909 -1924.05 2363.06 4201 1</property>

Start:  
Position -332  -800  
Size  2363   4201  

End  
Position -332   -1924  
Size  2363   4201  


The subtitles are defined like this:

```xml
 <producer id="producer4" in="00:00:00.000" out="04:00:00.000">
    <property name="length">04:00:00.033</property>
    <property name="eof">pause</property>
    <property name="resource">#00000000</property>
    <property name="aspect_ratio">1</property>
    <property name="mlt_service">color</property>
    <property name="mlt_image_format">rgba</property>
    <property name="shotcut:caption">transparent</property>
    <property name="ignore_points">0</property>
    <property name="meta.shotcut.vui">1</property>
    <property name="xml">was here</property>
    <property name="seekable">1</property>
    <filter id="filter4" out="00:00:02.607">
      <property name="argument">¡Hola México!</property>
      <property name="geometry">0 900 1920 180 1</property>
      <property name="family">Verdana</property>
      <property name="size">80</property>
      <property name="weight">1000</property>
      <property name="style">normal</property>
      <property name="fgcolour">#ffffffff</property>
      <property name="bgcolour">#00000000</property>
      <property name="olcolour">#aa000000</property>
      <property name="pad">0</property>
      <property name="halign">center</property>
      <property name="valign">bottom</property>
      <property name="outline">3</property>
      <property name="mlt_service">dynamictext</property>
      <property name="shotcut:filter">dynamicText</property>
      <property name="shotcut:usePointSize">1</property>
      <property name="shotcut:pointSize">60</property>
    </filter>
  </producer>
  <producer id="producer5" in="00:00:00.000" out="04:00:00.000">
    <property name="length">04:00:00.033</property>
    <property name="eof">pause</property>
    <property name="resource">#00000000</property>
    <property name="aspect_ratio">1</property>
    <property name="mlt_service">color</property>
    <property name="mlt_image_format">rgba</property>
    <property name="shotcut:caption">transparent</property>
    <property name="ignore_points">0</property>
    <property name="meta.shotcut.vui">1</property>
    <property name="xml">was here</property>
    <property name="seekable">1</property>
    <filter id="filter5" out="00:00:02.541">
      <property name="argument">Minja el signal es 
demasiado Alto por ti</property>
      <property name="geometry">0 900 1920 180 1</property>
      <property name="family">Verdana</property>
      <property name="size">80</property>
      <property name="weight">1000</property>
      <property name="style">normal</property>
      <property name="fgcolour">#ffffffff</property>
      <property name="bgcolour">#00000000</property>
      <property name="olcolour">#aa000000</property>
      <property name="pad">0</property>
      <property name="halign">center</property>
      <property name="valign">bottom</property>
      <property name="outline">3</property>
      <property name="mlt_service">dynamictext</property>
      <property name="shotcut:filter">dynamicText</property>
      <property name="shotcut:usePointSize">1</property>
      <property name="shotcut:pointSize">60</property>
    </filter>
  </producer>
  <producer id="producer6" in="00:00:00.000" out="04:00:00.000">
    <property name="length">04:00:00.033</property>
    <property name="eof">pause</property>
    <property name="resource">#00000000</property>
    <property name="aspect_ratio">1</property>
    <property name="mlt_service">color</property>
    <property name="mlt_image_format">rgba</property>
    <property name="shotcut:caption">transparent</property>
    <property name="ignore_points">0</property>
    <property name="meta.shotcut.vui">1</property>
    <property name="xml">was here</property>
    <property name="seekable">1</property>
    <filter id="filter6" in="00:00:00.165" out="00:00:02.739">
      <property name="argument">Murales es mejor que graffiti</property>
      <property name="geometry">0 900 1920 180 1</property>
      <property name="family">Verdana</property>
      <property name="size">80</property>
      <property name="weight">1000</property>
      <property name="style">normal</property>
      <property name="fgcolour">#ffffffff</property>
      <property name="bgcolour">#00000000</property>
      <property name="olcolour">#aa000000</property>
      <property name="pad">0</property>
      <property name="halign">center</property>
      <property name="valign">bottom</property>
      <property name="outline">3</property>
      <property name="mlt_service">dynamictext</property>
      <property name="shotcut:filter">dynamicText</property>
      <property name="shotcut:usePointSize">1</property>
      <property name="shotcut:pointSize">60</property>
    </filter>
  </producer>
  <producer id="producer7" in="00:00:00.000" out="04:00:00.000">
    <property name="length">04:00:00.033</property>
    <property name="eof">pause</property>
    <property name="resource">#00000000</property>
    <property name="aspect_ratio">1</property>
    <property name="mlt_service">color</property>
    <property name="mlt_image_format">rgba</property>
    <property name="shotcut:caption">transparent</property>
    <property name="ignore_points">0</property>
    <property name="meta.shotcut.vui">1</property>
    <property name="xml">was here</property>
    <property name="seekable">1</property>
    <filter id="filter7" in="00:00:00.165" out="00:00:02.706">
      <property name="argument">Mucho murales en Mexico</property>
      <property name="geometry">0 900 1920 180 1</property>
      <property name="family">Verdana</property>
      <property name="size">80</property>
      <property name="weight">1000</property>
      <property name="style">normal</property>
      <property name="fgcolour">#ffffffff</property>
      <property name="bgcolour">#00000000</property>
      <property name="olcolour">#aa000000</property>
      <property name="pad">0</property>
      <property name="halign">center</property>
      <property name="valign">bottom</property>
      <property name="outline">3</property>
      <property name="mlt_service">dynamictext</property>
      <property name="shotcut:filter">dynamicText</property>
      <property name="shotcut:usePointSize">1</property>
      <property name="shotcut:pointSize">60</property>
    </filter>
  </producer>
  <playlist id="playlist2">
    <property name="shotcut:video">1</property>
    <property name="shotcut:name">V3</property>
    <entry producer="producer4" in="00:00:00.000" out="00:00:02.607"/>
    <blank length="00:00:00.363"/>
    <entry producer="producer5" in="00:00:00.000" out="00:00:02.541"/>
    <blank length="00:00:00.363"/>
    <entry producer="producer6" in="00:00:00.165" out="00:00:02.739"/>
    <blank length="00:00:00.330"/>
    <entry producer="producer7" in="00:00:00.165" out="00:00:02.706"/>
  </playlist>
  ```

## Convert image sizes

Interestingly enough, PhotoFilmStrip and Shotcut have chosen the opposite approach to image size.

In PhotoFilmStrip the original size of the image is stored. Then a rectangle with a proportion of screen 16:9 is drawn inside the image. It represents the screen size. With these two sizes, we can easily understand what part of the image is shown on the screen. The in-drawn rectangle can never be outside the image. There are never black stripes on the screen without the image. Images are sometimes not full on the screen. But that is ok because we have image animation.

In Shotcut they use the opposite approach. First, they adjust the image size to the screen size. In all my cases (16:9 landscape, 9:16 portrait, 4:3) they adjust the size proportionally using the height, not the width. This sometimes causes left and right black stripes on the screen. The image is always full inside the screen but can become very small.  
Then you can use a filter to alter the size and make the image bigger to show only partially on the screen. These are the numbers stored in the data.

The formula for converting the size is simple:

1. PhotoFilmStrip: calculate the ratio of height
ratio=photo_new_height/original_heigh
2. we need the reversed ratio
reversed_ratio = 1/ratio
3. Shotcut: get the size of the image that can be contained in the screen.
 This can be limited by height or by width.
 r1 = 1920/original_width
 r2 = 1080/original_heigh

 if r1 < r2 then
    shot_image_width=original_width*r1
    shot_image_height=original_heigh*r1
else
    shot_image_width=original_width*r2
    shot_image_height=original_heigh*r2
end
4. we use the reversed_ration to calculate the new size in Shotcut
shot_new_width=shot_image_width*reversed_ratio
shot_new_height=shot_image_height*reversed_ratio


Test the formula with these numbers

First image 4:3:
original_width=4000
original_heigh=3000
photo_new_height=2250
pos_x1=0  pos_y1=181
pos_x2= 79 pos_y2=744


  ratio= 2250/3000=0,75
  reversed_ratio= 1/0,75=1,3333

  r1=1920/4000=0,48
  r2=1080/3000=0,36
  r1 !< r2
  shot_image_width=4000*0,36=1440
  shot_image_height=3000*0,36=1080

  shot_new_width=1440*1,3333=1920
  shot_new_height=1080*1,3333=1440

pos_x1=0*-1,3333=0  pos_y1=181*-1,3333=-241
pos_x2= 79*-1,3333=-106 pos_y2=744*-1,3333=-992

Second image is portrait 9:16:
original_width=2268
original_heigh=4032
photo_new_height=1275
pos_x1=0  pos_y1=772
pos_x2=0 pos_y2=1793

  ratio= 1275/4032=0,3162
  reversed_ratio= 1/0,3162=3,1624

  r1=1920/2268=0,8466
  r2=1080/4032=0,2679
  r1 !< r2
  shot_image_width=2268*0,2679=607,5
  shot_image_height=4032*0,2679=1080

  shot_new_width=607,5*3,1624=1920
  shot_new_height=1080*3,16242=3415

pos_x1=0*-3,1624=0  pos_y1=772*-3,1624=-2441
pos_x2= 0*-3,1624=0 pos_y2=1793*-3,1624=-5670

Third image 16:9:
original_width=6000
original_heigh=3376
photo_new_height=2531
pos_x1=0  pos_y1=0
pos_x2=973 pos_y2=802

  ratio= 2531/3376=0,7497
  reversed_ratio= 1/0,7497=1,3339

  r1=1920/6000=0,32
  r2=1080/3376=0,32
  r1 = r2
  shot_image_width=6000*0,32=1920
  shot_image_height=3376*0,32=1080

  shot_new_width=1920*1,3339=2562
  shot_new_height=1080*1,3339=1440

pos_x1=0*-1,3339=0  pos_y1=0*-1,3339=-0
pos_x2= 973*-1,3339=-1298 pos_y2=802*-1,3339=-1070


Fourth image 16:9
original_width=6000
original_heigh=3376
photo_new_height=2531
pos_x1=1500  pos_y1=0
pos_x2=0 pos_y2=0

  ratio= 2531/3376=0,7497
  reversed_ratio= 1/0,7497=1,3339

  r1=1920/6000=0,32
  r2=1080/3376=0,32
  r1 = r2
  shot_image_width=6000*0,32=1920
  shot_image_height=3376*0,32=1080

  shot_new_width=1920*1,3339=2562
  shot_new_height=1080*1,3339=1440

pos_x1=1500*-1,3339=2000  pos_y1=0*-1,3339=-0
pos_x2= 0*-1,3339=0 pos_y2=0*-1,3339=0

## Convert image position

From the image size calculation, we have the reversed_ratio.
We multiply the pos_X and pos_y and make it negative.

## Open-source and free as a beer

My open-source projects are free as a beer (MIT license).  
I just love programming.  
But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer by donating to my [PayPal](https://paypal.me/LucianoBestia).  
You know the price of a beer in your local bar ;-)  
So I can drink a free beer for your health :-)  
[Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻

[//bestia.dev](https://bestia.dev)  
[//github.com/bestia-dev](https://github.com/bestia-dev)  
[//bestiadev.substack.com](https://bestiadev.substack.com)  
[//youtube.com/@bestia-dev-tutorials](https://youtube.com/@bestia-dev-tutorials)  

[//]: # (auto_md_to_doc_comments segment end A)
