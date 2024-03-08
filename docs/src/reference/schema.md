This schema provides the structure for which all perception streams must adhere to in order to use the STREM tool, accordingly.

## Overview

The schema is structured as a [JSON](https://www.json.org/json-en.html) object populated with all relevant data needed for searching over a perception stream.

!!! example

    A valid perception data stream adhereing to the STREM format which contains a single frame where with one channel that contains two detections would look follows:

    ```json
    {
        "version": "1.0.0",
        "frames": [
            {
                "index": 0,
                "timestamp": 00000012342,
                "samples": [
                    {
                        "channel": "cam::back",
                        "timestamp": 0000131343243423,
                        "image": {
                            "path": "images/00000.png",
                            "dimensions": {
                                "width": 640.0,
                                "height": 480.0
                            }
                        },
                        "annotations": [
                            {
                                "class": "car",
                                "score": 1.00,
                                "bbox": {
                                    "x": 922.065544729849,
                                    "y": 1237.456155890169,
                                    "w": 259.14260407440264,
                                    "h": 291.2843224092133
                                }
                            },
                            {
                                "class": "pedestrian",
                                "score": 0.76,
                                "bbox": {
                                    "x": 1064.6944198405668,
                                    "y": 978.3135518157661,
                                    "w": 156.69087688219645,
                                    "h": 146.64658990256876
                                }
                            },
                        ]
                    }
                ]
            },
        ]
    }
    ```


## Format

The format below is separated into four distinct JSON object literals (assuming curly braces surround each block to form a syntactically valid object).

```json
"version": str,
"frames": [ frame ]
```

```json title="frame"
"index": int,
"timestamp": float,
"samples": [ sample ]
```

```json title="sample"
"channel": str,
"timestamp": float,
"image": {
    "path": str,//(1)!
    "dimensions": {
        "width": float,
        "height": float
    }
},
"annotations": [ annotation ]
```

1. The `path` to the associated image is relative to the JSON file location. If no file provided, it is relative to the working directory from which the `strem` command was invoked.

```json title="annotation"
"class": str,
"score": float,
"bbox": {
    "x": float,//(1)!
    "y": float,//(2)!
    "w": float,
    "h": float
}
```

1. The `x` coordinate represents the left-most boundary of an axis-aligned bounding box.
2. The `y` coordinate represents the top-most boundary of an axis-aligned bounding box.
