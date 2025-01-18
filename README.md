# The Best Fuzzer

```
      Yonatan Reicher              &            Tomer Melnik
yreicher@campus.technion.ac.il           tomermelnik@campus.technion.ac.il
        214762718                                  322212291
```

## Installation

### Using Docker
To build the fuzzer, run the following command:
```
docker-compose build
```
To run the fuzzer, run the following command:
```
docker-compose up
```
To stop the fuzzer, run the following command:
```
docker-compose down
```

If you want to run the fuzzer in a detached mode, run the following command:
```
docker-compose up -d
```
If you want to run the fuzzer with a specific command, run the following command:
```
docker-compose run --rm fuzzer-sandbox <command>
```

## Unspecified Behaviour

## Resources Used

This list of strings is representetive of cases that are likely to cause issues
which would be improbable to generate randomly. We thought of using a list of
predefined edge cases in inspiration of `kotest`, the property testing library.
https://github.com/danielmiessler/SecLists/blob/master/Fuzzing/big-list-of-naughty-strings.txt

We also read about various fuzzing techniques, which mostly involved testing
which branches of the code are executed, or looking at the output, but we do
not have access to the program's source code and it was explicitly stated that
the output is not relevant, so we could not use these techniques.
