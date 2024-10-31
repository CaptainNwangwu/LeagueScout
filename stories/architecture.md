# Key Components

## Client Interface

//

- Web App to input player name and trigger scouting (Expand this functionality to include an entire team)

## API Wrapper

- Component responsible for sending requests to Riot API to fetch player info and match data.

## Champion Pool Analyzer

- Service / Component responsible for processing raw match data and determines a player's champion pool based on criteria such as: Frequency, performance metrics, mastery points, etc.

## Database

- Store historical data about players, champions, and performance for faster access + future analysis. Should also assist in storing competitive league data for lookup purposes.

## Output / Display

- Component that displays analyzed data (likely in a tabular format)
