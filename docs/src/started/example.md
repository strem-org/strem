In this example, we'll download a small perception dataset, convert it to the proper format, and search over it with a couple queries.

## Download Dataset

For this example, we'll be using the [Woven Planet Perception](https://woven.toyota/en/perception-dataset/) dataset which provides perception stream data from autonomous vehicles fleets.

After downloading the dataset, extract the items from the `*.tar` file.

## Convert Dataset Format

The selected dataset follows the [nuScenes Format](https://www.nuscenes.org/nuscenes#data-format), so it must first be converted to the STREM format. To convert the dataset, follow the steps below:

1. Install nuScenes to STREM stream dataset converter ([here](https://github.com/strem-org/stremf)).
2. Follow [README](https://github.com/strem-org/stremf/blob/main/README.md) instructions to perform conversion

## Search Stream

After converting the dataset into the STREM format, we can begin searching for scenarios of interest using [SpRE](../reference/spre.md) patterns.

!!! example "Example 1"

    For this example, we want to find all instances of the scene where a bus is detected from the stream. This can be written as a SpRE pattern as follows:
	
	```console
	[[:bus:]]
	```
	
	To search perception stream for this scenario, run:
	
	```console
	$ strem '[[:bus:]]' /path/to/scene.json
	```

!!! example "Example 2"

    We can also provide more complex queries that capture sequences of events (i.e., temporal-based). For example, consider searching for frames that contain a bus followed by frames with no bus (i.e., the bus should leave the view). This can be written as a SpRE pattern as follows:
	
	```python
	[[:bus:]]{1,}[![:bus:]]
	```
	
	To search the perception stream for this scenario, run:
	
	```console
	$ strem '[[:bus:]]{1,}[![:bus:]]' /path/to/scene.json #(1)!
	```
	
	1. The `!` should be used cautiously as some shells interpret it as history expansion. Therefore, you may either enclose the query with single quotes (`'`) or escape it, accordingly.

!!! example "Example 3"

    An important feature of SpRE patterns is the ability to match spatial scenarios. For example, consider the scenario where we want to find the first longest sequence of frames where a pedestrian overlaps with a car. This can be written as a SpRE pattern as follows:
	
	```console
	[<nonempty>([:pedestrian:] & [:car:])]*
	```
	
	To search the perception stream for this scenario, run:
	
	```console
	$ strem --max-count=1 '[<nonempty>([:pedestrian:] & [:car:])]*' /path/to/scene.json
	```
