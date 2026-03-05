# Feature PRD: Viewership Metrics

## Overview

The most basic thing developers need to understand is what videos are being watched, how much, and by how many people. Quality of Experience and other advanced metrics are not as meaningful with that information.

The following data is that most important:
Plays:
- Attempted Plays: the number of times viewers attempted to play a video 
- Plays: the number of times viewers started watching a video (i.e. the video started after they pushed play)
- Watches: the number of times viewers watched a specified amount of the video (could be % of video watched or time-based measure)
Play Time:
- Playback Time: The amount of time videos spent playing
- Watch Time: The amount of time viewers spent watching (or trying to watch) the video, which can include startup time, playback time, rebuffering, etc.)
Viewers:
- Viewers: The number of viewers that attempted to play a video
- Unique Viewers: The number of distinct viewers that attempted to play a video in the specified time range

## Viewer ID

To track the unique viewers, there will need to be a Viewer ID metadata property. This should be a string value to allow the developer to populate the property with a value format of their choosing.

The Viewer ID should default to a string of "v_" + UUID. The developer can override the value, they like. 

The Viewer ID value should be persisted to local storage, using the appropriate storage location for each platform. For example, using a cookie on the web platform. 


## Personas

- Developers: need to be able to understand how many viewers are watching, how many are not able to, and when so they can plan for capacity, understand the impact of disruptions, and measure success.
- Product Managers: Measure adoption and viewership trends over time, identify areas for improvement and issues that may not be directly related to QoE issues
- Technical Leadership: Highlight service KPIs for tracking business health

## Goals

- Ability to track and report on viewership metrics
- Reliable reporting on how many users are watching, what is watched, and how much