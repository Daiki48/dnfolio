---
title: 29 November 2024 Daily Report
description: This is the daily report for 29 November 2024.
tags:
  - Diary
  - Deno
  - TypeScript
  - Rust
---

Today, I was developing a daily report management system for my work.
My goal is to have a system that extracts and manages the day's work data from the various services I use at work.
I planned to use **Google Apps Script** when I started developing.
However, if anyone other than me uses the CLI, they may not use Google, so I decided to develop the CLI with Deno, taking into account the case of users who do not use Google.
I and people around me are Windows users.
So, it is a prerequisite that the CLI runs on Windows.
To complete this system, I began studying Deno.

[Deno](https://deno.com) has a cute mascot character.

And the service is excellent.

This is why I like Deno.

- It can be developed quickly.
- The official documentation is extensive.
- Easy CLI development.

I had other candidates besides Deno to develop this daily report management system.

- Develop CLI with Rust.
- Develop desktop applications using Electron or Tauri.
- Develop Web applications using Qwik.

The reason I did not develop the CLI in Rust is because I am not technical enough.

The service I am using provides a Web API.
I have considered [reqwest](https://crates.io/crates/reqwest) crate in Rust to use its API.
However, the expected behavior could not be achieved.
I can understand the basic syntax of Rust.
However, when I checked the reqwest documentation, I could not understand the content...
Later, when I moved from Rust to Deno, I was able to implement it quickly.
In other words, I don't understand **Rust**, but I can understand **TypeScript** .

Before implementing the system in the CLI, I also considered the idea developing the daily report management system as a Web or desktop application.
For desktop application development, I considered **Electron** or **Tauri** .
The requirement for desktop applications is to reduce size.
If developed in Electron, it was rejected because of its potentially large size.
I wanted to adopt Electron because I can develop with TypeScript.
Tauri was recently upgraded to v2.
Tauri is a very attractive framework.
However Tauri rejected the CLI for the same reason it changed from Rust to Deno.
If I gain enough technical skills to be able to hire Rust, I would like to hire Tauri.

The idea of developing a web application with Qwik came about because I was interested in Qwik.
The reason is that it is a web framework that I have never used before.
Qwik is rumored to perform better than Svelte, which I like.
However, it was not accepted because I could not find the time to study Qwik.
I use it when I have time in my private life.

Today, I spent the day just preparing to develop a daily report management system.
Tomorrow, I'm off work.
So I start building a daily report management system.
Looking forward to it!!
