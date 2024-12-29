# Fuzzi the Fuzzer

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
