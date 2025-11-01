# App Reference
This page will guide you through using the F1FM app.
For more details on the format, such as what a team is, what the different scoring options are, or how ties are broken, see [Format](./format.md).

## Season Creation
To create a new season, first hit the **build new season** button on the home screen of the app. This will open the season editor.

### Teams and Drivers
- Adjust the number of drivers per fantasy team using the **+** and **-** buttons
- Adjust the number of teams in this fantasy season using the **add a team** button, or the delete button
- Type each team name in the **name of team** text box
- Assign each team their starting lineup in the numbered text entry boxes next to the team name

### Season Settings
- Choose the **Score Mode** for this season using the dropdown menu on the left
- Choose the **Draft Mode** for this season using the dropdown menu on the right
- set the number of drivers in the Formula One grid for this season (for scoring purposes)
- set the year to download race results for
- set if driver's can only be drafted by one team at a time using the **Enforce Uniqueness** toggle

### Season Name
- set the name for this season

### Finalizing the Season
The **Build Season** button will be enabled once the season can be created. Each of the following conditions must be met before this will occur:
1. All teams should have all drivers assigned, and names set
2. Score and draft mode should be set
3. If **Enforce Uniqueness** is selected, all driver numbers must be unique

## Season Management

### Navigation
Use the arrow keys or arrow buttons in the bottom corners to switch between rounds in a season.
By default, F1FM loads round 1. Round names are automatically downloaded and displayed at the top if a server connection can be made.

### Main View

### Team Management and Scoring
When a round is eligible for drafting or scoring, the orange button on the bottom row can be selected. There are a number of conditions that must be met first before drafting or scoring can take place. Ensure each condition is met, as the orange button cannot be selected otherwise.

For a round to be draft eligible:
  1. Drafting cannot have already occured
  2. The previous round must have already drafted
  3. All drivers on a team must have raced in that round

For a round to be scoring eligible:
  1. Scoring cannot have already occured
  2. The current round must have already drafted
  3. Round results must be downloaded

If team lineups need to be edited, select the **edit lineup** button.
This allows for trades between teams or temporary swaps if a driver is out for a race to occur.
This button cannot be selected if scoring for the round has already occured or if drafting has not occured.

If round results need to be updated (say a penalty was applied after the race concluded), and scoring potentially redone, select the **delete round** button.
This button will delete downloaded results, and delete any scoring that has taken place if any.
To re-download race results, navigate away from this round and then return, as round results automatically download when a round is loaded.
This button cannot be selected if scoring has not occured and race results are not downloaded.

If a round lineup needs to be deleted, select the **delete lineup** button.
This will delete lineups as if drafting for that round never occured.
This button cannot be selected if scoring has aready occured or if the round is the first round of the season.

### Handling Errors
