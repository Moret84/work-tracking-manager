# Work tracking manager (wtm)  

**wtm** is a small command-line utility designed to help me track my activity. It is closely related to the procedures in place at my current company regarding activity tracking. Essentially, it can read a YAML file containing information about time spent on certain activities and display them. It can potentially perform sorting or summing operations on this information before presenting it. The software may evolve according to my needs.

## Concepts  

There are three levels of segmentation:

### WorkDuration  

This represents the duration spent on a task, expressed in days, hours, and minutes.  
- A day consists of 7.5 hours, and an hour consists of 60 minutes.

### TrackingRecord  

A record of information related to time spent:
- Associated task ID (backlog ticket numbers can be used), in string format.
- Description, an optional string field that may contain additional information about the record.
- Time spent, in the form of a WorkDuration.

### TrackingDay  

A day characterized by its date and a list of TrackingRecords.
The date is in the format "%A %-d %B %Y", e.g., "lundi 4 mars 2024"

## Usage  


```
wtm [OPTIONS] <INPUT_PATH>

Arguments:
  <INPUT_PATH>  The path of the input file

Options:
  -o, --output-path <OUTPUT_PATH>  The output path where saving the result
  -f, --filter-id <FILTER_ID>      The filter to use for the query
  -r, --remove-empty               Whether to remove days whose tracking is empty or not
  -t, --total                      Whether to compute the total per item or not
  -w, --write                      Whether to write back the result or not
      --show-total                 Whether we should show total next to result
  -h, --help                       Print help
  ```
