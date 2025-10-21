# Format
This page describes the format of F1FM. The details of how drafting, scoring, and editing lineups takes place is described here. For information on how to navigate the F1FM app, see [App Reference](./app-reference.md).

## Teams
Each team has a set number of drivers that score points for that team.
The number of drivers per team can be configured by the league manager, though the number of drivers per team must be consistent across rounds and must be the same for each team.

## Drafting
Drafting is how teams select drivers for their team.
League managers can configure if drivers are "unique", meaning each driver is allowed to be drafted to only one team at only one time or not, when creating a season.
League managers can also configure when drafting occurs, and who is replaced when drafting by selecting the draft strategy at when creating a season.
The draft strategies currently in F1FM are:

| Draft Strategy  | When Drafting Occurs            | Who is Replaced                  |
|-----------------|---------------------------------|----------------------------------|
| **Skip**        | At the start of the season only | all drivers                      |
| **Replace All** | After each round                | all drivers                      |
| **Roll On**     | After each round                | only the last driver in a lineup |

> In order to draft lineups, drafting must have taken place for the previous round

## Editing Lineups
There are a number of reasons that league managers may need to edit the lineup of a team outside of a season's draft.
For one, real drivers may occasionally be replaced long term or short term due to sickness, injury, or performance, leaving fantasy teams with a non-racing driver.
For another, a league manager may want to facilitate driver trades between teams mid-season.
To account for these cases, F1FM allows a league manager to edit the lineup of any team.
A league manager can also delete all team's lineups for a given round, as if drafting never took place.

> In order to edit or delete lineups, drafting must have taken place for the round being edited, but scoring for the current round and drafting for the next round must not have taken place.

## Scoring
Scoring is how points are assigned to teams for each round.
Scoring is based on the round results that are downloaded automatically by F1FM.
Once downloaded, the race results are saved so that point calculations never change.
If Formula One standings change (because a penalty is imposed after the race, for example) and the league manager wants to update the race results, they can delete the standings and re-score the race.
League managers can configure how points are calculated according to a scoring strategy.
The draft strategies currently in F1FM are:

| Scoring Strategy  | How Points are Calculated                                                                                                                            | Examples for a 20 driver grid                                                                                                                                 |
|-------------------|------------------------------------------------------------------------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------|
| **Formula One**   | Points are awarded to each driver following the current F1 points distribution. No points are awarded for fastest lap, or sprint race results.       | 1st place gets 25 points, 2nd place gets 18, etc                                                                                                              |
| **Race Position** | Each driver earns one more point than the driver that finished before them, with the last place driver earning 1 point                               | 1st place gets 20 points, 2nd place gets 19, etc                                                                                                              |
| **Improvement**   | Same as Race Position, but drivers also get one point added or taken away for every positioned they gained or lost from their starting grid position | 1st place starting 1st on the grid gets 20 points, 1st place starting 5th on the grid gets 24 points, 15th place starting 1st on the grid gets -8 points, etc |
| **Domination**    | Same as Race Position, but drivers also get points for their starting grid position following the same rules as Race Position                        | 1st place starting 1st on the grid gets 40 points, 1st place starting 5th on the grid gets 36 points, 15th place starting 1st on the grid gets 26 points, etc |

> In order to score a round, drafting must have taken place and the round results need to be downloaded

### Breaking Ties

There are no ties in the F1FM season rankings, as there is always a way to break ties if one scoring method results in a tie.
Every time an ordering is needed, either for showing the results of a race or the current race standings, all teams are first considered tied, and then ties are broken by the first row which differentiates a team, according to the following list.

1. Total points
2. Highest scoring round
3. Lowest scoring round (more points beats less points)
4. Order in which the team was entered at the season creation

> As a consequence of these ordering rules, teams should always be created in tiebreaker order when a season is created.
