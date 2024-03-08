A Spatial Regular Expression (SpRE) is a spatial- and temporal-based pattern that describes a perception scenario of interest. This querying language is based on traditional Regular Expression patterns found in popular tools such as [grep](https://www.gnu.org/software/grep/manual/grep.html).

!!! example

    Find two to five frames where a car and pedestrian are detected.

    ```
    [[:car:] & [:pedestrian:]]{2,5}
    ```

## Grammar

The grammar below provides a method for developing valid SpRE patterns.

```
<spre>   ::= '(' <spre> ')'
         | <spre> '*'
         | <spre> <spre>
         | <spre> '|' <spre>
         | <spre> <range>
         | '[' <s4u> ']'
       
<s4u>    ::= '(' <s4u> ')'
         | '!' <s4u>
         | <s4u> '&' <s4u>
         | <s4u> '|' <s4u>
         | '<nonempty>' <class>
         | '<nonempty>' '(' <s4> ')'
         | <class>

<s4>     ::= '(' <s4> ')'
         | <s4> '&' <s4>
         | <s4> '|' <s4>
         | <class>

<class>  ::= <object>

<object> ::= '[' ':' <string> ':' ']'

<range>  ::= '{' <integer> '}'
         | '{' <integer> ',' '}'
         | '{' <integer> ',' <integer> '}'
```

The `<string>` and `<integer>` rules follow C-like standards for valid tokens. For additional examples of SpRE patterns, see [here](https://github.com/strem-org/strem/tree/main/examples).
