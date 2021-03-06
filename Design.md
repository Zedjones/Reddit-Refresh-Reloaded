# Ideas for revamping Reddit Refresh

## Database
 - Store all previous results in the database, which provides these benefits: 
   - Can find the most recent by simply sorting by insertion date
   - Allows us to display history of results on web interface in case a notification gets deleted
 - Tables
   - Users
     - JWT Token
     - Email (or username)
     - Hashed Password
     - Refresh time
     - Any notifier specific configuration options
   - Searches
     - ID
     - Username
     - Subreddit
     - Search term
   - Results
     - Time inserted
     - Search ID
     - Title

## Notifiers
 - Instead of being totally tied to Pushbullet, or Gotify, or any other notification service, make a `Notifier` trait which simply implements a function to send a notification. This could write the notification to a file, or use a service, or write to stdout.
 - Using `dyn Notifier`, we can pass these around and store them in our global state.

## Handling Changes
 - Instead of using a REST API, using GraphQL might make it easier to update searches.
    - Additionally, if we display previous results, subscriptions will make this easier to do.

## Authentication
 - Since we're not relying on Pushbullet, we'll have to figure out how to do authentication without that OAuth.
   - JWT tokens will probably be the easiest for our needs

## Display of searches and results
 - Show subreddit and some searches on card, and then in Dialog show searches, as well as a dropdown for results