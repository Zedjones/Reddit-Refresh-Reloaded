# Reddit Refresh Reloaded
[![GPL licensed](https://img.shields.io/badge/license-GPL-blue.svg)](./LICENSE.md)

Reddit Refresh Reloaded is a web server/frontend which allows users to check a set of subreddits/search queries on a schedule and receive notifications when a new result appears for a search query (using any [Apprise](https://github.com/caronc/apprise) compatible notification service).

## Motivation

The motivation for the original [Reddit Refresh](https://github.com/Zedjones/Reddit-Refresh) was my desire to keep tracking of new posts for particular keyboards/keycaps on [/r/mechmarket](https://old.reddit.com/r/mechmarket/). I also wanted to get some experience with API access since it was much closer to when I first began my CS career.

After a while, I decided to create a number of follow on projects implementing Reddit Refresh in a variety of languages, including Rust and Go. I eventually settled on Go and created a web server/frontend (with the help of my friend [airfork](https://github.com/airfork)) called [Reddit Refresh Online](https://github.com/Zedjones/Reddit-Refresh-Online) using Echo and Materialize.js. 

However, both of these services were limited to Pushbullet being the only notifier supported. Additionally, the results were originally written out to the filesystem. This was eventually transitioned to using a proper database, but the code was fairly poorly organized as I was still early on in my CS career when RROnline as created.

As such, I wanted to take the lessons learned from my initial implementations and make something more fully featured and better organized. 

## Design

The original Reddit Refresh Online implentation was written in Rust, and then re-written in Go once I began collaborating with my friend. However, for Reloaded, I want to go back to Rust for a few reasons. The main ones being saner error handling (including easier error propagation using the `?` operator), better threading safety, easier async/await abstractions for my purposes (compared to Go routines), and overall a more thought out type system. 

I was originally going to use [Rocket](https://rocket.rs) as the web backend, but the async/await work was still very early on when I picked this project up again. As such, I ended up going with [Actix](https://actix.rs/). In the end, it didn't matter too much since the star of the show was going to be [async_graphql](https://github.com/async-graphql/async-graphql), which I used over Juniper at the time since it had better subscription and async support. 

I decided to use GraphQL for this project mainly to enforce strong type safety between the frontend and backend, as well as to make live updating easier. Using genearted TypeScript types on the frontend and [sqlx](https://github.com/launchbadge/sqlx) to do compile-time type checking on my database, I could guarantee type safety from end to end. Going into `frontend` and running `npm run schema:generate` will generate a `schema.graphql` file based on the code-first definitions within the Rust code. Creating a new query/subscription/mutation in `frontend/graphql/queries.graphql` and running `npm run schema:codegen` will generate a `useX` hook that can be accessed directly in React.

Which brings me to my frontend choice: React using TypeScript with [Material UI](https://mui.com/) as the component library. I used React for a few reasons:
- I find the declarative nature and uni-directional data flow of React to be the most intuitive for creating a client.
- I've used React extensively with GraphQL in my current position.
- Material UI is one of the best component libraries for any JavaScript framework right now.

When I first made Reddit Refresh, the only way to receive notifications was through Pushbullet (in fact, the login for RROnline was OAuth through Pushbullet). However, as I started self-hosting my own services, including things like [Gotify](https://gotify.net/) (a notification service) and [Matrix](https://matrix.org/) (a federated chat platform), I quickly realized that having a more generic interface would be better.

Originally, I wrote a Rust trait called `Notifier` that would be implemented for different services. Upon discovering [Apprise](https://github.com/caronc/apprise), however, I pivoted to storing everything as an Apprise configuration on the backend and utilizing it through the [sidecar pattern](https://docs.microsoft.com/en-us/azure/architecture/patterns/sidecar). On the frontend, however, I can create custom forms for different services if I want this to be opaque to the end user.