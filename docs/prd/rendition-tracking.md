# Feature PRD: Rendition Tracking

## Overview

Rendition quality and rendition changes are an important driver of quality and playback issues. If quality starts off low, switching many times during playback, or errors at a rendition due to segment issues or device capability, it can negatively impact the playback experience for viewers. It is important to track these issues so the issues can be addressed.

## Goals

The data collected should allow for:
- Calculating the initial video/audio renditions that are retrevied and first played
- Visualization of when rendition changes happen during the playback
- Calculation of rendition switching metrics such as number of video rendition up switches, down switches, and total number of rendition switches
- Analysis or which renditions are commonly used and those that are not to make decisions about which renditions may not be worth generating in the future. 
