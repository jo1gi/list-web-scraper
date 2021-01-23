# Spanreed
Easy web scraping of lists

## Usage
Spanreed uses yaml files that describes which sites should be scraped and what
information from those sites.

Structure of config files
```yaml
sites:
  - name: "Bar"
    url: "https://bar.com"
    structure:
      CONTAINER:
        selector: ".foo"
      element:
        selector: ".buzz"
      value_example:
        value: "bar"
```

`sites` contains a list of sites to scrape. Spanreed finds a list of elements on
each site and finds data from each of those elements. The elements are described
with the `CONTAINER` option. The other elements under `structure` describes what
information should be scraped from each element. Both the `CONTAINER` and the
elements uses css selector to choose which elements should be scraped. The
information elements can also contain other options to further describe what
should be scraped.

### Information options

| Option   | Description                                                 |
|----------|-------------------------------------------------------------|
| selector | A css selector to find the correct element                  |
| value    | A static value                                              |
| index    | Index in the list of found elements from the selector tag   |
| get      | Which data from the found tag to use (default: text)        |
| prefix   | A string to put before the rest of the output               |
| suffix   | A string to put after the rest of the output                |
| trim     | A boolean that chooses if the text output should be trimmed |
