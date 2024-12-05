---
title: Voreanation
description: Donations web app to solve problem of catered residence students throwing away food they get from DH which they forget to eat or don't get a chance to do so.
tags: Axum, Nextjs, SeaORM, Typescript, Postgres, Rust
date: 2024-12-05
published: true
---

# Voreanation

## Problem description

Its a donations app dedicated to solve the problem of catered residence students throwing away food they get from the DH which intern expires before being eaten.

We discovered that there are students on campus and off campus who usually due to bursary or supplier complications can struggle to find food for themselves while catered students throw away the food that they maybe don't find time to eat.

> Why not give away the food then ?

## The app

Voreanation which translated to giving-nation. This app breaches the gap between students in need and students with ample items to give. The donor donates the food(posts the items they are giving away on the platform) and the donee posts the things they are in need of. An algorithm in the map connects the two people. The donor shares where and when they can meet the donee (_for security this has to be within the campus at specific locations, which are provided by the app_). We will use the student emails for verification(_to be built_).

---

## Project Structure

### Frontend technologies

1. [Next.js(_Typescript_)](https://nextjs.org/) client side rendering.

### Backend technologies

1. [Axum(_Rust_)](https://github.com/tokio-rs/axum)
2. [SeaORM(_Rust_)](https://www.sea-ql.org/SeaORM/)
3. Postgresql

---

## Phase 1 User Stories for an MVP

#### Donee

1. I want to be able to specify my needs at most 3 provided by the drop down.
2. I need to be able to notify the donor that I can make it to the location they specified.
3. I need to be able to notify the donor that I can make it at that time.
4. I need to be able to login.
5. I need to be able to create an account.
6. I need to be able to logout.

#### Donor

1. I need to be able to able to enter the items that I have.
2. I need to be able to delete.
3. I need to be able to notify the status of the donation process.
4. I need to choose location of the drop.
5. I need to be able to create an account.
6. I need to be able to login.
7. I need to be able to logout.

---

- This is a basic or high-level description of the project.
- More features will be added.
- It's not something new but it's something I think is needed on my University campus.
- I don't have any idea on how to monetize this but it's not something that I'm worried about right now since the app hasn't been deployed and has no users yet.

## What motivated me to build this or at least initiate the project ?

- The lack that students have especially those that cook for themselves or stay off campus, the time to prepare food usually takes up their time.
- Students might not have enough money to but food needed to make a well nourished meal for their lunch or supper.
- Food that is usually kept by catered students that goes bad or stale because maybe they didn't want the food or they didn't get the opportunity to give it to someone.

## How many people are working on this ?

- I'm still the only one working on this project but would be open and grateful to have more developers to join me on this project.
