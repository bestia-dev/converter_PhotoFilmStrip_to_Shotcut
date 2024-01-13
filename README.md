[//]: # (auto_md_to_doc_comments segment start A)

# converter_PhotoFilmStrip_to_Shotcut  

[//]: # (auto_cargo_toml_to_md start)

**converts PhotoFilmStrip projects to Shotcut projects**  
***version: 2024.112.904 date: 2024-01-12 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/converter_PhotoFilmStrip_to_Shotcut)***  

[//]: # (auto_cargo_toml_to_md end)

 ![status](https://img.shields.io/badge/work_in_progress-yellow)

[//]: # (auto_lines_of_code start)

[//]: # (auto_lines_of_code end)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/converter_PhotoFilmStrip_to_Shotcut/blob/master/LICENSE)

Hashtags: #rustlang #tutorial #sqlite #xml #cli  
My projects on GitHub are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).

## Motivation

PhotoFilmStrip has a great simple UI to make slideshows. I use it to make videos of my travels.  
First I choose a song as a background. That defines the duration of the video.  
Then I add images. PhotoFilmStrip adds some random animations to every image. That's ok.  
The duration of the image is automatically calculated to fill the song duration. Great.  
I then add subtitles to every image. Eventually, I can modify the animation and change the order of the images.  
The UI is so easy, that even my wife likes to use it.  
Finally, PhotoFilmStrip renders the slideshow as a mp4 file and the subtitles as a srt file. That is fine, but...

I prefer to have the subtitles embedded in the video, so every video player will show them exactly how I want it. I encountered a lot of odd problems with separated srt files.  

There are some technics to embed the subtitles into mp4 using ffmpeg or VLC. But I am a programmer and every problem is an excuse to start a new project. In Rust, of course.

For my video editing, I use Shotcut. It is simple and works for me. I would like to export the PhotoFilmStrip project to Shotcut.  
Shotcut can embed "subtitles" into the video and maybe later I could make some video editing that is not possible in PhotoFilmStrip.

So the problem is simple. I need a simple tool to make a one-time conversion from PhotoFilmStrip to Shotcut.  
After that, I can work in Shotcut to modify the video at my will without the limitations of PhotoFilmStrip.

PhotoFilmStrip saves the project in an SQLite database in a pfs file. I could read that with Rust.  
Shotcut saves the projects in XML format in an mlt file. I could write that in Rust.

It is time to discover the elements of these 2 formats and how to transform them.

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
